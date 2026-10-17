# Bench results

Requested budget: 1000 quads. Median over 5 seeds; Quad Remesher, AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

## blob (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 941 | 400 | 1026 | 990 | 1362 | 1340 |
| Irregular vertices (S) | 36 | 24 | 24 | 24 | 26 | 24 |
| … of which above the minimum | 28 | 16 | 16 | 16 | 18 | 16 |
| Base complex patches (C) | 941 | 314 | 664 | 790 | 520 | 1148 |
| Alignment, degrees (A) | 18.04 | 8.83 | 5.27 | 4.43 | 3.42 | 4.87 |
| Mean angle deviation, degrees (F) | 9.26 | 12.46 | 6.39 | 5.85 | 6.86 | 5.86 |
| Angle deviation, 95th percentile (F) | 25.27 | 33.98 | 16.66 | 14.88 | 20.12 | 15.76 |
| Mean aspect ratio (F) | 1.221 | 1.652 | 1.373 | 1.350 | 2.760 | 1.346 |
| Area variation (F) | 0.218 | 0.577 | 0.453 | 0.463 | 0.625 | 0.434 |
| Hausdorff, ‰ of the diagonal (H) | 14.52 | 16.95 | 10.26 | 6.34 | 7.74 | 4.33 |
| Mean distance, ‰ (H) | 1.325 | 5.131 | 0.540 | 0.596 | 1.172 | 0.396 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.58 | 92.19 | 100.05 | 100.07 | 98.08 | 100.04 |
| Edges on closed loops, % (L) | 0.0 | 10.8 | — | 10.1 | 33.7 | — |
| Edges on spiralling loops, % (L) | 25.3 | 35.2 | — | 58.2 | 18.2 | — |
| Spiralling loops (L) | 4 | 7 | — | 8 | 4 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 74.7 | 54.0 | — | 31.7 | 48.1 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 108 | 44 | — | 94 | 113 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 5.9 | 60.0 | 2.6 | 1.0 | 36.2 | 1.6 |
| Time, seconds (T) | 1.82 | 0.43 | 2.76 | 1.98 | 14.01 | 1.86 |

## rock (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 947 | 778 | 978 | 981 | 1290 | 1298 |
| Irregular vertices (S) | 8 | 10 | 26 | 26 | 90 | 34 |
| … of which above the minimum | 0 | 2 | 18 | 18 | 82 | 26 |
| Base complex patches (C) | 941 | 506 | 767 | 981 | 1290 | 1298 |
| Alignment, degrees (A) | 23.14 | 19.26 | 17.02 | 16.85 | 13.52 | 16.65 |
| Mean angle deviation, degrees (F) | 6.16 | 7.46 | 5.76 | 4.95 | 11.01 | 5.62 |
| Angle deviation, 95th percentile (F) | 16.14 | 18.13 | 16.05 | 14.03 | 27.61 | 15.70 |
| Mean aspect ratio (F) | 1.162 | 1.494 | 1.257 | 1.228 | 1.631 | 1.227 |
| Area variation (F) | 0.179 | 0.473 | 0.354 | 0.330 | 0.354 | 0.293 |
| Hausdorff, ‰ of the diagonal (H) | 9.38 | 14.09 | 8.83 | 7.73 | 9.88 | 7.55 |
| Mean distance, ‰ (H) | 1.512 | 2.480 | 1.242 | 1.241 | 1.291 | 1.059 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.30 | 98.22 | 100.02 | 99.99 | 99.46 | 100.02 |
| Edges on closed loops, % (L) | 0.0 | 3.7 | — | 0.0 | 0.0 | — |
| Edges on spiralling loops, % (L) | 60.3 | 69.2 | — | 31.4 | 3.8 | — |
| Spiralling loops (L) | 3 | 6 | — | 4 | 1 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 35.9 | 27.1 | — | 68.6 | 96.2 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 133 | 82 | — | 134 | 75 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 5.3 | 22.2 | 2.2 | 1.9 | 29.0 | 0.6 |
| Time, seconds (T) | 1.10 | 0.33 | 2.47 | 1.70 | 28.00 | 1.69 |

## creature (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 812 | 604 | 965 | 1001 | 1208 | 1207 |
| Irregular vertices (S) | 16 | 12 | 26 | 40 | 42 | 40 |
| … of which above the minimum | 8 | 4 | 18 | 32 | 34 | 32 |
| Base complex patches (C) | 812 | 308 | 773 | 355 | 588 | 465 |
| Alignment, degrees (A) | 14.79 | 11.08 | 8.03 | 5.67 | 4.65 | 5.92 |
| Mean angle deviation, degrees (F) | 12.55 | 6.78 | 5.01 | 5.53 | 6.49 | 5.24 |
| Angle deviation, 95th percentile (F) | 30.83 | 16.26 | 15.21 | 15.71 | 18.32 | 14.86 |
| Mean aspect ratio (F) | 1.391 | 1.697 | 1.246 | 1.347 | 1.473 | 1.299 |
| Area variation (F) | 0.298 | 0.418 | 0.352 | 0.481 | 0.461 | 0.490 |
| Hausdorff, ‰ of the diagonal (H) | 40.53 | 8.78 | 5.03 | 3.25 | 3.74 | 3.30 |
| Mean distance, ‰ (H) | 1.486 | 1.732 | 0.465 | 0.499 | 0.604 | 0.401 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 97.34 | 96.54 | 100.08 | 100.07 | 98.75 | 100.05 |
| Edges on closed loops, % (L) | 0.0 | 24.5 | — | 32.3 | 25.7 | — |
| Edges on spiralling loops, % (L) | 43.6 | 54.3 | — | 10.0 | 17.5 | — |
| Spiralling loops (L) | 6 | 4 | — | 2 | 7 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 56.4 | 21.2 | — | 57.7 | 56.8 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 141 | 52 | — | 95 | 84 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 18.8 | 39.6 | 3.5 | 0.1 | 20.8 | 0.1 |
| Time, seconds (T) | 1.18 | 0.45 | 3.06 | 1.75 | 11.95 | 1.68 |

## cage (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 1043 | 1720 | 1001 | 1001 | 3224 | 3168 |
| Irregular vertices (S) | 64 | 52 | 48 | 48 | 61 | 48 |
| … of which above the minimum | 32 | — | 16 | 16 | 29 | 16 |
| Base complex patches (C) | 716 | — | 263 | 208 | 741 | 438 |
| Alignment, degrees (A) | 7.66 | 8.80 | 4.06 | 3.17 | 2.70 | 2.27 |
| Mean angle deviation, degrees (F) | 6.96 | 10.60 | 4.72 | 5.00 | 4.46 | 3.14 |
| Angle deviation, 95th percentile (F) | 21.11 | 30.15 | 14.08 | 15.30 | 15.38 | 9.61 |
| Mean aspect ratio (F) | 1.250 | 2.067 | 1.251 | 1.243 | 2.906 | 1.205 |
| Area variation (F) | 0.155 | 0.381 | 0.242 | 0.345 | 0.384 | 0.365 |
| Hausdorff, ‰ of the diagonal (H) | 21.45 | 52.83 | 25.69 | 10.70 | 7.11 | 10.68 |
| Mean distance, ‰ (H) | 1.546 | 2.978 | 2.627 | 1.276 | 0.849 | 0.492 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.12 | 96.35 | 99.47 | 99.93 | 99.30 | 99.97 |
| Edges on closed loops, % (L) | 17.2 | 16.0 | — | 53.8 | 48.0 | — |
| Edges on spiralling loops, % (L) | 23.4 | 36.5 | — | 12.9 | 28.6 | — |
| Spiralling loops (L) | 10 | 20 | — | 4 | 12 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 58.8 | 47.5 | — | 33.3 | 23.4 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 47 | 74 | — | 20 | 64 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 4.3 | 72.0 | 0.1 | 0.1 | 222.4 | 1.7 |
| Time, seconds (T) | 0.38 | 0.75 | 0.97 | 2.01 | 5.08 | 2.40 |

