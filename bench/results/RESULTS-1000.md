# Bench results

Requested budget: 1000 quads. Median over 5 seeds; Quad Remesher, AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

## blob (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 400 | 1026 | 990 | 1340 |
| Irregular vertices (S) | 24 | 24 | 24 | 24 |
| … of which above the minimum | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 314 | 664 | 790 | 1148 |
| Alignment, degrees (A) | 8.83 | 5.27 | 4.43 | 4.87 |
| Mean angle deviation, degrees (F) | 12.46 | 6.39 | 5.85 | 5.86 |
| Angle deviation, 95th percentile (F) | 33.98 | 16.66 | 14.88 | 15.76 |
| Mean aspect ratio (F) | 1.652 | 1.373 | 1.350 | 1.346 |
| Area variation (F) | 0.577 | 0.453 | 0.463 | 0.434 |
| Hausdorff, ‰ of the diagonal (H) | 16.95 | 10.26 | 6.34 | 4.33 |
| Mean distance, ‰ (H) | 5.131 | 0.540 | 0.596 | 0.396 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 92.19 | 100.05 | 100.07 | 100.04 |
| Off budget, % (N) | 60.0 | 2.6 | 1.0 | 1.6 |
| Time, seconds (T) | 0.80 | 2.76 | 2.53 | 1.86 |

## rock (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 778 | 978 | 981 | 1298 |
| Irregular vertices (S) | 10 | 26 | 26 | 34 |
| … of which above the minimum | 2 | 18 | 18 | 26 |
| Base complex patches (C) | 506 | 767 | 981 | 1298 |
| Alignment, degrees (A) | 19.26 | 17.02 | 16.85 | 16.65 |
| Mean angle deviation, degrees (F) | 7.46 | 5.76 | 4.95 | 5.62 |
| Angle deviation, 95th percentile (F) | 18.13 | 16.05 | 14.03 | 15.70 |
| Mean aspect ratio (F) | 1.494 | 1.257 | 1.228 | 1.227 |
| Area variation (F) | 0.473 | 0.354 | 0.330 | 0.293 |
| Hausdorff, ‰ of the diagonal (H) | 14.09 | 8.83 | 7.73 | 7.55 |
| Mean distance, ‰ (H) | 2.480 | 1.242 | 1.241 | 1.059 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 98.22 | 100.02 | 99.99 | 100.02 |
| Off budget, % (N) | 22.2 | 2.2 | 1.9 | 0.6 |
| Time, seconds (T) | 0.58 | 2.47 | 1.82 | 1.69 |

## creature (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 604 | 965 | 1001 | 1207 |
| Irregular vertices (S) | 12 | 26 | 40 | 40 |
| … of which above the minimum | 4 | 18 | 32 | 32 |
| Base complex patches (C) | 308 | 773 | 355 | 465 |
| Alignment, degrees (A) | 11.08 | 8.03 | 5.67 | 5.92 |
| Mean angle deviation, degrees (F) | 6.78 | 5.01 | 5.53 | 5.24 |
| Angle deviation, 95th percentile (F) | 16.26 | 15.21 | 15.71 | 14.86 |
| Mean aspect ratio (F) | 1.697 | 1.246 | 1.347 | 1.299 |
| Area variation (F) | 0.418 | 0.352 | 0.481 | 0.490 |
| Hausdorff, ‰ of the diagonal (H) | 8.78 | 5.03 | 3.25 | 3.30 |
| Mean distance, ‰ (H) | 1.732 | 0.465 | 0.499 | 0.401 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 96.54 | 100.08 | 100.07 | 100.05 |
| Off budget, % (N) | 39.6 | 3.5 | 0.1 | 0.1 |
| Time, seconds (T) | 0.89 | 3.06 | 1.70 | 1.68 |

## cage (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 1720 | 1001 | 1001 | 3168 |
| Irregular vertices (S) | 52 | 48 | 48 | 48 |
| … of which above the minimum | — | 16 | 16 | 16 |
| Base complex patches (C) | — | 263 | 208 | 438 |
| Alignment, degrees (A) | 8.80 | 4.06 | 3.17 | 2.27 |
| Mean angle deviation, degrees (F) | 10.60 | 4.72 | 5.00 | 3.14 |
| Angle deviation, 95th percentile (F) | 30.15 | 14.08 | 15.30 | 9.61 |
| Mean aspect ratio (F) | 2.067 | 1.251 | 1.243 | 1.205 |
| Area variation (F) | 0.381 | 0.242 | 0.345 | 0.365 |
| Hausdorff, ‰ of the diagonal (H) | 52.83 | 25.69 | 10.70 | 10.68 |
| Mean distance, ‰ (H) | 2.978 | 2.627 | 1.276 | 0.492 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 96.35 | 99.47 | 99.93 | 99.97 |
| Off budget, % (N) | 72.0 | 0.1 | 0.1 | 1.7 |
| Time, seconds (T) | 0.60 | 0.97 | 1.18 | 2.40 |

