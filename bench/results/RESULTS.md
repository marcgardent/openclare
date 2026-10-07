# Bench results

Requested budget: 3000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. Lower is better for every metric; "—" means it could not be measured.

## blob (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 38 | 143 | 26 | 24 | 26 | 24 |
| … of which above the minimum | 30 | 135 | 18 | 16 | 18 | 16 |
| Base complex patches (C) | 2697 | 3048 | 951 | 2361 | 1826 | 1350 |
| Alignment, degrees (A) | 17.48 | 15.74 | 6.23 | 5.92 | 3.93 | 4.55 |
| Mean angle deviation, degrees (F) | 8.71 | 5.87 | 7.91 | 7.88 | 5.76 | 5.45 |
| Angle deviation, 95th percentile (F) | 27.30 | 18.26 | 21.21 | 21.12 | 17.21 | 15.61 |
| Mean aspect ratio (F) | 1.187 | 1.174 | 1.428 | 1.441 | 1.618 | 1.276 |
| Area variation (F) | 0.205 | 0.175 | 0.352 | 0.342 | 0.442 | 0.313 |
| Hausdorff, ‰ of the diagonal (H) | 8.52 | 10.14 | 7.34 | 6.47 | 4.21 | 7.28 |
| Mean distance, ‰ (H) | 0.486 | 0.459 | 0.582 | 0.478 | 0.407 | 0.513 |
| Off budget, % (N) | 7.3 | 3.0 | 16.4 | 0.7 | 12.3 | 8.2 |
| Time, seconds (T) | 1.89 | 1.68 | 4.15 | 1.66 | 16.38 | 12.28 |

## rock (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 91 | 50 | 50 | 114 | 101 |
| … of which above the minimum | 0 | 83 | 42 | 42 | 106 | 93 |
| Base complex patches (C) | 2855 | 2896 | 2687 | 2968 | 3539 | 2531 |
| Alignment, degrees (A) | 22.96 | 23.21 | 15.80 | 15.79 | 14.17 | 14.68 |
| Mean angle deviation, degrees (F) | 5.84 | 5.18 | 8.83 | 8.93 | 10.07 | 10.61 |
| Angle deviation, 95th percentile (F) | 16.05 | 14.78 | 22.19 | 21.37 | 24.88 | 26.21 |
| Mean aspect ratio (F) | 1.164 | 1.146 | 1.253 | 1.267 | 1.470 | 1.393 |
| Area variation (F) | 0.202 | 0.151 | 0.227 | 0.234 | 0.308 | 0.269 |
| Hausdorff, ‰ of the diagonal (H) | 5.45 | 5.28 | 5.55 | 6.28 | 5.13 | 5.79 |
| Mean distance, ‰ (H) | 0.788 | 0.820 | 0.861 | 0.816 | 0.731 | 0.889 |
| Off budget, % (N) | 2.2 | 3.4 | 10.4 | 1.1 | 18.0 | 15.6 |
| Time, seconds (T) | 2.44 | 0.83 | 5.58 | 1.87 | 20.49 | 16.98 |

## creature (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 24 | 111 | 41 | 40 | 42 | 46 |
| … of which above the minimum | 16 | 103 | 33 | 32 | 34 | 38 |
| Base complex patches (C) | 2667 | 2856 | 913 | 1301 | 1465 | 1301 |
| Alignment, degrees (A) | 12.89 | 12.09 | 6.95 | 6.83 | 5.18 | 5.32 |
| Mean angle deviation, degrees (F) | 6.85 | 4.65 | 8.17 | 8.12 | 6.12 | 6.53 |
| Angle deviation, 95th percentile (F) | 18.98 | 14.73 | 23.87 | 22.61 | 17.13 | 18.98 |
| Mean aspect ratio (F) | 1.189 | 1.144 | 1.300 | 1.303 | 1.332 | 1.334 |
| Area variation (F) | 0.237 | 0.146 | 0.289 | 0.311 | 0.348 | 0.313 |
| Hausdorff, ‰ of the diagonal (H) | 8.01 | 3.47 | 3.16 | 2.86 | 2.51 | 2.37 |
| Mean distance, ‰ (H) | 0.320 | 0.279 | 0.370 | 0.305 | 0.297 | 0.348 |
| Off budget, % (N) | 11.1 | 2.3 | 13.4 | 1.4 | 2.0 | 9.4 |
| Time, seconds (T) | 1.64 | 0.80 | 5.46 | 2.25 | 11.24 | 9.11 |

