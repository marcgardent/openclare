# Bench results

Requested budget: 3000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. Lower is better for every metric; "—" means it could not be measured.

## blob (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 38 | 143 | 26 | 24 | 24 | 26 | 24 |
| … of which above the minimum | 30 | 135 | 18 | 16 | 16 | 18 | 16 |
| Base complex patches (C) | 2697 | 3048 | 951 | 2361 | 2045 | 1826 | 1350 |
| Alignment, degrees (A) | 17.48 | 15.74 | 6.23 | 5.35 | 4.89 | 3.93 | 4.55 |
| Mean angle deviation, degrees (F) | 8.71 | 5.87 | 7.89 | 6.14 | 5.67 | 5.76 | 5.45 |
| Angle deviation, 95th percentile (F) | 27.30 | 18.26 | 21.16 | 16.75 | 15.14 | 17.21 | 15.61 |
| Mean aspect ratio (F) | 1.187 | 1.174 | 1.427 | 1.413 | 1.369 | 1.618 | 1.276 |
| Area variation (F) | 0.205 | 0.175 | 0.351 | 0.374 | 0.394 | 0.442 | 0.313 |
| Hausdorff, ‰ of the diagonal (H) | 8.52 | 10.14 | 5.25 | 3.20 | 2.43 | 4.21 | 7.28 |
| Mean distance, ‰ (H) | 0.486 | 0.459 | 0.272 | 0.196 | 0.188 | 0.407 | 0.513 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.56 | 99.63 | 100.03 | 100.02 | 100.02 | 99.40 | 99.52 |
| Off budget, % (N) | 7.3 | 3.0 | 16.4 | 0.7 | 0.2 | 12.3 | 8.2 |
| Time, seconds (T) | 2.63 | 1.71 | 5.79 | 3.60 | 3.12 | 16.38 | 12.28 |

## rock (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 91 | 50 | 50 | 66 | 114 | 101 |
| … of which above the minimum | 0 | 83 | 42 | 42 | 58 | 106 | 93 |
| Base complex patches (C) | 2855 | 2896 | 2687 | 2968 | 2984 | 3539 | 2531 |
| Alignment, degrees (A) | 22.96 | 23.21 | 15.81 | 16.41 | 15.76 | 14.17 | 14.68 |
| Mean angle deviation, degrees (F) | 5.84 | 5.18 | 8.82 | 5.36 | 5.07 | 10.07 | 10.61 |
| Angle deviation, 95th percentile (F) | 16.05 | 14.78 | 22.11 | 13.98 | 13.33 | 24.88 | 26.21 |
| Mean aspect ratio (F) | 1.164 | 1.146 | 1.253 | 1.213 | 1.213 | 1.470 | 1.393 |
| Area variation (F) | 0.202 | 0.151 | 0.229 | 0.270 | 0.290 | 0.308 | 0.269 |
| Hausdorff, ‰ of the diagonal (H) | 5.45 | 5.28 | 4.49 | 4.60 | 4.65 | 5.13 | 5.79 |
| Mean distance, ‰ (H) | 0.788 | 0.820 | 0.740 | 0.698 | 0.695 | 0.731 | 0.889 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.78 | 99.70 | 100.01 | 100.00 | 100.00 | 99.73 | 99.62 |
| Off budget, % (N) | 2.2 | 3.4 | 10.4 | 1.1 | 0.5 | 18.0 | 15.6 |
| Time, seconds (T) | 1.41 | 1.03 | 8.10 | 2.76 | 2.84 | 20.49 | 16.98 |

