/*
 * openclare.h: the C interface of openclare, automatic quad retopology.
 *
 * Copyright 2026 Marc Gardent. GPL-3.0-or-later.
 *
 * Written by hand from crates/openclare-capi/src/lib.rs, and changed with it:
 * the release workflow compares the two (with cbindgen) and compiles a test
 * against this header on Linux, Windows and macOS.
 *
 * The library is libopenclare_capi.so (Linux), libopenclare_capi.dylib
 * (macOS) or openclare_capi.dll (Windows, linked through openclare_capi.dll.lib).
 *
 * Ownership, in short:
 * - Arrays and texts given to the library belong to the caller; they are
 *   read during the call only, never kept.
 * - The four calls that do work (openclare_remesh, openclare_bake_height,
 *   openclare_analyse, openclare_hard_edges, openclare_hard_edge_finder,
 *   openclare_hard_edge_finder_find) never return NULL: they return
 *   a handle, which holds either what was made or the reason it was not.
 *   Each handle is freed by its own function, once: openclare_result_free,
 *   openclare_baked_free, openclare_analysis_free, openclare_hard_edges_free,
 *   openclare_hard_edge_finder_free.
 *   Freeing NULL is allowed.
 * - The copy functions write into arrays the caller allocates, of the size
 *   the matching count function gives.
 * - Texts returned by the library (versions, errors, reports) belong to it:
 *   never free them. The version is valid for ever; the others until their
 *   handle is freed.
 * - Every function taking a handle accepts NULL and then does nothing,
 *   returns 0, or returns an empty text (an error text for _error).
 * - Nothing unwinds into the caller: a failure inside the library comes back
 *   as the error of the handle.
 */

#ifndef OPENCLARE_H
#define OPENCLARE_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ------------------------------------------------------------------------ */
/* Version                                                                  */
/* ------------------------------------------------------------------------ */

/*
 * Version of the library, as "major.minor.patch". Static text: never free it.
 */
const char *openclare_version(void);

/* ------------------------------------------------------------------------ */
/* Remeshing                                                                */
/* ------------------------------------------------------------------------ */

/* What openclare_remesh gives back. Opaque; free with openclare_result_free. */
typedef struct OpenclareResult OpenclareResult;

/*
 * Called when a stage of the run begins, with its name ("cross field", or
 * "piece 3 of 12: cross field"; valid during the call only) and the `user`
 * pointer given to openclare_remesh. Return 0 to go on, anything else to
 * stop the run: the result then holds the error "stopped on request".
 */
typedef int32_t (*OpenclareOnStage)(const char *stage, void *user);

/* In the flags of a guide curve: its last point joins the first. */
#define OPENCLARE_CURVE_CLOSED 1
/* In the flags of a guide curve: the edges along it are hard edges of the
 * result, marked sharp and creased, and not only a line of edges. */
#define OPENCLARE_CURVE_SHARP 2

/*
 * Remeshes a triangle mesh into quads.
 *
 * - positions: 3 * vertex_count coordinates, x y z per vertex.
 * - triangles: 3 * triangle_count indices: of vertices if corner_vertices
 *   is NULL, of corners otherwise.
 * - corner_vertices: NULL, or the vertex of each of corner_count corners
 *   (the way Blender stores faces).
 * - density: NULL, or one weight per vertex from 0 (least dense) to 1 (most
 *   dense).
 * - triangle_groups: NULL, or a group for each triangle (its material, say).
 *   The quad mesh has a line of edges where two groups meet, and
 *   openclare_result_copy_face_groups gives the group of each of its faces.
 * - curve_points, curve_sizes, curve_flags, curve_count: guide curves, lines
 *   on the surface for the quad mesh to have edges along. curve_count curves,
 *   one after the other in curve_points (x y z per point), with as many
 *   points each as curve_sizes says, and in curve_flags what each is
 *   (OPENCLARE_CURVE_CLOSED, OPENCLARE_CURVE_SHARP, or both; curve_flags may
 *   be NULL for none). All NULL and 0 for no curve.
 * - preset: NULL or "" for the default one, or the name of a shipped preset:
 *   "organic", "animation", "hard-surface".
 * - settings: NULL, or "key = value" lines that adjust the preset, such as
 *   "quads = 3000\n".
 * - on_stage: NULL, or a function called as each stage begins, with user,
 *   from the thread that called openclare_remesh.
 *
 * The arrays must hold what their counts say, and the texts end with a NUL;
 * they are read during the call only.
 *
 * Never returns NULL. Ask openclare_result_ok whether the result holds a
 * mesh; free it with openclare_result_free in either case.
 */
