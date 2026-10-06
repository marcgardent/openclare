//! Minimal Wavefront OBJ reader/writer (positions and faces only).

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use super::{PolyMesh, TriMesh};
use crate::error::{Error, Result};
use crate::math::Vec3;

pub fn read_obj(path: impl AsRef<Path>) -> Result<TriMesh> {
    parse_obj(BufReader::new(File::open(path)?))
}

pub fn read_obj_poly(path: impl AsRef<Path>) -> Result<PolyMesh> {
    parse_obj_poly(BufReader::new(File::open(path)?))
}

/// Parses `v` and `f` records; polygons are fan-triangulated, everything else
/// (normals, UVs, groups, materials) is ignored.
pub fn parse_obj(reader: impl BufRead) -> Result<TriMesh> {
    Ok(parse_obj_poly(reader)?.triangulated())
}

/// Like [`parse_obj`], keeping the polygons as they are.
pub fn parse_obj_poly(reader: impl BufRead) -> Result<PolyMesh> {
    let mut mesh = PolyMesh::default();

    for (line_index, line) in reader.lines().enumerate() {
        let line = line?;
        let error = |message: &str| Error::Parse { line: line_index + 1, message: message.into() };
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some("v") => {
                let mut coord = || -> Result<f64> {
                    tokens.next().and_then(|t| t.parse().ok()).ok_or_else(|| error("expected three vertex coordinates"))
                };
                mesh.positions.push(Vec3::new(coord()?, coord()?, coord()?));
            }
            Some("f") => {
                let mut polygon = Vec::new();
                for token in tokens {
                    // "v", "v/vt", "v//vn" or "v/vt/vn"; indices are 1-based,
                    // negative ones count back from the last vertex read.
                    let index: i64 = token
                        .split('/')
                        .next()
                        .and_then(|t| t.parse().ok())
                        .ok_or_else(|| error("malformed face index"))?;
                    let count = mesh.positions.len() as i64;
                    let resolved = if index < 0 { count + index } else { index - 1 };
                    if !(0..count).contains(&resolved) {
                        return Err(error("face index out of range"));
                    }
                    polygon.push(resolved as u32);
                }
                if polygon.len() < 3 {
                    return Err(error("face with fewer than three vertices"));
                }
                mesh.faces.push(polygon);
            }
            _ => {}
        }
    }
    Ok(mesh)
}

pub fn write_obj_poly(path: impl AsRef<Path>, mesh: &PolyMesh) -> Result<()> {
    let mut out = BufWriter::new(File::create(path)?);
    write_obj(&mut out, &mesh.positions, mesh.faces.iter().map(Vec::as_slice))
}

pub fn write_obj_tri(path: impl AsRef<Path>, mesh: &TriMesh) -> Result<()> {
    let mut out = BufWriter::new(File::create(path)?);
    write_obj(&mut out, &mesh.positions, mesh.triangles.iter().map(|t| t.as_slice()))
}

pub fn write_obj<'a>(out: &mut impl Write, positions: &[Vec3], faces: impl Iterator<Item = &'a [u32]>) -> Result<()> {
    for p in positions {
        writeln!(out, "v {} {} {}", p.x, p.y, p.z)?;
    }
    for face in faces {
        write!(out, "f")?;
        for v in face {
            write!(out, " {}", v + 1)?;
        }
        writeln!(out)?;
    }
    out.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_polygons_slashes_and_relative_indices() {
        let text = "# quad\nv 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nvn 0 0 1\nf 1/1/1 2//1 3/3 -1\n";
        let mesh = parse_obj(text.as_bytes()).unwrap();
        assert_eq!(mesh.positions.len(), 4);
        assert_eq!(mesh.triangles, vec![[0, 1, 2], [0, 2, 3]]);
    }

    #[test]
    fn rejects_out_of_range_indices() {
        assert!(parse_obj("v 0 0 0\nf 1 2 3\n".as_bytes()).is_err());
    }

    #[test]
    fn round_trips() {
        let mesh = crate::mesh::primitives::icosphere(1);
        let mut buffer = Vec::new();
        write_obj(&mut buffer, &mesh.positions, mesh.triangles.iter().map(|t| t.as_slice())).unwrap();
        let back = parse_obj(buffer.as_slice()).unwrap();
        assert_eq!(back.triangles, mesh.triangles);
        assert_eq!(back.positions, mesh.positions);
    }
}
