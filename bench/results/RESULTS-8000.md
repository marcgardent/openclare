# Bench results

Requested budget: 8000 quads. Median over 5 seeds; Quad Remesher, AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

## blob (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7590 | 2795 | 8068 | 7979 | 7911 | 7949 |
| Irregular vertices (S) | 40 | 28 | 24 | 24 | 24 | 24 |
| … of which above the minimum | 32 | 20 | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 7412 | 1171 | 6808 | 6819 | 4563 | 6789 |
| Alignment, degrees (A) | 14.77 | 5.92 | 5.81 | 5.46 | 4.08 | 5.28 |
| Mean angle deviation, degrees (F) | 7.77 | 9.19 | 6.29 | 5.78 | 5.52 | 5.52 |
| Angle deviation, 95th percentile (F) | 19.95 | 34.45 | 17.15 | 15.29 | 14.94 | 14.63 |
| Mean aspect ratio (F) | 1.152 | 2.101 | 1.309 | 1.290 | 1.414 | 1.279 |
| Area variation (F) | 0.177 | 0.585 | 0.340 | 0.360 | 0.395 | 0.358 |
| Hausdorff, ‰ of the diagonal (H) | 5.18 | 2.78 | 1.88 | 1.76 | 3.00 | 1.73 |
| Mean distance, ‰ (H) | 0.171 | 0.760 | 0.082 | 0.078 | 0.185 | 0.077 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.84 | 98.72 | 100.01 | 100.01 | 99.77 | 100.01 |
| Edges on closed loops, % (L) | 1.2 | 33.0 | — | 7.3 | 21.2 | — |
| Edges on spiralling loops, % (L) | 35.9 | 18.0 | — | 50.0 | 46.2 | — |
| Spiralling loops (L) | 12 | 4 | — | 12 | 11 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 62.6 | 49.1 | — | 42.7 | 32.6 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 766 | 292 | — | 1216 | 601 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 5.1 | 65.1 | 0.9 | 0.3 | 1.1 | 0.5 |
| Time, seconds (T) | 4.32 | 2.57 | 6.20 | 6.58 | 12.37 | 7.04 |

## rock (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7342 | 5842 | 7943 | 7940 | 7071 | 7061 |
| Irregular vertices (S) | 10 | 34 | 116 | 134 | 68 | 116 |
| … of which above the minimum | 2 | 26 | 108 | 126 | 60 | 108 |
| Base complex patches (C) | 7342 | 5052 | 7943 | 7940 | 7071 | 7061 |
| Alignment, degrees (A) | 22.36 | 18.66 | 16.60 | 16.32 | 17.28 | 16.10 |
| Mean angle deviation, degrees (F) | 6.22 | 6.90 | 5.93 | 5.42 | 9.80 | 5.49 |
| Angle deviation, 95th percentile (F) | 18.32 | 17.38 | 14.01 | 13.19 | 23.73 | 14.12 |
| Mean aspect ratio (F) | 1.158 | 1.393 | 1.216 | 1.215 | 1.350 | 1.221 |
| Area variation (F) | 0.244 | 0.344 | 0.254 | 0.264 | 0.287 | 0.259 |
| Hausdorff, ‰ of the diagonal (H) | 6.95 | 3.88 | 2.87 | 2.57 | 4.18 | 3.35 |
| Mean distance, ‰ (H) | 0.478 | 0.669 | 0.395 | 0.389 | 0.515 | 0.420 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.91 | 99.79 | 100.00 | 100.00 | 99.83 | 100.00 |
| Edges on closed loops, % (L) | 0.0 | 6.9 | — | 0.0 | 0.0 | — |
| Edges on spiralling loops, % (L) | 70.9 | 30.4 | — | 3.9 | 8.2 | — |
| Spiralling loops (L) | 6 | 7 | — | 3 | 3 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 29.1 | 62.6 | — | 96.1 | 91.8 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1193 | 618 | — | 297 | 490 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 8.2 | 27.0 | 0.7 | 0.8 | 11.6 | 0.1 |
| Time, seconds (T) | 4.18 | 3.32 | 5.70 | 7.03 | 18.31 | 6.46 |

## creature (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 8170 | 6819 | 7884 | 7825 | 7186 | 7168 |
| Irregular vertices (S) | 22 | 18 | 40 | 42 | 40 | 40 |
| … of which above the minimum | 14 | 10 | 32 | 34 | 32 | 32 |
| Base complex patches (C) | 8170 | 3683 | 2864 | 3225 | 3236 | 4316 |
| Alignment, degrees (A) | 13.70 | 9.77 | 6.02 | 5.63 | 6.64 | 5.54 |
| Mean angle deviation, degrees (F) | 6.02 | 3.78 | 6.71 | 5.60 | 6.13 | 5.25 |
| Angle deviation, 95th percentile (F) | 15.98 | 10.36 | 18.89 | 14.96 | 17.40 | 14.35 |
| Mean aspect ratio (F) | 1.154 | 1.965 | 1.345 | 1.322 | 1.277 | 1.344 |
| Area variation (F) | 0.157 | 0.565 | 0.364 | 0.470 | 0.277 | 0.477 |
| Hausdorff, ‰ of the diagonal (H) | 3.19 | 1.09 | 0.96 | 0.90 | 1.24 | 0.95 |
| Mean distance, ‰ (H) | 0.100 | 0.161 | 0.085 | 0.082 | 0.144 | 0.091 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.83 | 99.74 | 100.01 | 100.01 | 99.72 | 100.01 |
| Edges on closed loops, % (L) | 0.0 | 23.0 | — | 29.4 | 27.5 | — |
| Edges on spiralling loops, % (L) | 44.8 | 43.6 | — | 31.7 | 30.8 | — |
| Spiralling loops (L) | 5 | 9 | — | 15 | 13 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 55.2 | 33.4 | — | 38.9 | 41.7 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1471 | 594 | — | 551 | 510 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 2.1 | 14.8 | 1.5 | 2.2 | 10.2 | 0.3 |
| Time, seconds (T) | 5.64 | 3.36 | 11.47 | 15.82 | 12.49 | 10.68 |

## cage (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7932 | 5928 | 8112 | 8057 | 11009 | 11166 |
| Irregular vertices (S) | 72 | 58 | 48 | 58 | 62 | 64 |
| … of which above the minimum | 40 | — | 16 | 26 | 30 | 32 |
| Base complex patches (C) | 7283 | — | 1654 | 275 | 3227 | 120 |
| Alignment, degrees (A) | 8.22 | 5.16 | 2.47 | 1.94 | 2.06 | 1.91 |
| Mean angle deviation, degrees (F) | 5.83 | 7.40 | 2.13 | 2.26 | 3.22 | 2.04 |
| Angle deviation, 95th percentile (F) | 16.15 | 22.71 | 6.74 | 6.75 | 9.04 | 6.24 |
| Mean aspect ratio (F) | 1.140 | 2.161 | 1.195 | 1.233 | 1.693 | 1.194 |
| Area variation (F) | 0.139 | 0.487 | 0.214 | 0.325 | 0.338 | 0.344 |
| Hausdorff, ‰ of the diagonal (H) | 6.65 | 7.28 | 3.02 | 2.86 | 4.53 | 1.87 |
| Mean distance, ‰ (H) | 0.294 | 0.343 | 0.232 | 0.206 | 0.451 | 0.120 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.64 | 99.99 | 99.99 | 100.00 | 99.38 | 100.00 |
| Edges on closed loops, % (L) | 5.0 | 25.6 | — | 82.5 | 42.9 | — |
| Edges on spiralling loops, % (L) | 55.9 | 48.4 | — | 3.8 | 42.0 | — |
| Spiralling loops (L) | 33 | 29 | — | 1 | 14 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 40.8 | 26.0 | — | 13.7 | 15.1 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 332 | 108 | — | 40 | 129 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 0.9 | 25.9 | 1.4 | 0.7 | 37.6 | 1.4 |
| Time, seconds (T) | 5.53 | 3.23 | 4.34 | 8.20 | 5.76 | 9.59 |

