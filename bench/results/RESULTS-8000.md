# Bench results

Requested budget: 8000 quads. Median over 5 seeds; Quad Remesher, AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

## blob (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2795 | 8068 | 7979 | 7949 |
| Irregular vertices (S) | 28 | 24 | 24 | 24 |
| … of which above the minimum | 20 | 16 | 16 | 16 |
| Base complex patches (C) | 1171 | 6808 | 6819 | 6789 |
| Alignment, degrees (A) | 5.92 | 5.81 | 5.46 | 5.28 |
| Mean angle deviation, degrees (F) | 9.19 | 6.29 | 5.78 | 5.52 |
| Angle deviation, 95th percentile (F) | 34.45 | 17.15 | 15.29 | 14.63 |
| Mean aspect ratio (F) | 2.101 | 1.309 | 1.290 | 1.279 |
| Area variation (F) | 0.585 | 0.340 | 0.360 | 0.358 |
| Hausdorff, ‰ of the diagonal (H) | 2.78 | 1.88 | 1.76 | 1.73 |
| Mean distance, ‰ (H) | 0.760 | 0.082 | 0.078 | 0.077 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 98.72 | 100.01 | 100.01 | 100.01 |
| Off budget, % (N) | 65.1 | 0.9 | 0.3 | 0.5 |
| Time, seconds (T) | 2.72 | 6.20 | 7.32 | 7.04 |

## rock (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 5842 | 7943 | 7940 | 7061 |
| Irregular vertices (S) | 34 | 116 | 134 | 116 |
| … of which above the minimum | 26 | 108 | 126 | 108 |
| Base complex patches (C) | 5052 | 7943 | 7940 | 7061 |
| Alignment, degrees (A) | 18.66 | 16.60 | 16.32 | 16.10 |
| Mean angle deviation, degrees (F) | 6.90 | 5.93 | 5.42 | 5.49 |
| Angle deviation, 95th percentile (F) | 17.38 | 14.01 | 13.19 | 14.12 |
| Mean aspect ratio (F) | 1.393 | 1.216 | 1.215 | 1.221 |
| Area variation (F) | 0.344 | 0.254 | 0.264 | 0.259 |
| Hausdorff, ‰ of the diagonal (H) | 3.88 | 2.87 | 2.57 | 3.35 |
| Mean distance, ‰ (H) | 0.669 | 0.395 | 0.389 | 0.420 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 99.79 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 27.0 | 0.7 | 0.8 | 0.1 |
| Time, seconds (T) | 2.72 | 5.70 | 6.32 | 6.46 |

## creature (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 6819 | 7884 | 7825 | 7168 |
| Irregular vertices (S) | 18 | 40 | 42 | 40 |
| … of which above the minimum | 10 | 32 | 34 | 32 |
| Base complex patches (C) | 3683 | 2864 | 3225 | 4316 |
| Alignment, degrees (A) | 9.77 | 6.02 | 5.63 | 5.54 |
| Mean angle deviation, degrees (F) | 3.78 | 6.71 | 5.60 | 5.25 |
| Angle deviation, 95th percentile (F) | 10.36 | 18.89 | 14.96 | 14.35 |
| Mean aspect ratio (F) | 1.965 | 1.345 | 1.322 | 1.344 |
| Area variation (F) | 0.565 | 0.364 | 0.470 | 0.477 |
| Hausdorff, ‰ of the diagonal (H) | 1.09 | 0.96 | 0.90 | 0.95 |
| Mean distance, ‰ (H) | 0.161 | 0.085 | 0.082 | 0.091 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 99.74 | 100.01 | 100.01 | 100.01 |
| Off budget, % (N) | 14.8 | 1.5 | 2.2 | 0.3 |
| Time, seconds (T) | 2.77 | 11.47 | 13.61 | 10.68 |

## cage (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 5928 | 8112 | 8057 | 11166 |
| Irregular vertices (S) | 58 | 48 | 58 | 64 |
| … of which above the minimum | — | 16 | 26 | 32 |
| Base complex patches (C) | — | 1654 | 275 | 120 |
| Alignment, degrees (A) | 5.16 | 2.47 | 1.94 | 1.91 |
| Mean angle deviation, degrees (F) | 7.40 | 2.13 | 2.26 | 2.04 |
| Angle deviation, 95th percentile (F) | 22.71 | 6.74 | 6.75 | 6.24 |
| Mean aspect ratio (F) | 2.161 | 1.195 | 1.233 | 1.194 |
| Area variation (F) | 0.487 | 0.214 | 0.325 | 0.344 |
| Hausdorff, ‰ of the diagonal (H) | 7.28 | 3.02 | 2.86 | 1.87 |
| Mean distance, ‰ (H) | 0.343 | 0.232 | 0.206 | 0.120 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 99.99 | 99.99 | 100.00 | 100.00 |
| Off budget, % (N) | 25.9 | 1.4 | 0.7 | 1.4 |
| Time, seconds (T) | 2.96 | 4.34 | 5.99 | 9.59 |

