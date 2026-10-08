# Bench results

Requested budget: 3000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. Lower is better for every metric; "—" means it could not be measured.

## blob (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 38 | 143 | 26 | 24 | 24 | 26 | 24 |
| … of which above the minimum | 30 | 135 | 18 | 16 | 16 | 18 | 16 |
| Base complex patches (C) | 2697 | 3048 | 951 | 2361 | 2045 | 1826 | 1350 |
| Alignment, degrees (A) | 17.48 | 15.74 | 6.23 | 5.36 | 4.90 | 3.93 | 4.55 |
| Mean angle deviation, degrees (F) | 8.71 | 5.87 | 7.91 | 6.16 | 5.68 | 5.76 | 5.45 |
| Angle deviation, 95th percentile (F) | 27.30 | 18.26 | 21.21 | 16.82 | 15.30 | 17.21 | 15.61 |
| Mean aspect ratio (F) | 1.187 | 1.174 | 1.428 | 1.414 | 1.370 | 1.618 | 1.276 |
| Area variation (F) | 0.205 | 0.175 | 0.352 | 0.376 | 0.396 | 0.442 | 0.313 |
| Hausdorff, ‰ of the diagonal (H) | 8.52 | 10.14 | 7.34 | 4.40 | 3.50 | 4.21 | 7.28 |
| Mean distance, ‰ (H) | 0.486 | 0.459 | 0.582 | 0.459 | 0.456 | 0.407 | 0.513 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.56 | 99.63 | 99.39 | 99.45 | 99.44 | 99.40 | 99.52 |
| Off budget, % (N) | 7.3 | 3.0 | 16.4 | 0.7 | 0.2 | 12.3 | 8.2 |
| Time, seconds (T) | 2.63 | 1.71 | 4.48 | 2.49 | 2.48 | 16.38 | 12.28 |

## rock (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 91 | 50 | 50 | 66 | 114 | 101 |
| … of which above the minimum | 0 | 83 | 42 | 42 | 58 | 106 | 93 |
| Base complex patches (C) | 2855 | 2896 | 2687 | 2968 | 2984 | 3539 | 2531 |
| Alignment, degrees (A) | 22.96 | 23.21 | 15.80 | 16.38 | 15.74 | 14.17 | 14.68 |
| Mean angle deviation, degrees (F) | 5.84 | 5.18 | 8.83 | 5.30 | 5.00 | 10.07 | 10.61 |
| Angle deviation, 95th percentile (F) | 16.05 | 14.78 | 22.19 | 13.73 | 13.17 | 24.88 | 26.21 |
| Mean aspect ratio (F) | 1.164 | 1.146 | 1.253 | 1.211 | 1.211 | 1.470 | 1.393 |
| Area variation (F) | 0.202 | 0.151 | 0.227 | 0.269 | 0.290 | 0.308 | 0.269 |
| Hausdorff, ‰ of the diagonal (H) | 5.45 | 5.28 | 5.55 | 5.70 | 5.60 | 5.13 | 5.79 |
| Mean distance, ‰ (H) | 0.788 | 0.820 | 0.861 | 0.791 | 0.787 | 0.731 | 0.889 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.78 | 99.70 | 99.68 | 99.74 | 99.72 | 99.73 | 99.62 |
| Off budget, % (N) | 2.2 | 3.4 | 10.4 | 1.1 | 0.5 | 18.0 | 15.6 |
| Time, seconds (T) | 1.41 | 1.03 | 6.92 | 1.90 | 2.58 | 20.49 | 16.98 |

## creature (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 24 | 111 | 41 | 40 | 40 | 42 | 46 |
| … of which above the minimum | 16 | 103 | 33 | 32 | 32 | 34 | 38 |
| Base complex patches (C) | 2667 | 2856 | 913 | 1301 | 2839 | 1465 | 1301 |
| Alignment, degrees (A) | 12.89 | 12.09 | 6.95 | 6.95 | 5.75 | 5.18 | 5.32 |
| Mean angle deviation, degrees (F) | 6.85 | 4.65 | 8.17 | 5.70 | 5.07 | 6.12 | 6.53 |
| Angle deviation, 95th percentile (F) | 18.98 | 14.73 | 23.87 | 14.93 | 13.48 | 17.13 | 18.98 |
| Mean aspect ratio (F) | 1.189 | 1.144 | 1.300 | 1.277 | 1.293 | 1.332 | 1.334 |
| Area variation (F) | 0.237 | 0.146 | 0.289 | 0.340 | 0.498 | 0.348 | 0.313 |
| Hausdorff, ‰ of the diagonal (H) | 8.01 | 3.47 | 3.16 | 2.38 | 2.45 | 2.51 | 2.37 |
| Mean distance, ‰ (H) | 0.320 | 0.279 | 0.370 | 0.295 | 0.302 | 0.297 | 0.348 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.41 | 99.47 | 99.23 | 99.41 | 99.38 | 99.39 | 99.28 |
| Off budget, % (N) | 11.1 | 2.3 | 13.4 | 1.4 | 1.5 | 2.0 | 9.4 |
| Time, seconds (T) | 1.69 | 0.93 | 6.12 | 2.35 | 2.47 | 11.24 | 9.11 |