## suzanne (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7983 | 2647 | 8040 | 8103 | 9246 | 9380 |
| Irregular vertices (S) | 55 | 84 | 64 | 82 | 78 | 86 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 6194 | — | 3543 | 4783 | 4088 | 5997 |
| Alignment, degrees (A) | 14.27 | 12.11 | 10.00 | 9.80 | 8.58 | 9.40 |
| Mean angle deviation, degrees (F) | 6.09 | 9.37 | 5.45 | 5.99 | 7.04 | 5.96 |
| Angle deviation, 95th percentile (F) | 18.06 | 32.08 | 15.25 | 15.22 | 18.57 | 15.65 |
| Mean aspect ratio (F) | 1.208 | 1.892 | 1.211 | 1.231 | 1.470 | 1.243 |
| Area variation (F) | 0.208 | 0.650 | 0.251 | 0.377 | 0.366 | 0.363 |
| Hausdorff, ‰ of the diagonal (H) | 6.87 | 31.86 | 4.31 | 2.96 | 3.57 | 2.59 |
| Mean distance, ‰ (H) | 0.267 | 1.112 | 0.228 | 0.179 | 0.322 | 0.156 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.4 | 9.5 | — | 5.8 | 20.0 | — |
| Edges on spiralling loops, % (L) | 11.3 | 20.3 | — | 28.6 | 5.2 | — |
| Spiralling loops (L) | 6 | 12 | — | 17 | 4 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 68.7 | 70.2 | — | 50.0 | 62.0 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 512 | 98 | — | 370 | 564 | — |
| Edges on loops from a border to a border, % (L) | 19.0 | 0.0 | — | 15.5 | 12.8 | — |
| Off budget, % (N) | 0.9 | 66.9 | 0.5 | 1.3 | 15.6 | 1.4 |
| Time, seconds (T) | 6.83 | 1.17 | 6.04 | 9.52 | 7.68 | 7.85 |

## ogre (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7763 | 4622 | 8009 | 7997 | 12717 | 12532 |
| Irregular vertices (S) | 162 | 141 | 202 | 240 | 338 | 278 |
| … of which above the minimum | 154 | — | 194 | 232 | 330 | 270 |
| Base complex patches (C) | 7352 | — | 5239 | 6131 | 9903 | 9054 |
| Alignment, degrees (A) | 17.78 | 12.94 | 11.36 | 10.70 | 8.50 | 10.28 |
| Mean angle deviation, degrees (F) | 11.59 | 7.43 | 6.29 | 6.32 | 7.95 | 5.86 |
| Angle deviation, 95th percentile (F) | 33.33 | 22.75 | 17.34 | 16.91 | 22.55 | 15.78 |
| Mean aspect ratio (F) | 1.339 | 1.812 | 1.372 | 1.377 | 2.025 | 1.356 |
| Area variation (F) | 0.312 | 0.668 | 0.331 | 0.432 | 0.643 | 0.436 |
| Hausdorff, ‰ of the diagonal (H) | 27.28 | 27.93 | 11.29 | 8.51 | 5.23 | 4.50 |
| Mean distance, ‰ (H) | 0.521 | 1.054 | 0.323 | 0.297 | 0.346 | 0.208 |
| Thin parts: mean distance, % of a quad edge (V) | 22.1 | 78.5 | 12.7 | 4.1 | 7.4 | 2.5 |
| Volume kept, % of the source (V) | 99.49 | 98.66 | 99.99 | 100.00 | 99.59 | 100.01 |
| Edges on closed loops, % (L) | 2.0 | 17.3 | — | 11.7 | 11.1 | — |
| Edges on spiralling loops, % (L) | 13.7 | 18.1 | — | 15.1 | 16.8 | — |
| Spiralling loops (L) | 23 | 23 | — | 24 | 29 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 84.3 | 64.7 | — | 73.2 | 72.2 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 484 | 175 | — | 237 | 333 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 3.0 | 42.2 | 0.1 | 0.0 | 59.0 | 1.5 |
| Time, seconds (T) | 4.83 | 3.89 | 10.98 | 11.88 | 23.50 | 18.23 |

## spot (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7874 | 2895 | 7967 | 8093 | 7965 | 8007 |
| Irregular vertices (S) | 76 | 74 | 80 | 78 | 76 | 78 |
| … of which above the minimum | 68 | — | 72 | 70 | 68 | 70 |
| Base complex patches (C) | 7772 | — | 5329 | 5007 | 4605 | 5259 |
| Alignment, degrees (A) | 16.85 | 9.88 | 7.93 | 7.55 | 7.56 | 7.36 |
| Mean angle deviation, degrees (F) | 6.23 | 7.56 | 6.47 | 6.49 | 5.56 | 6.09 |
| Angle deviation, 95th percentile (F) | 16.10 | 20.17 | 16.91 | 17.44 | 14.90 | 15.73 |
| Mean aspect ratio (F) | 1.177 | 1.481 | 1.326 | 1.331 | 1.342 | 1.339 |
| Area variation (F) | 0.178 | 0.712 | 0.330 | 0.429 | 0.301 | 0.429 |
| Hausdorff, ‰ of the diagonal (H) | 5.30 | 6.25 | 2.24 | 2.06 | 4.73 | 2.11 |
| Mean distance, ‰ (H) | 0.161 | 0.635 | 0.082 | 0.078 | 0.169 | 0.078 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.79 | 98.89 | 100.01 | 100.01 | 99.73 | 100.01 |
| Edges on closed loops, % (L) | 0.1 | 16.4 | — | 20.2 | 21.9 | — |
| Edges on spiralling loops, % (L) | 19.0 | 13.5 | — | 33.7 | 24.0 | — |
| Spiralling loops (L) | 9 | 7 | — | 16 | 10 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 79.3 | 70.1 | — | 46.1 | 54.1 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 578 | 134 | — | 279 | 349 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 1.6 | 63.8 | 0.4 | 1.2 | 0.4 | 0.5 |
| Time, seconds (T) | 3.72 | 2.50 | 11.30 | 14.00 | 17.71 | 11.73 |