## creature (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 24 | 111 | 41 | 40 | 40 | 42 | 46 |
| … of which above the minimum | 16 | 103 | 33 | 32 | 32 | 34 | 38 |
| Base complex patches (C) | 2667 | 2856 | 913 | 1301 | 2839 | 1465 | 1301 |
| Alignment, degrees (A) | 12.89 | 12.09 | 6.96 | 6.95 | 5.75 | 5.18 | 5.32 |
| Mean angle deviation, degrees (F) | 6.85 | 4.65 | 8.21 | 5.72 | 5.09 | 6.12 | 6.53 |
| Angle deviation, 95th percentile (F) | 18.98 | 14.73 | 24.05 | 15.03 | 13.57 | 17.13 | 18.98 |
| Mean aspect ratio (F) | 1.189 | 1.144 | 1.305 | 1.280 | 1.297 | 1.332 | 1.334 |
| Area variation (F) | 0.237 | 0.146 | 0.286 | 0.338 | 0.496 | 0.348 | 0.313 |
| Hausdorff, ‰ of the diagonal (H) | 8.01 | 3.47 | 2.16 | 2.06 | 2.05 | 2.51 | 2.37 |
| Mean distance, ‰ (H) | 0.320 | 0.279 | 0.231 | 0.191 | 0.201 | 0.297 | 0.348 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.41 | 99.47 | 100.00 | 100.02 | 100.03 | 99.39 | 99.28 |
| Off budget, % (N) | 11.1 | 2.3 | 13.4 | 1.4 | 1.5 | 2.0 | 9.4 |
| Time, seconds (T) | 1.69 | 0.93 | 7.49 | 3.99 | 3.06 | 11.24 | 9.11 |

## cage (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 66 | 200 | 41 | 48 | 48 | 44 | 43 |
| … of which above the minimum | 34 | 168 | 9 | 16 | 16 | 12 | 11 |
| Base complex patches (C) | 1943 | 2319 | 427 | 689 | 304 | 1092 | 1001 |
| Alignment, degrees (A) | 8.10 | 7.97 | 3.38 | 3.16 | 2.31 | 2.04 | 2.57 |
| Mean angle deviation, degrees (F) | 5.86 | 6.39 | 4.00 | 2.94 | 3.22 | 3.91 | 4.30 |
| Angle deviation, 95th percentile (F) | 17.11 | 23.33 | 12.37 | 8.99 | 10.15 | 11.90 | 12.82 |
| Mean aspect ratio (F) | 1.183 | 1.252 | 1.202 | 1.184 | 1.227 | 1.993 | 1.318 |
| Area variation (F) | 0.147 | 0.203 | 0.193 | 0.205 | 0.332 | 0.314 | 0.238 |
| Hausdorff, ‰ of the diagonal (H) | 11.45 | 14.46 | 11.74 | 10.64 | 10.57 | 5.74 | 9.66 |
| Mean distance, ‰ (H) | 0.734 | 0.860 | 0.814 | 0.790 | 0.494 | 0.693 | 1.276 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.99 | 98.70 | 99.89 | 99.93 | 99.98 | 99.00 | 97.85 |
| Off budget, % (N) | 3.6 | 1.9 | 4.2 | 1.3 | 1.1 | 93.6 | 7.2 |
| Time, seconds (T) | 1.57 | 0.34 | 8.60 | 1.89 | 3.25 | 4.21 | 3.92 |

## suzanne (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 2/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 78 | 212 | 52 | 53 | 56 | 80 | 71 |
| … of which above the minimum | 54 | — | — | — | — | — | — |
| Base complex patches (C) | 2884 | 3002 | 1858 | 2472 | 2000 | 2513 | 1838 |
| Alignment, degrees (A) | 15.33 | 17.02 | 10.46 | 10.35 | 9.87 | 8.75 | 10.24 |
| Mean angle deviation, degrees (F) | 8.61 | 7.08 | 7.14 | 4.34 | 4.31 | 7.87 | 7.87 |
| Angle deviation, 95th percentile (F) | 26.08 | 25.30 | 18.16 | 11.83 | 12.01 | 21.85 | 20.46 |
| Mean aspect ratio (F) | 1.260 | 1.263 | 1.221 | 1.213 | 1.197 | 1.668 | 1.284 |
| Area variation (F) | 0.837 | 0.221 | 0.250 | 0.278 | 0.374 | 0.473 | 0.236 |
| Hausdorff, ‰ of the diagonal (H) | 42.26 | 20.15 | 8.61 | 9.76 | 7.83 | 6.77 | 9.47 |
| Mean distance, ‰ (H) | 0.834 | 0.906 | 0.738 | 0.575 | 0.477 | 0.603 | 1.053 |
| Thin parts: mean distance, % of a quad edge (V) | 16.4 | 21.4 | 16.0 | 8.6 | 6.8 | 5.1 | 17.1 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | 1.0 | 2.3 | 8.1 | 0.7 | 0.5 | 43.7 | 12.4 |
| Time, seconds (T) | 3.22 | 0.28 | 8.52 | 1.85 | 2.77 | 3.44 | 3.21 |

