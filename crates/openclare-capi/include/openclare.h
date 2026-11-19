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
 * - The calls that do work (openclare_remesh, openclare_bake_height,
 *   openclare_analyse, openclare_hard_edges, openclare_hard_edge_finder,
 *   openclare_hard_edge_finder_find, openclare_reflection_check,
 *   openclare_reflection_repair, openclare_project_level) never return
 *   NULL: they return a handle, which holds either what was made or the
 *   reason it was not. Each handle is freed by its own function, once:
 *   openclare_result_free, openclare_baked_free, openclare_level_free,
 *   openclare_analysis_free,
 *   openclare_hard_edges_free, openclare_hard_edge_finder_free,
 *   openclare_reflection_free, openclare_repair_free.
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
 * - edges, edge_count: NULL and 0, or edge_count edges of the mesh, two
 *   vertices each, for the quad mesh to have lines of edges along and keep
 *   as hard edges, whatever their angle: the edges a modeller marked sharp,
 *   say. They go in as sharp guide curves through their vertices, joined
 *   into chains: hard edges of the result, marked sharp and creased.
 * - curve_points, curve_sizes, curve_flags, curve_count: guide curves, lines
 *   on the surface for the quad mesh to have edges along. curve_count curves,
 *   one after the other in curve_points (x y z per point), with as many
 *   points each as curve_sizes says, and in curve_flags what each is
 *   (OPENCLARE_CURVE_CLOSED, OPENCLARE_CURVE_SHARP, or both; curve_flags may
 *   be NULL for none). All NULL and 0 for no curve.
 * - pole_points, pole_valences, pole_count: poles placed by the user, x y z
 *   each, and the valence the vertex of the quad mesh there is to have, 3 or
 *   5. The others are placed freely; openclare_result_copy_poles tells which
 *   of the result answers which. Both NULL and 0 for none.
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
                                  const uint32_t *edges,
                                  size_t edge_count,
                                  const float *curve_points,
                                  const uint32_t *curve_sizes,
                                  const uint8_t *curve_flags,
                                  size_t curve_count,
                                  const float *pole_points,
                                  const uint8_t *pole_valences,
                                  size_t pole_count,
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

/* How many edges of the mesh run along a guide curve. */
size_t openclare_result_curve_edge_count(const OpenclareResult *result);

/* Writes the edges along a guide curve, two vertices per edge, into vertices
 * (room for 2 * openclare_result_curve_edge_count numbers), and the index of
 * the curve each runs along, among the curves given to openclare_remesh, into
 * curves (room for openclare_result_curve_edge_count numbers). Either may be
 * NULL. */
void openclare_result_copy_curve_edges(const OpenclareResult *result, uint32_t *vertices, uint32_t *curves);

/* How many irregular vertices the mesh has away from its borders. */
size_t openclare_result_pole_count(const OpenclareResult *result);

/* Writes the irregular vertices of the mesh away from its borders: where each
 * lies on the input, as a triangle of it into triangles, three barycentric
 * coordinates into barycentric and the point, x y z, into points (room for
 * 3 * openclare_result_pole_count numbers each); its valence into valences;
 * and into imposed the index of the pole given to openclare_remesh it
 * answers, or -1 for one placed freely. Any may be NULL. */
void openclare_result_copy_poles(const OpenclareResult *result, uint32_t *triangles, float *barycentric,
                                 float *points, uint8_t *valences, int32_t *imposed);

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
/* Subdivision levels                                                       */
/* ------------------------------------------------------------------------ */

/* What openclare_project_level gives back. Opaque; free with
 * openclare_level_free. */
typedef struct OpenclareLevel OpenclareLevel;

/*
 * Places a subdivision level on its source: each vertex onto the side of
 * the source that faces its way, no farther than its reach, a vertex inside
 * a creased line onto the source's hard edges, faces that fold taken back
 * (the guarded projection of the Multires, PRO #6).
 *
 * - positions: 3 * vertex_count floats, the level as the subdivision left it.
 * - quads: 4 * quad_count indices of its vertices.
 * - crease_edges: 2 * crease_count indices, the creased edges of the level;
 *   crease_weights: their crease_count weights. Both may be NULL when
 *   crease_count is 0.
 * - cage_face: quad_count indices, for each face of the level the face of
 *   the cage it was subdivided from, of cage_face_count; or NULL. Given,
 *   the missing detail is also measured over each cage face, and each told
 *   what it calls for (openclare_level_copy_missing,
 *   openclare_level_copy_verdict).
 * - previous_missing: cage_face_count floats, what
 *   openclare_level_copy_missing gave for the level below; or NULL, for the
 *   first level.
 * - source_positions (3 * source_vertex_count floats), source_triangles
 *   (3 * source_triangle_count indices): the source.
 * - settings: "key = value" lines (reach, far, stretch, tilt,
 *   crease_angle, samples, seed, target, stall, stuck), or NULL for the
 *   defaults.
 *
 * The arrays are read during the call only. Never returns NULL; free the
 * handle with openclare_level_free.
 */