## cage (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 66 | 200 | 41 | 48 | 48 | 44 | 43 |
| … of which above the minimum | 34 | 168 | 9 | 16 | 16 | 12 | 11 |
| Base complex patches (C) | 1943 | 2319 | 427 | 689 | 304 | 1092 | 1001 |
| Alignment, degrees (A) | 8.10 | 7.97 | 3.39 | 3.15 | 2.31 | 2.04 | 2.57 |
| Mean angle deviation, degrees (F) | 5.86 | 6.39 | 3.96 | 2.90 | 3.20 | 3.91 | 4.30 |
| Angle deviation, 95th percentile (F) | 17.11 | 23.33 | 12.27 | 8.99 | 10.08 | 11.90 | 12.82 |
| Mean aspect ratio (F) | 1.183 | 1.252 | 1.203 | 1.183 | 1.230 | 1.993 | 1.318 |
| Area variation (F) | 0.147 | 0.203 | 0.190 | 0.205 | 0.332 | 0.314 | 0.238 |
| Hausdorff, ‰ of the diagonal (H) | 11.45 | 14.46 | 16.28 | 13.74 | 14.20 | 5.74 | 9.66 |
| Mean distance, ‰ (H) | 0.734 | 0.860 | 1.017 | 0.924 | 0.681 | 0.693 | 1.276 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.99 | 98.70 | 98.43 | 98.62 | 98.95 | 99.00 | 97.85 |
| Off budget, % (N) | 3.6 | 1.9 | 4.2 | 1.3 | 1.1 | 93.6 | 7.2 |
| Time, seconds (T) | 1.57 | 0.34 | 7.14 | 1.89 | 3.04 | 4.21 | 3.92 |

## suzanne (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 2/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 78 | 212 | 52 | 53 | 56 | 80 | 71 |
| … of which above the minimum | 54 | — | — | — | — | — | — |
| Base complex patches (C) | 2884 | 3002 | 1858 | 2472 | 2000 | 2513 | 1838 |
| Alignment, degrees (A) | 15.33 | 17.02 | 10.40 | 10.26 | 9.82 | 8.75 | 10.24 |
| Mean angle deviation, degrees (F) | 8.61 | 7.08 | 7.14 | 4.28 | 4.31 | 7.87 | 7.87 |
| Angle deviation, 95th percentile (F) | 26.08 | 25.30 | 18.15 | 11.60 | 11.96 | 21.85 | 20.46 |
| Mean aspect ratio (F) | 1.260 | 1.263 | 1.223 | 1.213 | 1.202 | 1.668 | 1.284 |
| Area variation (F) | 0.837 | 0.221 | 0.243 | 0.271 | 0.381 | 0.473 | 0.236 |
| Hausdorff, ‰ of the diagonal (H) | 42.26 | 20.15 | 11.31 | 9.13 | 8.95 | 6.77 | 9.47 |
| Mean distance, ‰ (H) | 0.834 | 0.906 | 0.980 | 0.756 | 0.642 | 0.603 | 1.053 |
| Thin parts: mean distance, % of a quad edge (V) | 16.4 | 21.4 | 22.9 | 12.8 | 10.9 | 5.1 | 17.1 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | 1.0 | 2.3 | 8.1 | 0.7 | 0.5 | 43.7 | 12.4 |
| Time, seconds (T) | 3.22 | 0.28 | 8.08 | 1.98 | 3.06 | 3.44 | 3.21 |