## cage (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 66 | 200 | 41 | 48 | 44 | 43 |
| … of which above the minimum | 34 | 168 | 9 | 16 | 12 | 11 |
| Base complex patches (C) | 1943 | 2319 | 427 | 689 | 1092 | 1001 |
| Alignment, degrees (A) | 8.10 | 7.97 | 3.39 | 3.70 | 2.04 | 2.57 |
| Mean angle deviation, degrees (F) | 5.86 | 6.39 | 3.96 | 4.24 | 3.91 | 4.30 |
| Angle deviation, 95th percentile (F) | 17.11 | 23.33 | 12.27 | 14.11 | 11.90 | 12.82 |
| Mean aspect ratio (F) | 1.183 | 1.252 | 1.203 | 1.258 | 1.993 | 1.318 |
| Area variation (F) | 0.147 | 0.203 | 0.190 | 0.190 | 0.314 | 0.238 |
| Hausdorff, ‰ of the diagonal (H) | 11.45 | 14.46 | 16.28 | 12.58 | 5.74 | 9.66 |
| Mean distance, ‰ (H) | 0.734 | 0.860 | 1.017 | 0.975 | 0.693 | 1.276 |
| Off budget, % (N) | 3.6 | 1.9 | 4.2 | 1.3 | 93.6 | 7.2 |
| Time, seconds (T) | 2.19 | 0.61 | 6.83 | 2.23 | 4.21 | 3.92 |

## suzanne (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 2/5 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 78 | 212 | 52 | 53 | 80 | 71 |
| … of which above the minimum | 54 | — | — | — | — | — |
| Base complex patches (C) | 2884 | 3002 | 1858 | 2475 | 2513 | 1838 |
| Alignment, degrees (A) | 15.33 | 17.02 | 10.40 | 10.00 | 8.75 | 10.24 |
| Mean angle deviation, degrees (F) | 8.61 | 7.08 | 7.14 | 6.67 | 7.87 | 7.87 |
| Angle deviation, 95th percentile (F) | 26.08 | 25.30 | 18.15 | 17.04 | 21.85 | 20.46 |
| Mean aspect ratio (F) | 1.260 | 1.263 | 1.223 | 1.246 | 1.668 | 1.284 |
| Area variation (F) | 0.837 | 0.221 | 0.243 | 0.246 | 0.473 | 0.236 |
| Hausdorff, ‰ of the diagonal (H) | 42.26 | 20.15 | 11.31 | 8.90 | 6.77 | 9.47 |
| Mean distance, ‰ (H) | 0.834 | 0.906 | 0.980 | 0.849 | 0.603 | 1.053 |
| Off budget, % (N) | 1.0 | 2.3 | 8.1 | 0.7 | 43.7 | 12.4 |
| Time, seconds (T) | 3.20 | 0.30 | 8.05 | 2.19 | 3.44 | 3.21 |

## ogre (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 4/5 | 0/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 106 | 216 | 73 | 76 | 314 | 228 |
| … of which above the minimum | 98 | 202 | — | 68 | 306 | 220 |
| Base complex patches (C) | 2857 | 2802 | — | 2727 | 5170 | 2009 |
| Alignment, degrees (A) | 19.31 | 18.57 | 12.62 | 12.42 | 8.42 | 10.81 |
| Mean angle deviation, degrees (F) | 12.01 | 7.49 | 9.90 | 9.27 | 9.10 | 11.45 |
| Angle deviation, 95th percentile (F) | 32.96 | 24.38 | 27.27 | 24.77 | 25.01 | 30.48 |
| Mean aspect ratio (F) | 1.403 | 1.283 | 1.557 | 1.605 | 2.364 | 1.612 |
| Area variation (F) | 0.324 | 0.224 | 0.412 | 0.376 | 0.716 | 0.393 |
| Hausdorff, ‰ of the diagonal (H) | 36.59 | 46.14 | 37.31 | 31.20 | 5.98 | 39.35 |
| Mean distance, ‰ (H) | 1.268 | 1.460 | 1.394 | 1.163 | 0.565 | 1.366 |
| Off budget, % (N) | 4.3 | 4.2 | 17.7 | 2.0 | 104.5 | 19.4 |
| Time, seconds (T) | 3.76 | 2.39 | 9.22 | 5.54 | 19.04 | 17.16 |