## ogre (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 4/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 106 | 216 | 73 | 76 | 155 | 314 | 228 |
| … of which above the minimum | 98 | 202 | — | 68 | 147 | 306 | 220 |
| Base complex patches (C) | 2857 | 2802 | — | 2727 | 2483 | 5170 | 2009 |
| Alignment, degrees (A) | 19.31 | 18.57 | 12.52 | 12.31 | 11.63 | 8.42 | 10.81 |
| Mean angle deviation, degrees (F) | 12.01 | 7.49 | 9.91 | 6.60 | 6.32 | 9.10 | 11.45 |
| Angle deviation, 95th percentile (F) | 32.96 | 24.38 | 27.23 | 18.60 | 17.99 | 25.01 | 30.48 |
| Mean aspect ratio (F) | 1.403 | 1.283 | 1.572 | 1.569 | 1.414 | 2.364 | 1.612 |
| Area variation (F) | 0.324 | 0.224 | 0.405 | 0.384 | 0.506 | 0.716 | 0.393 |
| Hausdorff, ‰ of the diagonal (H) | 36.59 | 46.14 | 36.52 | 26.80 | 28.44 | 5.98 | 39.35 |
| Mean distance, ‰ (H) | 1.268 | 1.460 | 1.091 | 0.765 | 0.695 | 0.565 | 1.366 |
| Thin parts: mean distance, % of a quad edge (V) | 54.0 | 99.5 | 63.2 | 28.1 | 6.3 | 6.7 | 75.2 |
| Volume kept, % of the source (V) | 98.56 | 98.55 | 99.76 | 99.93 | 100.01 | 99.24 | 98.42 |
| Off budget, % (N) | 4.3 | 4.2 | 17.7 | 2.0 | 0.6 | 104.5 | 19.4 |
| Time, seconds (T) | 3.94 | 2.70 | 10.04 | 4.72 | 5.31 | 19.04 | 17.16 |

## spot (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 68 | 153 | 69 | 70 | 70 | 82 | 87 |
| … of which above the minimum | 60 | 145 | 61 | 62 | 62 | 74 | 79 |
| Base complex patches (C) | 2902 | 2978 | 1790 | 1730 | 1663 | 2428 | 2118 |
| Alignment, degrees (A) | 18.12 | 17.17 | 9.25 | 8.71 | 7.92 | 6.29 | 7.56 |
| Mean angle deviation, degrees (F) | 7.25 | 5.59 | 8.93 | 5.60 | 5.62 | 6.35 | 7.05 |
| Angle deviation, 95th percentile (F) | 20.42 | 18.71 | 23.20 | 15.03 | 14.93 | 17.74 | 19.43 |
| Mean aspect ratio (F) | 1.194 | 1.179 | 1.358 | 1.343 | 1.343 | 1.460 | 1.318 |
| Area variation (F) | 0.177 | 0.172 | 0.271 | 0.319 | 0.422 | 0.392 | 0.268 |
| Hausdorff, ‰ of the diagonal (H) | 17.71 | 19.38 | 6.58 | 4.98 | 5.58 | 6.64 | 10.63 |
| Mean distance, ‰ (H) | 0.448 | 0.479 | 0.232 | 0.198 | 0.197 | 0.372 | 0.476 |
| Thin parts: mean distance, % of a quad edge (V) | 56.3 | 53.8 | 5.3 | 4.1 | 3.8 | 8.5 | 21.2 |
| Volume kept, % of the source (V) | 99.37 | 99.35 | 100.00 | 100.03 | 100.01 | 99.35 | 99.27 |
| Off budget, % (N) | 1.2 | 1.5 | 10.5 | 1.6 | 0.7 | 15.9 | 10.5 |
| Time, seconds (T) | 2.06 | 1.23 | 9.65 | 3.78 | 3.73 | 19.14 | 15.16 |