OpenclareLevel *openclare_project_level(const float *positions,
                                        size_t vertex_count,
                                        const uint32_t *quads,
                                        size_t quad_count,
                                        const uint32_t *crease_edges,
                                        const float *crease_weights,
                                        size_t crease_count,
                                        const uint32_t *cage_face,
                                        size_t cage_face_count,
                                        const float *previous_missing,
                                        const float *source_positions,
                                        size_t source_vertex_count,
                                        const uint32_t *source_triangles,
                                        size_t source_triangle_count,
                                        const char *settings);

/* The functions below take a handle from openclare_project_level not yet
 * freed, or NULL. */

/* 1 if the level was placed, 0 if not or if level is NULL. */
int32_t openclare_level_ok(const OpenclareLevel *level);

/* Why the level was not placed; "" if it was. Owned by the handle. */
const char *openclare_level_error(const OpenclareLevel *level);

/* What happened, as "key = value" lines: the count of vertices in each
 * status, the missing detail and stretched edges of the level once placed
 * and the advice (done, more, denser), unless samples = 0; with cage faces,
 * the count of those that want more quads round them (denser_faces) and of
 * those above the target; seconds. Owned by the handle. */
const char *openclare_level_report(const OpenclareLevel *level);

/* Copies where the vertices now are, 3 floats each, into positions. */
void openclare_level_copy_positions(const OpenclareLevel *level, float *positions);

/* Copies what became of each vertex, one byte each, into status: 0 moved,
 * 1 onto a hard edge, 2 stopped by the reach, 3 nothing found, 4 taken
 * back, 5 held. */
void openclare_level_copy_status(const OpenclareLevel *level, uint8_t *status);

/* Copies what the level misses of the source over each cage face,
 * cage_face_count floats, in percent of the diagonal of the source's
 * bounding box: the 99th percentile of the distances from the points spread
 * on the source closest to that face's part of the level, 0 where no point
 * is. Copies nothing without cage faces, or with samples = 0. */
void openclare_level_copy_missing(const OpenclareLevel *level, float *missing);

/* Copies what each cage face calls for, cage_face_count bytes: 0 below the
 * target; 1 above it and still gaining, another level helps; 2 above it and
 * no longer gaining, or its vertices found little of the source: more quads
 * round it in the cage. Copies nothing when openclare_level_copy_missing
 * does not. */
void openclare_level_copy_verdict(const OpenclareLevel *level, uint8_t *verdict);

/* Frees a handle of a level, and its texts. NULL is fine; twice is not. */
void openclare_level_free(OpenclareLevel *level);

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

/* ------------------------------------------------------------------------ */
/* Reflection check                                                         */
/* ------------------------------------------------------------------------ */

/* What openclare_reflection_check gives back: the faults of a cage under
 * Catmull-Clark, worst first. Opaque; free with openclare_reflection_free. */
typedef struct OpenclareReflection OpenclareReflection;

/* The kinds of fault openclare_reflection_copy_faults writes. */
#define OPENCLARE_FAULT_DEGENERATE 0  /* a face of no area */
#define OPENCLARE_FAULT_FOLDED 1      /* corner quads that fold */
#define OPENCLARE_FAULT_NOT_FLAT 2    /* a face of 5 sides or more off its plane */
#define OPENCLARE_FAULT_PINCH 3       /* a triangle or an n-gon on a curved zone */
#define OPENCLARE_FAULT_CURVED_POLE 4 /* a pole on a curved zone */
#define OPENCLARE_FAULT_THIN 5        /* corner quads much thinner than the others */

/*
 * The reflection check of a cage: what of it will make a reflection pinch or
 * wobble once Catmull-Clark subdivides it, fault by fault, worst first.
 * Angles are measured on the cage, not on the subdivided surface.
 *
 * - positions: 3 * vertex_count floats.
 * - face_sizes: the number of corners of each of face_count faces;
 *   corner_vertices: the vertex of each corner, corner_count of them, face
 *   after face, as Blender keeps its polygons.
 * - crease_edges: 2 * crease_count vertices, two per edge, and
 *   crease_weights one weight from 0 to 1 per edge; both NULL and
 *   crease_count 0 for none: only the borders are then sharp.
 * - flat_tolerance (a share of the radius of a face), angle_tolerance
 *   (degrees), thin_ratio: the thresholds of what counts as a fault; a
 *   negative one takes its default (0.002, 1, 0.15).
 *
 * The arrays are read during the call only. Never returns NULL; free the
 * handle with openclare_reflection_free.
 */