## suzanne (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2647 | 8040 | 8103 | 9380 |
| Irregular vertices (S) | 84 | 64 | 82 | 86 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | 3543 | 4783 | 5997 |
| Alignment, degrees (A) | 12.11 | 10.00 | 9.80 | 9.40 |
| Mean angle deviation, degrees (F) | 9.37 | 5.45 | 5.99 | 5.96 |
| Angle deviation, 95th percentile (F) | 32.08 | 15.25 | 15.22 | 15.65 |
| Mean aspect ratio (F) | 1.892 | 1.211 | 1.231 | 1.243 |
| Area variation (F) | 0.650 | 0.251 | 0.377 | 0.363 |
| Hausdorff, ‰ of the diagonal (H) | 31.86 | 4.31 | 2.96 | 2.59 |
| Mean distance, ‰ (H) | 1.112 | 0.228 | 0.179 | 0.156 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 66.9 | 0.5 | 1.3 | 1.4 |
| Time, seconds (T) | 1.58 | 6.04 | 7.59 | 7.85 |

## ogre (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 4622 | 8009 | 7997 | 12532 |
| Irregular vertices (S) | 141 | 202 | 240 | 278 |
| … of which above the minimum | — | 194 | 232 | 270 |
| Base complex patches (C) | — | 5239 | 6131 | 9054 |
| Alignment, degrees (A) | 12.94 | 11.36 | 10.70 | 10.28 |
| Mean angle deviation, degrees (F) | 7.43 | 6.29 | 6.32 | 5.86 |
| Angle deviation, 95th percentile (F) | 22.75 | 17.34 | 16.91 | 15.78 |
| Mean aspect ratio (F) | 1.812 | 1.372 | 1.377 | 1.356 |
| Area variation (F) | 0.668 | 0.331 | 0.432 | 0.436 |
| Hausdorff, ‰ of the diagonal (H) | 27.93 | 11.29 | 8.51 | 4.50 |
| Mean distance, ‰ (H) | 1.054 | 0.323 | 0.297 | 0.208 |
| Thin parts: mean distance, % of a quad edge (V) | 78.5 | 12.7 | 4.1 | 2.5 |
| Volume kept, % of the source (V) | 98.66 | 99.99 | 100.00 | 100.01 |
| Off budget, % (N) | 42.2 | 0.1 | 0.0 | 1.5 |
| Time, seconds (T) | 3.68 | 10.98 | 12.03 | 18.23 |

## spot (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2895 | 7967 | 8093 | 8007 |
| Irregular vertices (S) | 74 | 80 | 78 | 78 |
| … of which above the minimum | — | 72 | 70 | 70 |
| Base complex patches (C) | — | 5329 | 5007 | 5259 |
| Alignment, degrees (A) | 9.88 | 7.93 | 7.55 | 7.36 |
| Mean angle deviation, degrees (F) | 7.56 | 6.47 | 6.49 | 6.09 |
| Angle deviation, 95th percentile (F) | 20.17 | 16.91 | 17.44 | 15.73 |
| Mean aspect ratio (F) | 1.481 | 1.326 | 1.331 | 1.339 |
| Area variation (F) | 0.712 | 0.330 | 0.429 | 0.429 |
| Hausdorff, ‰ of the diagonal (H) | 6.25 | 2.24 | 2.06 | 2.11 |
| Mean distance, ‰ (H) | 0.635 | 0.082 | 0.078 | 0.078 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 98.89 | 100.01 | 100.01 | 100.01 |
| Off budget, % (N) | 63.8 | 0.4 | 1.2 | 0.5 |
| Time, seconds (T) | 2.19 | 11.30 | 13.16 | 11.73 |

## bob (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 4012 | 7968 | 8014 | 6770 |
| Irregular vertices (S) | 20 | 20 | 20 | 20 |
| … of which above the minimum | 20 | 20 | 20 | 20 |
| Base complex patches (C) | 1318 | 1279 | 2898 | 2729 |
| Alignment, degrees (A) | 5.28 | 3.64 | 3.75 | 3.65 |
| Mean angle deviation, degrees (F) | 4.91 | 3.64 | 3.76 | 3.62 |
| Angle deviation, 95th percentile (F) | 13.45 | 11.19 | 11.90 | 10.92 |
| Mean aspect ratio (F) | 2.024 | 1.402 | 1.380 | 1.387 |
| Area variation (F) | 0.485 | 0.374 | 0.433 | 0.416 |
| Hausdorff, ‰ of the diagonal (H) | 1.62 | 0.55 | 0.96 | 0.99 |
| Mean distance, ‰ (H) | 0.346 | 0.056 | 0.056 | 0.064 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 99.32 | 100.01 | 100.01 | 100.01 |
| Off budget, % (N) | 49.9 | 0.4 | 0.2 | 1.8 |
| Time, seconds (T) | 2.11 | 8.92 | 7.45 | 6.95 |