## ogre (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 4/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 106 | 216 | 73 | 76 | 155 | 314 | 228 |
| … of which above the minimum | 98 | 202 | — | 68 | 147 | 306 | 220 |
| Base complex patches (C) | 2857 | 2802 | — | 2727 | 2483 | 5170 | 2009 |
| Alignment, degrees (A) | 19.31 | 18.57 | 12.62 | 12.22 | 11.63 | 8.42 | 10.81 |
| Mean angle deviation, degrees (F) | 12.01 | 7.49 | 9.90 | 6.46 | 6.20 | 9.10 | 11.45 |
| Angle deviation, 95th percentile (F) | 32.96 | 24.38 | 27.27 | 18.16 | 17.66 | 25.01 | 30.48 |
| Mean aspect ratio (F) | 1.403 | 1.283 | 1.557 | 1.558 | 1.400 | 2.364 | 1.612 |
| Area variation (F) | 0.324 | 0.224 | 0.412 | 0.384 | 0.516 | 0.716 | 0.393 |
| Hausdorff, ‰ of the diagonal (H) | 36.59 | 46.14 | 37.31 | 27.60 | 27.31 | 5.98 | 39.35 |
| Mean distance, ‰ (H) | 1.268 | 1.460 | 1.394 | 0.991 | 0.944 | 0.565 | 1.366 |
| Thin parts: mean distance, % of a quad edge (V) | 54.0 | 99.5 | 67.6 | 30.5 | 9.1 | 6.7 | 75.2 |
| Volume kept, % of the source (V) | 98.56 | 98.55 | 98.38 | 98.87 | 98.85 | 99.24 | 98.42 |
| Off budget, % (N) | 4.3 | 4.2 | 17.7 | 2.0 | 0.6 | 104.5 | 19.4 |
| Time, seconds (T) | 3.94 | 2.70 | 8.44 | 4.14 | 6.42 | 19.04 | 17.16 |

## spot (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 68 | 153 | 69 | 70 | 70 | 82 | 87 |
| … of which above the minimum | 60 | 145 | 61 | 62 | 62 | 74 | 79 |
| Base complex patches (C) | 2902 | 2978 | 1790 | 1730 | 1663 | 2428 | 2118 |
| Alignment, degrees (A) | 18.12 | 17.17 | 9.23 | 8.70 | 7.87 | 6.29 | 7.56 |
| Mean angle deviation, degrees (F) | 7.25 | 5.59 | 8.93 | 5.59 | 5.60 | 6.35 | 7.05 |
| Angle deviation, 95th percentile (F) | 20.42 | 18.71 | 23.29 | 14.97 | 14.87 | 17.74 | 19.43 |
| Mean aspect ratio (F) | 1.194 | 1.179 | 1.356 | 1.341 | 1.340 | 1.460 | 1.318 |
| Area variation (F) | 0.177 | 0.172 | 0.277 | 0.324 | 0.427 | 0.392 | 0.268 |
| Hausdorff, ‰ of the diagonal (H) | 17.71 | 19.38 | 7.35 | 6.29 | 7.46 | 6.64 | 10.63 |
| Mean distance, ‰ (H) | 0.448 | 0.479 | 0.477 | 0.416 | 0.422 | 0.372 | 0.476 |
| Thin parts: mean distance, % of a quad edge (V) | 56.3 | 53.8 | 7.4 | 7.0 | 7.2 | 8.5 | 21.2 |
| Volume kept, % of the source (V) | 99.37 | 99.35 | 99.23 | 99.32 | 99.29 | 99.35 | 99.27 |
| Off budget, % (N) | 1.2 | 1.5 | 10.5 | 1.6 | 0.7 | 15.9 | 10.5 |
| Time, seconds (T) | 2.06 | 1.23 | 7.63 | 2.82 | 4.57 | 19.14 | 15.16 |

## bob (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 32 | 130 | 20 | 20 | 20 | 28 | 30 |
| … of which above the minimum | 32 | 130 | 20 | 20 | 20 | 28 | 30 |
| Base complex patches (C) | 2903 | 3040 | 714 | 1290 | 237 | 2390 | 2568 |
| Alignment, degrees (A) | 19.37 | 20.80 | 4.35 | 4.00 | 3.61 | 4.85 | 5.15 |
| Mean angle deviation, degrees (F) | 6.41 | 5.33 | 4.82 | 3.28 | 3.24 | 5.52 | 5.94 |
| Angle deviation, 95th percentile (F) | 18.61 | 17.96 | 14.62 | 8.81 | 9.07 | 15.59 | 15.77 |
| Mean aspect ratio (F) | 1.175 | 1.158 | 1.409 | 1.385 | 1.370 | 1.368 | 1.360 |
| Area variation (F) | 0.159 | 0.161 | 0.338 | 0.373 | 0.432 | 0.360 | 0.324 |
| Hausdorff, ‰ of the diagonal (H) | 6.82 | 7.32 | 3.63 | 2.49 | 2.10 | 1.96 | 2.27 |
| Mean distance, ‰ (H) | 0.309 | 0.325 | 0.378 | 0.328 | 0.336 | 0.322 | 0.355 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.48 | 99.44 | 99.25 | 99.36 | 99.34 | 99.37 | 99.31 |
| Off budget, % (N) | 2.4 | 1.3 | 11.9 | 1.4 | 1.7 | 2.7 | 10.1 |
| Time, seconds (T) | 2.03 | 1.45 | 6.58 | 2.12 | 3.91 | 17.24 | 17.35 |