## spot (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 68 | 153 | 69 | 70 | 82 | 87 |
| … of which above the minimum | 60 | 145 | 61 | 62 | 74 | 79 |
| Base complex patches (C) | 2902 | 2978 | 1790 | 1730 | 2428 | 2118 |
| Alignment, degrees (A) | 18.12 | 17.17 | 9.23 | 8.89 | 6.29 | 7.56 |
| Mean angle deviation, degrees (F) | 7.25 | 5.59 | 8.93 | 8.41 | 6.35 | 7.05 |
| Angle deviation, 95th percentile (F) | 20.42 | 18.71 | 23.29 | 21.78 | 17.74 | 19.43 |
| Mean aspect ratio (F) | 1.194 | 1.179 | 1.356 | 1.381 | 1.460 | 1.318 |
| Area variation (F) | 0.177 | 0.172 | 0.277 | 0.291 | 0.392 | 0.268 |
| Hausdorff, ‰ of the diagonal (H) | 17.71 | 19.38 | 7.35 | 9.25 | 6.64 | 10.63 |
| Mean distance, ‰ (H) | 0.448 | 0.479 | 0.477 | 0.432 | 0.372 | 0.476 |
| Off budget, % (N) | 1.2 | 1.5 | 10.5 | 1.6 | 15.9 | 10.5 |
| Time, seconds (T) | 1.89 | 1.35 | 7.82 | 5.25 | 19.14 | 15.16 |

## bob (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 32 | 130 | 20 | 20 | 28 | 30 |
| … of which above the minimum | 32 | 130 | 20 | 20 | 28 | 30 |
| Base complex patches (C) | 2903 | 3040 | 714 | 1290 | 2390 | 2568 |
| Alignment, degrees (A) | 19.37 | 20.80 | 4.35 | 4.03 | 4.85 | 5.15 |
| Mean angle deviation, degrees (F) | 6.41 | 5.33 | 4.82 | 4.60 | 5.52 | 5.94 |
| Angle deviation, 95th percentile (F) | 18.61 | 17.96 | 14.62 | 13.35 | 15.59 | 15.77 |
| Mean aspect ratio (F) | 1.175 | 1.158 | 1.409 | 1.418 | 1.368 | 1.360 |
| Area variation (F) | 0.159 | 0.161 | 0.338 | 0.344 | 0.360 | 0.324 |
| Hausdorff, ‰ of the diagonal (H) | 6.82 | 7.32 | 3.63 | 2.69 | 1.96 | 2.27 |
| Mean distance, ‰ (H) | 0.309 | 0.325 | 0.378 | 0.328 | 0.322 | 0.355 |
| Off budget, % (N) | 2.4 | 1.3 | 11.9 | 1.4 | 2.7 | 10.1 |
| Time, seconds (T) | 1.83 | 1.33 | 6.12 | 1.59 | 17.24 | 17.35 |

## blub (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 52 | 158 | 44 | 44 | 55 | 46 |
| … of which above the minimum | 44 | 150 | — | 36 | 47 | 38 |
| Base complex patches (C) | 2584 | 2922 | — | 1287 | 2003 | 2057 |
| Alignment, degrees (A) | 15.95 | 15.45 | 8.38 | 7.79 | 5.26 | 7.01 |
| Mean angle deviation, degrees (F) | 10.72 | 5.55 | 8.60 | 7.81 | 6.30 | 7.24 |
| Angle deviation, 95th percentile (F) | 28.70 | 19.72 | 23.94 | 22.35 | 16.74 | 18.88 |
| Mean aspect ratio (F) | 1.271 | 1.192 | 1.336 | 1.336 | 1.828 | 1.351 |
| Area variation (F) | 0.261 | 0.178 | 0.261 | 0.272 | 0.445 | 0.269 |
| Hausdorff, ‰ of the diagonal (H) | 40.70 | 15.68 | 14.68 | 11.57 | 5.19 | 11.32 |
| Mean distance, ‰ (H) | 0.588 | 0.390 | 0.456 | 0.373 | 0.230 | 0.392 |
| Off budget, % (N) | 10.1 | 1.2 | 11.6 | 0.2 | 29.6 | 9.9 |
| Time, seconds (T) | 2.09 | 1.30 | 5.78 | 3.57 | 14.34 | 12.32 |