## bob (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 8263 | 4012 | 7968 | 8014 | 6896 | 6770 |
| Irregular vertices (S) | 36 | 20 | 20 | 20 | 34 | 20 |
| … of which above the minimum | 36 | 20 | 20 | 20 | 34 | 20 |
| Base complex patches (C) | 8263 | 1318 | 1279 | 2898 | 4859 | 2729 |
| Alignment, degrees (A) | 17.31 | 5.28 | 3.64 | 3.75 | 5.48 | 3.65 |
| Mean angle deviation, degrees (F) | 6.39 | 4.91 | 3.64 | 3.76 | 6.43 | 3.62 |
| Angle deviation, 95th percentile (F) | 17.71 | 13.45 | 11.19 | 11.90 | 18.71 | 10.92 |
| Mean aspect ratio (F) | 1.142 | 2.024 | 1.402 | 1.380 | 1.407 | 1.387 |
| Area variation (F) | 0.159 | 0.485 | 0.374 | 0.433 | 0.369 | 0.416 |
| Hausdorff, ‰ of the diagonal (H) | 3.23 | 1.62 | 0.55 | 0.96 | 1.50 | 0.99 |
| Mean distance, ‰ (H) | 0.104 | 0.346 | 0.056 | 0.056 | 0.150 | 0.064 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.83 | 99.32 | 100.01 | 100.01 | 99.70 | 100.01 |
| Edges on closed loops, % (L) | 0.0 | 35.6 | — | 29.5 | 14.8 | — |
| Edges on spiralling loops, % (L) | 23.4 | 40.2 | — | 61.8 | 40.8 | — |
| Spiralling loops (L) | 7 | 10 | — | 10 | 12 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 76.6 | 24.2 | — | 8.7 | 44.4 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 840 | 228 | — | 261 | 848 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 3.3 | 49.9 | 0.4 | 0.2 | 13.8 | 1.8 |
| Time, seconds (T) | 3.56 | 2.33 | 8.92 | 9.05 | 20.38 | 6.95 |

## blub (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7229 | 3438 | 7990 | 7994 | 8513 | 8369 |
| Irregular vertices (S) | 70 | 53 | 48 | 56 | 51 | 56 |
| … of which above the minimum | 62 | — | 40 | 48 | 43 | 48 |
| Base complex patches (C) | 6275 | — | 3338 | 4424 | 6215 | 2721 |
| Alignment, degrees (A) | 15.23 | 7.01 | 6.93 | 6.43 | 6.32 | 6.60 |
| Mean angle deviation, degrees (F) | 10.30 | 6.30 | 6.56 | 6.11 | 6.44 | 6.28 |
| Angle deviation, 95th percentile (F) | 28.50 | 19.86 | 18.56 | 16.87 | 16.31 | 17.19 |
| Mean aspect ratio (F) | 1.267 | 2.320 | 1.340 | 1.351 | 1.595 | 1.339 |
| Area variation (F) | 0.371 | 0.598 | 0.297 | 0.394 | 0.365 | 0.395 |
| Hausdorff, ‰ of the diagonal (H) | 21.90 | 8.37 | 3.62 | 2.36 | 4.29 | 2.30 |
| Mean distance, ‰ (H) | 0.213 | 0.399 | 0.071 | 0.059 | 0.128 | 0.057 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.48 | 99.18 | 99.99 | 100.00 | 99.74 | 100.00 |
| Edges on closed loops, % (L) | 6.4 | 28.0 | — | 24.4 | 13.5 | — |
| Edges on spiralling loops, % (L) | 20.2 | 18.6 | — | 46.7 | 45.9 | — |
| Spiralling loops (L) | 12 | 5 | — | 17 | 17 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 73.2 | 53.3 | — | 28.9 | 40.6 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 660 | 163 | — | 299 | 515 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 9.6 | 57.0 | 0.1 | 0.1 | 6.4 | 1.7 |
| Time, seconds (T) | 3.58 | 3.90 | 7.95 | 10.14 | 15.56 | 9.48 |

## pig (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | 6973 | 4682 | 8093 | 8108 | 9824 | 9900 |
| Irregular vertices (S) | 136 | 135 | 88 | 98 | 101 | 98 |
| … of which above the minimum | 128 | — | 80 | 90 | — | 90 |
| Base complex patches (C) | 6841 | — | 5985 | 5376 | — | 6168 |
| Alignment, degrees (A) | 15.84 | 11.22 | 9.47 | 8.20 | 7.39 | 8.48 |
| Mean angle deviation, degrees (F) | 11.44 | 6.80 | 7.06 | 5.68 | 6.40 | 5.99 |
| Angle deviation, 95th percentile (F) | 34.02 | 21.80 | 20.08 | 17.10 | 18.75 | 18.61 |
| Mean aspect ratio (F) | 1.339 | 1.773 | 1.342 | 1.326 | 1.546 | 1.328 |
| Area variation (F) | 0.383 | 0.668 | 0.333 | 0.494 | 0.462 | 0.499 |
| Hausdorff, ‰ of the diagonal (H) | 56.23 | 9.84 | 7.22 | 6.09 | 9.11 | 6.69 |
| Mean distance, ‰ (H) | 0.610 | 0.627 | 0.142 | 0.118 | 0.222 | 0.100 |
| Thin parts: mean distance, % of a quad edge (V) | 64.0 | 21.7 | 15.9 | 5.9 | 3.7 | 4.7 |
| Volume kept, % of the source (V) | 99.23 | 99.18 | 99.99 | 100.00 | 99.74 | 100.00 |
| Edges on closed loops, % (L) | 0.5 | 9.6 | — | 16.8 | 24.7 | — |
| Edges on spiralling loops, % (L) | 21.6 | 22.2 | — | 21.0 | 15.3 | — |
| Spiralling loops (L) | 22 | 18 | — | 30 | 21 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 77.2 | 68.1 | — | 62.2 | 60.0 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 500 | 161 | — | 310 | 456 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 12.8 | 41.5 | 1.2 | 1.4 | 22.8 | 0.8 |
| Time, seconds (T) | 3.13 | 4.60 | 7.71 | 13.45 | 16.84 | 14.26 |

## jerry (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7781 | 2740 | 7903 | 7951 | 13714 | 13817 |
| Irregular vertices (S) | 112 | 109 | 168 | 279 | 244 | 355 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 4991 | — | 5458 | 6820 | 11709 | 11799 |
| Alignment, degrees (A) | 17.56 | 15.73 | 12.91 | 12.37 | 10.64 | 11.71 |
| Mean angle deviation, degrees (F) | 7.80 | 10.26 | 5.22 | 5.71 | 8.15 | 5.90 |
| Angle deviation, 95th percentile (F) | 22.58 | 29.98 | 14.50 | 16.65 | 22.38 | 15.73 |
| Mean aspect ratio (F) | 1.257 | 1.889 | 1.249 | 1.266 | 1.879 | 1.284 |
| Area variation (F) | 0.245 | 0.652 | 0.313 | 0.526 | 0.651 | 0.462 |
| Hausdorff, ‰ of the diagonal (H) | 31.29 | 25.08 | 15.26 | 10.48 | 4.22 | 10.37 |
| Mean distance, ‰ (H) | 0.581 | 2.010 | 0.359 | 0.279 | 0.310 | 0.170 |
| Thin parts: mean distance, % of a quad edge (V) | 24.5 | 54.4 | 23.3 | 9.6 | 3.9 | 11.0 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | — | — | 1.1 | 3.3 | — |
| Edges on spiralling loops, % (L) | 3.1 | — | — | 3.5 | 9.1 | — |
| Spiralling loops (L) | 5 | — | — | 8 | 17 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 74.6 | — | — | 89.2 | 83.4 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 404 | — | — | 276 | 467 | — |
| Edges on loops from a border to a border, % (L) | 19.8 | — | — | 6.2 | 4.2 | — |
| Off budget, % (N) | 2.7 | 65.8 | 1.2 | 0.6 | 71.4 | 0.8 |
| Time, seconds (T) | 6.02 | 2.44 | 5.59 | 8.38 | 9.49 | 15.47 |