OpenclareReflection *openclare_reflection_check(const float *positions,
                                                size_t vertex_count,
                                                const uint32_t *face_sizes,
                                                size_t face_count,
                                                const uint32_t *corner_vertices,
                                                size_t corner_count,
                                                const uint32_t *crease_edges,
                                                const float *crease_weights,
                                                size_t crease_count,
                                                double flat_tolerance,
                                                double angle_tolerance,
                                                double thin_ratio);

/* The functions below take a handle from openclare_reflection_check not yet
 * freed, or NULL. */

/* 1 if the cage was checked, 0 if not or if reflection is NULL. */
int32_t openclare_reflection_ok(const OpenclareReflection *reflection);

/* Why the cage could not be checked; "" if it was. Owned by the handle. */
const char *openclare_reflection_error(const OpenclareReflection *reflection);

/* How many faults were found. */
size_t openclare_reflection_fault_count(const OpenclareReflection *reflection);

/* How many poles were looked at, on a curved zone or not. */
size_t openclare_reflection_pole_count(const OpenclareReflection *reflection);

/* How many parts (corners and edges of faces) the faults name in all. */
size_t openclare_reflection_part_count(const OpenclareReflection *reflection);

/*
 * Writes the faults, worst first, one number per fault into each array that
 * is not NULL: kinds (OPENCLARE_FAULT_*); elements, 0 for a face and 1 for a
 * vertex, and indices, which one; sides, of the face or valence of the
 * vertex; counts, how many of its parts the fault concerns; values, an angle
 * in degrees for NOT_FLAT, PINCH and CURVED_POLE, the smallest triangle
 * (centre, edge) over the mean one for FOLDED and THIN, 0 for DEGENERATE.
 */
void openclare_reflection_copy_faults(const OpenclareReflection *reflection,
                                      uint8_t *kinds,
                                      uint8_t *elements,
                                      uint32_t *indices,
                                      uint32_t *sides,
                                      uint32_t *counts,
                                      double *values);

/*
 * Writes where each fault is to be drawn, into each array that is not NULL:
 * centres and normals, 3 floats per fault (the centre of the face or the
 * vertex; the unit normal of the face or the mean one round the vertex,
 * zero for a degenerate face); inner and radius, one per fault (for a face,
 * the distance from its centre to its nearest edge and to its furthest
 * vertex; for a vertex, the mean length of its edges, twice).
 */
void openclare_reflection_copy_frames(const OpenclareReflection *reflection,
                                      float *centres,
                                      float *normals,
                                      float *inner,
                                      float *radius);

/*
 * Writes the parts of the faces at fault: offsets, fault count + 1 numbers,
 * the parts of fault i being those from offsets[i] to offsets[i + 1]; for
 * each part, its kind in part_kinds (0 a corner, 1 an edge) and its place in
 * the face in part_indices (corner k is the face's vertex k; edge k runs
 * from its vertex k to its vertex k + 1). Any array may be NULL.
 */
void openclare_reflection_copy_parts(const OpenclareReflection *reflection,
                                     uint32_t *offsets,
                                     uint8_t *part_kinds,
                                     uint32_t *part_indices);

/* Frees a handle of a reflection check. NULL is fine; twice is not. */
void openclare_reflection_free(OpenclareReflection *reflection);

/* ------------------------------------------------------------------------ */
/* Reflection repair                                                        */
/* ------------------------------------------------------------------------ */

/* What openclare_reflection_repair gives back: the cage repaired in place,
 * the creases proposed, the scores before and after, the faults left and
 * why. Opaque; free with openclare_repair_free. */
typedef struct OpenclareRepair OpenclareRepair;

/* The scores openclare_repair_copy_score writes. */
#define OPENCLARE_REPAIR_BEFORE 0       /* of the cage given */
#define OPENCLARE_REPAIR_AFTER 1        /* after the moves, with the creases given */
#define OPENCLARE_REPAIR_WITH_CREASES 2 /* after the moves, with the proposed creases set too */

/* Why a fault is left, as openclare_repair_copy_left writes it. */
#define OPENCLARE_REASON_CHANGE_TOPOLOGY 0   /* moving vertices does not mend it */
#define OPENCLARE_REASON_TOO_FAR 1           /* mending it would move a vertex beyond the bound */
#define OPENCLARE_REASON_WOULD_WORSEN 2      /* every move tried makes the score worse */
#define OPENCLARE_REASON_PROPOSED_CREASE 3   /* gone once the proposed creases are set */
#define OPENCLARE_REASON_OUTSIDE_SELECTION 4 /* not on the faces asked for */
#define OPENCLARE_REASON_OUT_OF_PASSES 5     /* appeared in the last pass */