## blub (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 3438 | 7990 | 7994 | 8369 |
| Irregular vertices (S) | 53 | 48 | 56 | 56 |
| … of which above the minimum | — | 40 | 48 | 48 |
| Base complex patches (C) | — | 3338 | 4424 | 2721 |
| Alignment, degrees (A) | 7.01 | 6.93 | 6.43 | 6.60 |
| Mean angle deviation, degrees (F) | 6.30 | 6.56 | 6.11 | 6.28 |
| Angle deviation, 95th percentile (F) | 19.86 | 18.56 | 16.87 | 17.19 |
| Mean aspect ratio (F) | 2.320 | 1.340 | 1.351 | 1.339 |
| Area variation (F) | 0.598 | 0.297 | 0.394 | 0.395 |
| Hausdorff, ‰ of the diagonal (H) | 8.37 | 3.62 | 2.36 | 2.30 |
| Mean distance, ‰ (H) | 0.399 | 0.071 | 0.059 | 0.057 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 99.18 | 99.99 | 100.00 | 100.00 |
| Off budget, % (N) | 57.0 | 0.1 | 0.1 | 1.7 |
| Time, seconds (T) | 2.53 | 7.95 | 8.84 | 9.48 |

## pig (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 4682 | 8093 | 8108 | 9900 |
| Irregular vertices (S) | 135 | 88 | 98 | 98 |
| … of which above the minimum | — | 80 | 90 | 90 |
| Base complex patches (C) | — | 5985 | 5376 | 6168 |
| Alignment, degrees (A) | 11.22 | 9.47 | 8.20 | 8.48 |
| Mean angle deviation, degrees (F) | 6.80 | 7.06 | 5.68 | 5.99 |
| Angle deviation, 95th percentile (F) | 21.80 | 20.08 | 17.10 | 18.61 |
| Mean aspect ratio (F) | 1.773 | 1.342 | 1.326 | 1.328 |
| Area variation (F) | 0.668 | 0.333 | 0.494 | 0.499 |
| Hausdorff, ‰ of the diagonal (H) | 9.84 | 7.22 | 6.09 | 6.69 |
| Mean distance, ‰ (H) | 0.627 | 0.142 | 0.118 | 0.100 |
| Thin parts: mean distance, % of a quad edge (V) | 21.7 | 15.9 | 5.9 | 4.7 |
| Volume kept, % of the source (V) | 99.18 | 99.99 | 100.00 | 100.00 |
| Off budget, % (N) | 41.5 | 1.2 | 1.4 | 0.8 |
| Time, seconds (T) | 3.96 | 7.71 | 12.03 | 14.26 |

## jerry (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2740 | 7903 | 7951 | 13817 |
| Irregular vertices (S) | 109 | 168 | 279 | 355 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | 5458 | 6820 | 11799 |
| Alignment, degrees (A) | 15.73 | 12.91 | 12.37 | 11.71 |
| Mean angle deviation, degrees (F) | 10.26 | 5.22 | 5.71 | 5.90 |
| Angle deviation, 95th percentile (F) | 29.98 | 14.50 | 16.65 | 15.73 |
| Mean aspect ratio (F) | 1.889 | 1.249 | 1.266 | 1.284 |
| Area variation (F) | 0.652 | 0.313 | 0.526 | 0.462 |
| Hausdorff, ‰ of the diagonal (H) | 25.08 | 15.26 | 10.48 | 10.37 |
| Mean distance, ‰ (H) | 2.010 | 0.359 | 0.279 | 0.170 |
| Thin parts: mean distance, % of a quad edge (V) | 54.4 | 23.3 | 9.6 | 11.0 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 65.8 | 1.2 | 0.6 | 0.8 |
| Time, seconds (T) | 2.87 | 5.59 | 7.68 | 15.47 |