OpenclareResult *openclare_remesh(const float *positions,
                                  size_t vertex_count,
                                  const uint32_t *triangles,
                                  size_t triangle_count,
                                  const uint32_t *corner_vertices,
                                  size_t corner_count,
                                  const float *density,
                                  const uint32_t *triangle_groups,
                                  const float *curve_points,
                                  const uint32_t *curve_sizes,
                                  const uint8_t *curve_flags,
                                  size_t curve_count,
                                  const char *preset,
                                  const char *settings,
                                  OpenclareOnStage on_stage,
                                  void *user);

/*
 * The functions below take a result from openclare_remesh not yet freed, or
 * NULL.
 */

/* 1 if the result holds a mesh, 0 if it holds an error (or is NULL). */
int32_t openclare_result_ok(const OpenclareResult *result);

/* Why there is no mesh; "" if there is one. Owned by the result: valid until
 * it is freed, not to be freed. */
const char *openclare_result_error(const OpenclareResult *result);

/* What happened, as "key = value" lines: quads made, pieces, seconds. Owned
 * by the result: valid until it is freed, not to be freed. */
const char *openclare_result_report(const OpenclareResult *result);

/* Vertices of the quad mesh. */
size_t openclare_result_vertex_count(const OpenclareResult *result);

/* Faces of the quad mesh. */
size_t openclare_result_face_count(const OpenclareResult *result);

/* Corners of all the faces together: four per quad. */
size_t openclare_result_corner_count(const OpenclareResult *result);

/* Writes the coordinates of the vertices, x y z each, into positions, which
 * has room for 3 * openclare_result_vertex_count floats. */
void openclare_result_copy_positions(const OpenclareResult *result, float *positions);

/* Writes the faces: into sizes, how many corners each has (room for
 * openclare_result_face_count numbers); into corners, the vertex of each
 * corner, face after face (room for openclare_result_corner_count). Nothing
 * is written if either is NULL. */
void openclare_result_copy_faces(const OpenclareResult *result, uint32_t *sizes, uint32_t *corners);

/* Layers of texture coordinates, for openclare_result_copy_uv.
 * The grid: every quad a unit square, coordinates in whole numbers of quads,
 * parts of the surface on top of each other; for patterns along the edge
 * flow. */
#define OPENCLARE_UV_GRID 0
/* The atlas: islands that cover nothing twice, packed into the unit square;
 * for painting. Its cuts are given by openclare_result_copy_seams. */
#define OPENCLARE_UV_ATLAS 1

/* Writes a layer of texture coordinates, u v for each corner of each face,
 * in the order of openclare_result_copy_faces, into coordinates (room for
 * 2 * openclare_result_corner_count floats). Returns 1 if it did, 0 if layer
 * is not one of the two or coordinates is NULL. The layers are worked out on
 * the first call that needs them. */
int32_t openclare_result_copy_uv(const OpenclareResult *result, uint32_t layer, float *coordinates);

/* How many edges the atlas is cut along. */
size_t openclare_result_seam_count(const OpenclareResult *result);

/* Writes the seams of the atlas, two vertices per edge, into vertices (room
 * for 2 * openclare_result_seam_count numbers). */
void openclare_result_copy_seams(const OpenclareResult *result, uint32_t *vertices);

/* How many edges of the mesh run along a hard edge of the input. */
size_t openclare_result_hard_edge_count(const OpenclareResult *result);