## rose (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 6758 | 3828 | 8296 | 8343 | 53874 | 53886 |
| Irregular vertices (S) | 917 | 556 | 1222 | 1643 | 1899 | 2544 |
| … of which above the minimum | — | — | 606 | 1027 | — | 1928 |
| Base complex patches (C) | 6524 | — | 6769 | 6482 | — | 43115 |
| Alignment, degrees (A) | 18.21 | 18.28 | 12.75 | 12.50 | 9.05 | 9.34 |
| Mean angle deviation, degrees (F) | 14.81 | 17.84 | 12.15 | 13.46 | 8.61 | 6.23 |
| Angle deviation, 95th percentile (F) | 41.35 | 56.33 | 44.68 | 41.03 | 24.64 | 19.22 |
| Mean aspect ratio (F) | 1.515 | 3.415 | 3.302 | 2.138 | 1.839 | 1.365 |
| Area variation (F) | 0.407 | 0.904 | 0.629 | 1.432 | 1.220 | 0.474 |
| Hausdorff, ‰ of the diagonal (H) | 74.30 | 58.87 | 16.29 | 15.48 | 73.68 | 7.95 |
| Mean distance, ‰ (H) | 1.489 | 4.497 | 0.678 | 0.911 | 0.689 | 0.113 |
| Thin parts: mean distance, % of a quad edge (V) | 44.1 | 83.6 | 13.1 | 12.2 | 22.2 | 10.4 |
| Volume kept, % of the source (V) | — | — | 97.30 | 95.76 | — | 99.78 |
| Edges on closed loops, % (L) | 0.4 | — | — | 11.4 | — | — |
| Edges on spiralling loops, % (L) | 17.0 | — | — | 22.6 | — | — |
| Spiralling loops (L) | 130 | — | — | 448 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 80.4 | — | — | 66.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 90 | — | — | 49 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 15.5 | 52.1 | 3.7 | 4.3 | 573.4 | 0.0 |
| Time, seconds (T) | 3.62 | 1.88 | 3.05 | 6.08 | 9.67 | 19.58 |

## bridge (organic)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | — | 4954 | 7880 | 8109 | 8805 | 8799 |
| Irregular vertices (S) | — | 58 | 70 | 81 | 82 | 89 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 2987 | 2725 | 2942 | 3637 |
| Alignment, degrees (A) | — | 10.49 | 9.87 | 9.03 | 8.33 | 9.22 |
| Mean angle deviation, degrees (F) | — | 6.02 | 4.82 | 4.66 | 6.17 | 4.39 |
| Angle deviation, 95th percentile (F) | — | 18.67 | 12.45 | 12.17 | 17.33 | 12.50 |
| Mean aspect ratio (F) | — | 1.719 | 1.271 | 1.263 | 1.480 | 1.274 |
| Area variation (F) | — | 0.719 | 0.264 | 0.329 | 0.354 | 0.363 |
| Hausdorff, ‰ of the diagonal (H) | — | 8.89 | 6.83 | 6.75 | 6.69 | 7.19 |
| Mean distance, ‰ (H) | — | 0.521 | 0.220 | 0.215 | 0.264 | 0.202 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 20.8 | — | 19.1 | 19.3 | — |
| Edges on spiralling loops, % (L) | — | 8.5 | — | 7.2 | 7.6 | — |
| Spiralling loops (L) | — | 5 | — | 7 | 8 | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 53.1 | — | 50.2 | 51.4 | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 154 | — | 224 | 183 | — |
| Edges on loops from a border to a border, % (L) | — | 17.6 | — | 23.5 | 21.7 | — |
| Off budget, % (N) | — | 38.1 | 1.5 | 1.4 | 10.1 | 0.1 |
| Time, seconds (T) | — | 3.19 | 6.11 | 9.38 | 15.51 | 8.18 |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7983 | 4954 | 8078 | 8098 | 7353 | 7279 |
| Irregular vertices (S) | 38 | 94 | 24 | 24 | 24 | 24 |
| … of which above the minimum | 30 | — | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 1171 | — | 429 | 92 | 730 | 316 |
| Alignment, degrees (A) | 13.31 | 14.04 | 11.83 | 10.32 | 10.87 | 10.48 |
| Mean angle deviation, degrees (F) | 2.56 | 13.77 | 1.25 | 1.00 | 3.63 | 1.27 |
| Angle deviation, 95th percentile (F) | 9.55 | 47.48 | 3.80 | 3.80 | 14.81 | 4.15 |
| Mean aspect ratio (F) | 1.074 | 3.591 | 1.093 | 1.129 | 1.301 | 1.116 |
| Area variation (F) | 0.087 | 0.912 | 0.126 | 0.328 | 0.244 | 0.339 |
| Hausdorff, ‰ of the diagonal (H) | 5.14 | 11.34 | 0.31 | 0.25 | 0.68 | 0.27 |
| Mean distance, ‰ (H) | 0.034 | 0.506 | 0.022 | 0.024 | 0.040 | 0.026 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.06 | — | 100.00 | 100.00 | 100.06 | 100.00 |
| Edges on closed loops, % (L) | 59.2 | — | — | 89.3 | 67.0 | — |
| Edges on spiralling loops, % (L) | 22.9 | — | — | 0.0 | 18.3 | — |
| Spiralling loops (L) | 8 | — | — | 0 | 6 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 17.7 | — | — | 10.7 | 14.7 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 160 | — | — | 82 | 130 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 0.3 | 38.1 | 1.0 | 1.2 | 8.1 | 1.0 |
| Time, seconds (T) | 8.90 | 3.06 | 6.05 | 7.08 | 2.65 | 5.62 |

## cube (mechanical)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7920 | 2774 | 8066 | 8066 | 7920 | 7776 |
| Irregular vertices (S) | 8 | 10 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | — | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 6 | — | 6 | 6 | 6 | 6 |
| Alignment, degrees (A) | 0.46 | 2.60 | 0.11 | 0.27 | 0.11 | 0.28 |
| Mean angle deviation, degrees (F) | 0.36 | 5.97 | 0.01 | 0.20 | 0.00 | 0.21 |
| Angle deviation, 95th percentile (F) | 1.18 | 24.98 | 0.02 | 0.74 | 0.00 | 0.76 |
| Mean aspect ratio (F) | 1.020 | 2.866 | 1.018 | 1.074 | 1.019 | 1.066 |
| Area variation (F) | 0.023 | 0.740 | 0.013 | 0.093 | 0.013 | 0.091 |
| Hausdorff, ‰ of the diagonal (H) | 0.85 | 4.81 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.007 | 0.165 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.99 | 100.00 | 100.00 | 100.00 | 100.00 |
| Edges on closed loops, % (L) | 97.2 | 27.9 | — | 97.3 | 97.2 | — |
| Edges on spiralling loops, % (L) | 0.0 | 63.5 | — | 0.0 | 0.0 | — |
| Spiralling loops (L) | 0 | 6 | — | 0 | 0 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 2.8 | 8.5 | — | 2.7 | 2.8 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 37 | 85 | — | 37 | 37 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 1.0 | 65.3 | 0.8 | 0.8 | 1.0 | 1.8 |
| Time, seconds (T) | 4.14 | 1.30 | 4.51 | 5.13 | 3.50 | 4.67 |