## rose (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 3828 | 8296 | 8343 | 53886 |
| Irregular vertices (S) | 556 | 1222 | 1643 | 2544 |
| … of which above the minimum | — | 606 | 1027 | 1928 |
| Base complex patches (C) | — | 6769 | 6482 | 43115 |
| Alignment, degrees (A) | 18.28 | 12.75 | 12.50 | 9.34 |
| Mean angle deviation, degrees (F) | 17.84 | 12.15 | 13.46 | 6.23 |
| Angle deviation, 95th percentile (F) | 56.33 | 44.68 | 41.03 | 19.22 |
| Mean aspect ratio (F) | 3.415 | 3.302 | 2.138 | 1.365 |
| Area variation (F) | 0.904 | 0.629 | 1.432 | 0.474 |
| Hausdorff, ‰ of the diagonal (H) | 58.87 | 16.29 | 15.48 | 7.95 |
| Mean distance, ‰ (H) | 4.497 | 0.678 | 0.911 | 0.113 |
| Thin parts: mean distance, % of a quad edge (V) | 83.6 | 13.1 | 12.2 | 10.4 |
| Volume kept, % of the source (V) | — | 97.30 | 95.76 | 99.78 |
| Off budget, % (N) | 52.1 | 3.7 | 4.3 | 0.0 |
| Time, seconds (T) | 3.15 | 3.05 | 7.61 | 19.58 |

## bridge (organic)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 4954 | 7880 | 8109 | 8799 |
| Irregular vertices (S) | 58 | 70 | 81 | 89 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | 2987 | 2725 | 3637 |
| Alignment, degrees (A) | 10.49 | 9.87 | 9.03 | 9.22 |
| Mean angle deviation, degrees (F) | 6.02 | 4.82 | 4.66 | 4.39 |
| Angle deviation, 95th percentile (F) | 18.67 | 12.45 | 12.17 | 12.50 |
| Mean aspect ratio (F) | 1.719 | 1.271 | 1.263 | 1.274 |
| Area variation (F) | 0.719 | 0.264 | 0.329 | 0.363 |
| Hausdorff, ‰ of the diagonal (H) | 8.89 | 6.83 | 6.75 | 7.19 |
| Mean distance, ‰ (H) | 0.521 | 0.220 | 0.215 | 0.202 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 38.1 | 1.5 | 1.4 | 0.1 |
| Time, seconds (T) | 3.19 | 6.11 | 7.15 | 8.18 |

## part (mechanical)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 4954 | 8078 | 8098 | 7279 |
| Irregular vertices (S) | 94 | 24 | 24 | 24 |
| … of which above the minimum | — | 16 | 16 | 16 |
| Base complex patches (C) | — | 429 | 92 | 316 |
| Alignment, degrees (A) | 14.04 | 11.83 | 10.32 | 10.48 |
| Mean angle deviation, degrees (F) | 13.77 | 1.25 | 1.00 | 1.27 |
| Angle deviation, 95th percentile (F) | 47.48 | 3.80 | 3.80 | 4.15 |
| Mean aspect ratio (F) | 3.591 | 1.093 | 1.129 | 1.116 |
| Area variation (F) | 0.912 | 0.126 | 0.328 | 0.339 |
| Hausdorff, ‰ of the diagonal (H) | 11.34 | 0.31 | 0.25 | 0.27 |
| Mean distance, ‰ (H) | 0.506 | 0.022 | 0.024 | 0.026 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | — | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 38.1 | 1.0 | 1.2 | 1.0 |
| Time, seconds (T) | 3.16 | 6.05 | 6.20 | 5.62 |

## cube (mechanical)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2774 | 8066 | 8066 | 7776 |
| Irregular vertices (S) | 10 | 8 | 8 | 8 |
| … of which above the minimum | — | 0 | 0 | 0 |
| Base complex patches (C) | — | 6 | 6 | 6 |
| Alignment, degrees (A) | 2.60 | 0.11 | 0.27 | 0.28 |
| Mean angle deviation, degrees (F) | 5.97 | 0.01 | 0.20 | 0.21 |
| Angle deviation, 95th percentile (F) | 24.98 | 0.02 | 0.74 | 0.76 |
| Mean aspect ratio (F) | 2.866 | 1.018 | 1.074 | 1.066 |
| Area variation (F) | 0.740 | 0.013 | 0.093 | 0.091 |
| Hausdorff, ‰ of the diagonal (H) | 4.81 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.165 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 99.99 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 65.3 | 0.8 | 0.8 | 1.8 |
| Time, seconds (T) | 1.34 | 4.51 | 5.49 | 4.67 |

## lblock (mechanical)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 3035 | 8064 | 8064 | 8064 |
| Irregular vertices (S) | 20 | 12 | 12 | 12 |
| … of which above the minimum | — | 4 | 4 | 4 |
| Base complex patches (C) | — | 14 | 14 | 14 |
| Alignment, degrees (A) | 3.67 | 0.61 | 0.92 | 0.92 |
| Mean angle deviation, degrees (F) | 6.33 | 0.00 | 0.49 | 0.49 |
| Angle deviation, 95th percentile (F) | 24.33 | 0.01 | 1.49 | 1.49 |
| Mean aspect ratio (F) | 2.553 | 1.000 | 1.078 | 1.078 |
| Area variation (F) | 0.668 | 0.000 | 0.096 | 0.096 |
| Hausdorff, ‰ of the diagonal (H) | 5.67 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.155 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 99.97 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 62.1 | 0.8 | 0.8 | 0.0 |
| Time, seconds (T) | 1.61 | 4.20 | 4.28 | 4.54 |