## blub (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 52 | 158 | 44 | 44 | 58 | 55 | 46 |
| … of which above the minimum | 44 | 150 | — | 36 | 50 | 47 | 38 |
| Base complex patches (C) | 2584 | 2922 | — | 1287 | 1333 | 2003 | 2057 |
| Alignment, degrees (A) | 15.95 | 15.45 | 8.38 | 7.86 | 7.52 | 5.26 | 7.01 |
| Mean angle deviation, degrees (F) | 10.72 | 5.55 | 8.60 | 5.15 | 4.84 | 6.30 | 7.24 |
| Angle deviation, 95th percentile (F) | 28.70 | 19.72 | 23.94 | 15.61 | 14.41 | 16.74 | 18.88 |
| Mean aspect ratio (F) | 1.271 | 1.192 | 1.336 | 1.296 | 1.302 | 1.828 | 1.351 |
| Area variation (F) | 0.261 | 0.178 | 0.261 | 0.296 | 0.419 | 0.445 | 0.269 |
| Hausdorff, ‰ of the diagonal (H) | 40.70 | 15.68 | 14.68 | 10.36 | 8.71 | 5.19 | 11.32 |
| Mean distance, ‰ (H) | 0.588 | 0.390 | 0.456 | 0.343 | 0.324 | 0.230 | 0.392 |
| Thin parts: mean distance, % of a quad edge (V) | 46.5 | 24.8 | 31.7 | 14.2 | 10.6 | 7.7 | 20.0 |
| Volume kept, % of the source (V) | 98.93 | 99.17 | 99.00 | 99.23 | 99.28 | 99.55 | 99.13 |
| Off budget, % (N) | 10.1 | 1.2 | 11.6 | 0.2 | 0.9 | 29.6 | 9.9 |
| Time, seconds (T) | 1.82 | 1.25 | 6.03 | 2.25 | 3.29 | 14.34 | 12.32 |

## pig (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 122 | 209 | 73 | 78 | 102 | 106 | 96 |
| … of which above the minimum | 114 | 192 | — | 70 | 94 | 98 | 88 |
| Base complex patches (C) | 2612 | 2912 | — | 2401 | 1849 | 3004 | 1840 |
| Alignment, degrees (A) | 17.41 | 16.24 | 10.65 | 10.56 | 8.95 | 7.22 | 9.36 |
| Mean angle deviation, degrees (F) | 14.32 | 6.83 | 7.67 | 5.87 | 5.19 | 7.22 | 7.93 |
| Angle deviation, 95th percentile (F) | 41.52 | 23.86 | 22.18 | 16.42 | 15.88 | 21.68 | 23.91 |
| Mean aspect ratio (F) | 1.383 | 1.244 | 1.251 | 1.239 | 1.271 | 1.876 | 1.322 |
| Area variation (F) | 0.398 | 0.210 | 0.243 | 0.293 | 0.473 | 0.636 | 0.287 |
| Hausdorff, ‰ of the diagonal (H) | 44.59 | 48.32 | 23.75 | 18.14 | 10.87 | 10.22 | 12.72 |
| Mean distance, ‰ (H) | 1.056 | 1.012 | 0.860 | 0.607 | 0.525 | 0.375 | 0.724 |
| Thin parts: mean distance, % of a quad edge (V) | 67.8 | 26.2 | 37.6 | 19.3 | 5.4 | 3.7 | 16.4 |
| Volume kept, % of the source (V) | 98.78 | 98.84 | 98.78 | 99.18 | 99.28 | 99.54 | 98.93 |
| Off budget, % (N) | 10.8 | 1.1 | 9.9 | 1.6 | 1.0 | 56.6 | 14.9 |
| Time, seconds (T) | 1.94 | 1.15 | 9.47 | 2.21 | 3.29 | 14.59 | 16.02 |