## suzanne (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 357 | 1002 | 994 | 1771 |
| Irregular vertices (S) | 48 | 44 | 48 | 50 |
| … of which above the minimum | 24 | — | — | — |
| Base complex patches (C) | 357 | 601 | 554 | 848 |
| Alignment, degrees (A) | 18.27 | 11.38 | 10.36 | 10.68 |
| Mean angle deviation, degrees (F) | 15.83 | 4.85 | 5.61 | 4.71 |
| Angle deviation, 95th percentile (F) | 49.29 | 17.33 | 18.44 | 14.65 |
| Mean aspect ratio (F) | 2.035 | 1.231 | 1.303 | 1.235 |
| Area variation (F) | 0.760 | 0.287 | 0.433 | 0.389 |
| Hausdorff, ‰ of the diagonal (H) | 137.08 | 16.89 | 12.92 | 9.24 |
| Mean distance, ‰ (H) | 13.357 | 1.672 | 1.473 | 0.799 |
| Thin parts: mean distance, % of a quad edge (V) | 112.8 | 9.7 | 5.1 | 3.9 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 64.3 | 0.2 | 0.6 | 1.1 |
| Time, seconds (T) | 0.37 | 1.25 | 1.44 | 1.37 |

## ogre (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 311 | 972 | 1019 | 2891 |
| Irregular vertices (S) | 36 | 46 | 102 | 153 |
| … of which above the minimum | — | 38 | 94 | 145 |
| Base complex patches (C) | — | 492 | 689 | 2583 |
| Alignment, degrees (A) | 17.10 | 10.49 | 11.09 | 11.69 |
| Mean angle deviation, degrees (F) | 13.19 | 7.32 | 9.05 | 6.46 |
| Angle deviation, 95th percentile (F) | 35.34 | 20.26 | 26.58 | 18.43 |
| Mean aspect ratio (F) | 1.883 | 1.713 | 1.659 | 1.416 |
| Area variation (F) | 0.671 | 0.407 | 0.805 | 0.509 |
| Hausdorff, ‰ of the diagonal (H) | 126.19 | 28.58 | 30.14 | 29.77 |
| Mean distance, ‰ (H) | 13.988 | 1.640 | 1.796 | 0.768 |
| Thin parts: mean distance, % of a quad edge (V) | 180.3 | 20.3 | 5.9 | 7.6 |
| Volume kept, % of the source (V) | 87.06 | 99.70 | 99.75 | 100.02 |
| Off budget, % (N) | 68.9 | 2.8 | 1.9 | 0.0 |
| Time, seconds (T) | 1.05 | 2.81 | 3.42 | 4.36 |

## spot (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 404 | 990 | 986 | 1514 |
| Irregular vertices (S) | 39 | 64 | 68 | 68 |
| … of which above the minimum | — | 56 | 60 | 60 |
| Base complex patches (C) | — | 770 | 754 | 968 |
| Alignment, degrees (A) | 16.98 | 9.51 | 7.53 | 7.80 |
| Mean angle deviation, degrees (F) | 9.29 | 7.26 | 6.93 | 6.13 |
| Angle deviation, 95th percentile (F) | 25.28 | 19.69 | 20.12 | 17.28 |
| Mean aspect ratio (F) | 1.483 | 1.389 | 1.378 | 1.364 |
| Area variation (F) | 0.498 | 0.397 | 0.460 | 0.427 |
| Hausdorff, ‰ of the diagonal (H) | 66.11 | 12.54 | 14.44 | 14.02 |
| Mean distance, ‰ (H) | 5.964 | 0.659 | 0.606 | 0.411 |
| Thin parts: mean distance, % of a quad edge (V) | 41.7 | 13.4 | 16.2 | 17.7 |
| Volume kept, % of the source (V) | 91.73 | 100.04 | 100.01 | 100.00 |
| Off budget, % (N) | 59.6 | 1.0 | 1.4 | 1.1 |
| Time, seconds (T) | 0.47 | 1.81 | 2.60 | 2.76 |

## bob (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 576 | 993 | 991 | 951 |
| Irregular vertices (S) | 12 | 18 | 20 | 20 |
| … of which above the minimum | 12 | 18 | 20 | 20 |
| Base complex patches (C) | 125 | 237 | 326 | 185 |
| Alignment, degrees (A) | 7.87 | 4.29 | 4.18 | 3.79 |
| Mean angle deviation, degrees (F) | 9.54 | 4.49 | 5.06 | 4.80 |
| Angle deviation, 95th percentile (F) | 23.72 | 11.75 | 13.49 | 13.43 |
| Mean aspect ratio (F) | 2.466 | 1.380 | 1.415 | 1.419 |
| Area variation (F) | 0.448 | 0.435 | 0.433 | 0.449 |
| Hausdorff, ‰ of the diagonal (H) | 11.15 | 4.85 | 5.18 | 4.39 |
| Mean distance, ‰ (H) | 2.793 | 0.384 | 0.390 | 0.398 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 94.76 | 100.05 | 100.03 | 100.05 |
| Off budget, % (N) | 42.4 | 0.7 | 0.9 | 0.3 |
| Time, seconds (T) | 0.83 | 1.53 | 1.76 | 3.44 |