/*
 * The reflection repair of a cage, in place: every vertex, edge and face
 * kept. Vertices are moved to flatten the n-gons off their plane and to make
 * the faces whose corner quads fold or are thin star-shaped again; creases
 * are proposed on edges already sharp where they cure a pinch or a pole on a
 * curved zone, never set. Each repair is kept only if the score of the
 * reflection check gets better. Angles are measured on the cage, not on the
 * subdivided surface.
 *
 * - The cage and its creases as for openclare_reflection_check.
 * - within: within_count faces, the only ones repaired (only their vertices
 *   move, only their edges get a crease); NULL and 0 for the whole cage.
 * - flat_tolerance, angle_tolerance, thin_ratio as for the check; bound, the
 *   largest displacement of a vertex as a share of the shortest of its edges
 *   (default 0.1); crease_fold, the fold in degrees between two faces from
 *   which an edge may get a crease (default 30). A negative one takes its
 *   default.
 *
 * The arrays are read during the call only. Never returns NULL; free the
 * handle with openclare_repair_free.
 */
OpenclareRepair *openclare_reflection_repair(const float *positions,
                                             size_t vertex_count,
                                             const uint32_t *face_sizes,
                                             size_t face_count,
                                             const uint32_t *corner_vertices,
                                             size_t corner_count,
                                             const uint32_t *crease_edges,
                                             const float *crease_weights,
                                             size_t crease_count,
                                             const uint32_t *within,
                                             size_t within_count,
                                             double flat_tolerance,
                                             double angle_tolerance,
                                             double thin_ratio,
                                             double bound,
                                             double crease_fold);

/* The functions below take a handle from openclare_reflection_repair not
 * yet freed, or NULL. */

/* 1 if the cage was repaired (even with nothing to change), 0 if not or if
 * repair is NULL. */
int32_t openclare_repair_ok(const OpenclareRepair *repair);

/* Why the cage could not be repaired; "" if it was. Owned by the handle. */
const char *openclare_repair_error(const OpenclareRepair *repair);

/* How many vertices the cage has, and how many of them moved. */
size_t openclare_repair_vertex_count(const OpenclareRepair *repair);
size_t openclare_repair_moved_count(const OpenclareRepair *repair);

/* How far the furthest vertex moved; with share not NULL, the largest
 * displacement over the shortest edge of its vertex is written there. */
double openclare_repair_largest_displacement(const OpenclareRepair *repair, double *share);

/* How many repairs were kept; with passes not NULL, how many passes ran. */
size_t openclare_repair_repair_count(const OpenclareRepair *repair, size_t *passes);

/* Writes the positions of the repaired cage, 3 floats per vertex, numbered
 * as given; a vertex not moved is written as it was given. */
void openclare_repair_copy_positions(const OpenclareRepair *repair, float *positions);

/*
 * Writes one score, stage OPENCLARE_REPAIR_*: for each of the six kinds of
 * fault, in the order of OPENCLARE_FAULT_*, how many, the worst value and
 * the median one (NaN where there is none), into each array not NULL, of 6
 * numbers. Returns how many poles were looked at; 0 for a stage unknown.
 */
size_t openclare_repair_copy_score(const OpenclareRepair *repair,
                                   uint32_t stage,
                                   uint32_t *counts,
                                   double *worst,
                                   double *median);

/* How many creases are proposed. */
size_t openclare_repair_crease_count(const OpenclareRepair *repair);

/*
 * Writes the creases proposed, into each array not NULL: edges, 2 vertices
 * per crease; folds, the fold between its two faces in degrees; and the
 * fault each is proposed for: kind (OPENCLARE_FAULT_*), element (0 a face,
 * 1 a vertex), index, angle in degrees, and cured, 1 if that fault is gone
 * once every proposed crease is set.
 */
void openclare_repair_copy_creases(const OpenclareRepair *repair,
                                   uint32_t *edges,
                                   double *folds,
                                   uint8_t *kinds,
                                   uint8_t *elements,
                                   uint32_t *indices,
                                   double *values,
                                   uint8_t *cured);

/* How many faults are left after the moves, with the creases given. */
size_t openclare_repair_left_count(const OpenclareRepair *repair);

/*
 * Writes the faults left, worst first, as openclare_reflection_copy_faults
 * does, and in reasons why each is left (OPENCLARE_REASON_*); any array may
 * be NULL.
 */
void openclare_repair_copy_left(const OpenclareRepair *repair,
                                uint8_t *kinds,
                                uint8_t *elements,
                                uint32_t *indices,
                                uint32_t *sides,
                                double *values,
                                uint8_t *reasons);

/* Frees a handle of a reflection repair. NULL is fine; twice is not. */
void openclare_repair_free(OpenclareRepair *repair);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* OPENCLARE_H */