## pig (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 122 | 209 | 73 | 78 | 106 | 96 |
| … of which above the minimum | 114 | 192 | — | 70 | 98 | 88 |
| Base complex patches (C) | 2612 | 2912 | — | 2401 | 3004 | 1840 |
| Alignment, degrees (A) | 17.41 | 16.24 | 10.65 | 10.39 | 7.22 | 9.36 |
| Mean angle deviation, degrees (F) | 14.32 | 6.83 | 7.67 | 7.78 | 7.22 | 7.93 |
| Angle deviation, 95th percentile (F) | 41.52 | 23.86 | 22.18 | 21.74 | 21.68 | 23.91 |
| Mean aspect ratio (F) | 1.383 | 1.244 | 1.251 | 1.294 | 1.876 | 1.322 |
| Area variation (F) | 0.398 | 0.210 | 0.243 | 0.289 | 0.636 | 0.287 |
| Hausdorff, ‰ of the diagonal (H) | 44.59 | 48.32 | 23.75 | 17.32 | 10.22 | 12.72 |
| Mean distance, ‰ (H) | 1.056 | 1.012 | 0.860 | 0.697 | 0.375 | 0.724 |
| Off budget, % (N) | 10.8 | 1.1 | 9.9 | 1.6 | 56.6 | 14.9 |
| Time, seconds (T) | 1.91 | 1.18 | 9.15 | 2.64 | 14.59 | 16.02 |

## jerry (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/5 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 104 | 265 | 95 | 86 | 206 | 181 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 2211 | 2959 | 1963 | 2043 | — | 2088 |
| Alignment, degrees (A) | 17.93 | 18.35 | 15.03 | 14.85 | 11.02 | 13.58 |
| Mean angle deviation, degrees (F) | 11.25 | 8.58 | 9.56 | 8.99 | 9.67 | 10.03 |
| Angle deviation, 95th percentile (F) | 29.73 | 29.29 | 24.76 | 22.66 | 26.54 | 28.28 |
| Mean aspect ratio (F) | 1.301 | 1.357 | 1.329 | 1.340 | 2.063 | 1.483 |
| Area variation (F) | 0.278 | 0.258 | 0.272 | 0.283 | 0.724 | 0.324 |
| Hausdorff, ‰ of the diagonal (H) | 38.82 | 49.21 | 44.34 | 35.33 | 6.74 | 15.07 |
| Mean distance, ‰ (H) | 1.467 | 2.158 | 2.165 | 1.850 | 0.529 | 1.488 |
| Off budget, % (N) | 4.1 | 1.4 | 12.8 | 0.0 | 129.1 | 14.2 |
| Time, seconds (T) | 1.77 | 0.75 | 5.83 | 1.16 | 6.74 | 6.79 |

## rose (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | 539 | 499 | 236 | 236 | 1772 | 763 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 1872 | 1954 | — | — | — | — |
| Alignment, degrees (A) | 18.53 | 19.07 | 16.09 | 16.09 | 8.98 | 14.12 |
| Mean angle deviation, degrees (F) | 16.45 | 15.58 | 12.48 | 12.48 | 9.06 | 13.63 |
| Angle deviation, 95th percentile (F) | 48.37 | 48.45 | 37.87 | 37.87 | 26.38 | 39.32 |
| Mean aspect ratio (F) | 1.647 | 1.752 | 1.763 | 1.763 | 1.790 | 1.653 |
| Area variation (F) | 0.592 | 0.465 | 0.440 | 0.440 | 1.388 | 0.599 |
| Hausdorff, ‰ of the diagonal (H) | 75.61 | 79.25 | 75.28 | 75.28 | 74.46 | 78.76 |
| Mean distance, ‰ (H) | 3.765 | 4.183 | 3.990 | 3.990 | 0.842 | 3.206 |
| Off budget, % (N) | 30.5 | 25.8 | 31.3 | 31.3 | 1095.8 | 10.4 |
| Time, seconds (T) | 2.33 | 1.87 | 23.35 | 16.79 | 9.53 | 10.86 |