## blub (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 604 | 1027 | 984 | 1663 |
| Irregular vertices (S) | 58 | 40 | 50 | 50 |
| … of which above the minimum | — | 32 | 42 | 42 |
| Base complex patches (C) | — | 463 | 532 | 1069 |
| Alignment, degrees (A) | 14.60 | 8.33 | 9.03 | 8.06 |
| Mean angle deviation, degrees (F) | 10.41 | 4.92 | 5.70 | 5.03 |
| Angle deviation, 95th percentile (F) | 32.33 | 14.45 | 17.17 | 15.73 |
| Mean aspect ratio (F) | 1.771 | 1.283 | 1.313 | 1.322 |
| Area variation (F) | 0.634 | 0.350 | 0.464 | 0.413 |
| Hausdorff, ‰ of the diagonal (H) | 70.95 | 22.61 | 15.87 | 13.14 |
| Mean distance, ‰ (H) | 4.041 | 0.655 | 0.653 | 0.327 |
| Thin parts: mean distance, % of a quad edge (V) | 31.8 | 12.1 | 8.2 | 7.1 |
| Volume kept, % of the source (V) | — | 99.68 | 99.74 | 99.90 |
| Off budget, % (N) | 39.6 | 2.7 | 1.6 | 1.5 |
| Time, seconds (T) | 0.71 | 2.60 | 2.31 | 2.30 |

## pig (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 548 | 989 | 1008 | 1953 |
| Irregular vertices (S) | 69 | 60 | 90 | 94 |
| … of which above the minimum | — | 52 | 82 | 86 |
| Base complex patches (C) | — | 587 | 588 | 1181 |
| Alignment, degrees (A) | 14.59 | 12.40 | 9.21 | 10.16 |
| Mean angle deviation, degrees (F) | 12.13 | 7.11 | 7.11 | 6.09 |
| Angle deviation, 95th percentile (F) | 38.98 | 23.49 | 21.71 | 18.37 |
| Mean aspect ratio (F) | 1.947 | 1.332 | 1.445 | 1.297 |
| Area variation (F) | 0.836 | 0.325 | 0.885 | 0.514 |
| Hausdorff, ‰ of the diagonal (H) | 63.50 | 27.73 | 14.22 | 21.90 |
| Mean distance, ‰ (H) | 8.335 | 1.047 | 1.035 | 0.474 |
| Thin parts: mean distance, % of a quad edge (V) | 44.9 | 13.1 | 4.3 | 7.3 |
| Volume kept, % of the source (V) | 90.89 | 99.45 | 99.77 | 99.93 |
| Off budget, % (N) | 45.2 | 1.1 | 0.8 | 1.1 |
| Time, seconds (T) | 0.58 | 2.07 | 2.27 | 2.98 |

## jerry (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 305 | 1004 | 987 | 3034 |
| Irregular vertices (S) | 33 | 51 | 83 | 172 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | 609 | 863 | 2606 |
| Alignment, degrees (A) | 20.52 | 14.39 | 15.03 | 14.05 |
| Mean angle deviation, degrees (F) | 18.53 | 7.86 | 9.31 | 7.10 |
| Angle deviation, 95th percentile (F) | 57.15 | 21.83 | 27.74 | 21.75 |
| Mean aspect ratio (F) | 1.887 | 1.436 | 1.467 | 1.348 |
| Area variation (F) | 1.686 | 0.324 | 0.752 | 0.539 |
| Hausdorff, ‰ of the diagonal (H) | 201.66 | 36.46 | 26.83 | 26.17 |
| Mean distance, ‰ (H) | 12.497 | 3.005 | 2.933 | 0.836 |
| Thin parts: mean distance, % of a quad edge (V) | 71.5 | 21.9 | 14.0 | 16.8 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 69.5 | 0.4 | 1.3 | 1.9 |
| Time, seconds (T) | 0.40 | 2.44 | 1.68 | 2.93 |