## suzanne (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 1005 | 357 | 1002 | 994 | 1790 | 1771 |
| Irregular vertices (S) | 67 | 48 | 44 | 48 | 68 | 50 |
| … of which above the minimum | — | 24 | — | — | — | — |
| Base complex patches (C) | 995 | 357 | 601 | 554 | 1538 | 848 |
| Alignment, degrees (A) | 15.73 | 18.27 | 11.38 | 10.36 | 8.58 | 10.68 |
| Mean angle deviation, degrees (F) | 9.65 | 15.83 | 4.85 | 5.61 | 8.74 | 4.71 |
| Angle deviation, 95th percentile (F) | 29.23 | 49.29 | 17.33 | 18.44 | 23.66 | 14.65 |
| Mean aspect ratio (F) | 1.325 | 2.035 | 1.231 | 1.303 | 1.823 | 1.235 |
| Area variation (F) | 0.506 | 0.760 | 0.287 | 0.433 | 0.551 | 0.389 |
| Hausdorff, ‰ of the diagonal (H) | 44.79 | 137.08 | 16.89 | 12.92 | 10.72 | 9.24 |
| Mean distance, ‰ (H) | 2.245 | 13.357 | 1.672 | 1.473 | 1.112 | 0.799 |
| Thin parts: mean distance, % of a quad edge (V) | 10.6 | 112.8 | 9.7 | 5.1 | 4.8 | 3.9 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 2.3 | 0.0 | — | 18.9 | 4.2 | — |
| Edges on spiralling loops, % (L) | 7.9 | 5.7 | — | 8.0 | 15.1 | — |
| Spiralling loops (L) | 5 | 2 | — | 4 | 7 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.6 | 94.3 | — | 67.9 | 77.2 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 97 | 43 | — | 54 | 112 | — |
| Edges on loops from a border to a border, % (L) | 0.6 | 0.0 | — | 5.2 | 3.5 | — |
| Off budget, % (N) | 1.0 | 64.3 | 0.2 | 0.6 | 79.0 | 1.1 |
| Time, seconds (T) | 0.66 | 0.43 | 1.25 | 1.64 | 4.46 | 1.37 |

## ogre (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 891 | 311 | 972 | 1019 | 2890 | 2891 |
| Irregular vertices (S) | 73 | 36 | 46 | 102 | 245 | 153 |
| … of which above the minimum | 64 | — | 38 | 94 | 237 | 145 |
| Base complex patches (C) | 891 | — | 492 | 689 | 2186 | 2583 |
| Alignment, degrees (A) | 20.86 | 17.10 | 10.49 | 11.09 | 7.89 | 11.69 |
| Mean angle deviation, degrees (F) | 14.18 | 13.19 | 7.32 | 9.05 | 9.34 | 6.46 |
| Angle deviation, 95th percentile (F) | 39.23 | 35.34 | 20.26 | 26.58 | 26.48 | 18.43 |
| Mean aspect ratio (F) | 1.552 | 1.883 | 1.713 | 1.659 | 2.393 | 1.416 |
| Area variation (F) | 0.348 | 0.671 | 0.407 | 0.805 | 0.728 | 0.509 |
| Hausdorff, ‰ of the diagonal (H) | 95.01 | 126.19 | 28.58 | 30.14 | 27.76 | 29.77 |
| Mean distance, ‰ (H) | 4.418 | 13.988 | 1.640 | 1.796 | 0.968 | 0.768 |
| Thin parts: mean distance, % of a quad edge (V) | 115.0 | 180.3 | 20.3 | 5.9 | 4.5 | 7.6 |
| Volume kept, % of the source (V) | 94.57 | 87.06 | 99.70 | 99.75 | 98.71 | 100.02 |
| Edges on closed loops, % (L) | 0.3 | 7.8 | — | 16.2 | 12.2 | — |
| Edges on spiralling loops, % (L) | 10.9 | 25.0 | — | 13.3 | 9.3 | — |
| Spiralling loops (L) | 6 | 4 | — | 12 | 9 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.1 | 67.2 | — | 70.5 | 78.4 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 102 | 17 | — | 72 | 124 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 10.9 | 68.9 | 2.8 | 1.9 | 189.0 | 0.0 |
| Time, seconds (T) | 3.32 | 0.64 | 2.81 | 3.49 | 18.93 | 4.36 |

## spot (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 1024 | 404 | 990 | 986 | 1497 | 1514 |
| Irregular vertices (S) | 62 | 39 | 64 | 68 | 78 | 68 |
| … of which above the minimum | 54 | — | 56 | 60 | 70 | 60 |
| Base complex patches (C) | 965 | — | 770 | 754 | 1079 | 968 |
| Alignment, degrees (A) | 18.23 | 16.98 | 9.51 | 7.53 | 6.34 | 7.80 |
| Mean angle deviation, degrees (F) | 7.55 | 9.29 | 7.26 | 6.93 | 8.24 | 6.13 |
| Angle deviation, 95th percentile (F) | 22.42 | 25.28 | 19.69 | 20.12 | 22.62 | 17.28 |
| Mean aspect ratio (F) | 1.252 | 1.483 | 1.389 | 1.378 | 1.689 | 1.364 |
| Area variation (F) | 0.209 | 0.498 | 0.397 | 0.460 | 0.555 | 0.427 |
| Hausdorff, ‰ of the diagonal (H) | 20.44 | 66.11 | 12.54 | 14.44 | 8.32 | 14.02 |
| Mean distance, ‰ (H) | 1.330 | 5.964 | 0.659 | 0.606 | 0.810 | 0.411 |
| Thin parts: mean distance, % of a quad edge (V) | 29.6 | 41.7 | 13.4 | 16.2 | 12.2 | 17.7 |
| Volume kept, % of the source (V) | 98.16 | 91.73 | 100.04 | 100.01 | 98.52 | 100.00 |
| Edges on closed loops, % (L) | 1.8 | 2.1 | — | 11.8 | 14.0 | — |
| Edges on spiralling loops, % (L) | 13.9 | 4.8 | — | 12.8 | 5.8 | — |
| Spiralling loops (L) | 5 | 2 | — | 6 | 3 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.6 | 93.0 | — | 75.4 | 80.2 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 79 | 48 | — | 98 | 77 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 2.8 | 59.6 | 1.0 | 1.4 | 49.7 | 1.1 |
| Time, seconds (T) | 1.53 | 0.40 | 1.81 | 2.07 | 17.76 | 2.76 |