## jerry (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 104 | 265 | 95 | 86 | 162 | 206 | 181 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | 2211 | 2959 | 1963 | 2043 | 2491 | — | 2088 |
| Alignment, degrees (A) | 17.93 | 18.35 | 15.03 | 14.30 | 13.54 | 11.02 | 13.58 |
| Mean angle deviation, degrees (F) | 11.25 | 8.58 | 9.56 | 6.30 | 6.61 | 9.67 | 10.03 |
| Angle deviation, 95th percentile (F) | 29.73 | 29.29 | 24.76 | 16.07 | 20.85 | 26.54 | 28.28 |
| Mean aspect ratio (F) | 1.301 | 1.357 | 1.329 | 1.268 | 1.332 | 2.063 | 1.483 |
| Area variation (F) | 0.278 | 0.258 | 0.272 | 0.309 | 0.559 | 0.724 | 0.324 |
| Hausdorff, ‰ of the diagonal (H) | 38.82 | 49.21 | 44.34 | 31.62 | 22.51 | 6.74 | 15.07 |
| Mean distance, ‰ (H) | 1.467 | 2.158 | 2.165 | 1.372 | 1.107 | 0.529 | 1.488 |
| Thin parts: mean distance, % of a quad edge (V) | 23.4 | 40.3 | 58.6 | 31.3 | 16.8 | 4.8 | 19.2 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | 4.1 | 1.4 | 12.8 | 0.0 | 1.8 | 129.1 | 14.2 |
| Time, seconds (T) | 2.10 | 0.59 | 6.24 | 3.03 | 2.46 | 6.74 | 6.79 |

## rose (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | 539 | 499 | 236 | 911 | 1119 | 1772 | 763 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | 1872 | 1954 | — | — | 3248 | — | — |
| Alignment, degrees (A) | 18.39 | 19.07 | 16.09 | 14.64 | 14.61 | 8.98 | 14.12 |
| Mean angle deviation, degrees (F) | 16.45 | 15.58 | 12.48 | 19.74 | 21.02 | 9.06 | 13.63 |
| Angle deviation, 95th percentile (F) | 48.25 | 48.45 | 37.87 | 63.27 | 66.86 | 26.38 | 39.32 |
| Mean aspect ratio (F) | 1.647 | 1.752 | 1.763 | 3.866 | 4.066 | 1.790 | 1.653 |
| Area variation (F) | 0.587 | 0.465 | 0.440 | 0.895 | 0.917 | 1.388 | 0.599 |
| Hausdorff, ‰ of the diagonal (H) | 75.61 | 79.25 | 75.28 | 35.29 | 26.44 | 74.46 | 78.76 |
| Mean distance, ‰ (H) | 3.772 | 4.183 | 3.990 | 2.950 | 2.430 | 0.842 | 3.206 |
| Thin parts: mean distance, % of a quad edge (V) | 39.1 | 46.1 | 43.3 | 28.6 | 22.6 | 11.9 | 35.3 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | 30.5 | 25.8 | 31.3 | 8.1 | 18.9 | 1095.8 | 10.4 |
| Time, seconds (T) | 2.77 | 2.12 | 24.41 | 9.51 | 12.93 | 9.53 | 10.86 |

## bridge (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 5/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 |
| Irregular vertices (S) | — | 151 | 50 | 50 | 46 | 81 | 64 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 1879 | 763 | 1066 | 764 | — | — |
| Alignment, degrees (A) | — | 12.58 | 8.36 | 8.02 | 7.42 | 6.49 | 7.93 |
| Mean angle deviation, degrees (F) | — | 6.12 | 6.49 | 3.77 | 3.90 | 6.65 | 7.20 |
| Angle deviation, 95th percentile (F) | — | 20.51 | 18.53 | 11.08 | 11.53 | 18.96 | 19.72 |
| Mean aspect ratio (F) | — | 1.215 | 1.302 | 1.244 | 1.247 | 1.753 | 1.413 |
| Area variation (F) | — | 0.174 | 0.238 | 0.294 | 0.436 | 0.421 | 0.275 |
| Hausdorff, ‰ of the diagonal (H) | — | 28.19 | 6.91 | 6.89 | 6.65 | 6.05 | 7.00 |
| Mean distance, ‰ (H) | — | 0.638 | 0.680 | 0.597 | 0.577 | 0.414 | 0.670 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 3.2 | 10.6 | 1.6 | 1.4 | 51.1 | 9.1 |
| Time, seconds (T) | — | 2.98 | 8.82 | 3.89 | 3.37 | 11.41 | 11.88 |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 28 | 204 | 24 | 24 | 24 | 24 | 24 |
| … of which above the minimum | 20 | 196 | 16 | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 619 | 1782 | 78 | 78 | 285 | 501 | 501 |
| Alignment, degrees (A) | 12.97 | 13.96 | 11.66 | 11.96 | 10.48 | 10.90 | 10.90 |
| Mean angle deviation, degrees (F) | 2.66 | 6.10 | 2.61 | 1.35 | 2.25 | 3.02 | 3.02 |
| Angle deviation, 95th percentile (F) | 10.11 | 25.03 | 10.79 | 4.82 | 6.23 | 12.04 | 12.04 |
| Mean aspect ratio (F) | 1.116 | 1.233 | 1.164 | 1.159 | 1.162 | 1.326 | 1.326 |
| Area variation (F) | 0.100 | 0.199 | 0.126 | 0.113 | 0.365 | 0.196 | 0.196 |
| Hausdorff, ‰ of the diagonal (H) | 5.54 | 9.41 | 0.88 | 1.07 | 0.59 | 0.89 | 0.89 |
| Mean distance, ‰ (H) | 0.114 | 0.067 | 0.053 | 0.052 | 0.056 | 0.068 | 0.068 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.15 | 100.14 | 100.13 | 100.13 | 100.08 | 100.12 | 100.12 |
| Off budget, % (N) | 3.7 | 2.1 | 0.8 | 1.6 | 0.7 | 6.2 | 6.2 |
| Time, seconds (T) | 2.91 | 0.71 | 4.80 | 1.86 | 2.04 | 2.83 | 2.88 |