## rose (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 157 | 1571 | 1403 | 26471 |
| Irregular vertices (S) | 122 | 690 | 689 | 2259 |
| … of which above the minimum | — | — | — | 1643 |
| Base complex patches (C) | — | — | — | 19881 |
| Alignment, degrees (A) | 18.25 | 13.56 | — | 9.96 |
| Mean angle deviation, degrees (F) | 49.67 | 29.26 | 29.69 | 7.76 |
| Angle deviation, 95th percentile (F) | 87.99 | 75.25 | 77.33 | 25.00 |
| Mean aspect ratio (F) | 8.023 | 16.549 | 11.075 | 1.536 |
| Area variation (F) | 1.952 | 2.339 | 2.984 | 0.584 |
| Hausdorff, ‰ of the diagonal (H) | 195.94 | 73.60 | 89.42 | 7.95 |
| Mean distance, ‰ (H) | 46.413 | 17.915 | 20.570 | 0.214 |
| Thin parts: mean distance, % of a quad edge (V) | 184.7 | 78.8 | 87.6 | 9.1 |
| Volume kept, % of the source (V) | — | 13.95 | — | 99.50 |
| Off budget, % (N) | 84.3 | 57.1 | 40.3 | 0.5 |
| Time, seconds (T) | 0.53 | 3.79 | 3.88 | 16.06 |

## bridge (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 748 | 1008 | 1008 | 2120 |
| Irregular vertices (S) | 54 | 36 | 36 | 36 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | 254 | 451 | 603 |
| Alignment, degrees (A) | 11.10 | 6.96 | 6.66 | 7.40 |
| Mean angle deviation, degrees (F) | 16.94 | 4.11 | 5.57 | 4.40 |
| Angle deviation, 95th percentile (F) | 65.68 | 12.89 | 17.02 | 12.30 |
| Mean aspect ratio (F) | 2.304 | 1.284 | 1.330 | 1.265 |
| Area variation (F) | 1.107 | 0.270 | 0.535 | 0.430 |
| Hausdorff, ‰ of the diagonal (H) | 98.43 | 17.46 | 9.08 | 7.75 |
| Mean distance, ‰ (H) | 7.488 | 0.891 | 0.814 | 0.555 |
| Thin parts: mean distance, % of a quad edge (V) | 15.2 | 4.6 | 2.8 | — |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 25.2 | 0.8 | 0.8 | 0.7 |
| Time, seconds (T) | 0.86 | 2.69 | 3.19 | 3.30 |

## part (mechanical)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 348 | 976 | 1018 | 1018 |
| Irregular vertices (S) | 69 | 24 | 24 | 24 |
| … of which above the minimum | — | 16 | 16 | 16 |
| Base complex patches (C) | — | 86 | 68 | 68 |
| Alignment, degrees (A) | 19.45 | 12.46 | 10.80 | 10.82 |
| Mean angle deviation, degrees (F) | 19.61 | 2.07 | 2.16 | 2.17 |
| Angle deviation, 95th percentile (F) | 60.84 | 9.04 | 10.18 | 10.14 |
| Mean aspect ratio (F) | 2.662 | 1.219 | 1.177 | 1.177 |
| Area variation (F) | 0.647 | 0.175 | 0.381 | 0.381 |
| Hausdorff, ‰ of the diagonal (H) | 93.30 | 14.36 | 1.37 | 1.37 |
| Mean distance, ‰ (H) | 8.243 | 0.259 | 0.165 | 0.165 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | — | 100.01 | 100.01 | 100.01 |
| Off budget, % (N) | 65.2 | 2.4 | 1.8 | 0.5 |
| Time, seconds (T) | 0.34 | 1.03 | 1.13 | 1.07 |

## cube (mechanical)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 573 | 1014 | 1014 | 1014 |
| Irregular vertices (S) | 13 | 8 | 8 | 8 |
| … of which above the minimum | — | 0 | 0 | 0 |
| Base complex patches (C) | — | 6 | 6 | 6 |
| Alignment, degrees (A) | 3.25 | 0.11 | 0.25 | 0.25 |
| Mean angle deviation, degrees (F) | 4.61 | 0.00 | 0.33 | 0.33 |
| Angle deviation, 95th percentile (F) | 14.18 | 0.01 | 0.76 | 0.76 |
| Mean aspect ratio (F) | 1.707 | 1.000 | 1.071 | 1.071 |
| Area variation (F) | 0.500 | 0.000 | 0.082 | 0.081 |
| Hausdorff, ‰ of the diagonal (H) | 23.35 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 1.194 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 99.43 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 42.7 | 1.4 | 1.4 | 0.0 |
| Time, seconds (T) | 0.15 | 0.64 | 0.74 | 0.72 |

## lblock (mechanical)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 588 | 994 | 994 | 1028 |
| Irregular vertices (S) | 14 | 12 | 12 | 12 |
| … of which above the minimum | 6 | 4 | 4 | 4 |
| Base complex patches (C) | 132 | 14 | 14 | 14 |
| Alignment, degrees (A) | 4.35 | 0.56 | 0.64 | 0.64 |
| Mean angle deviation, degrees (F) | 4.89 | 0.10 | 0.37 | 0.39 |
| Angle deviation, 95th percentile (F) | 14.86 | 0.33 | 0.90 | 1.01 |
| Mean aspect ratio (F) | 1.571 | 1.072 | 1.099 | 1.098 |
| Area variation (F) | 0.415 | 0.070 | 0.096 | 0.096 |
| Hausdorff, ‰ of the diagonal (H) | 29.11 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 1.880 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 99.10 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 41.2 | 0.6 | 0.6 | 1.8 |
| Time, seconds (T) | 0.18 | 0.68 | 0.78 | 0.79 |