## cylinder (mechanical)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2565 | 7980 | 7899 | 7007 |
| Irregular vertices (S) | 10 | 8 | 8 | 8 |
| … of which above the minimum | — | 0 | 0 | 0 |
| Base complex patches (C) | — | 1452 | 1399 | 1247 |
| Alignment, degrees (A) | 4.10 | 1.32 | 1.28 | 1.25 |
| Mean angle deviation, degrees (F) | 6.60 | 2.31 | 2.33 | 2.23 |
| Angle deviation, 95th percentile (F) | 20.66 | 10.22 | 10.46 | 9.61 |
| Mean aspect ratio (F) | 2.021 | 1.288 | 1.398 | 1.369 |
| Area variation (F) | 0.636 | 0.296 | 0.213 | 0.191 |
| Hausdorff, ‰ of the diagonal (H) | 5.39 | 0.54 | 0.55 | 0.58 |
| Mean distance, ‰ (H) | 0.289 | 0.066 | 0.072 | 0.075 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 99.70 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 67.9 | 0.2 | 1.3 | 1.2 |
| Time, seconds (T) | 1.38 | 4.66 | 5.16 | 3.85 |

## venturi (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | 7212 | 7574 | 8376 | 19050 |
| Irregular vertices (S) | 2158 | 283 | 884 | 736 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | 5218 | 6123 | 14291 |
| Alignment, degrees (A) | 10.19 | 10.32 | 11.72 | 9.61 |
| Mean angle deviation, degrees (F) | 40.97 | 6.37 | 8.83 | 5.76 |
| Angle deviation, 95th percentile (F) | 89.08 | 21.47 | 32.44 | 19.44 |
| Mean aspect ratio (F) | 9.234 | 1.293 | 1.864 | 1.336 |
| Area variation (F) | 2.203 | 0.299 | 0.436 | 0.296 |
| Hausdorff, ‰ of the diagonal (H) | 37.63 | 36.17 | 36.17 | 15.92 |
| Mean distance, ‰ (H) | 2.866 | 2.139 | 2.025 | 0.870 |
| Thin parts: mean distance, % of a quad edge (V) | 28.8 | 28.1 | 26.0 | 25.0 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 9.8 | 5.3 | 4.7 | 3.3 |
| Time, seconds (T) | 2.30 | 50.73 | 36.28 | 97.65 |

## ogre_hp (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 4422 | 8133 | 7952 | 13743 |
| Irregular vertices (S) | 172 | 198 | 354 | 432 |
| … of which above the minimum | — | 190 | 346 | 424 |
| Base complex patches (C) | — | 6567 | 6122 | 11695 |
| Alignment, degrees (A) | 13.99 | 12.05 | 10.05 | 9.69 |
| Mean angle deviation, degrees (F) | 7.16 | 6.00 | 6.61 | 6.08 |
| Angle deviation, 95th percentile (F) | 21.73 | 16.64 | 19.28 | 17.35 |
| Mean aspect ratio (F) | 1.812 | 1.390 | 1.381 | 1.344 |
| Area variation (F) | 0.657 | 0.353 | 0.968 | 1.030 |
| Hausdorff, ‰ of the diagonal (H) | 22.79 | 10.14 | 7.19 | 6.66 |
| Mean distance, ‰ (H) | 1.109 | 0.316 | 0.336 | 0.222 |
| Thin parts: mean distance, % of a quad edge (V) | 44.3 | 9.3 | 1.4 | 1.4 |
| Volume kept, % of the source (V) | — | 100.00 | 100.01 | 100.01 |
| Off budget, % (N) | 44.7 | 1.7 | 0.6 | 0.4 |
| Time, seconds (T) | 16.88 | 58.59 | 62.94 | 64.78 |

## watermill (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 2507 | 6743 | 6739 | — |
| Irregular vertices (S) | 428 | 423 | 423 | — |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | — | — | — |
| Alignment, degrees (A) | 16.69 | 12.95 | 12.99 | — |
| Mean angle deviation, degrees (F) | 21.18 | 8.09 | 8.20 | — |
| Angle deviation, 95th percentile (F) | 68.70 | 28.49 | 29.38 | — |
| Mean aspect ratio (F) | 2.614 | 1.332 | 1.340 | — |
| Area variation (F) | 0.825 | 0.277 | 0.277 | — |
| Hausdorff, ‰ of the diagonal (H) | 90.14 | 53.45 | 53.45 | — |
| Mean distance, ‰ (H) | 7.482 | 2.383 | 2.383 | — |
| Thin parts: mean distance, % of a quad edge (V) | 69.7 | 32.9 | 32.9 | — |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 68.7 | 15.7 | 15.8 | — |
| Time, seconds (T) | 5.22 | 91.77 | 88.71 | — |