## cube (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 24 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 16 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 6 | 274 | 1102 | 6 | 6 | 6 | 6 |
| Alignment, degrees (A) | 0.29 | 1.97 | 0.75 | 0.14 | 0.36 | 0.13 | 0.13 |
| Mean angle deviation, degrees (F) | 0.18 | 1.65 | 0.75 | 0.01 | 0.36 | 0.00 | 0.00 |
| Angle deviation, 95th percentile (F) | 0.43 | 4.89 | 1.98 | 0.02 | 0.85 | 0.00 | 0.00 |
| Mean aspect ratio (F) | 1.005 | 1.062 | 1.041 | 1.031 | 1.088 | 1.030 | 1.030 |
| Area variation (F) | 0.004 | 0.090 | 0.048 | 0.021 | 0.098 | 0.021 | 0.021 |
| Hausdorff, ‰ of the diagonal (H) | 4.80 | 0.00 | 11.17 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.017 | 0.000 | 0.047 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 100.00 | 99.96 | 100.00 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 3.2 | 1.0 | 0.3 | 0.3 | 0.3 | 2.7 | 2.7 |
| Time, seconds (T) | 0.77 | 0.20 | 6.63 | 1.82 | 2.37 | 2.97 | 3.00 |

## lblock (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 12 | 24 | 12 | 12 | 12 | 12 | 12 |
| … of which above the minimum | 4 | 16 | 4 | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 86 | 464 | 14 | 14 | 14 | 14 | 14 |
| Alignment, degrees (A) | 1.09 | 1.54 | 0.71 | 0.52 | 0.86 | 0.63 | 0.63 |
| Mean angle deviation, degrees (F) | 0.80 | 1.56 | 0.67 | 0.01 | 0.53 | 0.37 | 0.37 |
| Angle deviation, 95th percentile (F) | 2.44 | 4.54 | 2.69 | 0.05 | 1.43 | 1.43 | 1.43 |
| Mean aspect ratio (F) | 1.047 | 1.051 | 1.058 | 1.030 | 1.097 | 1.037 | 1.037 |
| Area variation (F) | 0.039 | 0.063 | 0.036 | 0.033 | 0.106 | 0.021 | 0.021 |
| Hausdorff, ‰ of the diagonal (H) | 10.33 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.034 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 2.4 | 7.3 | 2.9 | 2.0 | 0.9 | 1.0 | 1.0 |
| Time, seconds (T) | 1.39 | 0.36 | 56.66 | 1.25 | 1.32 | 2.73 | 2.90 |

## cylinder (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 91 | 13 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 83 | 5 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 1003 | 3052 | 736 | 722 | 463 | 408 | 408 |
| Alignment, degrees (A) | 4.18 | 5.46 | 1.60 | 1.50 | 1.63 | 1.52 | 1.52 |
| Mean angle deviation, degrees (F) | 4.32 | 4.18 | 4.88 | 3.59 | 2.70 | 3.22 | 3.22 |
| Angle deviation, 95th percentile (F) | 17.78 | 15.60 | 22.84 | 13.76 | 8.52 | 11.87 | 11.87 |
| Mean aspect ratio (F) | 1.087 | 1.132 | 1.407 | 1.388 | 1.532 | 1.511 | 1.511 |
| Area variation (F) | 0.103 | 0.140 | 0.320 | 0.375 | 0.233 | 0.403 | 0.403 |
| Hausdorff, ‰ of the diagonal (H) | 2.07 | 3.74 | 0.98 | 0.94 | 0.93 | 1.43 | 1.43 |
| Mean distance, ‰ (H) | 0.144 | 0.121 | 0.219 | 0.176 | 0.165 | 0.369 | 0.369 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.91 | 99.90 | 99.77 | 99.80 | 99.82 | 99.56 | 99.56 |
| Off budget, % (N) | 3.3 | 3.3 | 12.0 | 0.5 | 1.3 | 18.1 | 18.1 |
| Time, seconds (T) | 1.79 | 0.17 | 3.24 | 1.60 | 2.35 | 2.68 | 2.49 |