## bob (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 32 | 130 | 20 | 20 | 20 | 28 | 30 |
| … of which above the minimum | 32 | 130 | 20 | 20 | 20 | 28 | 30 |
| Base complex patches (C) | 2903 | 3040 | 714 | 1290 | 237 | 2390 | 2568 |
| Alignment, degrees (A) | 19.37 | 20.80 | 4.37 | 4.01 | 3.61 | 4.85 | 5.15 |
| Mean angle deviation, degrees (F) | 6.41 | 5.33 | 4.83 | 3.29 | 3.25 | 5.52 | 5.94 |
| Angle deviation, 95th percentile (F) | 18.61 | 17.96 | 14.71 | 8.86 | 9.09 | 15.59 | 15.77 |
| Mean aspect ratio (F) | 1.175 | 1.158 | 1.409 | 1.385 | 1.370 | 1.368 | 1.360 |
| Area variation (F) | 0.159 | 0.161 | 0.339 | 0.374 | 0.433 | 0.360 | 0.324 |
| Hausdorff, ‰ of the diagonal (H) | 6.82 | 7.32 | 2.49 | 1.95 | 1.40 | 1.96 | 2.27 |
| Mean distance, ‰ (H) | 0.309 | 0.325 | 0.152 | 0.131 | 0.132 | 0.322 | 0.355 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.48 | 99.44 | 100.01 | 100.02 | 100.02 | 99.37 | 99.31 |
| Off budget, % (N) | 2.4 | 1.3 | 11.9 | 1.4 | 1.7 | 2.7 | 10.1 |
| Time, seconds (T) | 2.03 | 1.45 | 7.70 | 2.80 | 3.20 | 17.24 | 17.35 |

## blub (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 52 | 158 | 44 | 44 | 58 | 55 | 46 |
| … of which above the minimum | 44 | 150 | — | 36 | 50 | 47 | 38 |
| Base complex patches (C) | 2584 | 2922 | — | 1287 | 1333 | 2003 | 2057 |
| Alignment, degrees (A) | 15.95 | 15.45 | 8.31 | 7.76 | 7.47 | 5.26 | 7.01 |
| Mean angle deviation, degrees (F) | 10.72 | 5.55 | 8.57 | 5.18 | 4.87 | 6.30 | 7.24 |
| Angle deviation, 95th percentile (F) | 28.70 | 19.72 | 23.98 | 15.65 | 14.52 | 16.74 | 18.88 |
| Mean aspect ratio (F) | 1.271 | 1.192 | 1.333 | 1.296 | 1.300 | 1.828 | 1.351 |
| Area variation (F) | 0.261 | 0.178 | 0.267 | 0.300 | 0.412 | 0.445 | 0.269 |
| Hausdorff, ‰ of the diagonal (H) | 40.70 | 15.68 | 12.37 | 8.80 | 7.52 | 5.19 | 11.32 |
| Mean distance, ‰ (H) | 0.588 | 0.390 | 0.269 | 0.186 | 0.172 | 0.230 | 0.392 |
| Thin parts: mean distance, % of a quad edge (V) | 46.5 | 24.8 | 21.7 | 9.3 | 7.5 | 7.7 | 20.0 |
| Volume kept, % of the source (V) | 98.93 | 99.17 | 99.87 | 99.96 | 99.97 | 99.55 | 99.13 |
| Off budget, % (N) | 10.1 | 1.2 | 11.6 | 0.2 | 0.9 | 29.6 | 9.9 |
| Time, seconds (T) | 1.82 | 1.25 | 8.98 | 2.72 | 3.31 | 14.34 | 12.32 |

## pig (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 122 | 209 | 73 | 78 | 102 | 106 | 96 |
| … of which above the minimum | 114 | 192 | — | 70 | 94 | 98 | 88 |
| Base complex patches (C) | 2612 | 2912 | — | 2401 | 1849 | 3004 | 1840 |
| Alignment, degrees (A) | 17.41 | 16.24 | 10.96 | 10.72 | 9.07 | 7.22 | 9.36 |
| Mean angle deviation, degrees (F) | 14.32 | 6.83 | 7.78 | 6.02 | 5.33 | 7.22 | 7.93 |
| Angle deviation, 95th percentile (F) | 41.52 | 23.86 | 22.67 | 17.25 | 16.59 | 21.68 | 23.91 |
| Mean aspect ratio (F) | 1.383 | 1.244 | 1.254 | 1.241 | 1.281 | 1.876 | 1.322 |
| Area variation (F) | 0.398 | 0.210 | 0.252 | 0.291 | 0.457 | 0.636 | 0.287 |
| Hausdorff, ‰ of the diagonal (H) | 44.59 | 48.32 | 24.88 | 16.44 | 10.02 | 10.22 | 12.72 |
| Mean distance, ‰ (H) | 1.056 | 1.012 | 0.572 | 0.385 | 0.302 | 0.375 | 0.724 |
| Thin parts: mean distance, % of a quad edge (V) | 67.8 | 26.2 | 34.1 | 16.5 | 4.4 | 3.7 | 16.4 |
| Volume kept, % of the source (V) | 98.78 | 98.84 | 99.64 | 99.89 | 99.97 | 99.54 | 98.93 |
| Off budget, % (N) | 10.8 | 1.1 | 9.9 | 1.6 | 1.0 | 56.6 | 14.9 |
| Time, seconds (T) | 1.94 | 1.15 | 9.64 | 3.22 | 4.57 | 14.59 | 16.02 |