## lblock (mechanical)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 8016 | 3035 | 8064 | 8064 | 8064 | 8064 |
| Irregular vertices (S) | 13 | 20 | 12 | 12 | 12 | 12 |
| … of which above the minimum | 4 | — | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 402 | — | 14 | 14 | 14 | 14 |
| Alignment, degrees (A) | 0.86 | 3.67 | 0.61 | 0.92 | 0.54 | 0.92 |
| Mean angle deviation, degrees (F) | 0.40 | 6.33 | 0.00 | 0.49 | 0.00 | 0.49 |
| Angle deviation, 95th percentile (F) | 1.40 | 24.33 | 0.01 | 1.49 | 0.00 | 1.49 |
| Mean aspect ratio (F) | 1.018 | 2.553 | 1.000 | 1.078 | 1.000 | 1.078 |
| Area variation (F) | 0.024 | 0.668 | 0.000 | 0.096 | 0.000 | 0.096 |
| Hausdorff, ‰ of the diagonal (H) | 0.72 | 5.67 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.005 | 0.155 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.97 | 100.00 | 100.00 | 100.00 | 100.00 |
| Edges on closed loops, % (L) | 82.3 | 50.2 | — | 95.8 | 95.8 | — |
| Edges on spiralling loops, % (L) | 11.3 | 27.9 | — | 0.0 | 0.0 | — |
| Spiralling loops (L) | 1 | 5 | — | 0 | 0 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 6.1 | 22.0 | — | 4.2 | 4.2 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 132 | 134 | — | 72 | 72 | — |
| Edges on loops from a border to a border, % (L) | 0.3 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 0.6 | 62.1 | 0.8 | 0.8 | 0.8 | 0.0 |
| Time, seconds (T) | 3.24 | 1.54 | 4.20 | 4.70 | 3.10 | 4.54 |

Failures: quadriflow: timed out after 1800 s.

## cylinder (mechanical)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7916 | 2565 | 7980 | 7899 | 6922 | 7007 |
| Irregular vertices (S) | 8 | 10 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | — | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 2635 | — | 1452 | 1399 | 850 | 1247 |
| Alignment, degrees (A) | 4.37 | 4.10 | 1.32 | 1.28 | 1.73 | 1.25 |
| Mean angle deviation, degrees (F) | 3.95 | 6.60 | 2.31 | 2.33 | 4.41 | 2.23 |
| Angle deviation, 95th percentile (F) | 16.68 | 20.66 | 10.22 | 10.46 | 18.99 | 9.61 |
| Mean aspect ratio (F) | 1.075 | 2.021 | 1.288 | 1.398 | 1.349 | 1.369 |
| Area variation (F) | 0.096 | 0.636 | 0.296 | 0.213 | 0.322 | 0.191 |
| Hausdorff, ‰ of the diagonal (H) | 1.80 | 5.39 | 0.54 | 0.55 | 0.73 | 0.58 |
| Mean distance, ‰ (H) | 0.052 | 0.289 | 0.066 | 0.072 | 0.147 | 0.075 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.96 | 99.70 | 100.00 | 100.00 | 99.83 | 100.00 |
| Edges on closed loops, % (L) | 33.3 | 29.3 | — | 41.1 | 57.3 | — |
| Edges on spiralling loops, % (L) | 48.3 | 33.0 | — | 56.0 | 34.4 | — |
| Spiralling loops (L) | 5 | 3 | — | 2 | 4 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 18.6 | 37.7 | — | 2.8 | 8.3 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 762 | 316 | — | 91 | 205 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 1.1 | 67.9 | 0.2 | 1.3 | 13.5 | 1.2 |
| Time, seconds (T) | 4.60 | 1.04 | 4.66 | 5.51 | 4.42 | 3.85 |

## venturi (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 0/1 | 0/1 | 1/1 |
| Quads made | — | 7212 | 7574 | 8376 | 19704 | 19050 |
| Irregular vertices (S) | — | 2158 | 283 | 884 | 695 | 736 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 5218 | 6123 | — | 14291 |
| Alignment, degrees (A) | — | 10.19 | 10.32 | 11.72 | 7.51 | 9.61 |
| Mean angle deviation, degrees (F) | — | 40.97 | 6.37 | 8.83 | 8.05 | 5.76 |
| Angle deviation, 95th percentile (F) | — | 89.08 | 21.47 | 32.44 | 34.33 | 19.44 |
| Mean aspect ratio (F) | — | 9.234 | 1.293 | 1.864 | 2.096 | 1.336 |
| Area variation (F) | — | 2.203 | 0.299 | 0.436 | 1.067 | 0.296 |
| Hausdorff, ‰ of the diagonal (H) | — | 37.63 | 36.17 | 36.17 | 14.05 | 15.92 |
| Mean distance, ‰ (H) | — | 2.866 | 2.139 | 2.025 | 0.241 | 0.870 |
| Thin parts: mean distance, % of a quad edge (V) | — | 28.8 | 28.1 | 26.0 | 1.8 | 25.0 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | 14.6 | 6.4 | — |
| Edges on spiralling loops, % (L) | — | — | — | 22.6 | 1.5 | — |
| Spiralling loops (L) | — | — | — | 31 | 12 | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | 62.9 | 27.6 | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | 163 | 132 | — |
| Edges on loops from a border to a border, % (L) | — | — | — | 0.0 | 64.6 | — |
| Off budget, % (N) | — | 9.8 | 5.3 | 4.7 | 146.3 | 3.3 |
| Time, seconds (T) | — | 1.84 | 50.73 | 38.16 | 5.17 | 97.65 |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 4422 | 8133 | 7952 | 13801 | 13743 |
| Irregular vertices (S) | — | 172 | 198 | 354 | 368 | 432 |
| … of which above the minimum | — | — | 190 | 346 | 360 | 424 |
| Base complex patches (C) | — | — | 6567 | 6122 | 10719 | 11695 |
| Alignment, degrees (A) | — | 13.99 | 12.05 | 10.05 | 8.76 | 9.69 |
| Mean angle deviation, degrees (F) | — | 7.16 | 6.00 | 6.61 | 7.98 | 6.08 |
| Angle deviation, 95th percentile (F) | — | 21.73 | 16.64 | 19.28 | 22.44 | 17.35 |
| Mean aspect ratio (F) | — | 1.812 | 1.390 | 1.381 | 2.045 | 1.344 |
| Area variation (F) | — | 0.657 | 0.353 | 0.968 | 0.686 | 1.030 |
| Hausdorff, ‰ of the diagonal (H) | — | 22.79 | 10.14 | 7.19 | 7.42 | 6.66 |
| Mean distance, ‰ (H) | — | 1.109 | 0.316 | 0.336 | 0.321 | 0.222 |
| Thin parts: mean distance, % of a quad edge (V) | — | 44.3 | 9.3 | 1.4 | 3.6 | 1.4 |
| Volume kept, % of the source (V) | — | — | 100.00 | 100.01 | 99.56 | 100.01 |
| Edges on closed loops, % (L) | — | — | — | 11.6 | 11.3 | — |
| Edges on spiralling loops, % (L) | — | — | — | 10.9 | 12.1 | — |
| Spiralling loops (L) | — | — | — | 26 | 29 | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | 77.5 | 76.7 | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | 173 | 313 | — |
| Edges on loops from a border to a border, % (L) | — | — | — | 0.0 | 0.0 | — |
| Off budget, % (N) | — | 44.7 | 1.7 | 0.6 | 72.5 | 0.4 |
| Time, seconds (T) | — | 17.03 | 58.59 | 67.02 | 44.60 | 64.78 |