## bob (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 972 | 576 | 993 | 991 | 954 | 951 |
| Irregular vertices (S) | 28 | 12 | 18 | 20 | 21 | 20 |
| … of which above the minimum | 28 | 12 | 18 | 20 | 21 | 20 |
| Base complex patches (C) | 958 | 125 | 237 | 326 | 398 | 185 |
| Alignment, degrees (A) | 20.00 | 7.87 | 4.29 | 4.18 | 3.16 | 3.79 |
| Mean angle deviation, degrees (F) | 7.93 | 9.54 | 4.49 | 5.06 | 5.39 | 4.80 |
| Angle deviation, 95th percentile (F) | 22.21 | 23.72 | 11.75 | 13.49 | 15.59 | 13.43 |
| Mean aspect ratio (F) | 1.201 | 2.466 | 1.380 | 1.415 | 1.530 | 1.419 |
| Area variation (F) | 0.159 | 0.448 | 0.435 | 0.433 | 0.413 | 0.449 |
| Hausdorff, ‰ of the diagonal (H) | 12.64 | 11.15 | 4.85 | 5.18 | 7.54 | 4.39 |
| Mean distance, ‰ (H) | 0.966 | 2.793 | 0.384 | 0.390 | 1.012 | 0.398 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.35 | 94.76 | 100.05 | 100.03 | 97.96 | 100.05 |
| Edges on closed loops, % (L) | 0.0 | 49.1 | — | 42.7 | 36.2 | — |
| Edges on spiralling loops, % (L) | 27.7 | 17.7 | — | 19.0 | 32.4 | — |
| Spiralling loops (L) | 5 | 2 | — | 1 | 5 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 71.2 | 33.2 | — | 38.3 | 31.4 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 145 | 76 | — | 94 | 71 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 2.8 | 42.4 | 0.7 | 0.9 | 4.6 | 0.3 |
| Time, seconds (T) | 1.49 | 0.45 | 1.53 | 1.76 | 12.87 | 3.44 |

## blub (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 847 | 604 | 1027 | 984 | 1638 | 1663 |
| Irregular vertices (S) | 44 | 58 | 40 | 50 | 60 | 50 |
| … of which above the minimum | 32 | — | 32 | 42 | 52 | 42 |
| Base complex patches (C) | 785 | — | 463 | 532 | 836 | 1069 |
| Alignment, degrees (A) | 16.32 | 14.60 | 8.33 | 9.03 | 4.88 | 8.06 |
| Mean angle deviation, degrees (F) | 10.04 | 10.41 | 4.92 | 5.70 | 6.85 | 5.03 |
| Angle deviation, 95th percentile (F) | 28.85 | 32.33 | 14.45 | 17.17 | 19.46 | 15.73 |
| Mean aspect ratio (F) | 1.336 | 1.771 | 1.283 | 1.313 | 2.117 | 1.322 |
| Area variation (F) | 0.267 | 0.634 | 0.350 | 0.464 | 0.568 | 0.413 |
| Hausdorff, ‰ of the diagonal (H) | 38.91 | 70.95 | 22.61 | 15.87 | 8.09 | 13.14 |
| Mean distance, ‰ (H) | 1.731 | 4.041 | 0.655 | 0.653 | 0.521 | 0.327 |
| Thin parts: mean distance, % of a quad edge (V) | 30.4 | 31.8 | 12.1 | 8.2 | 4.0 | 7.1 |
| Volume kept, % of the source (V) | 96.59 | — | 99.68 | 99.74 | 98.99 | 99.90 |
| Edges on closed loops, % (L) | 4.2 | 6.4 | — | 23.0 | 24.5 | — |
| Edges on spiralling loops, % (L) | 19.3 | 7.7 | — | 20.5 | 18.7 | — |
| Spiralling loops (L) | 7 | 3 | — | 6 | 6 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 75.3 | 85.9 | — | 56.5 | 56.8 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 87 | 42 | — | 58 | 84 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 15.3 | 39.6 | 2.7 | 1.6 | 63.8 | 1.5 |
| Time, seconds (T) | 1.97 | 1.39 | 2.60 | 2.74 | 14.40 | 2.30 |

## pig (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 861 | 548 | 989 | 1008 | 1975 | 1953 |
| Irregular vertices (S) | 92 | 69 | 60 | 90 | 95 | 94 |
| … of which above the minimum | 76 | — | 52 | 82 | 87 | 86 |
| Base complex patches (C) | 848 | — | 587 | 588 | 1149 | 1181 |
| Alignment, degrees (A) | 17.33 | 14.59 | 12.40 | 9.21 | 7.39 | 10.16 |
| Mean angle deviation, degrees (F) | 15.46 | 12.13 | 7.11 | 7.11 | 8.27 | 6.09 |
| Angle deviation, 95th percentile (F) | 46.32 | 38.98 | 23.49 | 21.71 | 23.65 | 18.37 |
| Mean aspect ratio (F) | 1.533 | 1.947 | 1.332 | 1.445 | 2.045 | 1.297 |
| Area variation (F) | 0.437 | 0.836 | 0.325 | 0.885 | 0.753 | 0.514 |
| Hausdorff, ‰ of the diagonal (H) | 130.05 | 63.50 | 27.73 | 14.22 | 11.45 | 21.90 |
| Mean distance, ‰ (H) | 4.074 | 8.335 | 1.047 | 1.035 | 0.786 | 0.474 |
| Thin parts: mean distance, % of a quad edge (V) | 84.1 | 44.9 | 13.1 | 4.3 | 3.5 | 7.3 |
| Volume kept, % of the source (V) | 96.89 | 90.89 | 99.45 | 99.77 | 98.97 | 99.93 |
| Edges on closed loops, % (L) | 0.9 | 13.1 | — | 21.6 | 20.9 | — |
| Edges on spiralling loops, % (L) | 14.4 | 4.5 | — | 7.0 | 7.0 | — |
| Spiralling loops (L) | 9 | 4 | — | 5 | 4 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 83.9 | 82.4 | — | 71.3 | 72.1 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 94 | 24 | — | 72 | 130 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 13.9 | 45.2 | 1.1 | 0.8 | 97.5 | 1.1 |
| Time, seconds (T) | 2.16 | 0.55 | 2.07 | 2.02 | 17.10 | 2.98 |

## jerry (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | 900 | 305 | 1004 | 987 | 3093 | 3034 |
| Irregular vertices (S) | 69 | 33 | 51 | 83 | 182 | 172 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 724 | — | 609 | 863 | — | 2606 |
| Alignment, degrees (A) | 18.49 | 20.52 | 14.39 | 15.03 | 12.26 | 14.05 |
| Mean angle deviation, degrees (F) | 10.23 | 18.53 | 7.86 | 9.31 | 10.70 | 7.10 |
| Angle deviation, 95th percentile (F) | 28.90 | 57.15 | 21.83 | 27.74 | 30.61 | 21.75 |
| Mean aspect ratio (F) | 1.359 | 1.887 | 1.436 | 1.467 | 2.192 | 1.348 |
| Area variation (F) | 0.306 | 1.686 | 0.324 | 0.752 | 0.752 | 0.539 |
| Hausdorff, ‰ of the diagonal (H) | 58.10 | 201.66 | 36.46 | 26.83 | 7.66 | 26.17 |
| Mean distance, ‰ (H) | 4.435 | 12.497 | 3.005 | 2.933 | 0.950 | 0.836 |
| Thin parts: mean distance, % of a quad edge (V) | 28.0 | 71.5 | 21.9 | 14.0 | 4.8 | 16.8 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | 1.9 | 1.1 | — |
| Edges on spiralling loops, % (L) | 0.0 | 19.9 | — | 3.3 | 6.9 | — |
| Spiralling loops (L) | 0 | 3 | — | 3 | 8 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.7 | 80.1 | — | 90.3 | 91.4 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 76 | 24 | — | 57 | 129 | — |
| Edges on loops from a border to a border, % (L) | 14.3 | 0.0 | — | 4.6 | 0.7 | — |
| Off budget, % (N) | 10.0 | 69.5 | 0.4 | 1.3 | 209.3 | 1.9 |
| Time, seconds (T) | 0.81 | 0.36 | 2.44 | 2.55 | 6.67 | 2.93 |