/* Writes the edges along a hard edge of the input, two vertices per edge,
 * into vertices (room for 2 * openclare_result_hard_edge_count numbers). */
void openclare_result_copy_hard_edges(const OpenclareResult *result, uint32_t *vertices);

/* Writes the group of the input each face lies in, one number per face, into
 * groups (room for openclare_result_face_count numbers). Returns 1, or 0 and
 * writes nothing if the input came without groups. */
int32_t openclare_result_copy_face_groups(const OpenclareResult *result, uint32_t *groups);

/* How many edges carry a crease: those to keep sharp, wholly or in part,
 * when the mesh is subdivided. */
size_t openclare_result_crease_count(const OpenclareResult *result);

/* Writes the creased edges, two vertices per edge, into vertices (room for
 * 2 * openclare_result_crease_count numbers), and their weights from 0
 * (smooth) to 1 (sharp) into weights (room for openclare_result_crease_count).
 * Nothing is written if either is NULL. */
void openclare_result_copy_creases(const OpenclareResult *result, uint32_t *vertices, float *weights);

/* Frees a result, and the texts it gave. NULL is fine; twice is not. */
void openclare_result_free(OpenclareResult *result);

/* ------------------------------------------------------------------------ */
/* Baking                                                                   */
/* ------------------------------------------------------------------------ */

/* What openclare_bake_height gives back. Opaque; free with
 * openclare_baked_free. */
typedef struct OpenclareBaked OpenclareBaked;

/*
 * Bakes the height of a detailed surface above a plainer one into a texture:
 * for each texel, how far the source is from the target along the normal of
 * the target, signed, in the unit of the meshes.
 *
 * - positions, normals: 3 * vertex_count floats each, the target.
 * - triangles: 3 * triangle_count indices of its vertices.
 * - uvs: 6 * triangle_count floats, u and v for each corner of each
 *   triangle, (0, 0) at the bottom left of the texture.
 * - source_positions (3 * source_vertex_count floats), source_triangles
 *   (3 * source_triangle_count indices): the detailed surface.
 * - width, height: the size of the texture in texels.
 * - reach: how far from the target the source is looked for.
 * - padding: texels filled round the faces from their neighbours.
 *
 * The arrays are read during the call only. Never returns NULL; free the
 * handle with openclare_baked_free.
 */
OpenclareBaked *openclare_bake_height(const float *positions,
                                      const float *normals,
                                      size_t vertex_count,
                                      const uint32_t *triangles,
                                      const float *uvs,
                                      size_t triangle_count,
                                      const float *source_positions,
                                      size_t source_vertex_count,
                                      const uint32_t *source_triangles,
                                      size_t source_triangle_count,
                                      uint32_t width,
                                      uint32_t height,
                                      float reach,
                                      uint32_t padding);

/* The functions below take a handle from openclare_bake_height not yet
 * freed, or NULL. */

/* 1 if there is a texture, 0 if not or if baked is NULL. */
int32_t openclare_baked_ok(const OpenclareBaked *baked);

/* Why there is no texture; "" if there is one. Owned by the handle. */
const char *openclare_baked_error(const OpenclareBaked *baked);

/* What happened, as "key = value" lines: texels covered, texels where the
 * source was not found along the normal, least and greatest height, seconds.
 * Owned by the handle. */
const char *openclare_baked_report(const OpenclareBaked *baked);

/* Copies the heights, width * height floats, row after row from the bottom,
 * NaN where no face of the target lies, into heights (room for them). */
void openclare_baked_copy(const OpenclareBaked *baked, float *heights);

/* Frees a handle of a bake, and its texts. NULL is fine; twice is not. */
void openclare_baked_free(OpenclareBaked *baked);

/* ------------------------------------------------------------------------ */
/* Analysis                                                                 */
/* ------------------------------------------------------------------------ */

/* What openclare_analyse gives back: a shape measured once, to be asked as
 * often as wanted how many quads a largest and a smallest quad make of it.
 * Opaque; free with openclare_analysis_free. */