Failures: quadriflow: cancelled.

## watermill (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 7305 | 2507 | 6743 | 6739 | 42790 | — |
| Irregular vertices (S) | 642 | 428 | 423 | 423 | 1504 | — |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 6985 | — | — | — | — | — |
| Alignment, degrees (A) | 13.37 | 16.69 | 12.95 | 12.99 | 10.22 | — |
| Mean angle deviation, degrees (F) | 9.18 | 21.18 | 8.09 | 8.20 | 7.41 | — |
| Angle deviation, 95th percentile (F) | 32.10 | 68.70 | 28.49 | 29.38 | 22.87 | — |
| Mean aspect ratio (F) | 1.331 | 2.614 | 1.332 | 1.340 | 1.878 | — |
| Area variation (F) | 0.268 | 0.825 | 0.277 | 0.277 | 0.822 | — |
| Hausdorff, ‰ of the diagonal (H) | 50.24 | 90.14 | 53.45 | 53.45 | 30.04 | — |
| Mean distance, ‰ (H) | 1.990 | 7.482 | 2.383 | 2.383 | 0.641 | — |
| Thin parts: mean distance, % of a quad edge (V) | 23.3 | 69.7 | 32.9 | 32.9 | 13.0 | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 2.3 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 12.4 | — | — | — | — | — |
| Spiralling loops (L) | 125 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.2 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 151 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 0.1 | — | — | — | — | — |
| Off budget, % (N) | 8.7 | 68.7 | 15.7 | 15.8 | 434.9 | — |
| Time, seconds (T) | 7.40 | 4.79 | 91.77 | 102.07 | 261.56 | — |

Failures: openclare-matched: no Quad Remesher reference to match.

## towermill (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 7180 | 2664 | 6213 | — | 35806 | — |
| Irregular vertices (S) | 624 | 292 | 295 | — | 1135 | — |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 6952 | — | — | — | — | — |
| Alignment, degrees (A) | 11.45 | 14.32 | 11.00 | — | 10.17 | — |
| Mean angle deviation, degrees (F) | 9.29 | 18.29 | 8.57 | — | 7.80 | — |
| Angle deviation, 95th percentile (F) | 29.68 | 60.76 | 28.53 | — | 23.07 | — |
| Mean aspect ratio (F) | 1.309 | 2.412 | 1.381 | — | 2.088 | — |
| Area variation (F) | 0.258 | 0.707 | 0.314 | — | 0.757 | — |
| Hausdorff, ‰ of the diagonal (H) | 34.62 | 60.39 | 38.98 | — | 15.92 | — |
| Mean distance, ‰ (H) | 1.468 | 4.925 | 2.188 | — | 0.833 | — |
| Thin parts: mean distance, % of a quad edge (V) | 16.1 | 39.9 | 27.5 | — | 11.4 | — |
| Volume kept, % of the source (V) | — | — | — | — | 98.95 | — |
| Edges on closed loops, % (L) | 1.3 | — | — | — | 2.1 | — |
| Edges on spiralling loops, % (L) | 18.5 | — | — | — | 15.7 | — |
| Spiralling loops (L) | 50 | — | — | — | 91 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 80.1 | — | — | — | 82.3 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 117 | — | — | — | 229 | — |
| Edges on loops from a border to a border, % (L) | 0.1 | — | — | — | 0.0 | — |
| Off budget, % (N) | 10.2 | 66.7 | 22.3 | — | 347.6 | — |
| Time, seconds (T) | 7.17 | 4.13 | 99.95 | — | 321.94 | — |

Failures: openclare: timed out after 1800 s; openclare-matched: no Quad Remesher reference to match.

## stonemill (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 |
| Quads made | 6712 | 4327 | 7867 | 8182 | 33780 | — |
| Irregular vertices (S) | 400 | 121 | 190 | 280 | 963 | — |
| … of which above the minimum | 341 | — | 126 | 216 | 899 | — |
| Base complex patches (C) | 6543 | — | 6331 | 7587 | 31433 | — |
| Alignment, degrees (A) | 13.29 | 13.65 | 11.27 | 11.28 | 11.22 | — |
| Mean angle deviation, degrees (F) | 9.33 | 10.72 | 4.40 | 5.26 | 7.11 | — |
| Angle deviation, 95th percentile (F) | 30.16 | 31.96 | 14.37 | 17.03 | 20.64 | — |
| Mean aspect ratio (F) | 1.293 | 1.880 | 1.196 | 1.263 | 1.963 | — |
| Area variation (F) | 0.268 | 0.625 | 0.215 | 0.357 | 0.555 | — |
| Hausdorff, ‰ of the diagonal (H) | 17.23 | 58.87 | 11.27 | 14.15 | 10.81 | — |
| Mean distance, ‰ (H) | 1.190 | 2.121 | 1.074 | 1.019 | 0.585 | — |
| Thin parts: mean distance, % of a quad edge (V) | 24.8 | 185.8 | 22.9 | 22.5 | 12.3 | — |
| Volume kept, % of the source (V) | 99.56 | 99.45 | 100.01 | 100.02 | 99.37 | — |
| Edges on closed loops, % (L) | 2.3 | 0.8 | — | 4.1 | 3.7 | — |
| Edges on spiralling loops, % (L) | 8.9 | 24.3 | — | 15.5 | 6.8 | — |
| Spiralling loops (L) | 33 | 25 | — | 23 | 40 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.0 | 74.8 | — | 80.5 | 89.5 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 149 | 168 | — | 268 | 340 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 16.1 | 45.9 | 1.7 | 2.3 | 322.2 | — |
| Time, seconds (T) | 6.75 | 3.06 | 8.45 | 7.70 | 133.40 | — |

Failures: openclare-matched: no Quad Remesher reference to match.

## windmill (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 1/1 | 0/1 | 0/1 |
| Quads made | — | 2364 | 263 | 439 | 36766 | — |
| Irregular vertices (S) | — | 234 | 36 | 116 | 1245 | — |
| … of which above the minimum | — | — | 12 | 92 | — | — |
| Base complex patches (C) | — | — | 239 | 439 | — | — |
| Alignment, degrees (A) | — | 13.30 | 17.78 | 18.87 | 9.54 | — |
| Mean angle deviation, degrees (F) | — | 14.00 | 29.82 | 21.80 | 7.28 | — |
| Angle deviation, 95th percentile (F) | — | 51.31 | 69.36 | 56.93 | 22.98 | — |
| Mean aspect ratio (F) | — | 2.278 | 7.843 | 2.886 | 1.983 | — |
| Area variation (F) | — | 0.698 | 1.367 | 0.901 | 0.768 | — |
| Hausdorff, ‰ of the diagonal (H) | — | 93.86 | 531.90 | 529.94 | 11.92 | — |
| Mean distance, ‰ (H) | — | 6.515 | 244.649 | 232.732 | 0.466 | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 80.5 | 1851.6 | 1842.0 | 9.0 | — |
| Volume kept, % of the source (V) | — | — | 1.51 | 1.58 | — | — |
| Edges on closed loops, % (L) | — | — | — | 0.0 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | 8.4 | — | — |
| Spiralling loops (L) | — | — | — | 9 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | 91.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | 16 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 70.5 | 96.7 | 94.5 | 359.6 | — |
| Time, seconds (T) | — | 3.34 | 2.65 | 4.37 | 181.10 | — |