## cylinder (mechanical)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 612 | 990 | 1002 | 764 |
| Irregular vertices (S) | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 164 | 6 | 186 | 56 |
| Alignment, degrees (A) | 5.40 | 1.23 | 1.85 | 1.37 |
| Mean angle deviation, degrees (F) | 8.90 | 2.64 | 3.73 | 2.54 |
| Angle deviation, 95th percentile (F) | 26.84 | 7.63 | 8.66 | 9.04 |
| Mean aspect ratio (F) | 1.965 | 1.286 | 1.504 | 1.509 |
| Area variation (F) | 0.576 | 0.334 | 0.218 | 0.205 |
| Hausdorff, ‰ of the diagonal (H) | 14.27 | 1.47 | 1.70 | 1.93 |
| Mean distance, ‰ (H) | 2.062 | 0.311 | 0.349 | 0.457 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 98.02 | 99.99 | 100.02 | 99.99 |
| Off budget, % (N) | 38.8 | 1.0 | 0.2 | 2.3 |
| Time, seconds (T) | 0.24 | 0.81 | 1.01 | 0.79 |

## venturi (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 2390 | 757 | 836 | 12453 |
| Irregular vertices (S) | 1182 | 150 | 176 | 874 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | — | — | 10479 |
| Alignment, degrees (A) | 8.64 | 11.32 | 13.71 | 11.18 |
| Mean angle deviation, degrees (F) | 53.69 | 11.89 | 14.03 | 7.67 |
| Angle deviation, 95th percentile (F) | 89.78 | 37.21 | 45.64 | 28.89 |
| Mean aspect ratio (F) | 7.059 | 1.553 | 3.065 | 1.538 |
| Area variation (F) | 5.029 | 0.486 | 0.595 | 0.377 |
| Hausdorff, ‰ of the diagonal (H) | 174.38 | 82.83 | 82.83 | 21.31 |
| Mean distance, ‰ (H) | 17.134 | 7.832 | 8.266 | 1.274 |
| Thin parts: mean distance, % of a quad edge (V) | 31.9 | 21.1 | 22.9 | 22.8 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 139.0 | 24.3 | 16.4 | 2.4 |
| Time, seconds (T) | 0.46 | 7.28 | 8.91 | 81.12 |

## ogre_hp (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 334 | 1019 | 1011 | 2885 |
| Irregular vertices (S) | 38 | 46 | 118 | 209 |
| … of which above the minimum | — | 38 | 110 | 201 |
| Base complex patches (C) | — | 707 | 733 | 2351 |
| Alignment, degrees (A) | 18.93 | 14.59 | 15.15 | 13.03 |
| Mean angle deviation, degrees (F) | 14.98 | 8.13 | 9.39 | 6.94 |
| Angle deviation, 95th percentile (F) | 43.87 | 22.79 | 26.33 | 20.32 |
| Mean aspect ratio (F) | 1.799 | 1.668 | 1.559 | 1.430 |
| Area variation (F) | 0.758 | 0.375 | 0.892 | 0.775 |
| Hausdorff, ‰ of the diagonal (H) | 124.75 | 31.77 | 30.52 | 24.12 |
| Mean distance, ‰ (H) | 11.455 | 1.647 | 1.906 | 0.786 |
| Thin parts: mean distance, % of a quad edge (V) | 89.7 | 22.8 | 5.9 | 3.9 |
| Volume kept, % of the source (V) | 86.48 | 99.58 | 99.83 | 100.07 |
| Off budget, % (N) | 66.6 | 1.9 | 1.1 | 1.2 |
| Time, seconds (T) | 15.59 | 57.27 | 56.00 | 64.54 |

## watermill (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 225 | 592 | 592 | — |
| Irregular vertices (S) | 79 | 106 | 106 | — |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | — | — | — |
| Alignment, degrees (A) | 19.32 | 16.52 | 16.52 | — |
| Mean angle deviation, degrees (F) | 26.39 | 16.68 | 16.70 | — |
| Angle deviation, 95th percentile (F) | 82.13 | 62.19 | 62.19 | — |
| Mean aspect ratio (F) | 4.170 | 2.301 | 2.323 | — |
| Area variation (F) | 1.055 | 0.800 | 0.800 | — |
| Hausdorff, ‰ of the diagonal (H) | 298.17 | 119.51 | 119.51 | — |
| Mean distance, ‰ (H) | 82.620 | 15.502 | 15.509 | — |
| Thin parts: mean distance, % of a quad edge (V) | 197.5 | 39.7 | 39.7 | — |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 77.5 | 40.8 | 40.8 | — |
| Time, seconds (T) | 1.40 | 10.30 | 12.97 | — |