## bridge (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 5/5 | 1/1 | 1/1 | 0/1 | 0/1 |
| Irregular vertices (S) | — | 151 | 50 | 50 | 81 | 64 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | — | 1879 | 763 | 1066 | — | — |
| Alignment, degrees (A) | — | 12.58 | 8.36 | 8.31 | 6.49 | 7.93 |
| Mean angle deviation, degrees (F) | — | 6.12 | 6.49 | 6.11 | 6.65 | 7.20 |
| Angle deviation, 95th percentile (F) | — | 20.51 | 18.53 | 17.44 | 18.96 | 19.72 |
| Mean aspect ratio (F) | — | 1.215 | 1.302 | 1.291 | 1.753 | 1.413 |
| Area variation (F) | — | 0.174 | 0.238 | 0.279 | 0.421 | 0.275 |
| Hausdorff, ‰ of the diagonal (H) | — | 28.19 | 6.91 | 6.66 | 6.05 | 7.00 |
| Mean distance, ‰ (H) | — | 0.638 | 0.680 | 0.629 | 0.414 | 0.670 |
| Off budget, % (N) | — | 3.2 | 10.6 | 1.6 | 51.1 | 9.1 |
| Time, seconds (T) | — | 2.27 | 9.04 | 2.52 | 11.41 | 11.88 |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 28 | 204 | 24 | 24 | 24 | 24 |
| … of which above the minimum | 20 | 196 | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 619 | 1782 | 78 | 78 | 501 | 501 |
| Alignment, degrees (A) | 12.97 | 13.96 | 11.66 | 11.93 | 10.90 | 10.90 |
| Mean angle deviation, degrees (F) | 2.66 | 6.10 | 2.61 | 2.40 | 3.02 | 3.02 |
| Angle deviation, 95th percentile (F) | 10.11 | 25.03 | 10.79 | 7.82 | 12.04 | 12.04 |
| Mean aspect ratio (F) | 1.116 | 1.233 | 1.164 | 1.166 | 1.326 | 1.326 |
| Area variation (F) | 0.100 | 0.199 | 0.126 | 0.140 | 0.196 | 0.196 |
| Hausdorff, ‰ of the diagonal (H) | 5.54 | 9.41 | 0.88 | 11.26 | 0.89 | 0.89 |
| Mean distance, ‰ (H) | 0.114 | 0.067 | 0.053 | 0.109 | 0.068 | 0.068 |
| Off budget, % (N) | 3.7 | 2.1 | 0.8 | 0.9 | 6.2 | 6.2 |
| Time, seconds (T) | 3.08 | 1.37 | 3.92 | 1.37 | 2.83 | 2.88 |

## cube (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 24 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 16 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 6 | 274 | 1102 | 6 | 6 | 6 |
| Alignment, degrees (A) | 0.29 | 1.97 | 0.75 | 0.31 | 0.13 | 0.13 |
| Mean angle deviation, degrees (F) | 0.18 | 1.65 | 0.75 | 0.44 | 0.00 | 0.00 |
| Angle deviation, 95th percentile (F) | 0.43 | 4.89 | 1.98 | 2.14 | 0.00 | 0.00 |
| Mean aspect ratio (F) | 1.005 | 1.062 | 1.041 | 1.033 | 1.030 | 1.030 |
| Area variation (F) | 0.004 | 0.090 | 0.048 | 0.035 | 0.021 | 0.021 |
| Hausdorff, ‰ of the diagonal (H) | 4.80 | 0.00 | 11.17 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.017 | 0.000 | 0.047 | 0.000 | 0.000 | 0.000 |
| Off budget, % (N) | 3.2 | 1.0 | 0.3 | 0.3 | 2.7 | 2.7 |
| Time, seconds (T) | 0.75 | 0.27 | 5.48 | 1.06 | 2.97 | 3.00 |