## jerry (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 104 | 265 | 95 | 86 | 162 | 206 | 181 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | 2211 | 2959 | 1963 | 2043 | 2491 | — | 2088 |
| Alignment, degrees (A) | 17.93 | 18.35 | 14.82 | 14.14 | 13.59 | 11.02 | 13.58 |
| Mean angle deviation, degrees (F) | 11.25 | 8.58 | 9.51 | 6.43 | 6.72 | 9.67 | 10.03 |
| Angle deviation, 95th percentile (F) | 29.73 | 29.29 | 24.66 | 16.66 | 21.39 | 26.54 | 28.28 |
| Mean aspect ratio (F) | 1.301 | 1.357 | 1.334 | 1.283 | 1.343 | 2.063 | 1.483 |
| Area variation (F) | 0.278 | 0.258 | 0.271 | 0.311 | 0.543 | 0.724 | 0.324 |
| Hausdorff, ‰ of the diagonal (H) | 38.82 | 49.21 | 43.33 | 31.62 | 21.66 | 6.74 | 15.07 |
| Mean distance, ‰ (H) | 1.467 | 2.158 | 1.743 | 1.079 | 0.836 | 0.529 | 1.488 |
| Thin parts: mean distance, % of a quad edge (V) | 23.4 | 40.3 | 55.4 | 29.9 | 15.2 | 4.8 | 19.2 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | 4.1 | 1.4 | 12.8 | 0.0 | 1.8 | 129.1 | 14.2 |
| Time, seconds (T) | 2.10 | 0.59 | 9.58 | 1.78 | 2.58 | 6.74 | 6.79 |

## rose (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | 539 | 499 | 236 | 911 | 1119 | 1772 | 763 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | 1872 | 1954 | — | — | 3248 | — | — |
| Alignment, degrees (A) | 18.39 | 19.07 | 15.97 | 15.19 | 14.82 | 8.98 | 14.12 |
| Mean angle deviation, degrees (F) | 16.45 | 15.58 | 12.29 | 19.42 | 20.80 | 9.06 | 13.63 |
| Angle deviation, 95th percentile (F) | 48.25 | 48.45 | 37.18 | 61.71 | 66.52 | 26.38 | 39.32 |
| Mean aspect ratio (F) | 1.647 | 1.752 | 1.731 | 3.827 | 4.003 | 1.790 | 1.653 |
| Area variation (F) | 0.587 | 0.465 | 0.422 | 0.894 | 0.917 | 1.388 | 0.599 |
| Hausdorff, ‰ of the diagonal (H) | 75.61 | 79.25 | 73.31 | 35.59 | 24.90 | 74.46 | 78.76 |
| Mean distance, ‰ (H) | 3.772 | 4.183 | 3.468 | 2.574 | 2.030 | 0.842 | 3.206 |
| Thin parts: mean distance, % of a quad edge (V) | 39.1 | 46.1 | 38.3 | 25.5 | 19.8 | 11.9 | 35.3 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | 30.5 | 25.8 | 31.3 | 8.1 | 18.9 | 1095.8 | 10.4 |
| Time, seconds (T) | 2.77 | 2.12 | 20.99 | 8.70 | 12.52 | 9.53 | 10.86 |