Failures: openclare-matched: no Quad Remesher reference to match.

## towermill (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 405 | 1826 | 685 | — |
| Irregular vertices (S) | 121 | 311 | 94 | — |
| … of which above the minimum | — | 7 | — | — |
| Base complex patches (C) | — | 1826 | — | — |
| Alignment, degrees (A) | 18.17 | 16.89 | 14.91 | — |
| Mean angle deviation, degrees (F) | 27.63 | 25.86 | 15.35 | — |
| Angle deviation, 95th percentile (F) | 75.88 | 68.26 | 50.55 | — |
| Mean aspect ratio (F) | 16.899 | 5.045 | 1.801 | — |
| Area variation (F) | 0.903 | 1.939 | 0.428 | — |
| Hausdorff, ‰ of the diagonal (H) | 138.78 | 75.07 | 110.42 | — |
| Mean distance, ‰ (H) | 32.456 | 6.685 | 12.376 | — |
| Thin parts: mean distance, % of a quad edge (V) | 75.3 | 19.2 | 30.9 | — |
| Volume kept, % of the source (V) | — | 78.09 | — | — |
| Off budget, % (N) | 59.5 | 82.6 | 31.5 | — |
| Time, seconds (T) | 1.28 | 22.04 | 14.02 | — |

Failures: openclare-matched: no Quad Remesher reference to match.

## stonemill (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 0/1 | 0/1 |
| Quads made | 703 | 1013 | 1003 | — |
| Irregular vertices (S) | 73 | 92 | 155 | — |
| … of which above the minimum | — | 28 | 91 | — |
| Base complex patches (C) | — | 963 | 978 | — |
| Alignment, degrees (A) | 17.16 | 13.18 | 14.72 | — |
| Mean angle deviation, degrees (F) | 11.97 | 9.06 | 13.44 | — |
| Angle deviation, 95th percentile (F) | 36.94 | 32.14 | 40.37 | — |
| Mean aspect ratio (F) | 1.994 | 1.693 | 2.058 | — |
| Area variation (F) | 0.656 | 0.440 | 0.923 | — |
| Hausdorff, ‰ of the diagonal (H) | 186.96 | 36.21 | 30.67 | — |
| Mean distance, ‰ (H) | 9.114 | 3.555 | 3.498 | — |
| Thin parts: mean distance, % of a quad edge (V) | 76.4 | 22.2 | 13.8 | — |
| Volume kept, % of the source (V) | — | 99.38 | 99.40 | — |
| Off budget, % (N) | 29.7 | 1.3 | 0.3 | — |
| Time, seconds (T) | 0.53 | 1.48 | 4.61 | — |

Failures: openclare-matched: no Quad Remesher reference to match.

## windmill (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 370 | 141 | 164 | — |
| Irregular vertices (S) | 75 | 28 | 36 | — |
| … of which above the minimum | — | 4 | 12 | — |
| Base complex patches (C) | — | 141 | 164 | — |
| Alignment, degrees (A) | 14.45 | 17.99 | 21.40 | — |
| Mean angle deviation, degrees (F) | 28.12 | 30.30 | 31.43 | — |
| Angle deviation, 95th percentile (F) | 83.45 | 72.51 | 73.77 | — |
| Mean aspect ratio (F) | 3.970 | 4.563 | 4.437 | — |
| Area variation (F) | 0.923 | 1.077 | 1.342 | — |
| Hausdorff, ‰ of the diagonal (H) | 203.64 | 538.64 | 539.23 | — |
| Mean distance, ‰ (H) | 36.911 | 255.822 | 252.352 | — |
| Thin parts: mean distance, % of a quad edge (V) | 110.1 | 712.3 | 706.9 | — |
| Volume kept, % of the source (V) | — | 0.89 | 0.69 | — |
| Off budget, % (N) | 63.0 | 85.9 | 83.6 | — |
| Time, seconds (T) | 0.77 | 0.62 | 1.13 | — |

Failures: openclare-matched: no Quad Remesher reference to match.

## terrain (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 308 | 980 | 1000 | 1512 |
| Irregular vertices (S) | 30 | 34 | 38 | 42 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | 500 | 646 | 1026 |
| Alignment, degrees (A) | 22.42 | 18.03 | 17.66 | 18.51 |
| Mean angle deviation, degrees (F) | 26.16 | 5.77 | 6.04 | 5.65 |
| Angle deviation, 95th percentile (F) | 73.60 | 17.77 | 18.76 | 16.71 |
| Mean aspect ratio (F) | 1.911 | 1.319 | 1.321 | 1.293 |
| Area variation (F) | 0.775 | 0.353 | 0.460 | 0.454 |
| Hausdorff, ‰ of the diagonal (H) | 234.03 | 23.57 | 26.88 | 31.13 |
| Mean distance, ‰ (H) | 9.103 | 2.302 | 1.984 | 1.615 |
| Thin parts: mean distance, % of a quad edge (V) | 54.8 | 23.5 | 19.4 | 19.9 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 69.2 | 2.0 | 0.0 | 1.9 |
| Time, seconds (T) | 1.82 | 8.45 | 8.47 | 8.74 |