typedef struct OpenclareAnalysis OpenclareAnalysis;

/*
 * Measures what a shape calls for, without meshing it.
 *
 * - positions: 3 * vertex_count floats; triangles: 3 * triangle_count
 *   indices of vertices.
 * - preset, settings: as for openclare_remesh, NULL for none. The
 *   adaptivity, the quads round a thin part and the contrast of the paint
 *   count; the target does not.
 * - density: NULL, or one weight per vertex, from 0 to 1.
 *
 * The arrays and texts are read during the call only. Never returns NULL;
 * free the handle with openclare_analysis_free.
 */
OpenclareAnalysis *openclare_analyse(const float *positions,
                                     size_t vertex_count,
                                     const uint32_t *triangles,
                                     size_t triangle_count,
                                     const char *preset,
                                     const char *settings,
                                     const float *density);

/* The functions below take a handle from openclare_analyse not yet freed, or
 * NULL. */

/* 1 if the shape was analysed, 0 if not or if analysis is NULL. */
int32_t openclare_analysis_ok(const OpenclareAnalysis *analysis);

/* Why the shape could not be analysed; "" if it was. Owned by the handle. */
const char *openclare_analysis_error(const OpenclareAnalysis *analysis);

/* A largest and a smallest quad edge to start from, written to *largest and
 * *smallest. Returns 1, or 0 when there is no analysis. */
int32_t openclare_analysis_proposal(const OpenclareAnalysis *analysis, double *largest, double *smallest);

/* How many quads quads between smallest and largest make, written to *quads,
 * and the share of the surface that calls for smaller ones, from 0 to 1, to
 * *below. Returns 1, or 0 when there is no analysis or the sizes are not
 * 0 < smallest <= largest. */
int32_t openclare_analysis_count(const OpenclareAnalysis *analysis,
                                 double largest,
                                 double smallest,
                                 double *quads,
                                 double *below);

/* For each vertex given to openclare_analyse, the edge its quads would have
 * with quads between smallest and largest, to sizes (NaN for a piece that
 * could not be looked at), and 1 in coarser where its shape calls for smaller
 * quads than the smallest, 0 elsewhere. sizes and coarser have room for one
 * number per vertex. Returns 1, or 0 when there is no analysis. */
int32_t openclare_analysis_vertices(const OpenclareAnalysis *analysis,
                                    double largest,
                                    double smallest,
                                    float *sizes,
                                    uint8_t *coarser);

/* For each vertex given to openclare_analyse, the edge its quads would have
 * with `quads` quads asked for, to sizes (room for one number per vertex;
 * NaN for a piece that could not be looked at). Returns 1, or 0 when there is
 * no analysis or quads is not a positive number. */
int32_t openclare_analysis_vertices_for(const OpenclareAnalysis *analysis, double quads, float *sizes);

/* Frees a handle of an analysis. NULL is fine; twice is not. */
void openclare_analysis_free(OpenclareAnalysis *analysis);

/* ------------------------------------------------------------------------ */
/* Hard edges                                                               */
/* ------------------------------------------------------------------------ */

/* What openclare_hard_edges gives back: the hard edges a remesh would follow,
 * and those it would leave out. Opaque; free with openclare_hard_edges_free. */
typedef struct OpenclareHardEdges OpenclareHardEdges;

/*
 * Finds the hard edges openclare_remesh would follow on a mesh with the same
 * arguments, by the same code and at the size of the quads it would make,
 * without remeshing: what a host shows of the angle of the hard edges before
 * a run.
 *
 * - positions, vertex_count, triangles, triangle_count, corner_vertices,
 *   corner_count, density, preset, settings: as for openclare_remesh. The
 *   target, the angles, the adaptivity, the quads round a thin part and the
 *   paint count, as they set the size of the quads of each piece.
 *
 * The edges are given as two vertices of the mesh each. Kept: those sharper
 * than the angle of the hard edges ("edge_angle") that still are at the size
 * of the quads, and those sharper than "crease_angle"; open borders, which
 * are always followed, are not among them. Left out: those sharper than
 * "edge_angle" between their two triangles that are not at the size of the
 * quads, the grain of a sculpt. Where the mesh had vertices at the same place
 * along a torn border, which the remesh welds, an edge names one of them at
 * each end.
 *
 * The arrays and texts are read during the call only. Never returns NULL;
 * free the handle with openclare_hard_edges_free.
 */