## terrain (extra)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 112 | 115 | 55 | 56 | 99 | 231 | 167 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | 826 | 1209 | 997 | 1681 | 2281 | — | 1662 |
| Alignment, degrees (A) | 20.17 | 21.17 | 18.69 | 18.56 | 18.63 | 18.30 | 18.61 |
| Mean angle deviation, degrees (F) | 6.49 | 8.24 | 9.46 | 6.16 | 6.61 | 9.57 | 8.13 |
| Angle deviation, 95th percentile (F) | 23.46 | 27.33 | 27.18 | 17.46 | 19.83 | 26.86 | 23.75 |
| Mean aspect ratio (F) | 1.209 | 1.291 | 1.347 | 1.292 | 1.342 | 1.806 | 1.429 |
| Area variation (F) | 0.187 | 0.215 | 0.283 | 0.317 | 0.460 | 0.552 | 0.247 |
| Hausdorff, ‰ of the diagonal (H) | 23.91 | 143.82 | 26.92 | 21.40 | 19.20 | 12.55 | 25.16 |
| Mean distance, ‰ (H) | 1.851 | 2.384 | 2.069 | 1.537 | 1.323 | 0.967 | 1.780 |
| Thin parts: mean distance, % of a quad edge (V) | 32.8 | 36.9 | 42.7 | 28.9 | 22.7 | 14.4 | 32.3 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | 9.4 | 41.6 | 16.6 | 1.1 | 1.2 | 55.9 | 13.4 |
| Time, seconds (T) | 22.93 | 8.92 | 14.73 | 7.88 | 8.74 | 21.17 | 19.56 |

## mahaveer (extra)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | — | 175 | 65 | 453 | 455 | 3076 | 1079 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 1227 | 388 | 714 | 728 | — | — |
| Alignment, degrees (A) | — | 9.97 | 7.00 | 14.13 | 14.24 | 13.54 | 11.62 |
| Mean angle deviation, degrees (F) | — | 9.66 | 7.26 | 26.19 | 26.51 | 12.36 | 12.56 |
| Angle deviation, 95th percentile (F) | — | 34.02 | 25.91 | 79.74 | 83.44 | 39.42 | 48.70 |
| Mean aspect ratio (F) | — | 1.583 | 1.460 | 88.317 | 131.246 | 1.824 | 1.786 |
| Area variation (F) | — | 0.344 | 0.400 | 1.575 | 1.570 | 2.664 | 0.975 |
| Hausdorff, ‰ of the diagonal (H) | — | 52.26 | 30.70 | 41.14 | 41.14 | 13.60 | 23.83 |
| Mean distance, ‰ (H) | — | 7.890 | 4.238 | 3.418 | 3.440 | 0.929 | 2.320 |
| Thin parts: mean distance, % of a quad edge (V) | — | 75.1 | 40.0 | 29.8 | 29.7 | 11.6 | 21.9 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 43.4 | 29.1 | 0.4 | 0.7 | 1440.1 | 44.1 |
| Time, seconds (T) | — | 7.16 | 23.94 | 11.37 | 18.52 | 14.10 | 14.84 |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | — | 616 | — | 5577 | 5538 | 10315 | 4939 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 1136 | — | — | — | — | — |
| Alignment, degrees (A) | — | 17.78 | — | 12.17 | 12.66 | 10.18 | 13.63 |
| Mean angle deviation, degrees (F) | — | 23.80 | — | 26.27 | 26.41 | 9.86 | 17.03 |
| Angle deviation, 95th percentile (F) | — | 60.14 | — | 74.74 | 74.04 | 29.30 | 51.38 |
| Mean aspect ratio (F) | — | 2.545 | — | 7.329 | 14.891 | 1.580 | 1.889 |
| Area variation (F) | — | 0.789 | — | 2.632 | 2.459 | 2.386 | 2.005 |
| Hausdorff, ‰ of the diagonal (H) | — | 7.35 | — | 6.26 | 6.25 | 1.80 | 3.03 |
| Mean distance, ‰ (H) | — | 1.806 | — | 0.581 | 0.596 | 0.123 | 0.419 |
| Thin parts: mean distance, % of a quad edge (V) | — | 57.0 | — | 16.2 | 16.6 | 7.6 | 14.6 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 59.2 | — | 116.2 | 112.1 | 7033.9 | 278.0 |
| Time, seconds (T) | — | 4.00 | — | 37.95 | 55.61 | 12.21 | 10.92 |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 545 non-manifold or inconsistently oriented edges.