Failures: openclare-matched: no Quad Remesher reference to match.

## towermill (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 2664 | 6213 | — | — |
| Irregular vertices (S) | 292 | 295 | — | — |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | — | — | — |
| Alignment, degrees (A) | 14.32 | 11.00 | — | — |
| Mean angle deviation, degrees (F) | 18.29 | 8.57 | — | — |
| Angle deviation, 95th percentile (F) | 60.76 | 28.53 | — | — |
| Mean aspect ratio (F) | 2.412 | 1.381 | — | — |
| Area variation (F) | 0.707 | 0.314 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 60.39 | 38.98 | — | — |
| Mean distance, ‰ (H) | 4.925 | 2.188 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 39.9 | 27.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 66.7 | 22.3 | — | — |
| Time, seconds (T) | 3.69 | 99.95 | — | — |

Failures: openclare: timed out after 1800 s; openclare-matched: no Quad Remesher reference to match.

## stonemill (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 0/1 |
| Quads made | 4327 | 7867 | 8182 | — |
| Irregular vertices (S) | 121 | 190 | 280 | — |
| … of which above the minimum | — | 126 | 216 | — |
| Base complex patches (C) | — | 6331 | 7587 | — |
| Alignment, degrees (A) | 13.65 | 11.27 | 11.28 | — |
| Mean angle deviation, degrees (F) | 10.72 | 4.40 | 5.26 | — |
| Angle deviation, 95th percentile (F) | 31.96 | 14.37 | 17.03 | — |
| Mean aspect ratio (F) | 1.880 | 1.196 | 1.263 | — |
| Area variation (F) | 0.625 | 0.215 | 0.357 | — |
| Hausdorff, ‰ of the diagonal (H) | 58.87 | 11.27 | 14.15 | — |
| Mean distance, ‰ (H) | 2.121 | 1.074 | 1.019 | — |
| Thin parts: mean distance, % of a quad edge (V) | 185.8 | 22.9 | 22.5 | — |
| Volume kept, % of the source (V) | 99.45 | 100.01 | 100.02 | — |
| Off budget, % (N) | 45.9 | 1.7 | 2.3 | — |
| Time, seconds (T) | 2.66 | 8.45 | 10.03 | — |

Failures: openclare-matched: no Quad Remesher reference to match.

## windmill (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 1/1 | 0/1 |
| Quads made | 2364 | 263 | 439 | — |
| Irregular vertices (S) | 234 | 36 | 116 | — |
| … of which above the minimum | — | 12 | 92 | — |
| Base complex patches (C) | — | 239 | 439 | — |
| Alignment, degrees (A) | 13.30 | 17.78 | 18.87 | — |
| Mean angle deviation, degrees (F) | 14.00 | 29.82 | 21.80 | — |
| Angle deviation, 95th percentile (F) | 51.31 | 69.36 | 56.93 | — |
| Mean aspect ratio (F) | 2.278 | 7.843 | 2.886 | — |
| Area variation (F) | 0.698 | 1.367 | 0.901 | — |
| Hausdorff, ‰ of the diagonal (H) | 93.86 | 531.90 | 529.94 | — |
| Mean distance, ‰ (H) | 6.515 | 244.649 | 232.732 | — |
| Thin parts: mean distance, % of a quad edge (V) | 80.5 | 1851.6 | 1842.0 | — |
| Volume kept, % of the source (V) | — | 1.51 | 1.58 | — |
| Off budget, % (N) | 70.5 | 96.7 | 94.5 | — |
| Time, seconds (T) | 2.99 | 2.65 | 4.73 | — |

Failures: openclare-matched: no Quad Remesher reference to match.

## terrain (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2975 | 7941 | 8014 | 11855 |
| Irregular vertices (S) | 108 | 124 | 229 | 315 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | 5508 | 7314 | 10547 |
| Alignment, degrees (A) | 20.95 | 17.72 | 17.58 | 17.46 |
| Mean angle deviation, degrees (F) | 7.30 | 6.86 | 6.88 | 6.37 |
| Angle deviation, 95th percentile (F) | 26.48 | 18.80 | 18.66 | 16.76 |
| Mean aspect ratio (F) | 1.481 | 1.299 | 1.301 | 1.259 |
| Area variation (F) | 0.463 | 0.322 | 0.475 | 0.499 |
| Hausdorff, ‰ of the diagonal (H) | 26.34 | 11.35 | 8.75 | 8.30 |
| Mean distance, ‰ (H) | 1.562 | 0.744 | 0.629 | 0.503 |
| Thin parts: mean distance, % of a quad edge (V) | 47.6 | 23.6 | 17.9 | 19.2 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 62.8 | 0.7 | 0.2 | 0.3 |
| Time, seconds (T) | 4.30 | 11.42 | 13.26 | 15.75 |