## mahaveer (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 636 | 2037 | 1869 | 40889 |
| Irregular vertices (S) | 343 | 485 | 476 | 1380 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | — | — | — |
| Alignment, degrees (A) | 8.88 | 12.07 | 17.86 | 5.83 |
| Mean angle deviation, degrees (F) | 38.85 | 38.51 | 32.88 | 5.50 |
| Angle deviation, 95th percentile (F) | 87.58 | 84.45 | 85.94 | 21.91 |
| Mean aspect ratio (F) | 7.492 | 155.937 | 205.386 | 10.381 |
| Area variation (F) | 1.913 | 8.209 | 7.845 | 0.960 |
| Hausdorff, ‰ of the diagonal (H) | 88.93 | 325.51 | 325.54 | 13.40 |
| Mean distance, ‰ (H) | 14.421 | 40.240 | 40.314 | 0.638 |
| Thin parts: mean distance, % of a quad edge (V) | 65.0 | 154.1 | 154.1 | 34.0 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 36.4 | 103.7 | 86.9 | 2.2 |
| Time, seconds (T) | 1.40 | 8.98 | 8.65 | 26.05 |

## marine (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 1110 | 8170 | 7967 | 193727 |
| Irregular vertices (S) | 905 | 6210 | 6140 | 11613 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | — | — | — |
| Alignment, degrees (A) | — | — | — | 8.58 |
| Mean angle deviation, degrees (F) | 40.06 | 23.72 | 22.75 | 5.23 |
| Angle deviation, 95th percentile (F) | 88.92 | 71.02 | 69.67 | 20.16 |
| Mean aspect ratio (F) | 4.992 | 6.223 | 4.479 | 1.380 |
| Area variation (F) | 5.647 | 2.827 | 2.855 | 0.574 |
| Hausdorff, ‰ of the diagonal (H) | 11.21 | 7.06 | 6.26 | 3.02 |
| Mean distance, ‰ (H) | 3.155 | 0.435 | 0.452 | 0.038 |
| Thin parts: mean distance, % of a quad edge (V) | 56.2 | 7.9 | 8.2 | 18.3 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 11.0 | 717.0 | 696.7 | 5.6 |
| Time, seconds (T) | 1.33 | 14.02 | 20.68 | 56.46 |

## f16 (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 3146 | 2562 | 2662 | 28141 |
| Irregular vertices (S) | 1983 | 1767 | 1662 | 1942 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | — | — | — |
| Alignment, degrees (A) | 17.00 | 4.70 | 4.94 | 4.12 |
| Mean angle deviation, degrees (F) | 45.13 | 21.40 | 21.77 | 6.43 |
| Angle deviation, 95th percentile (F) | 87.51 | 78.34 | 77.58 | 27.97 |
| Mean aspect ratio (F) | 8.830 | 179.581 | 198.798 | 17.802 |
| Area variation (F) | 5.401 | 6.287 | 5.709 | 1.063 |
| Hausdorff, ‰ of the diagonal (H) | 128.15 | 112.60 | 107.04 | 13.12 |
| Mean distance, ‰ (H) | 13.795 | 5.348 | 4.618 | 0.351 |
| Thin parts: mean distance, % of a quad edge (V) | 52.9 | 16.5 | 14.1 | 7.9 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 214.6 | 156.2 | 166.2 | 7.5 |
| Time, seconds (T) | 1.42 | 14.15 | 13.18 | 92.21 |

## camera (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 988 | 1462 | 1670 | 8871 |
| Irregular vertices (S) | 272 | 136 | 182 | 314 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | — | 484 | — |
| Alignment, degrees (A) | 9.41 | 4.95 | 6.02 | 3.20 |
| Mean angle deviation, degrees (F) | 25.43 | 19.83 | 20.85 | 8.41 |
| Angle deviation, 95th percentile (F) | 86.02 | 70.71 | 71.98 | 42.32 |
| Mean aspect ratio (F) | 3.381 | 39.958 | 58.806 | 9.470 |
| Area variation (F) | 1.855 | 2.039 | 1.978 | 0.787 |
| Hausdorff, ‰ of the diagonal (H) | 72.47 | 52.85 | 39.36 | 19.79 |
| Mean distance, ‰ (H) | 8.475 | 6.157 | 5.818 | 0.941 |
| Thin parts: mean distance, % of a quad edge (V) | 17.9 | 13.6 | 13.0 | 11.8 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 1.2 | 46.2 | 67.0 | 5.5 |
| Time, seconds (T) | 0.30 | 2.72 | 2.92 | 13.06 |