## bridge (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 5/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 |
| Irregular vertices (S) | — | 151 | 50 | 50 | 46 | 81 | 64 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 1879 | 763 | 1066 | 764 | — | — |
| Alignment, degrees (A) | — | 12.58 | 8.32 | 8.00 | 7.39 | 6.49 | 7.93 |
| Mean angle deviation, degrees (F) | — | 6.12 | 6.56 | 3.92 | 4.04 | 6.65 | 7.20 |
| Angle deviation, 95th percentile (F) | — | 20.51 | 18.71 | 11.48 | 11.95 | 18.96 | 19.72 |
| Mean aspect ratio (F) | — | 1.215 | 1.308 | 1.253 | 1.253 | 1.753 | 1.413 |
| Area variation (F) | — | 0.174 | 0.238 | 0.294 | 0.430 | 0.421 | 0.275 |
| Hausdorff, ‰ of the diagonal (H) | — | 28.19 | 6.93 | 7.15 | 6.73 | 6.05 | 7.00 |
| Mean distance, ‰ (H) | — | 0.638 | 0.509 | 0.451 | 0.444 | 0.414 | 0.670 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 3.2 | 10.6 | 1.6 | 1.4 | 51.1 | 9.1 |
| Time, seconds (T) | — | 2.98 | 8.02 | 5.46 | 4.26 | 11.41 | 11.88 |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 28 | 204 | 24 | 24 | 24 | 24 | 24 |
| … of which above the minimum | 20 | 196 | 16 | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 619 | 1782 | 78 | 78 | 285 | 501 | 501 |
| Alignment, degrees (A) | 12.97 | 13.96 | 11.65 | 11.96 | 10.48 | 10.90 | 10.90 |
| Mean angle deviation, degrees (F) | 2.66 | 6.10 | 2.61 | 1.35 | 2.25 | 3.02 | 3.02 |
| Angle deviation, 95th percentile (F) | 10.11 | 25.03 | 10.79 | 4.82 | 6.23 | 12.04 | 12.04 |
| Mean aspect ratio (F) | 1.116 | 1.233 | 1.163 | 1.158 | 1.161 | 1.326 | 1.326 |
| Area variation (F) | 0.100 | 0.199 | 0.126 | 0.113 | 0.364 | 0.196 | 0.196 |
| Hausdorff, ‰ of the diagonal (H) | 5.54 | 9.41 | 0.80 | 0.92 | 0.47 | 0.89 | 0.89 |
| Mean distance, ‰ (H) | 0.114 | 0.067 | 0.054 | 0.052 | 0.056 | 0.068 | 0.068 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.15 | 100.14 | 100.00 | 100.00 | 100.00 | 100.12 | 100.12 |
| Off budget, % (N) | 3.7 | 2.1 | 0.8 | 1.6 | 0.7 | 6.2 | 6.2 |
| Time, seconds (T) | 2.91 | 0.71 | 4.07 | 2.59 | 2.92 | 2.83 | 2.88 |

## cube (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 24 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 16 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 6 | 274 | 1102 | 6 | 6 | 6 | 6 |
| Alignment, degrees (A) | 0.29 | 1.97 | 0.75 | 0.14 | 0.36 | 0.13 | 0.13 |
| Mean angle deviation, degrees (F) | 0.18 | 1.65 | 0.75 | 0.01 | 0.36 | 0.00 | 0.00 |
| Angle deviation, 95th percentile (F) | 0.43 | 4.89 | 2.01 | 0.02 | 0.85 | 0.00 | 0.00 |
| Mean aspect ratio (F) | 1.005 | 1.062 | 1.039 | 1.031 | 1.088 | 1.030 | 1.030 |
| Area variation (F) | 0.004 | 0.090 | 0.045 | 0.021 | 0.098 | 0.021 | 0.021 |
| Hausdorff, ‰ of the diagonal (H) | 4.80 | 0.00 | 8.84 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.017 | 0.000 | 0.067 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 100.00 | 99.99 | 100.00 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 3.2 | 1.0 | 0.3 | 0.3 | 0.3 | 2.7 | 2.7 |
| Time, seconds (T) | 0.77 | 0.20 | 8.23 | 1.95 | 1.41 | 2.97 | 3.00 |

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
| Time, seconds (T) | 1.39 | 0.36 | 55.94 | 1.75 | 1.41 | 2.73 | 2.90 |

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
| Mean aspect ratio (F) | 1.087 | 1.132 | 1.408 | 1.389 | 1.533 | 1.511 | 1.511 |
| Area variation (F) | 0.103 | 0.140 | 0.320 | 0.375 | 0.233 | 0.403 | 0.403 |
| Hausdorff, ‰ of the diagonal (H) | 2.07 | 3.74 | 0.98 | 0.93 | 0.83 | 1.43 | 1.43 |
| Mean distance, ‰ (H) | 0.144 | 0.121 | 0.162 | 0.131 | 0.125 | 0.369 | 0.369 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.91 | 99.90 | 100.00 | 100.00 | 100.00 | 99.56 | 99.56 |
| Off budget, % (N) | 3.3 | 3.3 | 12.0 | 0.5 | 1.3 | 18.1 | 18.1 |
| Time, seconds (T) | 1.79 | 0.17 | 3.12 | 1.72 | 1.70 | 2.68 | 2.49 |