## rose (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 494 | 157 | 1571 | 1403 | 26339 | 26471 |
| Irregular vertices (S) | 164 | 122 | 690 | 689 | 1682 | 2259 |
| … of which above the minimum | — | — | — | — | — | 1643 |
| Base complex patches (C) | 453 | — | — | — | — | 19881 |
| Alignment, degrees (A) | 18.87 | 18.25 | 13.56 | — | 9.47 | 9.96 |
| Mean angle deviation, degrees (F) | 18.64 | 49.67 | 29.26 | 29.69 | 9.03 | 7.76 |
| Angle deviation, 95th percentile (F) | 52.24 | 87.99 | 75.25 | 77.33 | 26.22 | 25.00 |
| Mean aspect ratio (F) | 2.033 | 8.023 | 16.549 | 11.075 | 1.662 | 1.536 |
| Area variation (F) | 0.598 | 1.952 | 2.339 | 2.984 | 1.972 | 0.584 |
| Hausdorff, ‰ of the diagonal (H) | 76.78 | 195.94 | 73.60 | 89.42 | 74.63 | 7.95 |
| Mean distance, ‰ (H) | 13.492 | 46.413 | 17.915 | 20.570 | 1.049 | 0.214 |
| Thin parts: mean distance, % of a quad edge (V) | 64.1 | 184.7 | 78.8 | 87.6 | 5.8 | 9.1 |
| Volume kept, % of the source (V) | — | — | 13.95 | — | — | 99.50 |
| Edges on closed loops, % (L) | 3.9 | — | — | 5.0 | — | — |
| Edges on spiralling loops, % (L) | 19.8 | — | — | 34.0 | — | — |
| Spiralling loops (L) | 46 | — | — | 392 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 75.7 | — | — | 61.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 16 | — | — | 25 | — | — |
| Edges on loops from a border to a border, % (L) | 1.5 | — | — | 0.0 | — | — |
| Off budget, % (N) | 50.6 | 84.3 | 57.1 | 40.3 | 2533.9 | 0.5 |
| Time, seconds (T) | 2.21 | 0.41 | 3.79 | 5.25 | 14.03 | 16.06 |

## bridge (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | — | 748 | 1008 | 1008 | 2105 | 2120 |
| Irregular vertices (S) | — | 54 | 36 | 36 | 76 | 36 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 254 | 451 | 1346 | 603 |
| Alignment, degrees (A) | — | 11.10 | 6.96 | 6.66 | 6.48 | 7.40 |
| Mean angle deviation, degrees (F) | — | 16.94 | 4.11 | 5.57 | 8.08 | 4.40 |
| Angle deviation, 95th percentile (F) | — | 65.68 | 12.89 | 17.02 | 22.80 | 12.30 |
| Mean aspect ratio (F) | — | 2.304 | 1.284 | 1.330 | 2.085 | 1.265 |
| Area variation (F) | — | 1.107 | 0.270 | 0.535 | 0.486 | 0.430 |
| Hausdorff, ‰ of the diagonal (H) | — | 98.43 | 17.46 | 9.08 | 5.71 | 7.75 |
| Mean distance, ‰ (H) | — | 7.488 | 0.891 | 0.814 | 0.640 | 0.555 |
| Thin parts: mean distance, % of a quad edge (V) | — | 15.2 | 4.6 | 2.8 | 3.4 | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 19.1 | — | 19.5 | 10.9 | — |
| Edges on spiralling loops, % (L) | — | 9.4 | — | 15.3 | 16.9 | — |
| Spiralling loops (L) | — | 4 | — | 3 | 7 | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 71.5 | — | 51.2 | 62.9 | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 41 | — | 47 | 108 | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | 14.0 | 9.3 | — |
| Off budget, % (N) | — | 25.2 | 0.8 | 0.8 | 110.5 | 0.7 |
| Time, seconds (T) | — | 0.55 | 2.69 | 3.52 | 13.07 | 3.30 |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 943 | 348 | 976 | 1018 | 1013 | 1018 |
| Irregular vertices (S) | 30 | 69 | 24 | 24 | 26 | 24 |
| … of which above the minimum | 22 | — | 16 | 16 | 18 | 16 |
| Base complex patches (C) | 227 | — | 86 | 68 | 367 | 68 |
| Alignment, degrees (A) | 13.41 | 19.45 | 12.46 | 10.80 | 11.10 | 10.82 |
| Mean angle deviation, degrees (F) | 3.78 | 19.61 | 2.07 | 2.16 | 4.39 | 2.17 |
| Angle deviation, 95th percentile (F) | 15.04 | 60.84 | 9.04 | 10.18 | 14.60 | 10.14 |
| Mean aspect ratio (F) | 1.191 | 2.662 | 1.219 | 1.177 | 1.323 | 1.177 |
| Area variation (F) | 0.176 | 0.647 | 0.175 | 0.381 | 0.296 | 0.381 |
| Hausdorff, ‰ of the diagonal (H) | 8.20 | 93.30 | 14.36 | 1.37 | 1.64 | 1.37 |
| Mean distance, ‰ (H) | 0.335 | 8.243 | 0.259 | 0.165 | 0.147 | 0.165 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.43 | — | 100.01 | 100.01 | 100.26 | 100.01 |
| Edges on closed loops, % (L) | 47.9 | — | — | 74.5 | 35.9 | — |
| Edges on spiralling loops, % (L) | 13.6 | — | — | 0.0 | 28.0 | — |
| Spiralling loops (L) | 4 | — | — | 0 | 7 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 36.6 | — | — | 25.5 | 36.0 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 36 | — | — | 26 | 50 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 5.7 | 65.2 | 2.4 | 1.8 | 1.3 | 0.5 |
| Time, seconds (T) | 0.97 | 0.32 | 1.03 | 1.01 | 3.06 | 1.07 |

## cube (mechanical)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 1014 | 573 | 1014 | 1014 | 1014 | 1014 |
| Irregular vertices (S) | 8 | 13 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | — | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 6 | — | 6 | 6 | 6 | 6 |
| Alignment, degrees (A) | 0.17 | 3.25 | 0.11 | 0.25 | 0.11 | 0.25 |
| Mean angle deviation, degrees (F) | 0.08 | 4.61 | 0.00 | 0.33 | 0.00 | 0.33 |
| Angle deviation, 95th percentile (F) | 0.23 | 14.18 | 0.01 | 0.76 | 0.00 | 0.76 |
| Mean aspect ratio (F) | 1.003 | 1.707 | 1.000 | 1.071 | 1.000 | 1.071 |
| Area variation (F) | 0.002 | 0.500 | 0.000 | 0.082 | 0.000 | 0.081 |
| Hausdorff, ‰ of the diagonal (H) | 2.31 | 23.35 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.015 | 1.194 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.43 | 100.00 | 100.00 | 100.00 | 100.00 |
| Edges on closed loops, % (L) | 92.3 | 44.9 | — | 92.3 | 92.3 | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Spiralling loops (L) | 0 | 0 | — | 0 | 0 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 7.7 | 55.1 | — | 7.7 | 7.7 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 13 | 40 | — | 13 | 13 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 1.4 | 42.7 | 1.4 | 1.4 | 1.4 | 0.0 |
| Time, seconds (T) | 0.30 | 0.24 | 0.64 | 0.70 | 2.35 | 0.72 |