Failures: quadriflow: cancelled; openclare-matched: no Quad Remesher reference to match.

## terrain (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | 6963 | 2975 | 7941 | 8014 | 11819 | 11855 |
| Irregular vertices (S) | 188 | 108 | 124 | 229 | 296 | 315 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 2067 | — | 5508 | 7314 | 8228 | 10547 |
| Alignment, degrees (A) | 21.04 | 20.95 | 17.72 | 17.58 | 18.10 | 17.46 |
| Mean angle deviation, degrees (F) | 5.79 | 7.30 | 6.86 | 6.88 | 8.69 | 6.37 |
| Angle deviation, 95th percentile (F) | 21.42 | 26.48 | 18.80 | 18.66 | 25.15 | 16.76 |
| Mean aspect ratio (F) | 1.187 | 1.481 | 1.299 | 1.301 | 1.668 | 1.259 |
| Area variation (F) | 0.185 | 0.463 | 0.322 | 0.475 | 0.518 | 0.499 |
| Hausdorff, ‰ of the diagonal (H) | 16.98 | 26.34 | 11.35 | 8.75 | 10.70 | 8.30 |
| Mean distance, ‰ (H) | 1.071 | 1.562 | 0.744 | 0.629 | 0.719 | 0.503 |
| Thin parts: mean distance, % of a quad edge (V) | 34.0 | 47.6 | 23.6 | 17.9 | 20.8 | 19.2 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | — | 3.1 | 0.3 | — |
| Spiralling loops (L) | 0 | 0 | — | 2 | 1 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 53.9 | 71.1 | — | 92.3 | 82.6 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 79 | 55 | — | 167 | 166 | — |
| Edges on loops from a border to a border, % (L) | 46.1 | 28.9 | — | 4.6 | 17.1 | — |
| Off budget, % (N) | 13.0 | 62.8 | 0.7 | 0.2 | 47.7 | 0.3 |
| Time, seconds (T) | 19.08 | 4.25 | 11.42 | 12.25 | 24.96 | 15.75 |

## mahaveer (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 2996 | 6472 | 7890 | 55181 | 53461 |
| Irregular vertices (S) | — | 419 | 533 | 567 | 3155 | 1198 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 9.27 | 4.91 | 4.84 | 11.56 | 5.06 |
| Mean angle deviation, degrees (F) | — | 19.93 | 9.89 | 8.25 | 11.13 | 5.63 |
| Angle deviation, 95th percentile (F) | — | 77.95 | 64.66 | 51.60 | 36.88 | 20.99 |
| Mean aspect ratio (F) | — | 3.957 | 54.906 | 64.831 | 1.978 | 7.246 |
| Area variation (F) | — | 1.337 | 3.121 | 1.751 | 1.998 | 0.912 |
| Hausdorff, ‰ of the diagonal (H) | — | 36.28 | 32.27 | 27.00 | 13.01 | 13.39 |
| Mean distance, ‰ (H) | — | 3.494 | 2.389 | 2.010 | 0.784 | 0.609 |
| Thin parts: mean distance, % of a quad edge (V) | — | 52.7 | 34.4 | 31.1 | 17.5 | 41.2 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | 45.0 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | 29.6 | — | — |
| Spiralling loops (L) | — | — | — | 561 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | 20.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | 136 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | 4.9 | — | — |
| Off budget, % (N) | — | 62.5 | 19.1 | 1.4 | 589.8 | 3.1 |
| Time, seconds (T) | — | 2.97 | 9.67 | 8.53 | 13.92 | 49.18 |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 2388 | 12356 | 12000 | 246063 | 231978 |
| Irregular vertices (S) | — | 1056 | 6142 | 6226 | 10419 | 11799 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 15.08 | 11.75 | 11.91 | 10.30 | 8.36 |
| Mean angle deviation, degrees (F) | — | 28.51 | 17.41 | 17.91 | 8.58 | 4.92 |
| Angle deviation, 95th percentile (F) | — | 82.76 | 62.96 | 63.61 | 25.88 | 18.22 |
| Mean aspect ratio (F) | — | 7.658 | 6.373 | 3.800 | 1.549 | 1.326 |
| Area variation (F) | — | 1.726 | 2.331 | 2.431 | 1.812 | 0.533 |
| Hausdorff, ‰ of the diagonal (H) | — | 6.92 | 3.85 | 3.85 | 2.08 | 3.02 |
| Mean distance, ‰ (H) | — | 1.765 | 0.309 | 0.330 | 0.091 | 0.033 |
| Thin parts: mean distance, % of a quad edge (V) | — | 94.0 | 15.2 | 16.1 | 10.1 | 19.1 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | 15.7 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | 24.9 | — | — |
| Spiralling loops (L) | — | — | — | 3982 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | 59.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | 36 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | 0.4 | — | — |
| Off budget, % (N) | — | 70.2 | 54.4 | 50.0 | 2975.8 | 5.7 |
| Time, seconds (T) | — | 1.13 | 10.56 | 18.77 | 13.22 | 44.98 |

Failures: quadriflow: cancelled.