## terrain (extra)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 112 | 115 | 55 | 56 | 99 | 231 | 167 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | 826 | 1209 | 997 | 1681 | 2281 | — | 1662 |
| Alignment, degrees (A) | 20.17 | 21.17 | 18.72 | 18.68 | 18.72 | 18.30 | 18.61 |
| Mean angle deviation, degrees (F) | 6.49 | 8.24 | 9.36 | 6.26 | 6.74 | 9.57 | 8.13 |
| Angle deviation, 95th percentile (F) | 23.46 | 27.33 | 27.14 | 17.87 | 20.36 | 26.86 | 23.75 |
| Mean aspect ratio (F) | 1.209 | 1.291 | 1.339 | 1.291 | 1.338 | 1.806 | 1.429 |
| Area variation (F) | 0.187 | 0.215 | 0.276 | 0.313 | 0.451 | 0.552 | 0.247 |
| Hausdorff, ‰ of the diagonal (H) | 23.91 | 143.82 | 24.92 | 20.21 | 18.45 | 12.55 | 25.16 |
| Mean distance, ‰ (H) | 1.851 | 2.384 | 1.715 | 1.361 | 1.148 | 0.967 | 1.780 |
| Thin parts: mean distance, % of a quad edge (V) | 32.8 | 36.9 | 35.4 | 26.3 | 20.2 | 14.4 | 32.3 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | 9.4 | 41.6 | 16.6 | 1.1 | 1.2 | 55.9 | 13.4 |
| Time, seconds (T) | 22.93 | 8.92 | 12.81 | 9.90 | 8.78 | 21.17 | 19.56 |

## mahaveer (extra)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | — | 175 | 65 | 453 | 455 | 3076 | 1079 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 1227 | 388 | 714 | 728 | — | — |
| Alignment, degrees (A) | — | 9.97 | 7.14 | 14.99 | 15.00 | 13.54 | 11.62 |
| Mean angle deviation, degrees (F) | — | 9.66 | 7.50 | 25.06 | 25.62 | 12.36 | 12.56 |
| Angle deviation, 95th percentile (F) | — | 34.02 | 26.29 | 77.75 | 82.76 | 39.42 | 48.70 |
| Mean aspect ratio (F) | — | 1.583 | 1.498 | 81.871 | 125.121 | 1.824 | 1.786 |
| Area variation (F) | — | 0.344 | 0.405 | 1.571 | 1.571 | 2.664 | 0.975 |
| Hausdorff, ‰ of the diagonal (H) | — | 52.26 | 29.97 | 39.18 | 39.18 | 13.60 | 23.83 |
| Mean distance, ‰ (H) | — | 7.890 | 3.812 | 2.888 | 2.906 | 0.929 | 2.320 |
| Thin parts: mean distance, % of a quad edge (V) | — | 75.1 | 36.5 | 25.8 | 25.8 | 11.6 | 21.9 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 43.4 | 29.1 | 0.4 | 0.7 | 1440.1 | 44.1 |
| Time, seconds (T) | — | 7.16 | 24.64 | 14.16 | 16.74 | 14.10 | 14.84 |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | — | 616 | — | 5577 | 5538 | 10315 | 4939 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 1136 | — | — | — | — | — |
| Alignment, degrees (A) | — | 17.78 | — | 12.45 | 13.51 | 10.18 | 13.63 |
| Mean angle deviation, degrees (F) | — | 23.80 | — | 25.76 | 25.86 | 9.86 | 17.03 |
| Angle deviation, 95th percentile (F) | — | 60.14 | — | 74.07 | 73.42 | 29.30 | 51.38 |
| Mean aspect ratio (F) | — | 2.545 | — | 7.161 | 14.969 | 1.580 | 1.889 |
| Area variation (F) | — | 0.789 | — | 2.571 | 2.396 | 2.386 | 2.005 |
| Hausdorff, ‰ of the diagonal (H) | — | 7.35 | — | 6.31 | 6.31 | 1.80 | 3.03 |
| Mean distance, ‰ (H) | — | 1.806 | — | 0.534 | 0.551 | 0.123 | 0.419 |
| Thin parts: mean distance, % of a quad edge (V) | — | 57.0 | — | 14.6 | 15.1 | 7.6 | 14.6 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 59.2 | — | 116.2 | 112.1 | 7033.9 | 278.0 |
| Time, seconds (T) | — | 4.00 | — | 40.99 | 64.11 | 12.21 | 10.92 |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 545 non-manifold or inconsistently oriented edges.