## sphere (calibration)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 97 | 32 | 32 | 32 | 10 | 10 |
| … of which above the minimum | 0 | 89 | 24 | 24 | 24 | 2 | 2 |
| Base complex patches (C) | 2730 | 3090 | 30 | 30 | 30 | 1794 | 1794 |
| Alignment, degrees (A) | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.49 | 4.19 | 8.38 | 3.93 | 3.94 | 6.09 | 6.09 |
| Angle deviation, 95th percentile (F) | 12.67 | 14.49 | 17.63 | 7.43 | 7.44 | 15.49 | 15.49 |
| Mean aspect ratio (F) | 1.125 | 1.124 | 1.081 | 1.083 | 1.083 | 1.185 | 1.185 |
| Area variation (F) | 0.135 | 0.136 | 0.190 | 0.302 | 0.302 | 0.312 | 0.312 |
| Hausdorff, ‰ of the diagonal (H) | 0.63 | 0.72 | 0.68 | 0.86 | 0.86 | 0.82 | 0.82 |
| Mean distance, ‰ (H) | 0.216 | 0.221 | 0.230 | 0.223 | 0.223 | 0.258 | 0.258 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.81 | 99.79 | 99.78 | 99.78 | 99.78 | 99.74 | 99.74 |
| Off budget, % (N) | 3.6 | 3.0 | 0.0 | 0.0 | 0.0 | 8.7 | 8.7 |
| Time, seconds (T) | 1.80 | 0.34 | 1.59 | 1.44 | 1.18 | 3.56 | 3.39 |

## torus (calibration)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 30 | 125 | 0 | 0 | 0 | 10 | 10 |
| … of which above the minimum | 30 | 125 | 0 | 0 | 0 | 10 | 10 |
| Base complex patches (C) | 2976 | 3078 | 1 | 1 | 1 | 2264 | 2264 |
| Alignment, degrees (A) | 10.14 | 14.28 | 0.65 | 0.62 | 0.62 | 6.90 | 6.90 |
| Mean angle deviation, degrees (F) | 10.25 | 5.57 | 1.45 | 1.44 | 1.44 | 5.23 | 5.23 |
| Angle deviation, 95th percentile (F) | 26.29 | 18.30 | 2.88 | 2.36 | 2.36 | 14.40 | 14.40 |
| Mean aspect ratio (F) | 1.239 | 1.163 | 1.352 | 1.323 | 1.323 | 1.213 | 1.213 |
| Area variation (F) | 0.223 | 0.163 | 0.279 | 0.307 | 0.307 | 0.233 | 0.233 |
| Hausdorff, ‰ of the diagonal (H) | 1.86 | 1.38 | 1.02 | 1.00 | 1.00 | 1.71 | 1.71 |
| Mean distance, ‰ (H) | 0.299 | 0.298 | 0.338 | 0.304 | 0.304 | 0.518 | 0.518 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.46 | 99.44 | 99.33 | 99.39 | 99.39 | 98.94 | 98.94 |
| Off budget, % (N) | 0.8 | 2.6 | 9.3 | 1.6 | 1.6 | 5.9 | 5.9 |
| Time, seconds (T) | 1.55 | 0.22 | 2.12 | 1.49 | 1.95 | 4.71 | 2.89 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 0 | 7 | 4 | 3 | 0 | 1 |
| Base complex patches (C) | 1 | 0 | 4 | 1 | 3 | 0 | 4 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 1 | 12 | 0 |
| Mean angle deviation, degrees (F) | 0 | 1 | 0 | 5 | 5 | 1 | 1 |
| Mean aspect ratio (F) | 1 | 8 | 0 | 3 | 1 | 0 | 0 |
| Valid meshes (V) | 36/65 | 47/65 | 9/13 | 12/13 | 12/13 | 8/13 | 10/13 |

### mechanical (4 models)

| Metric | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 0 | 3 | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 1 | 0 | 2 | 3 | 2 | 3 | 3 |
| Alignment, degrees (A) | 0 | 0 | 0 | 2 | 1 | 1 | 1 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 2 | 1 | 1 | 1 |
| Mean aspect ratio (F) | 3 | 0 | 0 | 1 | 0 | 0 | 0 |
| Valid meshes (V) | 18/20 | 20/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 |