## mahaveer (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 2996 | 6472 | 7890 | 53461 |
| Irregular vertices (S) | 419 | 533 | 567 | 1198 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | — | — | — |
| Alignment, degrees (A) | 9.27 | 4.91 | 4.84 | 5.06 |
| Mean angle deviation, degrees (F) | 19.93 | 9.89 | 8.25 | 5.63 |
| Angle deviation, 95th percentile (F) | 77.95 | 64.66 | 51.60 | 20.99 |
| Mean aspect ratio (F) | 3.957 | 54.906 | 64.831 | 7.246 |
| Area variation (F) | 1.337 | 3.121 | 1.751 | 0.912 |
| Hausdorff, ‰ of the diagonal (H) | 36.28 | 32.27 | 27.00 | 13.39 |
| Mean distance, ‰ (H) | 3.494 | 2.389 | 2.010 | 0.609 |
| Thin parts: mean distance, % of a quad edge (V) | 52.7 | 34.4 | 31.1 | 41.2 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 62.5 | 19.1 | 1.4 | 3.1 |
| Time, seconds (T) | 2.39 | 9.67 | 8.38 | 49.18 |

## marine (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 2388 | 12356 | 12000 | 231978 |
| Irregular vertices (S) | 1056 | 6142 | 6226 | 11799 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | — | — | — |
| Alignment, degrees (A) | 15.08 | 11.75 | 11.91 | 8.36 |
| Mean angle deviation, degrees (F) | 28.51 | 17.41 | 17.91 | 4.92 |
| Angle deviation, 95th percentile (F) | 82.76 | 62.96 | 63.61 | 18.22 |
| Mean aspect ratio (F) | 7.658 | 6.373 | 3.800 | 1.326 |
| Area variation (F) | 1.726 | 2.331 | 2.431 | 0.533 |
| Hausdorff, ‰ of the diagonal (H) | 6.92 | 3.85 | 3.85 | 3.02 |
| Mean distance, ‰ (H) | 1.765 | 0.309 | 0.330 | 0.033 |
| Thin parts: mean distance, % of a quad edge (V) | 94.0 | 15.2 | 16.1 | 19.1 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 70.2 | 54.4 | 50.0 | 5.7 |
| Time, seconds (T) | 1.38 | 10.56 | 15.92 | 44.98 |

## f16 (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 6227 | 9469 | 9512 | 43843 |
| Irregular vertices (S) | 2171 | 1840 | 1786 | 2032 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | — | — | — |
| Alignment, degrees (A) | 10.29 | 5.49 | 4.86 | 3.96 |
| Mean angle deviation, degrees (F) | 29.31 | 11.57 | 10.60 | 6.20 |
| Angle deviation, 95th percentile (F) | 84.69 | 58.92 | 59.08 | 23.11 |
| Mean aspect ratio (F) | 6.307 | 55.746 | 55.011 | 16.496 |
| Area variation (F) | 1.369 | 1.656 | 1.803 | 0.765 |
| Hausdorff, ‰ of the diagonal (H) | 80.26 | 18.18 | 23.76 | 12.17 |
| Mean distance, ‰ (H) | 5.085 | 0.629 | 0.640 | 0.214 |
| Thin parts: mean distance, % of a quad edge (V) | 60.6 | 7.4 | 7.8 | 8.0 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 22.2 | 18.4 | 18.9 | 5.1 |
| Time, seconds (T) | 5.92 | 10.54 | 10.16 | 85.72 |

## camera (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 4114 | 7811 | 8042 | 19734 |
| Irregular vertices (S) | 403 | 222 | 257 | 295 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | — | — | — |
| Alignment, degrees (A) | 8.73 | 3.30 | 3.42 | 3.31 |
| Mean angle deviation, degrees (F) | 18.87 | 13.22 | 14.02 | 8.39 |
| Angle deviation, 95th percentile (F) | 74.17 | 71.94 | 72.22 | 47.61 |
| Mean aspect ratio (F) | 3.158 | 17.277 | 16.284 | 5.441 |
| Area variation (F) | 1.607 | 1.028 | 1.095 | 0.761 |
| Hausdorff, ‰ of the diagonal (H) | 58.33 | 32.96 | 32.96 | 20.02 |
| Mean distance, ‰ (H) | 3.579 | 1.025 | 0.991 | 0.342 |
| Thin parts: mean distance, % of a quad edge (V) | 32.4 | 11.2 | 11.0 | 9.8 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 48.6 | 2.4 | 0.5 | 2.9 |
| Time, seconds (T) | 1.98 | 10.38 | 12.01 | 21.54 |

