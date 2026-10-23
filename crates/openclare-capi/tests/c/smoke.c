/*
 * The C interface called from C, through include/openclare.h and the shared
 * library: the version, an analysis, the hard edges and a remesh of a ball,
 * everything read back and freed. `make capi-test` runs it here; the release workflow on
 * Linux, Windows and macOS against the library it packages.
 *
 *     smoke [expected version]
 *
 * Prints what it did, and exits with 1 at the first thing that is wrong.
 */

#include "openclare.h"

#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define RINGS 16
#define SEGMENTS 32
#define QUADS 300

static int failures = 0;

static void check(int holds, const char *what) {
    if (!holds) {
        fprintf(stderr, "FAILED: %s\n", what);
        failures++;
    }
}

static int32_t stages_seen = 0;

static int32_t on_stage(const char *stage, void *user) {
    int32_t *count = (int32_t *)user;
    (*count)++;
    check(stage != NULL && stage[0] != '\0', "a stage has a name");
    return 0;
}

int main(int argc, char **argv) {
    const char *version = openclare_version();
    printf("openclare %s\n", version);
    if (argc > 1) {
        check(strcmp(version, argv[1]) == 0, "the version is the one expected");
    }

    /* A ball of radius 1: a pole at each end, RINGS - 1 rings of SEGMENTS
     * vertices between them. */
    const size_t vertex_count = 2 + (size_t)(RINGS - 1) * SEGMENTS;
    const size_t triangle_count = 2 * (size_t)SEGMENTS * (RINGS - 1);
    float *positions = malloc(3 * vertex_count * sizeof(float));
    uint32_t *triangles = malloc(3 * triangle_count * sizeof(uint32_t));
    const double pi = 3.14159265358979323846;
    size_t v = 0;
    positions[3 * v + 0] = 0.0f, positions[3 * v + 1] = 0.0f, positions[3 * v + 2] = 1.0f, v++;
    for (int ring = 1; ring < RINGS; ring++) {
        double polar = pi * ring / RINGS;
        for (int segment = 0; segment < SEGMENTS; segment++) {
            double azimuth = 2.0 * pi * segment / SEGMENTS;
            positions[3 * v + 0] = (float)(sin(polar) * cos(azimuth));
            positions[3 * v + 1] = (float)(sin(polar) * sin(azimuth));
            positions[3 * v + 2] = (float)cos(polar);
            v++;
        }
    }
    positions[3 * v + 0] = 0.0f, positions[3 * v + 1] = 0.0f, positions[3 * v + 2] = -1.0f, v++;
    const uint32_t north = 0, south = (uint32_t)(vertex_count - 1);
    size_t t = 0;
#define AT(ring, segment) ((uint32_t)(1 + ((ring) - 1) * SEGMENTS + ((segment) % SEGMENTS)))
#define TRIANGLE(a, b, c) (triangles[3 * t] = (a), triangles[3 * t + 1] = (b), triangles[3 * t + 2] = (c), t++)
    for (int segment = 0; segment < SEGMENTS; segment++) {
        TRIANGLE(north, AT(1, segment), AT(1, segment + 1));
        TRIANGLE(south, AT(RINGS - 1, segment + 1), AT(RINGS - 1, segment));
        for (int ring = 1; ring < RINGS - 1; ring++) {
            TRIANGLE(AT(ring, segment), AT(ring + 1, segment), AT(ring + 1, segment + 1));
            TRIANGLE(AT(ring, segment), AT(ring + 1, segment + 1), AT(ring, segment + 1));
        }
    }
    check(v == vertex_count && t == triangle_count, "the ball has the vertices and triangles counted");

    /* What the shape calls for, without meshing it. */
    OpenclareAnalysis *analysis =
        openclare_analyse(positions, vertex_count, triangles, triangle_count, "organic", NULL, NULL);
    check(analysis != NULL, "an analysis is never NULL");
    check(openclare_analysis_ok(analysis) == 1, openclare_analysis_error(analysis));
    double largest = 0.0, smallest = 0.0, quads = 0.0, below = 0.0;
    check(openclare_analysis_proposal(analysis, &largest, &smallest) == 1, "the analysis proposes sizes");
    check(openclare_analysis_count(analysis, largest, smallest, &quads, &below) == 1, "the analysis counts quads");
    printf("analysis: quads from %g to %g, %.0f quads\n", smallest, largest, quads);
    openclare_analysis_free(analysis);

    /* The hard edges a remesh would follow: none on a smooth ball, at the
     * angle of the preset. */
    OpenclareHardEdges *hard = openclare_hard_edges(positions, vertex_count, triangles, triangle_count, NULL, 0, NULL,
                                                    "organic", "quads = 300\n");
    check(hard != NULL, "hard edges are never NULL");
    check(openclare_hard_edges_ok(hard) == 1, openclare_hard_edges_error(hard));
    size_t kept = openclare_hard_edges_kept_count(hard), left_out = openclare_hard_edges_left_out_count(hard);
    uint32_t *edges = malloc(2 * (kept + left_out + 1) * sizeof(uint32_t));
    openclare_hard_edges_copy_kept(hard, edges);
    openclare_hard_edges_copy_left_out(hard, edges);
    printf("hard edges: %zu kept, %zu left out\n", kept, left_out);
    check(kept == 0 && left_out == 0, "a smooth ball has no hard edge");
    free(edges);
    openclare_hard_edges_free(hard);

    /* The remesh itself. */
    char settings[64];
    snprintf(settings, sizeof settings, "quads = %d\n", QUADS);
    OpenclareResult *result = openclare_remesh(positions, vertex_count, triangles, triangle_count, NULL, 0, NULL,
                                               NULL, NULL, NULL, NULL, 0, "organic", settings, on_stage, &stages_seen);
    check(result != NULL, "a result is never NULL");
    check(openclare_result_ok(result) == 1, openclare_result_error(result));
    check(strcmp(openclare_result_error(result), "") == 0, "no error in a mesh that was made");
    check(stages_seen > 0, "the stages were followed");

    size_t vertices = openclare_result_vertex_count(result);
    size_t faces = openclare_result_face_count(result);
    size_t corners = openclare_result_corner_count(result);
    printf("remesh: %zu vertices, %zu faces, %zu corners, %d stages\n", vertices, faces, corners, (int)stages_seen);
    check(faces > (size_t)QUADS / 2 && faces < (size_t)QUADS * 2, "about the quads asked for");
    check(corners == 4 * faces, "quads only");

    float *out_positions = malloc(3 * vertices * sizeof(float));
    uint32_t *sizes = malloc(faces * sizeof(uint32_t));
    uint32_t *out_corners = malloc(corners * sizeof(uint32_t));
    float *uv = malloc(2 * corners * sizeof(float));
    openclare_result_copy_positions(result, out_positions);
    openclare_result_copy_faces(result, sizes, out_corners);
    for (size_t i = 0; i < vertices; i++) {
        float x = out_positions[3 * i], y = out_positions[3 * i + 1], z = out_positions[3 * i + 2];
        if (fabs(sqrt(x * x + y * y + z * z) - 1.0) > 0.05) {
            check(0, "every vertex on the ball");
            break;
        }
    }
    for (size_t i = 0; i < faces; i++) {
        if (sizes[i] != 4) {
            check(0, "every face has four corners");
            break;
        }
    }
    for (size_t i = 0; i < corners; i++) {
        if (out_corners[i] >= vertices) {
            check(0, "every corner names a vertex");
            break;
        }
    }
    check(openclare_result_copy_uv(result, OPENCLARE_UV_ATLAS, uv) == 1, "an atlas of texture coordinates");
    check(openclare_result_copy_uv(result, 7, uv) == 0, "no layer 7");
    check(strstr(openclare_result_report(result), "quads_asked = ") != NULL, "the report says what was asked");
    openclare_result_free(result);

    /* An error comes back as the error of the result, not as a crash. */
    OpenclareResult *refused = openclare_remesh(positions, vertex_count, triangles, triangle_count, NULL, 0, NULL, NULL,
                                                NULL, NULL, NULL, 0, "no such preset", NULL, NULL, NULL);
    check(openclare_result_ok(refused) == 0, "an unknown preset is refused");
    printf("refused: %s\n", openclare_result_error(refused));
    openclare_result_free(refused);

    /* NULL handles are allowed everywhere. */
    check(openclare_result_ok(NULL) == 0 && openclare_result_vertex_count(NULL) == 0, "a NULL result is empty");
    openclare_result_free(NULL);
    openclare_baked_free(NULL);
    openclare_analysis_free(NULL);
    check(openclare_hard_edges_ok(NULL) == 0 && openclare_hard_edges_kept_count(NULL) == 0, "NULL hard edges are empty");
    openclare_hard_edges_free(NULL);

    free(uv);
    free(out_corners);
    free(sizes);
    free(out_positions);
    free(triangles);
    free(positions);
    if (failures) {
        fprintf(stderr, "%d failures\n", failures);
        return 1;
    }
    printf("all good\n");
    return 0;
}