## f16 (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 6227 | 9469 | 9512 | 46194 | 43843 |
| Irregular vertices (S) | — | 2171 | 1840 | 1786 | 2662 | 2032 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 10.29 | 5.49 | 4.86 | 4.69 | 3.96 |
| Mean angle deviation, degrees (F) | — | 29.31 | 11.57 | 10.60 | 9.22 | 6.20 |
| Angle deviation, 95th percentile (F) | — | 84.69 | 58.92 | 59.08 | 33.32 | 23.11 |
| Mean aspect ratio (F) | — | 6.307 | 55.746 | 55.011 | 2.322 | 16.496 |
| Area variation (F) | — | 1.369 | 1.656 | 1.803 | 1.514 | 0.765 |
| Hausdorff, ‰ of the diagonal (H) | — | 80.26 | 18.18 | 23.76 | 6.30 | 12.17 |
| Mean distance, ‰ (H) | — | 5.085 | 0.629 | 0.640 | 0.257 | 0.214 |
| Thin parts: mean distance, % of a quad edge (V) | — | 60.6 | 7.4 | 7.8 | 4.1 | 8.0 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — |
| Off budget, % (N) | — | 22.2 | 18.4 | 18.9 | 477.4 | 5.1 |
| Time, seconds (T) | — | 3.57 | 10.54 | 10.23 | 10.81 | 85.72 |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 4114 | 7811 | 8042 | 20326 | 19734 |
| Irregular vertices (S) | — | 403 | 222 | 257 | 458 | 295 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 8.73 | 3.30 | 3.42 | 3.48 | 3.31 |
| Mean angle deviation, degrees (F) | — | 18.87 | 13.22 | 14.02 | 5.75 | 8.39 |
| Angle deviation, 95th percentile (F) | — | 74.17 | 71.94 | 72.22 | 21.00 | 47.61 |
| Mean aspect ratio (F) | — | 3.158 | 17.277 | 16.284 | 6.194 | 5.441 |
| Area variation (F) | — | 1.607 | 1.028 | 1.095 | 1.143 | 0.761 |
| Hausdorff, ‰ of the diagonal (H) | — | 58.33 | 32.96 | 32.96 | 7.91 | 20.02 |
| Mean distance, ‰ (H) | — | 3.579 | 1.025 | 0.991 | 0.334 | 0.342 |
| Thin parts: mean distance, % of a quad edge (V) | — | 32.4 | 11.2 | 11.0 | 2.8 | 9.8 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 31.7 | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 8.2 | — |
| Spiralling loops (L) | — | — | — | — | 17 | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 33.6 | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 664 | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 26.5 | — |
| Off budget, % (N) | — | 48.6 | 2.4 | 0.5 | 154.1 | 2.9 |
| Time, seconds (T) | — | 1.11 | 10.38 | 11.40 | 7.25 | 21.54 |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 2119 | 6760 | 6901 | 28909 | 25241 |
| Irregular vertices (S) | — | 195 | 417 | 457 | 961 | 727 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 13.19 | 10.31 | 11.06 | 9.87 | 11.35 |
| Mean angle deviation, degrees (F) | — | 10.86 | 9.34 | 9.86 | 9.44 | 8.62 |
| Angle deviation, 95th percentile (F) | — | 32.41 | 26.93 | 28.71 | 27.87 | 25.31 |
| Mean aspect ratio (F) | — | 1.955 | 1.458 | 1.466 | 1.826 | 1.343 |
| Area variation (F) | — | 0.849 | 0.311 | 0.344 | 0.850 | 0.305 |
| Hausdorff, ‰ of the diagonal (H) | — | 36.12 | 12.56 | 12.56 | 9.82 | 8.60 |
| Mean distance, ‰ (H) | — | 3.139 | 0.688 | 0.677 | 0.330 | 0.207 |
| Thin parts: mean distance, % of a quad edge (V) | — | 103.8 | 34.0 | 33.4 | 20.6 | 39.9 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 12.3 | — | — | — | — |
| Edges on spiralling loops, % (L) | — | 3.9 | — | — | — | — |
| Spiralling loops (L) | — | 9 | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 83.8 | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 57 | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | — | — | — |
| Off budget, % (N) | — | 73.5 | 15.5 | 13.7 | 261.4 | 12.7 |
| Time, seconds (T) | — | 2.60 | 131.25 | 130.95 | 41.01 | 312.03 |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7736 | 6520 | 7984 | 8074 | 7088 | 7223 |
| Irregular vertices (S) | 12 | 26 | 26 | 26 | 8 | 18 |
| … of which above the minimum | 4 | 18 | 18 | 18 | 0 | 10 |
| Base complex patches (C) | 7729 | 6252 | 7984 | 8074 | 4992 | 7223 |
| Alignment, degrees (A) | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.93 | 5.85 | 4.34 | 4.52 | 7.65 | 5.94 |
| Angle deviation, 95th percentile (F) | 12.52 | 14.46 | 9.72 | 9.91 | 18.00 | 12.35 |
| Mean aspect ratio (F) | 1.109 | 1.337 | 1.130 | 1.134 | 1.187 | 1.198 |
| Area variation (F) | 0.134 | 0.283 | 0.174 | 0.168 | 0.391 | 0.198 |
| Hausdorff, ‰ of the diagonal (H) | 0.42 | 0.80 | 0.35 | 0.36 | 0.55 | 0.34 |
| Mean distance, ‰ (H) | 0.085 | 0.172 | 0.078 | 0.078 | 0.159 | 0.086 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.93 | 99.83 | 100.00 | 100.00 | 99.83 | 100.00 |
| Edges on closed loops, % (L) | 0.0 | 2.1 | — | 0.0 | 15.2 | — |
| Edges on spiralling loops, % (L) | 52.4 | 42.1 | — | 35.3 | 60.2 | — |
| Spiralling loops (L) | 5 | 10 | — | 6 | 4 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 47.6 | 55.8 | — | 64.7 | 24.6 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1548 | 584 | — | 1130 | 1726 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 3.3 | 18.5 | 0.2 | 0.9 | 11.4 | 1.9 |
| Time, seconds (T) | 4.47 | 1.94 | 4.72 | 6.10 | 4.18 | 5.35 |

## torus (calibration)

| | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 9147 | 7544 | 8024 | 8024 | 7401 | 7467 |
| Irregular vertices (S) | 22 | 0 | 0 | 0 | 20 | 0 |
| … of which above the minimum | 22 | 0 | 0 | 0 | 20 | 0 |
| Base complex patches (C) | 9070 | 1 | 1 | 1 | 5983 | 1 |
| Alignment, degrees (A) | 11.41 | 0.76 | 0.64 | 0.64 | 4.25 | 0.62 |
| Mean angle deviation, degrees (F) | 7.33 | 1.26 | 0.91 | 0.91 | 3.68 | 0.91 |
| Angle deviation, 95th percentile (F) | 19.57 | 2.30 | 1.42 | 1.42 | 10.81 | 1.47 |
| Mean aspect ratio (F) | 1.180 | 2.552 | 1.321 | 1.319 | 1.233 | 1.321 |
| Area variation (F) | 0.189 | 0.146 | 0.299 | 0.300 | 0.225 | 0.299 |
| Hausdorff, ‰ of the diagonal (H) | 0.60 | 0.79 | 0.35 | 0.35 | 0.69 | 0.34 |
| Mean distance, ‰ (H) | 0.109 | 0.178 | 0.104 | 0.104 | 0.222 | 0.108 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.82 | 99.67 | 100.01 | 100.01 | 99.56 | 100.01 |
| Edges on closed loops, % (L) | 0.0 | 100.0 | — | 100.0 | 9.9 | — |
| Edges on spiralling loops, % (L) | 38.2 | 0.0 | — | 0.0 | 69.8 | — |
| Spiralling loops (L) | 8 | 0 | — | 0 | 14 | — |
| Edges on loops that stop at an irregular vertex, % (L) | 61.8 | 0.0 | — | 0.0 | 20.3 | — |
| Longest loop that stops at an irregular vertex, edges (L) | 1065 | 0 | — | 0 | 450 | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | 0.0 | 0.0 | — |
| Off budget, % (N) | 14.3 | 5.7 | 0.3 | 0.3 | 7.5 | 0.9 |
| Time, seconds (T) | 5.42 | 2.20 | 4.21 | 5.21 | 6.20 | 4.39 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 7 | 5 | 2 | 1 | 2 |
| Base complex patches (C) | 1 | 2 | 4 | 3 | 1 | 2 |
| Alignment, degrees (A) | 0 | 0 | 1 | 0 | 8 | 4 |
| Mean angle deviation, degrees (F) | 0 | 1 | 2 | 3 | 1 | 6 |
| Mean aspect ratio (F) | 9 | 0 | 1 | 2 | 0 | 1 |
| Valid meshes (V) | 33/65 | 4/13 | 12/13 | 12/13 | 11/13 | 12/13 |

### mechanical (4 models)

| Metric | quadriflow | autoremesher | openclare-uniform | openclare | quadremesher | openclare-matched |
|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 0 | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 1 | 0 | 2 | 3 | 3 | 2 |
| Alignment, degrees (A) | 0 | 0 | 0 | 1 | 2 | 1 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 1 | 2 | 1 |
| Mean aspect ratio (F) | 2 | 0 | 1 | 0 | 1 | 0 |
| Valid meshes (V) | 14/20 | 0/4 | 4/4 | 4/4 | 4/4 | 4/4 |
