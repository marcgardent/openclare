//! Texture coordinates that come with a quad mesh for free: its own grid.
//!
//! A quad mesh is, almost everywhere, a piece of squared paper. Laying a
//! quad on a unit square and unfolding its neighbours next to it, edge by
//! edge, gives every corner a place on a whole-number grid, with no
//! stretching at all: this is the map the chain worked in, read back from
//! the mesh it made. Round an irregular vertex the paper does not close up,
//! and somewhere the unfolding has to be cut.
//!
//! Two layers come out of it, one pair of coordinates per corner of each
//! face, face after face:
//!
//! - the **grid**: one unfolding per connected piece, each quad a unit
//!   square. Stripes and checkers drawn with it follow the edges across the
//!   whole surface, but parts of it land on top of each other: it is for
//!   patterns that repeat, not for painting.
//! - the **atlas**: the same unfolding, stopped wherever it would land on
//!   ground already taken and started again from there. Each island is a
//!   patch of squares that covers nothing else, and the islands are packed
//!   into the unit square: this one can be painted on. Its cuts are the
//!   seams.
//!
//! In both, a quad takes the same room whatever its size on the surface:
//! where quads are smaller, the texture is finer.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::mesh::PolyMesh;

/// What [`unfold`] gives.
#[derive(Clone, Debug, Default)]
pub struct Layers {
    /// Per corner, in whole numbers of quads.
    pub grid: Vec<[f64; 2]>,
    /// Per corner, within the unit square.
    pub atlas: Vec<[f64; 2]>,
    /// Edges, as their two vertices, where the atlas is cut.
    pub seams: Vec<[u32; 2]>,
    /// How many islands the atlas has.
    pub islands: usize,
}

/// The corners of a unit cell, counter-clockwise from its origin.
const CORNERS: [[i64; 2]; 4] = [[0, 0], [1, 0], [1, 1], [0, 1]];
/// The cell across each side of a cell, side `s` running from corner `s`.
const ACROSS: [[i64; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];
/// About how many islands it takes to cross the atlas: an island reaches no
/// farther from where it started than its share of that.
const ISLANDS_ACROSS: f64 = 6.0;
/// However small the mesh, an island may reach this many quads: a mesh of
/// a few hundred quads is not worth cutting up.
const MIN_REACH: i64 = 16;
/// Room left round the atlas, in quads. Between two islands there is always
/// one quad's width.
const MARGIN: f64 = 0.5;

/// Where a quad sits in an unfolding: its cell, and which of the cell's
/// corners its first corner is.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Place {
    cell: [i64; 2],
    turn: usize,
}

impl Place {
    fn corner(self, k: usize) -> [i64; 2] {
        let [x, y] = CORNERS[(k + self.turn) % 4];
        [self.cell[0] + x, self.cell[1] + y]
    }

    /// Where the quad across edge `k` (from corner `k` to the next) sits, if
    /// that edge is its own edge `j`.
    fn across(self, k: usize, j: usize) -> Place {
        let side = (k + self.turn) % 4;
        let [dx, dy] = ACROSS[side];
        Place { cell: [self.cell[0] + dx, self.cell[1] + dy], turn: (side + 2 + 4 - j) % 4 }
    }
}