## wolf (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 0/1 |
| Quads made | 180 | 759 | 752 | 5955 |
| Irregular vertices (S) | 56 | 126 | 126 | 418 |
| … of which above the minimum | — | 62 | 62 | — |
| Base complex patches (C) | — | 465 | 458 | — |
| Alignment, degrees (A) | 14.42 | 9.99 | 10.11 | 10.58 |
| Mean angle deviation, degrees (F) | 24.14 | 13.62 | 13.11 | 9.51 |
| Angle deviation, 95th percentile (F) | 87.70 | 39.94 | 36.87 | 28.14 |
| Mean aspect ratio (F) | 3.965 | 1.995 | 1.981 | 1.475 |
| Area variation (F) | 0.945 | 0.494 | 0.485 | 0.342 |
| Hausdorff, ‰ of the diagonal (H) | 123.45 | 40.53 | 40.53 | 20.49 |
| Mean distance, ‰ (H) | 18.850 | 5.461 | 5.421 | 0.843 |
| Thin parts: mean distance, % of a quad edge (V) | 123.8 | 50.6 | 50.0 | 39.0 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 82.0 | 24.1 | 24.8 | 16.1 |
| Time, seconds (T) | 0.44 | 6.41 | 6.35 | 73.47 |

## sphere (calibration)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 768 | 1010 | 1010 | 900 |
| Irregular vertices (S) | 8 | 8 | 8 | 10 |
| … of which above the minimum | 0 | 0 | 0 | 2 |
| Base complex patches (C) | 768 | 1010 | 1010 | 846 |
| Alignment, degrees (A) | — | — | — | — |
| Mean angle deviation, degrees (F) | 5.13 | 4.61 | 4.61 | 4.08 |
| Angle deviation, 95th percentile (F) | 13.45 | 11.13 | 11.13 | 10.24 |
| Mean aspect ratio (F) | 1.336 | 1.180 | 1.180 | 1.153 |
| Area variation (F) | 0.229 | 0.225 | 0.225 | 0.240 |
| Hausdorff, ‰ of the diagonal (H) | 3.06 | 0.99 | 0.99 | 2.07 |
| Mean distance, ‰ (H) | 1.508 | 0.332 | 0.332 | 0.357 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 98.48 | 100.02 | 100.02 | 100.02 |
| Off budget, % (N) | 23.2 | 1.0 | 1.0 | 1.6 |
| Time, seconds (T) | 0.63 | 0.95 | 1.49 | 1.17 |

## torus (calibration)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 728 | 1008 | 1008 | 920 |
| Irregular vertices (S) | 0 | 0 | 0 | 0 |
| … of which above the minimum | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 1 | 1 | 1 | 1 |
| Alignment, degrees (A) | 1.94 | 0.16 | 0.16 | 0.64 |
| Mean angle deviation, degrees (F) | 4.57 | 2.40 | 2.40 | 2.52 |
| Angle deviation, 95th percentile (F) | 9.20 | 3.73 | 3.73 | 3.91 |
| Mean aspect ratio (F) | 3.376 | 1.295 | 1.295 | 1.293 |
| Area variation (F) | 0.121 | 0.350 | 0.351 | 0.356 |
| Hausdorff, ‰ of the diagonal (H) | 5.42 | 1.35 | 1.35 | 1.67 |
| Mean distance, ‰ (H) | 1.726 | 0.424 | 0.424 | 0.449 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 96.27 | 100.04 | 100.04 | 100.06 |
| Off budget, % (N) | 27.2 | 0.8 | 0.8 | 0.0 |
| Time, seconds (T) | 0.22 | 1.17 | 1.71 | 0.89 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| Irregular vertices (S) | 8 | 6 | 3 | 3 |
| Base complex patches (C) | 5 | 5 | 2 | 1 |
| Alignment, degrees (A) | 0 | 1 | 6 | 6 |
| Mean angle deviation, degrees (F) | 0 | 4 | 2 | 7 |
| Mean aspect ratio (F) | 0 | 4 | 0 | 9 |
| Valid meshes (V) | 4/13 | 12/13 | 12/13 | 12/13 |

### mechanical (4 models)

| Metric | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| Irregular vertices (S) | 1 | 4 | 4 | 4 |
| Base complex patches (C) | 0 | 3 | 3 | 3 |
| Alignment, degrees (A) | 0 | 3 | 1 | 0 |
| Mean angle deviation, degrees (F) | 0 | 3 | 0 | 1 |
| Mean aspect ratio (F) | 0 | 3 | 1 | 0 |
| Valid meshes (V) | 2/4 | 4/4 | 4/4 | 4/4 |