## lblock (mechanical)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 930 | 588 | 994 | 994 | 1010 | 1028 |
| Irregular vertices (S) | 12 | 14 | 12 | 12 | 12 | 12 |
| … of which above the minimum | 4 | 6 | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 85 | 132 | 14 | 14 | 14 | 14 |
| Alignment, degrees (A) | 1.36 | 4.35 | 0.56 | 0.64 | 0.56 | 0.64 |
| Mean angle deviation, degrees (F) | 1.26 | 4.89 | 0.10 | 0.37 | 0.30 | 0.39 |
| Angle deviation, 95th percentile (F) | 4.73 | 14.86 | 0.33 | 0.90 | 1.26 | 1.01 |
| Mean aspect ratio (F) | 1.064 | 1.571 | 1.072 | 1.099 | 1.090 | 1.098 |
| Area variation (F) | 0.059 | 0.415 | 0.070 | 0.096 | 0.053 | 0.096 |
| Hausdorff, ‰ of the diagonal (H) | 9.75 | 29.11 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.123 | 1.880 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.99 | 99.10 | 100.00 | 100.00 | 100.00 | 100.00 |
| Edges on closed loops, % (L) | 68.3 | 52.4 | — | 88.1 | 88.2 | — |
| Edges on spiralling loops, % (L) | 13.6 | 11.2 | — | 0.0 | 0.0 | — |
| Spiralling loops (L) | 1 | 2 | — | 0 | 0 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 16.6 | 36.4 | — | 11.9 | 11.8 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 34 | 50 | — | 25 | 27 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 7.0 | 41.2 | 0.6 | 0.6 | 1.0 | 1.8 |
| Time, seconds (T) | 0.21 | 0.17 | 0.68 | 0.82 | 2.79 | 0.79 |

## cylinder (mechanical)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 995 | 612 | 990 | 1002 | 782 | 764 |
| Irregular vertices (S) | 8 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 376 | 164 | 6 | 186 | 138 | 56 |
| Alignment, degrees (A) | 3.95 | 5.40 | 1.23 | 1.85 | 1.55 | 1.37 |
| Mean angle deviation, degrees (F) | 4.50 | 8.90 | 2.64 | 3.73 | 4.01 | 2.54 |
| Angle deviation, 95th percentile (F) | 17.64 | 26.84 | 7.63 | 8.66 | 11.66 | 9.04 |
| Mean aspect ratio (F) | 1.124 | 1.965 | 1.286 | 1.504 | 1.528 | 1.509 |
| Area variation (F) | 0.112 | 0.576 | 0.334 | 0.218 | 0.416 | 0.205 |
| Hausdorff, ‰ of the diagonal (H) | 3.89 | 14.27 | 1.47 | 1.70 | 2.59 | 1.93 |
| Mean distance, ‰ (H) | 0.373 | 2.062 | 0.311 | 0.349 | 0.938 | 0.457 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.73 | 98.02 | 99.99 | 100.02 | 99.00 | 99.99 |
| Edges on closed loops, % (L) | 30.7 | 36.6 | — | 40.7 | 41.2 | — |
| Edges on spiralling loops, % (L) | 45.4 | 50.2 | — | 55.9 | 54.9 | — |
| Spiralling loops (L) | 3 | 4 | — | 5 | 5 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 24.2 | 13.2 | — | 3.4 | 3.9 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 134 | 30 | — | 16 | 21 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 1.8 | 38.8 | 1.0 | 0.2 | 21.8 | 2.3 |
| Time, seconds (T) | 0.30 | 0.16 | 0.81 | 0.82 | 2.92 | 0.79 |

## venturi (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 2390 | 757 | 836 | 12157 | 12453 |
| Irregular vertices (S) | — | 1182 | 150 | 176 | 606 | 874 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | — | 10479 |
| Alignment, degrees (A) | — | 8.64 | 11.32 | 13.71 | 6.94 | 11.18 |
| Mean angle deviation, degrees (F) | — | 53.69 | 11.89 | 14.03 | 10.01 | 7.67 |
| Angle deviation, 95th percentile (F) | — | 89.78 | 37.21 | 45.64 | 45.68 | 28.89 |
| Mean aspect ratio (F) | — | 7.059 | 1.553 | 3.065 | 2.423 | 1.538 |
| Area variation (F) | — | 5.029 | 0.486 | 0.595 | 2.342 | 0.377 |
| Hausdorff, ‰ of the diagonal (H) | — | 174.38 | 82.83 | 82.83 | 14.79 | 21.31 |
| Mean distance, ‰ (H) | — | 17.134 | 7.832 | 8.266 | 0.450 | 1.274 |
| Thin parts: mean distance, % of a quad edge (V) | — | 31.9 | 21.1 | 22.9 | 1.4 | 22.8 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | 12.0 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | 25.8 | — | — |
| Spiralling loops (L) | — | — | — | 91 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | 62.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | 47 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 139.0 | 24.3 | 16.4 | 1115.7 | 2.4 |
| Time, seconds (T) | — | 0.36 | 7.28 | 7.94 | 6.97 | 81.12 |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | — | 334 | 1019 | 1011 | 2921 | 2885 |
| Irregular vertices (S) | — | 38 | 46 | 118 | 229 | 209 |
| … of which above the minimum | — | — | 38 | 110 | 221 | 201 |
| Base complex patches (C) | — | — | 707 | 733 | 2159 | 2351 |
| Alignment, degrees (A) | — | 18.93 | 14.59 | 15.15 | 10.54 | 13.03 |
| Mean angle deviation, degrees (F) | — | 14.98 | 8.13 | 9.39 | 9.73 | 6.94 |
| Angle deviation, 95th percentile (F) | — | 43.87 | 22.79 | 26.33 | 26.74 | 20.32 |
| Mean aspect ratio (F) | — | 1.799 | 1.668 | 1.559 | 2.436 | 1.430 |
| Area variation (F) | — | 0.758 | 0.375 | 0.892 | 0.740 | 0.775 |
| Hausdorff, ‰ of the diagonal (H) | — | 124.75 | 31.77 | 30.52 | 29.33 | 24.12 |
| Mean distance, ‰ (H) | — | 11.455 | 1.647 | 1.906 | 0.976 | 0.786 |
| Thin parts: mean distance, % of a quad edge (V) | — | 89.7 | 22.8 | 5.9 | 3.8 | 3.9 |
| Volume kept, % of the source (V) | — | 86.48 | 99.58 | 99.83 | 98.60 | 100.07 |
| Edges on closed loops, % (L) | — | 5.8 | — | 14.1 | 13.4 | — |
| Edges on spiralling loops, % (L) | — | 18.0 | — | 11.0 | 10.5 | — |
| Spiralling loops (L) | — | 9 | — | 14 | 14 | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 76.2 | — | 74.9 | 76.1 | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 28 | — | 70 | 85 | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | — | 66.6 | 1.9 | 1.1 | 192.1 | 1.2 |
| Time, seconds (T) | — | 17.00 | 57.27 | 52.03 | 36.93 | 64.54 |