/// Unfolds `mesh` into its two layers of texture coordinates.
pub fn unfold(mesh: &PolyMesh) -> Layers {
    let first_corner: Vec<usize> = mesh
        .faces
        .iter()
        .scan(0, |next, face| {
            let first = *next;
            *next += face.len();
            Some(first)
        })
        .collect();
    let corners: usize = mesh.faces.iter().map(Vec::len).sum();

    // Which face has the edge from `a` to `b`, and as which of its edges.
    let mut edge_of: HashMap<(u32, u32), (usize, usize)> = HashMap::new();
    for (f, face) in mesh.faces.iter().enumerate().filter(|(_, face)| face.len() == 4) {
        for k in 0..4 {
            edge_of.insert((face[k], face[(k + 1) % 4]), (f, k));
        }
    }
    let neighbour = |f: usize, k: usize| {
        let face = &mesh.faces[f];
        edge_of.get(&(face[(k + 1) % 4], face[k])).copied()
    };

    // The grid: every quad placed once, by the first to reach it.
    let mut grid = vec![[0.0; 2]; corners];
    let mut placed: Vec<Option<Place>> = vec![None; mesh.faces.len()];
    for seed in (0..mesh.faces.len()).filter(|&f| mesh.faces[f].len() == 4) {
        if placed[seed].is_some() {
            continue;
        }
        placed[seed] = Some(Place { cell: [0, 0], turn: 0 });
        let mut queue = VecDeque::from([seed]);
        while let Some(f) = queue.pop_front() {
            let place = placed[f].expect("queued once placed");
            for k in 0..4 {
                if let Some((g, j)) = neighbour(f, k)
                    && placed[g].is_none()
                {
                    placed[g] = Some(place.across(k, j));
                    queue.push_back(g);
                }
            }
        }
    }
    // Each piece from zero up, so that numbers stay small.
    let mut lowest: HashMap<usize, [i64; 2]> = HashMap::new();
    let piece = pieces_of(mesh, &edge_of);
    for (f, place) in placed.iter().enumerate() {
        if let Some(place) = place {
            let low = lowest.entry(piece[f]).or_insert(place.cell);
            *low = [low[0].min(place.cell[0]), low[1].min(place.cell[1])];
        }
    }
    for (f, place) in placed.iter().enumerate() {
        match place {
            Some(place) => {
                let low = lowest[&piece[f]];
                for k in 0..4 {
                    let [x, y] = place.corner(k);
                    grid[first_corner[f] + k] = [(x - low[0]) as f64, (y - low[1]) as f64];
                }
            }
            None => lay_polygon(&mut grid[first_corner[f]..][..mesh.faces[f].len()], [0.0, 0.0], 1.0),
        }
    }

    // The atlas: islands that never land on ground already taken, and that
    // stay within a square of their own. Left to itself an unfolding sprawls
    // (a cube opens into one long strip of six faces), and sprawling islands
    // leave most of an atlas empty.
    let quads = mesh.faces.iter().filter(|face| face.len() == 4).count();
    let reach = (((quads as f64).sqrt() / ISLANDS_ACROSS).round() as i64).max(MIN_REACH);
    let mut island_of: Vec<usize> = vec![usize::MAX; mesh.faces.len()];
    let mut place_in: Vec<Place> = vec![Place { cell: [0, 0], turn: 0 }; mesh.faces.len()];
    let mut boxes: Vec<[i64; 4]> = Vec::new();
    for seed in 0..mesh.faces.len() {
        if island_of[seed] != usize::MAX {
            continue;
        }
        let island = boxes.len();
        island_of[seed] = island;
        let mut bounds = [0, 0, 0, 0];
        if mesh.faces[seed].len() == 4 {
            let mut taken: HashSet<[i64; 2]> = HashSet::from([[0, 0]]);
            // Which vertex of the mesh sits at each corner of the paper. One
            // place, one vertex: two quads side by side on the paper are
            // then side by side on the surface, and four round a corner of
            // the paper are round one vertex. Without it a quad can land
            // next to one it has nothing to do with, round an irregular
            // vertex say, with no room between them; and what is painted or
            // baked there is read by both along the edge they seem to share.
            let mut vertex_at: HashMap<[i64; 2], u32> = HashMap::new();
            for k in 0..4 {
                vertex_at.insert(place_in[seed].corner(k), mesh.faces[seed][k]);
            }
            let mut queue = VecDeque::from([seed]);
            while let Some(f) = queue.pop_front() {
                let place = place_in[f];
                for k in 0..4 {
                    let Some((g, j)) = neighbour(f, k) else { continue };
                    let there = place.across(k, j);
                    let within = there.cell.iter().all(|c| c.abs() <= reach);
                    let agrees =
                        (0..4).all(|c| vertex_at.get(&there.corner(c)).is_none_or(|&vertex| vertex == mesh.faces[g][c]));
                    if island_of[g] == usize::MAX && within && agrees && taken.insert(there.cell) {
                        for c in 0..4 {
                            vertex_at.insert(there.corner(c), mesh.faces[g][c]);
                        }
                        island_of[g] = island;
                        place_in[g] = there;
                        let [x, y] = there.cell;
                        bounds = [bounds[0].min(x), bounds[1].min(y), bounds[2].max(x), bounds[3].max(y)];
                        queue.push_back(g);
                    }
                }
            }
        }
        boxes.push(bounds);
    }

    // Packing: the islands are fitted into one sheet of squared paper cell by
    // cell, the largest first, each at the first place where none of its
    // cells is taken or touches one that is. They are odd shapes, and boxes
    // round them would leave most of the sheet empty.
    let mut cells_of: Vec<Vec<[usize; 2]>> = vec![Vec::new(); boxes.len()];
    for f in 0..mesh.faces.len() {
        let (island, low) = (island_of[f], boxes[island_of[f]]);
        let cell = place_in[f].cell;
        cells_of[island].push([(cell[0] - low[0]) as usize, (cell[1] - low[1]) as usize]);
    }
    let width_of = |island: usize| (boxes[island][2] - boxes[island][0] + 1) as usize;
    let total: usize = cells_of.iter().map(Vec::len).sum();
    let widest = (0..boxes.len()).map(width_of).max().unwrap_or(1);
    let mut order: Vec<usize> = (0..boxes.len()).collect();
    order.sort_by(|&a, &b| cells_of[b].len().cmp(&cells_of[a].len()).then(a.cmp(&b)));
    // How wide a sheet packs best is not known beforehand: a few are tried,
    // and the one that ends up nearest to a square is kept.
    let pack = |sheet: usize| {
        // Rows of the sheet, added as needed: true where a cell is taken or is
        // next to one that is.
        let mut closed: Vec<Vec<bool>> = Vec::new();
        let mut corner_of = vec![[0usize; 2]; boxes.len()];
        let mut first_free_row = 0;
        for &island in &order {
            let cells = &cells_of[island];
            let fits = |closed: &Vec<Vec<bool>>, x: usize, y: usize| {
                cells.iter().all(|&[cx, cy]| closed.get(y + cy).is_none_or(|row| !row[x + cx]))
            };
            let mut found = None;
            'rows: for y in first_free_row.. {
                for x in 0..=sheet - width_of(island) {
                    if fits(&closed, x, y) {
                        found = Some([x, y]);
                        break 'rows;
                    }
                }
            }
            let [x, y] = found.expect("a row past the last one is always free");
            corner_of[island] = [x, y];
            for &[cx, cy] in cells {
                let (at_x, at_y) = (x + cx, y + cy);
                while closed.len() < at_y + 2 {
                    closed.push(vec![false; sheet]);
                }
                for row in &mut closed[at_y.saturating_sub(1)..=at_y + 1] {
                    for taken in &mut row[at_x.saturating_sub(1)..=(at_x + 1).min(sheet - 1)] {
                        *taken = true;
                    }
                }
            }
            // Rows that are full are not worth looking at again.
            while closed.get(first_free_row).is_some_and(|row| row.iter().all(|&taken| taken)) {
                first_free_row += 1;
            }
        }
        (corner_of, sheet.max(closed.len().saturating_sub(1)).max(1))
    };
    let (corner_of, side) = [1.05, 1.2, 1.35, 1.5, 1.7]
        .into_iter()
        .map(|room: f64| pack(((room * total as f64).sqrt().ceil() as usize).max(widest)))
        .min_by_key(|&(_, side)| side)
        .expect("there are widths to try");
    // Half a cell of margin round the sheet.
    let extent = side as f64 + 2.0 * MARGIN;
    let corner_of: Vec<[f64; 2]> = corner_of.iter().map(|c| [c[0] as f64 + MARGIN, c[1] as f64 + MARGIN]).collect();

    let mut atlas = vec![[0.0; 2]; corners];
    for (f, face) in mesh.faces.iter().enumerate() {
        let (island, low) = (island_of[f], boxes[island_of[f]]);
        let origin = corner_of[island];
        if face.len() == 4 {
            for k in 0..4 {
                let [x, y] = place_in[f].corner(k);
                atlas[first_corner[f] + k] =
                    [(origin[0] + (x - low[0]) as f64) / extent, (origin[1] + (y - low[1]) as f64) / extent];
            }
        } else {
            let slot = &mut atlas[first_corner[f]..][..face.len()];
            lay_polygon(slot, [origin[0] / extent, origin[1] / extent], 1.0 / extent);
        }
    }

    // A seam is an edge whose two sides do not meet in the atlas.
    let mut seams = Vec::new();
    for (&(a, b), &(f, k)) in &edge_of {
        if a < b || !edge_of.contains_key(&(b, a)) {
            let Some((g, j)) = neighbour(f, k) else { continue };
            let meets = island_of[f] == island_of[g]
                && place_in[f].corner(k) == place_in[g].corner((j + 1) % 4)
                && place_in[f].corner((k + 1) % 4) == place_in[g].corner(j);
            if !meets && a < b {
                seams.push([a, b]);
            } else if !meets && !edge_of.contains_key(&(b, a)) {
                seams.push([b, a]);
            }
        }
    }
    seams.sort_unstable();
    seams.dedup();
    Layers { grid, atlas, seams, islands: boxes.len() }
}