## sphere (calibration)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 97 | 32 | 32 | 10 | 10 |
| … of which above the minimum | 0 | 89 | 24 | 24 | 2 | 2 |
| Base complex patches (C) | 2730 | 3090 | 30 | 30 | 1794 | 1794 |
| Alignment, degrees (A) | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.49 | 4.19 | 8.38 | 8.31 | 6.09 | 6.09 |
| Angle deviation, 95th percentile (F) | 12.67 | 14.49 | 17.63 | 17.63 | 15.49 | 15.49 |
| Mean aspect ratio (F) | 1.125 | 1.124 | 1.081 | 1.096 | 1.185 | 1.185 |
| Area variation (F) | 0.135 | 0.136 | 0.190 | 0.220 | 0.312 | 0.312 |
| Hausdorff, ‰ of the diagonal (H) | 0.63 | 0.72 | 0.68 | 0.91 | 0.82 | 0.82 |
| Mean distance, ‰ (H) | 0.216 | 0.221 | 0.230 | 0.229 | 0.258 | 0.258 |
| Off budget, % (N) | 3.6 | 3.0 | 0.0 | 0.0 | 8.7 | 8.7 |
| Time, seconds (T) | 1.76 | 0.50 | 2.34 | 0.63 | 3.56 | 3.39 |

## torus (calibration)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 30 | 125 | 0 | 0 | 10 | 10 |
| … of which above the minimum | 30 | 125 | 0 | 0 | 10 | 10 |
| Base complex patches (C) | 2976 | 3078 | 1 | 1 | 2264 | 2264 |
| Alignment, degrees (A) | 10.14 | 14.28 | 0.65 | 0.65 | 6.90 | 6.90 |
| Mean angle deviation, degrees (F) | 10.25 | 5.57 | 1.45 | 1.45 | 5.23 | 5.23 |
| Angle deviation, 95th percentile (F) | 26.29 | 18.30 | 2.88 | 2.88 | 14.40 | 14.40 |
| Mean aspect ratio (F) | 1.239 | 1.163 | 1.352 | 1.352 | 1.213 | 1.213 |
| Area variation (F) | 0.223 | 0.163 | 0.279 | 0.279 | 0.233 | 0.233 |
| Hausdorff, ‰ of the diagonal (H) | 1.86 | 1.38 | 1.02 | 1.02 | 1.71 | 1.71 |
| Mean distance, ‰ (H) | 0.299 | 0.298 | 0.338 | 0.338 | 0.518 | 0.518 |
| Off budget, % (N) | 0.8 | 2.6 | 9.3 | 9.3 | 5.9 | 5.9 |
| Time, seconds (T) | 1.38 | 0.17 | 1.59 | 1.50 | 4.71 | 2.89 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 0 | 8 | 6 | 0 | 1 |
| Base complex patches (C) | 1 | 0 | 6 | 2 | 0 | 4 |
| Alignment, degrees (A) | 0 | 0 | 0 | 1 | 12 | 0 |
| Mean angle deviation, degrees (F) | 0 | 7 | 0 | 3 | 2 | 1 |
| Mean aspect ratio (F) | 3 | 9 | 1 | 0 | 0 | 0 |
| Valid meshes (V) | 36/65 | 47/65 | 9/13 | 12/13 | 8/13 | 10/13 |

### mechanical (2 models)

| Metric | quadriflow | instant-meshes | integer-grid | quad-layout | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| Irregular vertices (S) | 1 | 0 | 2 | 2 | 2 | 2 |
| Base complex patches (C) | 1 | 0 | 1 | 2 | 1 | 1 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 2 | 2 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 1 | 1 | 1 |
| Mean aspect ratio (F) | 2 | 0 | 0 | 0 | 0 | 0 |
| Valid meshes (V) | 9/10 | 10/10 | 2/2 | 2/2 | 2/2 | 2/2 |