Failures: quadriflow: cancelled.

## watermill (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 763 | 225 | 592 | 592 | 9441 | — |
| Irregular vertices (S) | 130 | 79 | 106 | 106 | 881 | — |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 751 | — | — | — | — | — |
| Alignment, degrees (A) | 16.26 | 19.32 | 16.52 | 16.52 | 11.80 | — |
| Mean angle deviation, degrees (F) | 15.96 | 26.39 | 16.68 | 16.70 | 10.11 | — |
| Angle deviation, 95th percentile (F) | 53.61 | 82.13 | 62.19 | 62.19 | 31.55 | — |
| Mean aspect ratio (F) | 1.692 | 4.170 | 2.301 | 2.323 | 1.746 | — |
| Area variation (F) | 0.466 | 1.055 | 0.800 | 0.800 | 0.926 | — |
| Hausdorff, ‰ of the diagonal (H) | 130.87 | 298.17 | 119.51 | 119.51 | 38.62 | — |
| Mean distance, ‰ (H) | 17.928 | 82.620 | 15.502 | 15.509 | 2.325 | — |
| Thin parts: mean distance, % of a quad edge (V) | 47.7 | 197.5 | 39.7 | 39.7 | 9.8 | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.7 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 6.5 | — | — | — | — | — |
| Spiralling loops (L) | 14 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 86.8 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 35 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 5.2 | — | — | — | — | — |
| Off budget, % (N) | 23.7 | 77.5 | 40.8 | 40.8 | 844.1 | — |
| Time, seconds (T) | 0.84 | 1.57 | 10.30 | 11.23 | 526.55 | — |

Failures: openclare-matched: no Quad Remesher reference to match.

## towermill (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 866 | 405 | 1826 | 685 | 6083 | — |
| Irregular vertices (S) | 165 | 121 | 311 | 94 | 505 | — |
| … of which above the minimum | — | — | 7 | — | — | — |
| Base complex patches (C) | 846 | — | 1826 | — | — | — |
| Alignment, degrees (A) | 13.39 | 18.17 | 16.89 | 14.91 | 9.77 | — |
| Mean angle deviation, degrees (F) | 14.47 | 27.63 | 25.86 | 15.35 | 10.07 | — |
| Angle deviation, 95th percentile (F) | 46.45 | 75.88 | 68.26 | 50.55 | 32.00 | — |
| Mean aspect ratio (F) | 1.621 | 16.899 | 5.045 | 1.801 | 2.374 | — |
| Area variation (F) | 0.403 | 0.903 | 1.939 | 0.428 | 0.688 | — |
| Hausdorff, ‰ of the diagonal (H) | 102.61 | 138.78 | 75.07 | 110.42 | 28.19 | — |
| Mean distance, ‰ (H) | 9.005 | 32.456 | 6.685 | 12.376 | 2.452 | — |
| Thin parts: mean distance, % of a quad edge (V) | 22.9 | 75.3 | 19.2 | 30.9 | 8.2 | — |
| Volume kept, % of the source (V) | — | — | 78.09 | — | 102.25 | — |
| Edges on closed loops, % (L) | 0.0 | — | — | — | 0.6 | — |
| Edges on spiralling loops, % (L) | 8.1 | — | — | — | 15.2 | — |
| Spiralling loops (L) | 7 | — | — | — | 51 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.2 | — | — | — | 84.3 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 23 | — | — | — | 100 | — |
| Edges on loops from a border to a border, % (L) | 1.5 | — | — | — | 0.0 | — |
| Off budget, % (N) | 13.4 | 59.5 | 82.6 | 31.5 | 508.3 | — |
| Time, seconds (T) | 0.64 | 1.20 | 22.04 | 10.24 | 79.69 | — |

Failures: openclare-matched: no Quad Remesher reference to match.

## stonemill (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 938 | 703 | 1013 | 1003 | 5437 | — |
| Irregular vertices (S) | 132 | 73 | 92 | 155 | 428 | — |
| … of which above the minimum | — | — | 28 | 91 | 376 | — |
| Base complex patches (C) | 926 | — | 963 | 978 | 4943 | — |
| Alignment, degrees (A) | 15.11 | 17.16 | 13.18 | 14.72 | 11.05 | — |
| Mean angle deviation, degrees (F) | 12.56 | 11.97 | 9.06 | 13.44 | 9.53 | — |
| Angle deviation, 95th percentile (F) | 35.62 | 36.94 | 32.14 | 40.37 | 29.45 | — |
| Mean aspect ratio (F) | 1.391 | 1.994 | 1.693 | 2.058 | 1.908 | — |
| Area variation (F) | 0.275 | 0.656 | 0.440 | 0.923 | 0.903 | — |
| Hausdorff, ‰ of the diagonal (H) | 98.74 | 186.96 | 36.21 | 30.67 | 20.43 | — |
| Mean distance, ‰ (H) | 4.506 | 9.114 | 3.555 | 3.498 | 2.028 | — |
| Thin parts: mean distance, % of a quad edge (V) | 38.2 | 76.4 | 22.2 | 13.8 | 12.4 | — |
| Volume kept, % of the source (V) | — | — | 99.38 | 99.40 | 98.75 | — |
| Edges on closed loops, % (L) | 0.3 | — | — | 1.3 | 4.7 | — |
| Edges on spiralling loops, % (L) | 4.3 | — | — | 10.4 | 6.6 | — |
| Spiralling loops (L) | 14 | — | — | 16 | 14 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 95.4 | — | — | 88.2 | 88.7 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 50 | — | — | 33 | 137 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 6.2 | 29.7 | 1.3 | 0.3 | 443.7 | — |
| Time, seconds (T) | 0.87 | 0.85 | 1.48 | 4.08 | 31.11 | — |

Failures: openclare-matched: no Quad Remesher reference to match.

## windmill (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 370 | 141 | 164 | 6933 | — |
| Irregular vertices (S) | — | 75 | 28 | 36 | 599 | — |
| … of which above the minimum | — | — | 4 | 12 | — | — |
| Base complex patches (C) | — | — | 141 | 164 | — | — |
| Alignment, degrees (A) | — | 14.45 | 17.99 | 21.40 | 9.68 | — |
| Mean angle deviation, degrees (F) | — | 28.12 | 30.30 | 31.43 | 9.49 | — |
| Angle deviation, 95th percentile (F) | — | 83.45 | 72.51 | 73.77 | 31.62 | — |
| Mean aspect ratio (F) | — | 3.970 | 4.563 | 4.437 | 1.954 | — |
| Area variation (F) | — | 0.923 | 1.077 | 1.342 | 0.818 | — |
| Hausdorff, ‰ of the diagonal (H) | — | 203.64 | 538.64 | 539.23 | 23.21 | — |
| Mean distance, ‰ (H) | — | 36.911 | 255.822 | 252.352 | 1.584 | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 110.1 | 712.3 | 706.9 | 7.5 | — |
| Volume kept, % of the source (V) | — | — | 0.89 | 0.69 | — | — |
| Edges on closed loops, % (L) | — | — | — | 0.0 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | 17.7 | — | — |
| Spiralling loops (L) | — | — | — | 6 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | 82.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | 16 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 63.0 | 85.9 | 83.6 | 593.3 | — |
| Time, seconds (T) | — | 1.40 | 0.62 | 1.47 | 101.96 | — |