/// A face that is not a quad, as a regular polygon inside a cell of side
/// `side` at `origin`.
fn lay_polygon(slot: &mut [[f64; 2]], origin: [f64; 2], side: f64) {
    let count = slot.len();
    for (k, uv) in slot.iter_mut().enumerate() {
        let turn = std::f64::consts::TAU * k as f64 / count as f64;
        *uv = [origin[0] + side * (0.5 + 0.5 * turn.cos()), origin[1] + side * (0.5 + 0.5 * turn.sin())];
    }
}

/// The connected piece each face belongs to, through the edges quads share.
fn pieces_of(mesh: &PolyMesh, edge_of: &HashMap<(u32, u32), (usize, usize)>) -> Vec<usize> {
    let mut piece = vec![usize::MAX; mesh.faces.len()];
    for seed in 0..mesh.faces.len() {
        if piece[seed] != usize::MAX {
            continue;
        }
        piece[seed] = seed;
        let mut stack = vec![seed];
        while let Some(f) = stack.pop() {
            let face = &mesh.faces[f];
            if face.len() != 4 {
                continue;
            }
            for k in 0..4 {
                if let Some(&(g, _)) = edge_of.get(&(face[(k + 1) % 4], face[k]))
                    && piece[g] == usize::MAX
                {
                    piece[g] = seed;
                    stack.push(g);
                }
            }
        }
    }
    piece
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::primitives;

    /// Twice the signed area of a quad of the layer.
    fn turn_of(uv: &[[f64; 2]]) -> f64 {
        (0..4)
            .map(|k| {
                let (a, b) = (uv[k], uv[(k + 1) % 4]);
                a[0] * b[1] - a[1] * b[0]
            })
            .sum()
    }

    #[test]
    fn every_quad_is_a_unit_square_of_the_grid() {
        for mesh in [primitives::cube_quads(5), primitives::torus_quads(1.0, 0.4, 12, 8), primitives::plane_quads(6)] {
            let layers = unfold(&mesh);
            assert_eq!(layers.grid.len(), 4 * mesh.faces.len());
            for quad in layers.grid.chunks_exact(4) {
                assert!((turn_of(quad) - 2.0).abs() < 1e-12, "{quad:?}");
                assert!(quad.iter().flatten().all(|&c| c >= 0.0 && c == c.round()));
            }
        }
    }

    #[test]
    fn a_flat_grid_is_one_island_without_a_seam() {
        let mesh = primitives::plane_quads(6);
        let layers = unfold(&mesh);
        assert_eq!((layers.islands, layers.seams.len()), (1, 0));
        // Neighbours in the mesh are neighbours in the grid: each vertex has
        // one place.
        let mut place: HashMap<u32, [f64; 2]> = HashMap::new();
        for (corner, &v) in mesh.faces.iter().flatten().enumerate() {
            assert_eq!(*place.entry(v).or_insert(layers.grid[corner]), layers.grid[corner]);
        }
    }

    #[test]
    fn the_islands_of_the_atlas_cover_nothing_twice() {
        for (name, mesh) in [
            ("cube", primitives::cube_quads(5)),
            ("torus", primitives::torus_quads(1.0, 0.4, 12, 8)),
            ("plane", primitives::plane_quads(6)),
        ] {
            let layers = unfold(&mesh);
            assert!(layers.atlas.iter().flatten().all(|&c| (0.0..=1.0).contains(&c)), "{name}");
            // Every quad is a square of the same size, the right way round,
            // and no two have the same middle.
            let side = (0.5 * turn_of(&layers.atlas[..4])).sqrt();
            let mut middles = HashSet::new();
            for quad in layers.atlas.chunks_exact(4) {
                assert!((0.5 * turn_of(quad) - side * side).abs() < 1e-12, "{name}");
                let middle =
                    [0, 1].map(|c| (quad.iter().map(|uv| uv[c]).sum::<f64>() / 4.0 / side * 2.0).round() as i64);
                assert!(middles.insert(middle), "{name}: two quads on the same square");
            }
            // The squares fill a fair share of the atlas.
            assert!(mesh.faces.len() as f64 * side * side > 0.3, "{name}: {} islands", layers.islands);
        }
    }

    #[test]
    fn one_place_of_the_atlas_is_one_vertex_of_the_mesh() {
        // A ball of quads has eight vertices where three quads meet: the
        // paper cannot lie flat round them, and must not close up on itself
        // with a quad next to one it does not touch.
        let ball = {
            let mut mesh = primitives::cube_quads(6);
            for p in &mut mesh.positions {
                *p = p.normalize();
            }
            mesh
        };
        // Blocks in a cross with one on top: vertices of five and six quads
        // along its inner edges, round which an unfolding comes back on
        // itself.
        let cross = primitives::blocks_quads(&[[0, 0, 0], [1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1]], 5);
        let shapes = [
            ("cube", primitives::cube_quads(5)),
            ("ball", ball),
            ("torus", primitives::torus_quads(1.0, 0.4, 12, 8)),
            ("cross", cross),
        ];
        for (name, mesh) in shapes {
            let layers = unfold(&mesh);
            let side = (0.5 * turn_of(&layers.atlas[..4])).sqrt();
            let mut vertex_at: HashMap<[i64; 2], u32> = HashMap::new();
            let vertices = mesh.faces.iter().flatten();
            for (uv, &vertex) in layers.atlas.iter().zip(vertices) {
                // Corners are at whole and a half numbers of squares: twice
                // that is a whole number.
                let place = [0, 1].map(|c| (2.0 * uv[c] / side).round() as i64);
                let there = *vertex_at.entry(place).or_insert(vertex);
                assert_eq!(there, vertex, "{name}: two vertices at one place of the atlas");
            }
        }
    }

    #[test]
    fn a_closed_shape_has_to_be_cut_somewhere() {
        let cube = unfold(&primitives::cube_quads(5));
        assert!(!cube.seams.is_empty());
        // A torus of 12 by 8 quads opens along one loop each way, at least.
        let torus = unfold(&primitives::torus_quads(1.0, 0.4, 12, 8));
        assert_eq!(torus.islands, 1);
        assert!(torus.seams.len() >= 12 + 8, "{} seams", torus.seams.len());
    }

    #[test]
    fn faces_that_are_not_quads_get_a_place_too() {
        let mut mesh = primitives::plane_quads(2);
        let (a, b, c) = (mesh.positions.len() as u32, mesh.positions.len() as u32 + 1, mesh.positions.len() as u32 + 2);
        mesh.positions.extend([crate::math::Vec3::X * 5.0, crate::math::Vec3::X * 6.0, crate::math::Vec3::Y * 5.0]);
        mesh.faces.push(vec![a, b, c]);
        let layers = unfold(&mesh);
        assert_eq!(layers.grid.len(), 4 * 4 + 3);
        assert_eq!(layers.islands, 2);
        assert!(layers.atlas.iter().flatten().all(|&c| (0.0..=1.0).contains(&c)));
    }
}
