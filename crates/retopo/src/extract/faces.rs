use super::LatticeGraph;
use crate::math::INVALID;
use crate::mesh::PolyMesh;

#[derive(Clone, Debug, Default)]
pub struct FaceExtraction {
    pub mesh: PolyMesh,
    /// Loops of the graph that were not turned into faces: the real
    /// boundaries of an open surface, and holes left by the extraction.
    pub open_loops: usize,
}

/// Traces the faces of the embedded graph.
///
/// With neighbours sorted counter-clockwise, "arrive at `v` from `u`, leave
/// towards the neighbour just before `u`" is a permutation of the directed
/// edges whose cycles are exactly the faces, each walked counter-clockwise.
/// Cycles longer than `max_degree` are boundaries or holes and are left open.
pub fn extract_faces(graph: &LatticeGraph, max_degree: usize) -> FaceExtraction {
    let adjacency = &graph.adjacency;
    let mut offsets = Vec::with_capacity(adjacency.len() + 1);
    offsets.push(0usize);
    for list in adjacency {
        offsets.push(offsets.last().unwrap() + list.len());
    }

    let mut visited = vec![false; *offsets.last().unwrap()];
    let mut faces: Vec<Vec<u32>> = Vec::new();
    let mut open_loops = 0;
    let mut face = Vec::new();
    let mut stack: Vec<u32> = Vec::new();
    // Position of each vertex in `stack`, INVALID when it is not on it.
    let mut stack_slot = vec![INVALID; adjacency.len()];
    let keep = |piece: &[u32], faces: &mut Vec<Vec<u32>>, open_loops: &mut usize| match piece.len() {
        0..=2 => {}
        n if n <= max_degree => faces.push(piece.to_vec()),
        _ => *open_loops += 1,
    };
    for start in 0..adjacency.len() {
        for start_slot in 0..adjacency[start].len() {
            if visited[offsets[start] + start_slot] {
                continue;
            }
            face.clear();
            let (mut vertex, mut slot) = (start, start_slot);
            loop {
                visited[offsets[vertex] + slot] = true;
                face.push(vertex as u32);
                let next = adjacency[vertex][slot] as usize;
                let back = adjacency[next].iter().position(|&v| v as usize == vertex).unwrap();
                let degree = adjacency[next].len();
                (vertex, slot) = (next, (back + degree - 1) % degree);
                if (vertex, slot) == (start, start_slot) {
                    break;
                }
            }

            // Where the fields are inconsistent (around position singularities)
            // the graph is not locally planar and a cycle can pass through a
            // vertex twice. Cut such a cycle at its pinch points: each piece
            // is a simple loop, and two-edge leftovers are edges with the
            // same face on both sides, which just disappear.
            stack.clear();
            for &v in &face {
                let at = stack_slot[v as usize];
                if at != INVALID {
                    for &popped in &stack[at as usize..] {
                        stack_slot[popped as usize] = INVALID;
                    }
                    keep(&stack[at as usize..], &mut faces, &mut open_loops);
                    stack.truncate(at as usize);
                }
                stack_slot[v as usize] = stack.len() as u32;
                stack.push(v);
            }
            for &v in &stack {
                stack_slot[v as usize] = INVALID;
            }
            keep(&stack, &mut faces, &mut open_loops);
        }
    }

    let mut mesh = PolyMesh { positions: graph.positions.clone(), faces };
    mesh.compact();
    FaceExtraction { mesh, open_loops }
}