Failures: quadriflow: cancelled; openclare-matched: no Quad Remesher reference to match.

## terrain (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 872 | 308 | 980 | 1000 | 1541 | 1512 |
| Irregular vertices (S) | 44 | 30 | 34 | 38 | 152 | 42 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 307 | — | 500 | 646 | 1446 | 1026 |
| Alignment, degrees (A) | 19.22 | 22.42 | 18.03 | 17.66 | 18.90 | 18.51 |
| Mean angle deviation, degrees (F) | 6.74 | 26.16 | 5.77 | 6.04 | 11.26 | 5.65 |
| Angle deviation, 95th percentile (F) | 23.83 | 73.60 | 17.77 | 18.76 | 32.77 | 16.71 |
| Mean aspect ratio (F) | 1.223 | 1.911 | 1.319 | 1.321 | 1.859 | 1.293 |
| Area variation (F) | 0.195 | 0.775 | 0.353 | 0.460 | 0.545 | 0.454 |
| Hausdorff, ‰ of the diagonal (H) | 30.58 | 234.03 | 23.57 | 26.88 | 18.41 | 31.13 |
| Mean distance, ‰ (H) | 3.158 | 9.103 | 2.302 | 1.984 | 1.506 | 1.615 |
| Thin parts: mean distance, % of a quad edge (V) | 29.7 | 54.8 | 23.5 | 19.4 | 11.3 | 19.9 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Edges on spiralling loops, % (L) | 0.0 | 7.8 | — | 9.9 | 0.0 | — |
| Spiralling loops (L) | 0 | 1 | — | 3 | 0 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 57.4 | 92.2 | — | 69.7 | 96.8 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 25 | 27 | — | 38 | 54 | — |
| Edges on loops from a border to a border, % (L) | 42.6 | 0.0 | — | 20.4 | 3.2 | — |
| Off budget, % (N) | 12.8 | 69.2 | 2.0 | 0.0 | 54.1 | 1.9 |
| Time, seconds (T) | 19.91 | 1.73 | 8.45 | 7.77 | 19.65 | 8.74 |

## mahaveer (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 636 | 2037 | 1869 | 41821 | 40889 |
| Irregular vertices (S) | — | 343 | 485 | 476 | 3109 | 1380 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 8.88 | 12.07 | 17.86 | 15.01 | 5.83 |
| Mean angle deviation, degrees (F) | — | 38.85 | 38.51 | 32.88 | 13.01 | 5.50 |
| Angle deviation, 95th percentile (F) | — | 87.58 | 84.45 | 85.94 | 40.35 | 21.91 |
| Mean aspect ratio (F) | — | 7.492 | 155.937 | 205.386 | 1.768 | 10.381 |
| Area variation (F) | — | 1.913 | 8.209 | 7.845 | 3.743 | 0.960 |
| Hausdorff, ‰ of the diagonal (H) | — | 88.93 | 325.51 | 325.54 | 13.85 | 13.40 |
| Mean distance, ‰ (H) | — | 14.421 | 40.240 | 40.314 | 1.081 | 0.638 |
| Thin parts: mean distance, % of a quad edge (V) | — | 65.0 | 154.1 | 154.1 | 7.4 | 34.0 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | 0.9 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | 86.9 | — | — |
| Spiralling loops (L) | — | — | — | 634 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | 12.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | 6 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 36.4 | 103.7 | 86.9 | 4082.1 | 2.2 |
| Time, seconds (T) | — | 1.37 | 8.98 | 7.12 | 16.61 | 26.05 |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 1110 | 8170 | 7967 | 205185 | 193727 |
| Irregular vertices (S) | — | 905 | 6210 | 6140 | 10625 | 11613 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | — | — |
| Alignment, degrees (A) | — | — | — | — | 10.09 | 8.58 |
| Mean angle deviation, degrees (F) | — | 40.06 | 23.72 | 22.75 | 10.19 | 5.23 |
| Angle deviation, 95th percentile (F) | — | 88.92 | 71.02 | 69.67 | 30.61 | 20.16 |
| Mean aspect ratio (F) | — | 4.992 | 6.223 | 4.479 | 1.591 | 1.380 |
| Area variation (F) | — | 5.647 | 2.827 | 2.855 | 3.347 | 0.574 |
| Hausdorff, ‰ of the diagonal (H) | — | 11.21 | 7.06 | 6.26 | 2.06 | 3.02 |
| Mean distance, ‰ (H) | — | 3.155 | 0.435 | 0.452 | 0.152 | 0.038 |
| Thin parts: mean distance, % of a quad edge (V) | — | 56.2 | 7.9 | 8.2 | 5.2 | 18.3 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — |
| Off budget, % (N) | — | 11.0 | 717.0 | 696.7 | 20418.5 | 5.6 |
| Time, seconds (T) | — | 1.28 | 14.02 | 17.66 | 14.01 | 56.46 |

Failures: quadriflow: cancelled.