## wolf (extra)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 2119 | 6760 | 6901 | 25241 |
| Irregular vertices (S) | 195 | 417 | 457 | 727 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | — | — | — |
| Alignment, degrees (A) | 13.19 | 10.31 | 11.06 | 11.35 |
| Mean angle deviation, degrees (F) | 10.86 | 9.34 | 9.86 | 8.62 |
| Angle deviation, 95th percentile (F) | 32.41 | 26.93 | 28.71 | 25.31 |
| Mean aspect ratio (F) | 1.955 | 1.458 | 1.466 | 1.343 |
| Area variation (F) | 0.849 | 0.311 | 0.344 | 0.305 |
| Hausdorff, ‰ of the diagonal (H) | 36.12 | 12.56 | 12.56 | 8.60 |
| Mean distance, ‰ (H) | 3.139 | 0.688 | 0.677 | 0.207 |
| Thin parts: mean distance, % of a quad edge (V) | 103.8 | 34.0 | 33.4 | 39.9 |
| Volume kept, % of the source (V) | — | — | — | — |
| Off budget, % (N) | 73.5 | 15.5 | 13.7 | 12.7 |
| Time, seconds (T) | 3.96 | 131.25 | 126.58 | 312.03 |

## sphere (calibration)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 6520 | 7984 | 8074 | 7223 |
| Irregular vertices (S) | 26 | 26 | 26 | 18 |
| … of which above the minimum | 18 | 18 | 18 | 10 |
| Base complex patches (C) | 6252 | 7984 | 8074 | 7223 |
| Alignment, degrees (A) | — | — | — | — |
| Mean angle deviation, degrees (F) | 5.85 | 4.34 | 4.52 | 5.94 |
| Angle deviation, 95th percentile (F) | 14.46 | 9.72 | 9.91 | 12.35 |
| Mean aspect ratio (F) | 1.337 | 1.130 | 1.134 | 1.198 |
| Area variation (F) | 0.283 | 0.174 | 0.168 | 0.198 |
| Hausdorff, ‰ of the diagonal (H) | 0.80 | 0.35 | 0.36 | 0.34 |
| Mean distance, ‰ (H) | 0.172 | 0.078 | 0.078 | 0.086 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 99.83 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 18.5 | 0.2 | 0.9 | 1.9 |
| Time, seconds (T) | 3.11 | 4.72 | 6.17 | 5.35 |

## torus (calibration)

| | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7544 | 8024 | 8024 | 7467 |
| Irregular vertices (S) | 0 | 0 | 0 | 0 |
| … of which above the minimum | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 1 | 1 | 1 | 1 |
| Alignment, degrees (A) | 0.76 | 0.64 | 0.64 | 0.62 |
| Mean angle deviation, degrees (F) | 1.26 | 0.91 | 0.91 | 0.91 |
| Angle deviation, 95th percentile (F) | 2.30 | 1.42 | 1.42 | 1.47 |
| Mean aspect ratio (F) | 2.552 | 1.321 | 1.319 | 1.321 |
| Area variation (F) | 0.146 | 0.299 | 0.300 | 0.299 |
| Hausdorff, ‰ of the diagonal (H) | 0.79 | 0.35 | 0.35 | 0.34 |
| Mean distance, ‰ (H) | 0.178 | 0.104 | 0.104 | 0.108 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — |
| Volume kept, % of the source (V) | 99.67 | 100.01 | 100.01 | 100.01 |
| Off budget, % (N) | 5.7 | 0.3 | 0.3 | 0.9 |
| Time, seconds (T) | 3.43 | 4.21 | 4.72 | 4.39 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| Irregular vertices (S) | 8 | 6 | 2 | 2 |
| Base complex patches (C) | 2 | 5 | 4 | 2 |
| Alignment, degrees (A) | 0 | 1 | 3 | 9 |
| Mean angle deviation, degrees (F) | 1 | 2 | 3 | 7 |
| Mean aspect ratio (F) | 0 | 3 | 5 | 5 |
| Valid meshes (V) | 4/13 | 12/13 | 12/13 | 12/13 |

### mechanical (4 models)

| Metric | autoremesher | openclare-uniform | openclare | openclare-matched |
|---|---|---|---|---|
| Irregular vertices (S) | 0 | 4 | 4 | 4 |
| Base complex patches (C) | 0 | 2 | 3 | 3 |
| Alignment, degrees (A) | 0 | 2 | 1 | 1 |
| Mean angle deviation, degrees (F) | 0 | 2 | 1 | 1 |
| Mean aspect ratio (F) | 0 | 4 | 0 | 0 |
| Valid meshes (V) | 0/4 | 4/4 | 4/4 | 4/4 |