## sphere (calibration)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 97 | 32 | 32 | 32 | 10 | 10 |
| … of which above the minimum | 0 | 89 | 24 | 24 | 24 | 2 | 2 |
| Base complex patches (C) | 2730 | 3090 | 30 | 30 | 30 | 1794 | 1794 |
| Alignment, degrees (A) | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.49 | 4.19 | 8.38 | 3.94 | 3.94 | 6.09 | 6.09 |
| Angle deviation, 95th percentile (F) | 12.67 | 14.49 | 17.63 | 7.44 | 7.44 | 15.49 | 15.49 |
| Mean aspect ratio (F) | 1.125 | 1.124 | 1.081 | 1.083 | 1.083 | 1.185 | 1.185 |
| Area variation (F) | 0.135 | 0.136 | 0.190 | 0.303 | 0.303 | 0.312 | 0.312 |
| Hausdorff, ‰ of the diagonal (H) | 0.63 | 0.72 | 0.42 | 0.50 | 0.50 | 0.82 | 0.82 |
| Mean distance, ‰ (H) | 0.216 | 0.221 | 0.168 | 0.162 | 0.162 | 0.258 | 0.258 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.81 | 99.79 | 100.01 | 100.01 | 100.01 | 99.74 | 99.74 |
| Off budget, % (N) | 3.6 | 3.0 | 0.0 | 0.0 | 0.0 | 8.7 | 8.7 |
| Time, seconds (T) | 1.80 | 0.34 | 3.13 | 1.04 | 1.12 | 3.56 | 3.39 |

## torus (calibration)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 30 | 125 | 0 | 0 | 0 | 10 | 10 |
| … of which above the minimum | 30 | 125 | 0 | 0 | 0 | 10 | 10 |
| Base complex patches (C) | 2976 | 3078 | 1 | 1 | 1 | 2264 | 2264 |
| Alignment, degrees (A) | 10.14 | 14.28 | 0.65 | 0.62 | 0.62 | 6.90 | 6.90 |
| Mean angle deviation, degrees (F) | 10.25 | 5.57 | 1.45 | 1.44 | 1.44 | 5.23 | 5.23 |
| Angle deviation, 95th percentile (F) | 26.29 | 18.30 | 2.88 | 2.35 | 2.35 | 14.40 | 14.40 |
| Mean aspect ratio (F) | 1.239 | 1.163 | 1.351 | 1.322 | 1.322 | 1.213 | 1.213 |
| Area variation (F) | 0.223 | 0.163 | 0.281 | 0.308 | 0.308 | 0.233 | 0.233 |
| Hausdorff, ‰ of the diagonal (H) | 1.86 | 1.38 | 0.60 | 0.59 | 0.59 | 1.71 | 1.71 |
| Mean distance, ‰ (H) | 0.299 | 0.298 | 0.208 | 0.191 | 0.191 | 0.518 | 0.518 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.46 | 99.44 | 100.02 | 100.02 | 100.02 | 98.94 | 98.94 |
| Off budget, % (N) | 0.8 | 2.6 | 9.3 | 1.6 | 1.6 | 5.9 | 5.9 |
| Time, seconds (T) | 1.55 | 0.22 | 2.02 | 1.65 | 3.07 | 4.71 | 2.89 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 0 | 7 | 4 | 3 | 0 | 1 |
| Base complex patches (C) | 1 | 0 | 4 | 1 | 3 | 0 | 4 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 1 | 12 | 0 |
| Mean angle deviation, degrees (F) | 0 | 2 | 0 | 3 | 6 | 1 | 1 |
| Mean aspect ratio (F) | 2 | 8 | 0 | 2 | 1 | 0 | 0 |
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