## f16 (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 3146 | 2562 | 2662 | 30430 | 28141 |
| Irregular vertices (S) | — | 1983 | 1767 | 1662 | 2729 | 1942 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 17.00 | 4.70 | 4.94 | 5.04 | 4.12 |
| Mean angle deviation, degrees (F) | — | 45.13 | 21.40 | 21.77 | 11.44 | 6.43 |
| Angle deviation, 95th percentile (F) | — | 87.51 | 78.34 | 77.58 | 42.16 | 27.97 |
| Mean aspect ratio (F) | — | 8.830 | 179.581 | 198.798 | 2.473 | 17.802 |
| Area variation (F) | — | 5.401 | 6.287 | 5.709 | 2.700 | 1.063 |
| Hausdorff, ‰ of the diagonal (H) | — | 128.15 | 112.60 | 107.04 | 10.84 | 13.12 |
| Mean distance, ‰ (H) | — | 13.795 | 5.348 | 4.618 | 0.478 | 0.351 |
| Thin parts: mean distance, % of a quad edge (V) | — | 52.9 | 16.5 | 14.1 | 2.5 | 7.9 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | 6.7 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | 31.6 | — | — |
| Spiralling loops (L) | — | — | — | 1288 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | 53.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | 25 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | 8.0 | — | — |
| Off budget, % (N) | — | 214.6 | 156.2 | 166.2 | 2943.0 | 7.5 |
| Time, seconds (T) | — | 1.26 | 14.15 | 10.68 | 11.27 | 92.21 |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 988 | 1462 | 1670 | 9385 | 8871 |
| Irregular vertices (S) | — | 272 | 136 | 182 | 368 | 314 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | 484 | — | — |
| Alignment, degrees (A) | — | 9.41 | 4.95 | 6.02 | 4.16 | 3.20 |
| Mean angle deviation, degrees (F) | — | 25.43 | 19.83 | 20.85 | 7.93 | 8.41 |
| Angle deviation, 95th percentile (F) | — | 86.02 | 70.71 | 71.98 | 28.21 | 42.32 |
| Mean aspect ratio (F) | — | 3.381 | 39.958 | 58.806 | 10.123 | 9.470 |
| Area variation (F) | — | 1.855 | 2.039 | 1.978 | 2.280 | 0.787 |
| Hausdorff, ‰ of the diagonal (H) | — | 72.47 | 52.85 | 39.36 | 21.81 | 19.79 |
| Mean distance, ‰ (H) | — | 8.475 | 6.157 | 5.818 | 1.030 | 0.941 |
| Thin parts: mean distance, % of a quad edge (V) | — | 17.9 | 13.6 | 13.0 | 2.4 | 11.8 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | 54.1 | 33.6 | — |
| Edges on spiralling loops, % (L) | — | — | — | 17.2 | 5.0 | — |
| Spiralling loops (L) | — | — | — | 86 | 19 | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | 25.6 | 38.1 | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | 19 | 76 | — |
| Edges on loops from a border to a border, % (L) | — | — | — | 3.0 | 23.4 | — |
| Off budget, % (N) | — | 1.2 | 46.2 | 67.0 | 838.5 | 5.5 |
| Time, seconds (T) | — | 0.39 | 2.72 | 2.62 | 5.98 | 13.06 |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 |
| Quads made | — | 180 | 759 | 752 | 7098 | 5955 |
| Irregular vertices (S) | — | 56 | 126 | 126 | 725 | 418 |
| … of which above the minimum | — | — | 62 | 62 | — | — |
| Base complex patches (C) | — | — | 465 | 458 | — | — |
| Alignment, degrees (A) | — | 14.42 | 9.99 | 10.11 | 9.10 | 10.58 |
| Mean angle deviation, degrees (F) | — | 24.14 | 13.62 | 13.11 | 9.97 | 9.51 |
| Angle deviation, 95th percentile (F) | — | 87.70 | 39.94 | 36.87 | 29.98 | 28.14 |
| Mean aspect ratio (F) | — | 3.965 | 1.995 | 1.981 | 1.773 | 1.475 |
| Area variation (F) | — | 0.945 | 0.494 | 0.485 | 1.130 | 0.342 |
| Hausdorff, ‰ of the diagonal (H) | — | 123.45 | 40.53 | 40.53 | 28.38 | 20.49 |
| Mean distance, ‰ (H) | — | 18.850 | 5.461 | 5.421 | 1.253 | 0.843 |
| Thin parts: mean distance, % of a quad edge (V) | — | 123.8 | 50.6 | 50.0 | 17.0 | 39.0 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 7.5 | — | 19.5 | 16.7 | — |
| Edges on spiralling loops, % (L) | — | 16.4 | — | 10.4 | 8.8 | — |
| Spiralling loops (L) | — | 14 | — | 52 | 55 | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 76.1 | — | 70.1 | 74.4 | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 11 | — | 35 | 59 | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | 0.0 | 0.1 | — |
| Off budget, % (N) | — | 82.0 | 24.1 | 24.8 | 609.8 | 16.1 |
| Time, seconds (T) | — | 0.58 | 6.41 | 7.21 | 27.33 | 73.47 |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 950 | 768 | 1010 | 1010 | 915 | 900 |
| Irregular vertices (S) | 8 | 8 | 8 | 8 | 10 | 10 |
| … of which above the minimum | 0 | 0 | 0 | 0 | 2 | 2 |
| Base complex patches (C) | 937 | 768 | 1010 | 1010 | 699 | 846 |
| Alignment, degrees (A) | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.56 | 5.13 | 4.61 | 4.61 | 6.03 | 4.08 |
| Angle deviation, 95th percentile (F) | 12.31 | 13.45 | 11.13 | 11.13 | 14.57 | 10.24 |
| Mean aspect ratio (F) | 1.148 | 1.336 | 1.180 | 1.180 | 1.213 | 1.153 |
| Area variation (F) | 0.136 | 0.229 | 0.225 | 0.225 | 0.318 | 0.240 |
| Hausdorff, ‰ of the diagonal (H) | 1.47 | 3.06 | 0.99 | 0.99 | 1.91 | 2.07 |
| Mean distance, ‰ (H) | 0.670 | 1.508 | 0.332 | 0.332 | 0.762 | 0.357 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.38 | 98.48 | 100.02 | 100.02 | 99.24 | 100.02 |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | 0.0 | 12.0 | — |
| Edges on spiralling loops, % (L) | 69.5 | 76.7 | — | 86.8 | 64.4 | — |
| Spiralling loops (L) | 4 | 4 | — | 5 | 5 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 25.1 | 23.3 | — | 13.2 | 23.6 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 145 | 126 | — | 88 | 110 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 5.0 | 23.2 | 1.0 | 1.0 | 8.5 | 1.6 |
| Time, seconds (T) | 0.45 | 0.18 | 0.95 | 1.01 | 4.77 | 1.17 |

## torus (calibration)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 1028 | 728 | 1008 | 1008 | 920 | 920 |
| Irregular vertices (S) | 20 | 0 | 0 | 0 | 0 | 0 |
| … of which above the minimum | 20 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 1016 | 1 | 1 | 1 | 1 | 1 |
| Alignment, degrees (A) | 13.64 | 1.94 | 0.16 | 0.16 | 0.63 | 0.64 |
| Mean angle deviation, degrees (F) | 7.73 | 4.57 | 2.40 | 2.40 | 2.49 | 2.52 |
| Angle deviation, 95th percentile (F) | 18.19 | 9.20 | 3.73 | 3.73 | 3.93 | 3.91 |
| Mean aspect ratio (F) | 1.178 | 3.376 | 1.295 | 1.295 | 1.372 | 1.293 |
| Area variation (F) | 0.180 | 0.121 | 0.350 | 0.351 | 0.280 | 0.356 |
| Hausdorff, ‰ of the diagonal (H) | 3.12 | 5.42 | 1.35 | 1.35 | 2.60 | 1.67 |
| Mean distance, ‰ (H) | 0.775 | 1.726 | 0.424 | 0.424 | 1.127 | 0.449 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.57 | 96.27 | 100.04 | 100.04 | 97.62 | 100.06 |
| Edges on closed loops, % (L) | 0.0 | 100.0 | — | 100.0 | 100.0 | — |
| Edges on spiralling loops, % (L) | 52.8 | 0.0 | — | 0.0 | 0.0 | — |
| Spiralling loops (L) | 11 | 0 | — | 0 | 0 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 44.7 | 0.0 | — | 0.0 | 0.0 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 109 | 0 | — | 0 | 0 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 2.8 | 27.2 | 0.8 | 0.8 | 8.0 | 0.0 |
| Time, seconds (T) | 0.33 | 0.44 | 1.17 | 0.82 | 4.41 | 0.89 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| Irregular vertices (S) | 1 | 7 | 6 | 3 | 0 | 3 |
| Base complex patches (C) | 1 | 5 | 5 | 2 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 12 | 1 |
| Mean angle deviation, degrees (F) | 0 | 0 | 4 | 2 | 0 | 7 |
| Mean aspect ratio (F) | 4 | 0 | 3 | 0 | 0 | 6 |
| Valid meshes (V) | 38/65 | 4/13 | 12/13 | 12/13 | 11/13 | 12/13 |

### mechanical (4 models)

| Metric | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 1 | 4 | 4 | 3 | 4 |
| Base complex patches (C) | 1 | 0 | 3 | 3 | 2 | 3 |
| Alignment, degrees (A) | 0 | 0 | 1 | 1 | 2 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 2 | 0 | 1 | 1 |
| Mean aspect ratio (F) | 2 | 0 | 0 | 1 | 1 | 0 |
| Valid meshes (V) | 20/20 | 2/4 | 4/4 | 4/4 | 4/4 | 4/4 |