OpenclareHardEdges *openclare_hard_edges(const float *positions,
                                         size_t vertex_count,
                                         const uint32_t *triangles,
                                         size_t triangle_count,
                                         const uint32_t *corner_vertices,
                                         size_t corner_count,
                                         const float *density,
                                         const char *preset,
                                         const char *settings);

/* The functions below take a handle from openclare_hard_edges or
 * openclare_hard_edge_finder_find not yet freed, or NULL. */

/* 1 if the hard edges were found, 0 if not or if found is NULL. */
int32_t openclare_hard_edges_ok(const OpenclareHardEdges *found);

/* Why the hard edges could not be found; "" if they were. Owned by the
 * handle. */
const char *openclare_hard_edges_error(const OpenclareHardEdges *found);

/* How many edges a remesh would follow. */
size_t openclare_hard_edges_kept_count(const OpenclareHardEdges *found);

/* Writes them, two vertices per edge, into vertices (room for twice
 * openclare_hard_edges_kept_count numbers). */
void openclare_hard_edges_copy_kept(const OpenclareHardEdges *found, uint32_t *vertices);

/* How many edges sharper than the angle a remesh would leave out. */
size_t openclare_hard_edges_left_out_count(const OpenclareHardEdges *found);

/* Writes them, two vertices per edge, into vertices (room for twice
 * openclare_hard_edges_left_out_count numbers). */
void openclare_hard_edges_copy_left_out(const OpenclareHardEdges *found, uint32_t *vertices);

/* Frees a handle of hard edges. NULL is fine; twice is not. */
void openclare_hard_edges_free(OpenclareHardEdges *found);

/* One mesh whose hard edges are found again and again under other settings,
 * as a host shows them while they are being set. Opaque; free with
 * openclare_hard_edge_finder_free. */
typedef struct OpenclareHardEdgeFinder OpenclareHardEdgeFinder;

/*
 * Takes in a mesh to find the hard edges of with
 * openclare_hard_edge_finder_find: what depends on the mesh alone is done
 * once, and the size of the quads is not looked for again for settings
 * already met. The arguments are as for openclare_remesh; the arrays are
 * copied, and may be freed on return.
 *
 * Never returns NULL; free the handle with openclare_hard_edge_finder_free.
 * A handle is used by one thread at a time.
 */
OpenclareHardEdgeFinder *openclare_hard_edge_finder(const float *positions,
                                                    size_t vertex_count,
                                                    const uint32_t *triangles,
                                                    size_t triangle_count,
                                                    const uint32_t *corner_vertices,
                                                    size_t corner_count);

/* 1 if the mesh was read, 0 if not or if finder is NULL. */
int32_t openclare_hard_edge_finder_ok(const OpenclareHardEdgeFinder *finder);

/* Why the mesh could not be read; "" if it was. Owned by the handle. */
const char *openclare_hard_edge_finder_error(const OpenclareHardEdgeFinder *finder);

/*
 * The hard edges of the mesh of finder under preset and settings, with
 * density (NULL, or one weight per vertex): what openclare_hard_edges gives
 * with the same mesh and arguments. The texts and density are read during
 * the call only. Never returns NULL; free the handle with
 * openclare_hard_edges_free.
 */
OpenclareHardEdges *openclare_hard_edge_finder_find(OpenclareHardEdgeFinder *finder,
                                                    const float *density,
                                                    const char *preset,
                                                    const char *settings);

/* Frees a finder. NULL is fine; twice is not. */
void openclare_hard_edge_finder_free(OpenclareHardEdgeFinder *finder);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* OPENCLARE_H */
