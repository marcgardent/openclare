# Bench results

Requested budget: 8000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

The last two columns compare at equal quad count: `quadremesher-as-made` is the mesh of `quadremesher` measured against the number of quads it has rather than the budget, and `openclare-matched` is `openclare` asked for that same number.

## blob (organic)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 8164 | 7124 | 8068 | 7979 | 7911 | 7035 | 7911 | 7949 |
| Irregular vertices (S) | 277 | 24 | 24 | 24 | 24 | 24 | 24 | 24 |
| … of which above the minimum | 269 | 16 | 16 | 16 | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 8164 | 5660 | 6808 | 6819 | 4563 | 2919 | 4563 | 6789 |
| Alignment, degrees (A) | 15.05 | 5.88 | 5.81 | 5.46 | 4.08 | 4.45 | 4.08 | 5.28 |
| Mean angle deviation, degrees (F) | 4.85 | 7.41 | 6.29 | 5.78 | 5.52 | 5.55 | 5.52 | 5.52 |
| Angle deviation, 95th percentile (F) | 15.77 | 19.73 | 17.15 | 15.29 | 14.94 | 14.71 | 14.94 | 14.63 |
| Mean aspect ratio (F) | 1.142 | 1.317 | 1.309 | 1.290 | 1.414 | 1.323 | 1.414 | 1.279 |
| Area variation (F) | 0.153 | 0.316 | 0.340 | 0.360 | 0.395 | 0.331 | 0.395 | 0.358 |
| Hausdorff, ‰ of the diagonal (H) | 4.97 | 2.41 | 1.88 | 1.76 | 3.00 | 3.66 | 3.00 | 1.73 |
| Mean distance, ‰ (H) | 0.178 | 0.097 | 0.082 | 0.078 | 0.185 | 0.216 | 0.185 | 0.077 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.86 | 100.01 | 100.01 | 100.01 | 99.77 | 99.81 | 99.77 | 100.01 |
| Off budget, % (N) | 2.1 | 10.9 | 0.9 | 0.3 | 1.1 | 12.1 | 0.0 | 0.5 |
| Time, seconds (T) | 1.85 | 10.94 | 5.68 | 6.88 | 12.37 | 12.61 | 12.37 | 8.32 |

## rock (organic)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7792 | 7393 | 7943 | 7940 | 7071 | 6767 | 7071 | 7061 |
| Irregular vertices (S) | 162 | 114 | 116 | 134 | 68 | 68 | 68 | 116 |
| … of which above the minimum | 154 | 106 | 108 | 126 | 60 | 60 | 60 | 108 |
| Base complex patches (C) | 7792 | 7393 | 7943 | 7940 | 7071 | 6767 | 7071 | 7061 |
| Alignment, degrees (A) | 22.82 | 16.04 | 16.60 | 16.32 | 17.28 | 17.32 | 16.72 | 16.10 |
| Mean angle deviation, degrees (F) | 4.13 | 9.24 | 5.93 | 5.42 | 9.80 | 9.89 | 9.80 | 5.49 |
| Angle deviation, 95th percentile (F) | 12.43 | 22.12 | 14.01 | 13.19 | 23.73 | 23.67 | 23.73 | 14.12 |
| Mean aspect ratio (F) | 1.115 | 1.246 | 1.216 | 1.215 | 1.350 | 1.340 | 1.350 | 1.221 |
| Area variation (F) | 0.128 | 0.204 | 0.254 | 0.264 | 0.287 | 0.281 | 0.287 | 0.259 |
| Hausdorff, ‰ of the diagonal (H) | 3.22 | 3.64 | 2.87 | 2.57 | 4.18 | 4.36 | 4.18 | 3.35 |
| Mean distance, ‰ (H) | 0.433 | 0.418 | 0.395 | 0.389 | 0.515 | 0.530 | 0.515 | 0.420 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.89 | 100.00 | 100.00 | 100.00 | 99.83 | 99.83 | 99.83 | 100.00 |
| Off budget, % (N) | 2.6 | 7.6 | 0.7 | 0.8 | 11.6 | 15.4 | 0.0 | 0.1 |
| Time, seconds (T) | 1.45 | 33.90 | 6.40 | 9.44 | 18.31 | 18.00 | 18.31 | 6.35 |

## creature (organic)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 8234 | 6922 | 7884 | 7825 | 7186 | 7207 | 7186 | 7168 |
| Irregular vertices (S) | 214 | 40 | 40 | 42 | 40 | 40 | 40 | 40 |
| … of which above the minimum | 206 | — | 32 | 34 | 32 | 32 | 32 | 32 |
| Base complex patches (C) | 7510 | 2490 | 2864 | 3225 | 3236 | 3467 | 3236 | 4316 |
| Alignment, degrees (A) | 11.81 | 6.40 | 6.02 | 5.63 | 6.64 | 6.66 | 6.64 | 5.54 |
| Mean angle deviation, degrees (F) | 3.91 | 8.75 | 6.71 | 5.60 | 6.13 | 6.16 | 6.13 | 5.25 |
| Angle deviation, 95th percentile (F) | 12.20 | 25.50 | 18.89 | 14.96 | 17.40 | 17.67 | 17.40 | 14.35 |
| Mean aspect ratio (F) | 1.116 | 1.352 | 1.345 | 1.322 | 1.277 | 1.278 | 1.277 | 1.344 |
| Area variation (F) | 0.127 | 0.334 | 0.364 | 0.470 | 0.277 | 0.275 | 0.277 | 0.477 |
| Hausdorff, ‰ of the diagonal (H) | 1.70 | 2.35 | 0.96 | 0.90 | 1.24 | 1.34 | 1.24 | 0.95 |
| Mean distance, ‰ (H) | 0.106 | 0.099 | 0.085 | 0.082 | 0.144 | 0.145 | 0.144 | 0.091 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.81 | — | 100.01 | 100.01 | 99.72 | 99.71 | 99.72 | 100.01 |
| Off budget, % (N) | 2.9 | 13.5 | 1.5 | 2.2 | 10.2 | 9.9 | 0.0 | 0.3 |
| Time, seconds (T) | 1.55 | 24.32 | 10.48 | 12.66 | 12.49 | 8.52 | 12.49 | 9.12 |

## cage (organic)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 8050 | 7765 | 8112 | 8057 | 11009 | 7510 | 11009 | 11166 |
| Irregular vertices (S) | 250 | 40 | 48 | 58 | 62 | 64 | 62 | 64 |
| … of which above the minimum | 218 | 8 | 16 | 26 | 30 | 32 | 30 | 32 |
| Base complex patches (C) | 7125 | 766 | 1654 | 275 | 3227 | 1954 | 3227 | 120 |
| Alignment, degrees (A) | 8.80 | 2.71 | 2.47 | 1.94 | 2.06 | 2.29 | 2.06 | 1.91 |
| Mean angle deviation, degrees (F) | 4.40 | 3.58 | 2.13 | 2.26 | 3.22 | 3.48 | 3.22 | 2.04 |
| Angle deviation, 95th percentile (F) | 14.23 | 10.77 | 6.74 | 6.75 | 9.04 | 10.22 | 9.04 | 6.24 |
| Mean aspect ratio (F) | 1.144 | 1.200 | 1.195 | 1.233 | 1.693 | 1.289 | 1.693 | 1.194 |
| Area variation (F) | 0.138 | 0.191 | 0.214 | 0.325 | 0.338 | 0.229 | 0.338 | 0.344 |
| Hausdorff, ‰ of the diagonal (H) | 9.87 | 4.82 | 3.02 | 2.86 | 4.53 | 5.93 | 4.53 | 1.87 |
| Mean distance, ‰ (H) | 0.344 | 0.275 | 0.232 | 0.206 | 0.451 | 0.614 | 0.451 | 0.120 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.48 | 99.98 | 99.99 | 100.00 | 99.38 | 99.02 | 99.38 | 100.00 |
| Off budget, % (N) | 0.6 | 2.9 | 1.4 | 0.7 | 37.6 | 6.1 | 0.0 | 1.4 |
| Time, seconds (T) | 0.57 | 25.95 | 4.73 | 6.01 | 5.76 | 4.92 | 5.76 | 8.49 |

## suzanne (organic)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 8098 | 7376 | 8040 | 8103 | 9246 | 7317 | 9246 | 9380 |
| Irregular vertices (S) | 346 | 67 | 64 | 82 | 78 | 72 | 78 | 86 |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 7878 | 4267 | 3543 | 4783 | 4088 | 3340 | 4088 | 5997 |
| Alignment, degrees (A) | 16.64 | 10.01 | 10.00 | 9.80 | 8.58 | 9.86 | 8.58 | 9.40 |
| Mean angle deviation, degrees (F) | 5.17 | 7.60 | 5.45 | 5.99 | 7.04 | 6.91 | 7.04 | 5.96 |
| Angle deviation, 95th percentile (F) | 17.78 | 21.30 | 15.25 | 15.22 | 18.57 | 19.33 | 18.57 | 15.65 |
| Mean aspect ratio (F) | 1.169 | 1.223 | 1.211 | 1.231 | 1.470 | 1.246 | 1.470 | 1.243 |
| Area variation (F) | 0.164 | 0.218 | 0.251 | 0.377 | 0.366 | 0.248 | 0.366 | 0.363 |
| Hausdorff, ‰ of the diagonal (H) | 5.94 | 4.86 | 4.31 | 2.96 | 3.57 | 4.99 | 3.57 | 2.59 |
| Mean distance, ‰ (H) | 0.286 | 0.272 | 0.228 | 0.179 | 0.322 | 0.422 | 0.322 | 0.156 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 1.2 | 7.8 | 0.5 | 1.3 | 15.6 | 8.5 | 0.0 | 1.4 |
| Time, seconds (T) | 0.87 | 74.82 | 6.74 | 9.25 | 7.68 | 4.94 | 7.68 | 8.97 |

## ogre (organic)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 |
| Quads made | 7836 | 6967 | 8009 | 7997 | 12717 | 6761 | 12717 | 12532 |
| Irregular vertices (S) | 455 | 170 | 202 | 240 | 338 | 322 | 338 | 278 |
| … of which above the minimum | 429 | — | 194 | 232 | 330 | — | 330 | 270 |
| Base complex patches (C) | 7656 | — | 5239 | 6131 | 9903 | — | 9903 | 9054 |
| Alignment, degrees (A) | 17.72 | 11.64 | 11.36 | 10.70 | 8.50 | 9.79 | 8.88 | 10.28 |
| Mean angle deviation, degrees (F) | 6.18 | 9.04 | 6.29 | 6.32 | 7.95 | 8.39 | 7.95 | 5.86 |
| Angle deviation, 95th percentile (F) | 20.84 | 24.13 | 17.34 | 16.91 | 22.55 | 22.69 | 22.55 | 15.78 |
| Mean aspect ratio (F) | 1.225 | 1.401 | 1.372 | 1.377 | 2.025 | 1.482 | 2.025 | 1.356 |
| Area variation (F) | 0.196 | 0.284 | 0.331 | 0.432 | 0.643 | 0.364 | 0.643 | 0.436 |
| Hausdorff, ‰ of the diagonal (H) | 30.74 | 26.72 | 11.29 | 8.51 | 5.23 | 25.57 | 5.23 | 4.50 |
| Mean distance, ‰ (H) | 0.601 | 0.408 | 0.323 | 0.297 | 0.346 | 0.548 | 0.346 | 0.208 |
| Thin parts: mean distance, % of a quad edge (V) | 35.7 | 24.3 | 12.7 | 4.1 | 7.4 | 18.6 | 9.1 | 2.5 |
| Volume kept, % of the source (V) | 99.41 | 99.93 | 99.99 | 100.00 | 99.59 | 99.39 | 99.59 | 100.01 |
| Off budget, % (N) | 2.1 | 12.9 | 0.1 | 0.0 | 59.0 | 15.5 | 0.0 | 1.5 |
| Time, seconds (T) | 2.51 | 34.81 | 11.15 | 11.31 | 23.50 | 24.03 | 23.50 | 16.75 |

## spot (organic)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 8098 | 7084 | 7967 | 8093 | 7965 | 7274 | 7965 | 8007 |
| Irregular vertices (S) | 286 | 80 | 80 | 78 | 76 | 80 | 76 | 78 |
| … of which above the minimum | 278 | 72 | 72 | 70 | 68 | 72 | 68 | 70 |
| Base complex patches (C) | 7942 | 3692 | 5329 | 5007 | 4605 | 3862 | 4605 | 5259 |
| Alignment, degrees (A) | 16.86 | 8.49 | 7.93 | 7.55 | 7.56 | 8.42 | 7.56 | 7.36 |
| Mean angle deviation, degrees (F) | 4.47 | 9.07 | 6.47 | 6.49 | 5.56 | 6.01 | 5.56 | 6.09 |
| Angle deviation, 95th percentile (F) | 14.93 | 23.35 | 16.91 | 17.44 | 14.90 | 15.81 | 14.90 | 15.73 |
| Mean aspect ratio (F) | 1.136 | 1.348 | 1.326 | 1.331 | 1.342 | 1.289 | 1.342 | 1.339 |
| Area variation (F) | 0.144 | 0.296 | 0.330 | 0.429 | 0.301 | 0.256 | 0.301 | 0.429 |
| Hausdorff, ‰ of the diagonal (H) | 7.10 | 2.61 | 2.24 | 2.06 | 4.73 | 5.88 | 4.73 | 2.11 |
| Mean distance, ‰ (H) | 0.172 | 0.093 | 0.082 | 0.078 | 0.169 | 0.187 | 0.169 | 0.078 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.78 | 100.01 | 100.01 | 100.01 | 99.73 | 99.73 | 99.73 | 100.01 |
| Off budget, % (N) | 1.2 | 11.5 | 0.4 | 1.2 | 0.4 | 9.1 | 0.0 | 0.5 |
| Time, seconds (T) | 1.23 | 39.51 | 9.65 | 11.86 | 17.71 | 17.39 | 17.71 | 13.19 |

## bob (organic)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 8072 | 7052 | 7968 | 8014 | 6896 | 6832 | 6896 | 6770 |
| Irregular vertices (S) | 245 | 18 | 20 | 20 | 34 | 30 | 34 | 20 |
| … of which above the minimum | 245 | — | 20 | 20 | 34 | 30 | 34 | 20 |
| Base complex patches (C) | 8072 | 1541 | 1279 | 2898 | 4859 | 3789 | 4859 | 2729 |
| Alignment, degrees (A) | 20.15 | 4.10 | 3.64 | 3.75 | 5.48 | 5.82 | 5.46 | 3.65 |
| Mean angle deviation, degrees (F) | 4.31 | 4.94 | 3.64 | 3.76 | 6.43 | 6.86 | 6.43 | 3.62 |
| Angle deviation, 95th percentile (F) | 14.25 | 16.54 | 11.19 | 11.90 | 18.71 | 21.48 | 18.71 | 10.92 |
| Mean aspect ratio (F) | 1.124 | 1.407 | 1.402 | 1.380 | 1.407 | 1.407 | 1.407 | 1.387 |
| Area variation (F) | 0.139 | 0.356 | 0.374 | 0.433 | 0.369 | 0.373 | 0.369 | 0.416 |
| Hausdorff, ‰ of the diagonal (H) | 2.84 | 3.52 | 0.55 | 0.96 | 1.50 | 1.30 | 1.50 | 0.99 |
| Mean distance, ‰ (H) | 0.120 | 0.063 | 0.056 | 0.056 | 0.150 | 0.153 | 0.150 | 0.064 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.79 | — | 100.01 | 100.01 | 99.70 | 99.70 | 99.70 | 100.01 |
| Off budget, % (N) | 0.9 | 11.8 | 0.4 | 0.2 | 13.8 | 14.6 | 0.0 | 1.8 |
| Time, seconds (T) | 1.37 | 16.12 | 9.20 | 8.78 | 20.38 | 16.98 | 20.38 | 6.27 |

## blub (organic)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 8100 | 7057 | 7990 | 7994 | 8513 | 6943 | 8513 | 8369 |
| Irregular vertices (S) | 280 | 48 | 48 | 56 | 51 | 48 | 51 | 56 |
| … of which above the minimum | 272 | 40 | 40 | 48 | 43 | 40 | 43 | 48 |
| Base complex patches (C) | 7910 | 2997 | 3338 | 4424 | 6215 | 4909 | 6215 | 2721 |
| Alignment, degrees (A) | 14.61 | 7.16 | 6.93 | 6.43 | 6.32 | 7.75 | 6.41 | 6.60 |
| Mean angle deviation, degrees (F) | 4.50 | 8.89 | 6.56 | 6.11 | 6.44 | 7.53 | 6.44 | 6.28 |
| Angle deviation, 95th percentile (F) | 15.34 | 24.31 | 18.56 | 16.87 | 16.31 | 19.46 | 16.31 | 17.19 |
| Mean aspect ratio (F) | 1.139 | 1.332 | 1.340 | 1.351 | 1.595 | 1.351 | 1.595 | 1.339 |
| Area variation (F) | 0.144 | 0.270 | 0.297 | 0.394 | 0.365 | 0.270 | 0.365 | 0.395 |
| Hausdorff, ‰ of the diagonal (H) | 5.99 | 4.33 | 3.62 | 2.36 | 4.29 | 8.04 | 4.29 | 2.30 |
| Mean distance, ‰ (H) | 0.132 | 0.085 | 0.071 | 0.059 | 0.128 | 0.176 | 0.128 | 0.057 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.71 | 99.99 | 99.99 | 100.00 | 99.74 | 99.61 | 99.74 | 100.00 |
| Off budget, % (N) | 1.2 | 11.8 | 0.1 | 0.1 | 6.4 | 13.2 | 0.0 | 1.7 |
| Time, seconds (T) | 1.16 | 19.91 | 8.10 | 6.98 | 15.56 | 13.36 | 15.56 | 9.53 |

## pig (organic)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | 8116 | 7187 | 8093 | 8108 | 9824 | 7070 | 9824 | 9900 |
| Irregular vertices (S) | 370 | 81 | 88 | 98 | 101 | 98 | 101 | 98 |
| … of which above the minimum | 362 | — | 80 | 90 | — | 90 | — | 90 |
| Base complex patches (C) | 7826 | 4517 | 5985 | 5376 | — | 4284 | — | 6168 |
| Alignment, degrees (A) | 14.90 | 9.35 | 9.47 | 8.20 | 7.39 | 8.84 | 7.49 | 8.48 |
| Mean angle deviation, degrees (F) | 5.04 | 8.83 | 7.06 | 5.68 | 6.40 | 6.59 | 6.40 | 5.99 |
| Angle deviation, 95th percentile (F) | 17.35 | 25.06 | 20.08 | 17.10 | 18.75 | 19.80 | 18.75 | 18.61 |
| Mean aspect ratio (F) | 1.161 | 1.356 | 1.342 | 1.326 | 1.546 | 1.329 | 1.546 | 1.328 |
| Area variation (F) | 0.161 | 0.307 | 0.333 | 0.494 | 0.462 | 0.260 | 0.462 | 0.499 |
| Hausdorff, ‰ of the diagonal (H) | 21.49 | 7.72 | 7.22 | 6.09 | 9.11 | 9.71 | 9.11 | 6.69 |
| Mean distance, ‰ (H) | 0.238 | 0.164 | 0.142 | 0.118 | 0.222 | 0.301 | 0.222 | 0.100 |
| Thin parts: mean distance, % of a quad edge (V) | 11.6 | 16.5 | 15.9 | 5.9 | 3.7 | 15.8 | 4.9 | 4.7 |
| Volume kept, % of the source (V) | 99.70 | — | 99.99 | 100.00 | 99.74 | 99.55 | 99.74 | 100.00 |
| Off budget, % (N) | 1.5 | 10.2 | 1.2 | 1.4 | 22.8 | 11.6 | 0.0 | 0.8 |
| Time, seconds (T) | 1.42 | 34.04 | 7.46 | 14.46 | 16.84 | 16.13 | 16.84 | 12.76 |

## jerry (organic)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 |
| Quads made | 7933 | 7345 | 7903 | 7951 | 13714 | 6947 | 13714 | 13817 |
| Irregular vertices (S) | 490 | 173 | 168 | 279 | 244 | 247 | 244 | 355 |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 7797 | 5208 | 5458 | 6820 | 11709 | — | 11709 | 11799 |
| Alignment, degrees (A) | 18.65 | 13.30 | 12.91 | 12.37 | 10.64 | 12.69 | 10.91 | 11.71 |
| Mean angle deviation, degrees (F) | 6.63 | 8.59 | 5.22 | 5.71 | 8.15 | 8.38 | 8.15 | 5.90 |
| Angle deviation, 95th percentile (F) | 23.27 | 23.17 | 14.50 | 16.65 | 22.38 | 23.18 | 22.38 | 15.73 |
| Mean aspect ratio (F) | 1.241 | 1.297 | 1.249 | 1.266 | 1.879 | 1.380 | 1.879 | 1.284 |
| Area variation (F) | 0.210 | 0.264 | 0.313 | 0.526 | 0.651 | 0.279 | 0.651 | 0.462 |
| Hausdorff, ‰ of the diagonal (H) | 29.67 | 21.23 | 15.26 | 10.48 | 4.22 | 10.31 | 4.22 | 10.37 |
| Mean distance, ‰ (H) | 0.629 | 0.483 | 0.359 | 0.279 | 0.310 | 0.598 | 0.310 | 0.170 |
| Thin parts: mean distance, % of a quad edge (V) | 38.5 | 34.4 | 23.3 | 9.6 | 3.9 | 14.2 | 5.1 | 11.0 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 0.8 | 8.2 | 1.2 | 0.6 | 71.4 | 13.2 | 0.0 | 0.8 |
| Time, seconds (T) | 0.92 | 36.84 | 6.61 | 7.83 | 9.49 | 8.46 | 9.49 | 12.75 |

## rose (organic)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 6932 | 6584 | 9321 | 10789 | 53874 | 6896 | 53874 | 55613 |
| Irregular vertices (S) | 1126 | 570 | 1302 | 1846 | 1899 | 1164 | 1899 | 2546 |
| … of which above the minimum | — | — | 686 | 1230 | — | — | — | 1930 |
| Base complex patches (C) | 6434 | — | 7450 | 8736 | — | — | — | 45384 |
| Alignment, degrees (A) | 18.49 | 14.12 | 12.01 | 11.48 | 9.05 | 12.40 | 11.32 | 9.30 |
| Mean angle deviation, degrees (F) | 12.06 | 10.22 | 11.08 | 11.94 | 8.61 | 11.61 | 8.61 | 6.21 |
| Angle deviation, 95th percentile (F) | 40.69 | 29.42 | 40.25 | 37.14 | 24.64 | 31.66 | 24.64 | 19.15 |
| Mean aspect ratio (F) | 1.595 | 1.412 | 3.095 | 2.042 | 1.839 | 1.518 | 1.839 | 1.360 |
| Area variation (F) | 0.359 | 0.322 | 0.594 | 1.061 | 1.220 | 0.433 | 1.220 | 0.475 |
| Hausdorff, ‰ of the diagonal (H) | 72.22 | 73.55 | 14.66 | 12.20 | 73.68 | 75.13 | 73.68 | 7.95 |
| Mean distance, ‰ (H) | 1.750 | 1.267 | 0.594 | 0.624 | 0.689 | 1.593 | 0.689 | 0.109 |
| Thin parts: mean distance, % of a quad edge (V) | 47.5 | 37.1 | 11.6 | 9.1 | 22.2 | 42.9 | 159.7 | 10.2 |
| Volume kept, % of the source (V) | — | — | 97.89 | 97.51 | — | — | — | 99.80 |
| Off budget, % (N) | 13.4 | 17.7 | 16.5 | 34.9 | 573.4 | 13.8 | 0.0 | 3.2 |
| Time, seconds (T) | 1.79 | 135.60 | 4.53 | 8.02 | 9.67 | 7.85 | 9.67 | 20.15 |

## bridge (organic)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 |
| Quads made | 7960 | 7468 | 7880 | 8109 | 8805 | 7054 | 8805 | 8799 |
| Irregular vertices (S) | 258 | 68 | 70 | 81 | 82 | 79 | 82 | 89 |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 5201 | 1643 | 2987 | 2725 | 2942 | — | 2942 | 3637 |
| Alignment, degrees (A) | 13.88 | 9.84 | 9.87 | 9.03 | 8.33 | 9.16 | 8.67 | 9.22 |
| Mean angle deviation, degrees (F) | 4.48 | 7.01 | 4.82 | 4.66 | 6.17 | 6.74 | 6.17 | 4.39 |
| Angle deviation, 95th percentile (F) | 14.92 | 19.83 | 12.45 | 12.17 | 17.33 | 19.99 | 17.33 | 12.50 |
| Mean aspect ratio (F) | 1.145 | 1.266 | 1.271 | 1.263 | 1.480 | 1.386 | 1.480 | 1.274 |
| Area variation (F) | 0.140 | 0.229 | 0.264 | 0.329 | 0.354 | 0.285 | 0.354 | 0.363 |
| Hausdorff, ‰ of the diagonal (H) | 9.03 | 7.15 | 6.83 | 6.75 | 6.69 | 6.45 | 6.69 | 7.19 |
| Mean distance, ‰ (H) | 0.281 | 0.238 | 0.220 | 0.215 | 0.264 | 0.325 | 0.264 | 0.202 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 0.5 | 6.7 | 1.5 | 1.4 | 10.1 | 11.8 | 0.0 | 0.1 |
| Time, seconds (T) | 2.16 | 23.83 | 6.52 | 6.67 | 15.51 | 11.76 | 15.51 | 7.18 |

## part (mechanical)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7732 | 7824 | 8078 | 8098 | 7353 | 7353 | 7353 | 7279 |
| Irregular vertices (S) | 337 | 24 | 24 | 24 | 24 | 24 | 24 | 24 |
| … of which above the minimum | 329 | 16 | 16 | 16 | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 4678 | 128 | 429 | 92 | 730 | 730 | 730 | 316 |
| Alignment, degrees (A) | 13.52 | 11.84 | 11.83 | 10.32 | 10.87 | 10.87 | 10.87 | 10.48 |
| Mean angle deviation, degrees (F) | 4.52 | 1.66 | 1.25 | 1.00 | 3.63 | 3.63 | 3.63 | 1.27 |
| Angle deviation, 95th percentile (F) | 18.21 | 7.02 | 3.80 | 3.80 | 14.81 | 14.81 | 14.81 | 4.15 |
| Mean aspect ratio (F) | 1.178 | 1.151 | 1.093 | 1.129 | 1.301 | 1.301 | 1.301 | 1.116 |
| Area variation (F) | 0.166 | 0.105 | 0.126 | 0.328 | 0.244 | 0.244 | 0.244 | 0.339 |
| Hausdorff, ‰ of the diagonal (H) | 5.05 | 0.32 | 0.31 | 0.25 | 0.68 | 0.68 | 0.68 | 0.27 |
| Mean distance, ‰ (H) | 0.023 | 0.021 | 0.022 | 0.024 | 0.040 | 0.040 | 0.040 | 0.026 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.06 | 100.00 | 100.00 | 100.00 | 100.06 | 100.06 | 100.06 | 100.00 |
| Off budget, % (N) | 3.4 | 2.2 | 1.0 | 1.2 | 8.1 | 8.1 | 0.0 | 1.0 |
| Time, seconds (T) | 2.22 | 10.17 | 5.39 | 6.16 | 2.65 | 2.72 | 2.65 | 4.72 |

## cube (mechanical)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 7802 | 8028 | 8066 | 8066 | 7920 | 7920 | 7920 | 7776 |
| Irregular vertices (S) | 18 | 8 | 8 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 10 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 376 | 8028 | 6 | 6 | 6 | 6 | 6 | 6 |
| Alignment, degrees (A) | 0.96 | 0.68 | 0.11 | 0.27 | 0.11 | 0.11 | 0.11 | 0.28 |
| Mean angle deviation, degrees (F) | 0.91 | 0.95 | 0.01 | 0.20 | 0.00 | 0.00 | 0.00 | 0.21 |
| Angle deviation, 95th percentile (F) | 2.60 | 2.23 | 0.02 | 0.74 | 0.00 | 0.00 | 0.00 | 0.76 |
| Mean aspect ratio (F) | 1.030 | 1.032 | 1.018 | 1.074 | 1.019 | 1.019 | 1.019 | 1.066 |
| Area variation (F) | 0.045 | 0.044 | 0.013 | 0.093 | 0.013 | 0.013 | 0.013 | 0.091 |
| Hausdorff, ‰ of the diagonal (H) | 0.00 | 5.40 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.000 | 0.046 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.99 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 2.5 | 0.4 | 0.8 | 0.8 | 1.0 | 1.0 | 0.0 | 1.8 |
| Time, seconds (T) | 0.78 | 65.82 | 4.18 | 3.94 | 3.50 | 2.90 | 3.50 | 3.81 |

## lblock (mechanical)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 8000 | 8064 | 8064 | 8064 | 8064 | 8064 | 8064 | 8064 |
| Irregular vertices (S) | 24 | 12 | 12 | 12 | 12 | 12 | 12 | 12 |
| … of which above the minimum | 16 | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 706 | 14 | 14 | 14 | 14 | 14 | 14 | 14 |
| Alignment, degrees (A) | 1.17 | 0.57 | 0.61 | 0.92 | 0.54 | 0.54 | 0.54 | 0.92 |
| Mean angle deviation, degrees (F) | 0.88 | 0.01 | 0.00 | 0.49 | 0.00 | 0.00 | 0.00 | 0.49 |
| Angle deviation, 95th percentile (F) | 2.62 | 0.03 | 0.01 | 1.48 | 0.00 | 0.00 | 0.00 | 1.49 |
| Mean aspect ratio (F) | 1.025 | 1.000 | 1.000 | 1.078 | 1.000 | 1.000 | 1.000 | 1.078 |
| Area variation (F) | 0.041 | 0.000 | 0.000 | 0.096 | 0.000 | 0.000 | 0.000 | 0.096 |
| Hausdorff, ‰ of the diagonal (H) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 0.1 | 0.8 | 0.8 | 0.8 | 0.8 | 0.8 | 0.0 | 0.0 |
| Time, seconds (T) | 0.34 | 5.37 | 4.11 | 4.12 | 3.10 | 3.18 | 3.10 | 5.33 |

## cylinder (mechanical)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 8024 | 7276 | 7980 | 7899 | 6922 | 6922 | 6922 | 7007 |
| Irregular vertices (S) | 165 | 8 | 8 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 157 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 4940 | 1298 | 1452 | 1399 | 850 | 850 | 850 | 1247 |
| Alignment, degrees (A) | 3.47 | 1.33 | 1.32 | 1.28 | 1.73 | 1.73 | 1.73 | 1.25 |
| Mean angle deviation, degrees (F) | 3.05 | 2.55 | 2.31 | 2.32 | 4.41 | 4.41 | 4.41 | 2.23 |
| Angle deviation, 95th percentile (F) | 11.70 | 13.62 | 10.22 | 10.45 | 18.99 | 18.99 | 18.99 | 9.61 |
| Mean aspect ratio (F) | 1.097 | 1.285 | 1.288 | 1.398 | 1.349 | 1.349 | 1.349 | 1.369 |
| Area variation (F) | 0.115 | 0.284 | 0.296 | 0.213 | 0.322 | 0.322 | 0.322 | 0.191 |
| Hausdorff, ‰ of the diagonal (H) | 2.07 | 0.55 | 0.54 | 0.55 | 0.73 | 0.73 | 0.73 | 0.58 |
| Mean distance, ‰ (H) | 0.045 | 0.071 | 0.066 | 0.072 | 0.147 | 0.147 | 0.147 | 0.075 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.96 | 100.00 | 100.00 | 100.00 | 99.83 | 99.83 | 99.83 | 100.00 |
| Off budget, % (N) | 0.5 | 9.0 | 0.2 | 1.3 | 13.5 | 13.5 | 0.0 | 1.2 |
| Time, seconds (T) | 0.57 | 7.70 | 4.61 | 4.33 | 4.42 | 4.47 | 4.42 | 4.44 |

## venturi (extra)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 | 1/1 |
| Quads made | 7693 | 6950 | 7574 | 8352 | 19704 | 14699 | 19704 | 19050 |
| Irregular vertices (S) | 977 | 270 | 283 | 884 | 695 | 691 | 695 | 736 |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 4255 | — | 5218 | 6107 | — | — | — | 14291 |
| Alignment, degrees (A) | 11.23 | 10.34 | 10.32 | 11.96 | 7.51 | 7.00 | 8.22 | 9.61 |
| Mean angle deviation, degrees (F) | 10.36 | 6.59 | 6.37 | 8.79 | 8.05 | 8.32 | 8.05 | 5.76 |
| Angle deviation, 95th percentile (F) | 43.42 | 21.59 | 21.47 | 32.60 | 34.33 | 41.53 | 34.33 | 19.44 |
| Mean aspect ratio (F) | 1.775 | 1.305 | 1.293 | 1.634 | 2.096 | 2.216 | 2.096 | 1.336 |
| Area variation (F) | 0.455 | 0.267 | 0.299 | 0.425 | 1.067 | 0.959 | 1.067 | 0.296 |
| Hausdorff, ‰ of the diagonal (H) | 24.82 | 35.65 | 36.17 | 36.17 | 14.05 | 15.28 | 14.05 | 15.92 |
| Mean distance, ‰ (H) | 1.379 | 2.882 | 2.139 | 2.033 | 0.241 | 0.383 | 0.241 | 0.870 |
| Thin parts: mean distance, % of a quad edge (V) | 18.1 | 40.3 | 28.1 | 26.1 | 1.8 | 2.8 | 2.5 | 25.0 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 3.8 | 13.1 | 5.3 | 4.4 | 146.3 | 83.7 | 0.0 | 3.3 |
| Time, seconds (T) | 1.45 | 87.87 | 56.71 | 41.81 | 5.17 | 5.88 | 5.17 | 114.66 |

## ogre_hp (extra)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 |
| Quads made | 7700 | 6805 | 8133 | 7952 | 13801 | 6640 | 13801 | 13743 |
| Irregular vertices (S) | 442 | 163 | 198 | 354 | 368 | 344 | 368 | 432 |
| … of which above the minimum | 446 | — | 190 | 346 | 360 | — | 360 | 424 |
| Base complex patches (C) | 7440 | — | 6567 | 6122 | 10719 | — | 10719 | 11695 |
| Alignment, degrees (A) | 17.01 | 12.30 | 12.05 | 10.05 | 8.76 | 10.53 | 9.13 | 9.69 |
| Mean angle deviation, degrees (F) | 6.55 | 8.73 | 6.00 | 6.61 | 7.98 | 8.73 | 7.98 | 6.08 |
| Angle deviation, 95th percentile (F) | 21.36 | 23.56 | 16.64 | 19.28 | 22.44 | 23.83 | 22.44 | 17.35 |
| Mean aspect ratio (F) | 1.226 | 1.417 | 1.390 | 1.381 | 2.045 | 1.480 | 2.045 | 1.344 |
| Area variation (F) | 0.198 | 0.304 | 0.353 | 0.968 | 0.686 | 0.354 | 0.686 | 1.030 |
| Hausdorff, ‰ of the diagonal (H) | 30.78 | 25.14 | 10.14 | 7.19 | 7.42 | 30.01 | 7.42 | 6.66 |
| Mean distance, ‰ (H) | 0.706 | 0.441 | 0.316 | 0.336 | 0.321 | 0.572 | 0.321 | 0.222 |
| Thin parts: mean distance, % of a quad edge (V) | 48.7 | 16.3 | 9.3 | 1.4 | 3.6 | 15.6 | 5.9 | 1.4 |
| Volume kept, % of the source (V) | 99.39 | 99.93 | 100.00 | 100.01 | 99.56 | 99.30 | 99.56 | 100.01 |
| Off budget, % (N) | 3.8 | 14.9 | 1.7 | 0.6 | 72.5 | 17.0 | 0.0 | 0.4 |
| Time, seconds (T) | 82.86 | 80.09 | 61.52 | 61.39 | 44.60 | 45.61 | 44.60 | 69.97 |

## terrain (extra)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | 4807 | 6773 | 7941 | 8014 | 11819 | 7183 | 11819 | 11855 |
| Irregular vertices (S) | 273 | 124 | 124 | 229 | 296 | 253 | 296 | 315 |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 3587 | 4296 | 5508 | 7314 | 8228 | 4351 | 8228 | 10547 |
| Alignment, degrees (A) | 21.43 | 17.68 | 17.72 | 17.58 | 18.10 | 18.39 | 18.33 | 17.46 |
| Mean angle deviation, degrees (F) | 7.45 | 10.25 | 6.86 | 6.88 | 8.69 | 7.98 | 8.69 | 6.37 |
| Angle deviation, 95th percentile (F) | 25.48 | 26.71 | 18.80 | 18.66 | 25.15 | 22.11 | 25.15 | 16.76 |
| Mean aspect ratio (F) | 1.264 | 1.320 | 1.299 | 1.301 | 1.668 | 1.342 | 1.668 | 1.259 |
| Area variation (F) | 0.215 | 0.287 | 0.322 | 0.475 | 0.518 | 0.237 | 0.518 | 0.499 |
| Hausdorff, ‰ of the diagonal (H) | 130.45 | 15.44 | 11.35 | 8.75 | 10.70 | 16.84 | 10.70 | 8.30 |
| Mean distance, ‰ (H) | 1.477 | 0.991 | 0.744 | 0.629 | 0.719 | 1.071 | 0.719 | 0.503 |
| Thin parts: mean distance, % of a quad edge (V) | 41.7 | 38.6 | 23.6 | 17.9 | 20.8 | 34.8 | 27.4 | 19.2 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 39.9 | 15.3 | 0.7 | 0.2 | 47.7 | 10.2 | 0.0 | 0.3 |
| Time, seconds (T) | 10.05 | 45.81 | 10.06 | 12.57 | 24.96 | 24.23 | 24.96 | 16.75 |

## mahaveer (extra)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 5345 | 6148 | 11485 | 12840 | 55181 | 8791 | 55181 | 63166 |
| Irregular vertices (S) | 420 | 166 | 559 | 658 | 3155 | 1146 | 3155 | 1154 |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 4008 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 9.69 | 6.08 | 7.42 | 7.16 | 11.56 | 8.99 | 12.11 | 6.10 |
| Mean angle deviation, degrees (F) | 8.33 | 7.42 | 9.10 | 8.55 | 11.13 | 8.88 | 11.13 | 5.73 |
| Angle deviation, 95th percentile (F) | 32.25 | 24.85 | 49.50 | 47.90 | 36.88 | 32.39 | 36.88 | 20.28 |
| Mean aspect ratio (F) | 1.365 | 1.551 | 20.798 | 28.528 | 1.978 | 1.569 | 1.978 | 6.720 |
| Area variation (F) | 0.299 | 0.358 | 1.120 | 1.435 | 1.998 | 0.626 | 1.998 | 0.670 |
| Hausdorff, ‰ of the diagonal (H) | 32.21 | 14.95 | 15.73 | 15.73 | 13.01 | 18.76 | 13.01 | 12.89 |
| Mean distance, ‰ (H) | 3.712 | 2.684 | 1.090 | 0.997 | 0.784 | 1.567 | 0.784 | 0.486 |
| Thin parts: mean distance, % of a quad edge (V) | 60.8 | 45.3 | 18.1 | 16.5 | 17.5 | 26.0 | 49.4 | 34.5 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 33.2 | 23.2 | 43.6 | 60.5 | 589.8 | 9.9 | 0.0 | 14.5 |
| Time, seconds (T) | 6.80 | 72.10 | 10.08 | 10.41 | 13.92 | 11.71 | 13.92 | 65.07 |

## marine (extra)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 4146 | — | 15417 | 19753 | 246063 | 14915 | 246063 | 256346 |
| Irregular vertices (S) | 1524 | — | 6178 | 6845 | 10419 | 5525 | 10419 | 11577 |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 3685 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 17.92 | — | 10.91 | 11.15 | 10.30 | 13.42 | 11.42 | 8.18 |
| Mean angle deviation, degrees (F) | 20.55 | — | 14.53 | 13.34 | 8.58 | 15.13 | 8.58 | 4.67 |
| Angle deviation, 95th percentile (F) | 58.40 | — | 58.61 | 55.36 | 25.88 | 47.93 | 25.88 | 16.88 |
| Mean aspect ratio (F) | 3.745 | — | 3.495 | 3.442 | 1.549 | 1.767 | 1.549 | 1.312 |
| Area variation (F) | 0.689 | — | 1.712 | 2.245 | 1.812 | 1.232 | 1.812 | 0.499 |
| Hausdorff, ‰ of the diagonal (H) | 6.21 | — | 3.25 | 3.67 | 2.08 | 2.57 | 2.08 | 3.00 |
| Mean distance, ‰ (H) | 1.017 | — | 0.248 | 0.258 | 0.091 | 0.324 | 0.091 | 0.030 |
| Thin parts: mean distance, % of a quad edge (V) | 58.2 | — | 13.0 | 14.1 | 10.1 | 19.7 | 52.2 | 18.8 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 48.2 | — | 92.7 | 146.9 | 2975.8 | 86.4 | 0.0 | 4.2 |
| Time, seconds (T) | 4.87 | — | 14.47 | 19.35 | 13.22 | 13.40 | 13.22 | 59.65 |

Failures: integer-grid: error: invalid mesh: work mesh has 10 non-manifold or inconsistently oriented edges.

## f16 (extra)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 4548 | 6707 | 9533 | 9578 | 46194 | 10673 | 46194 | 42585 |
| Irregular vertices (S) | 863 | 645 | 757 | 643 | 2662 | 1428 | 2662 | 1064 |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 3548 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 8.12 | 5.58 | 3.72 | 3.76 | 4.69 | 5.48 | 4.75 | 3.32 |
| Mean angle deviation, degrees (F) | 14.41 | 9.45 | 6.64 | 6.62 | 9.22 | 10.50 | 9.22 | 5.45 |
| Angle deviation, 95th percentile (F) | 54.18 | 32.95 | 28.59 | 28.30 | 33.32 | 37.63 | 33.32 | 20.46 |
| Mean aspect ratio (F) | 1.909 | 1.788 | 32.310 | 51.882 | 2.322 | 2.777 | 2.322 | 15.323 |
| Area variation (F) | 0.470 | 0.391 | 0.860 | 0.854 | 1.514 | 0.867 | 1.514 | 0.742 |
| Hausdorff, ‰ of the diagonal (H) | 73.99 | 36.01 | 29.68 | 29.68 | 6.30 | 42.51 | 6.30 | 29.52 |
| Mean distance, ‰ (H) | 10.384 | 2.811 | 2.336 | 2.332 | 0.257 | 1.405 | 0.257 | 0.724 |
| Thin parts: mean distance, % of a quad edge (V) | 128.6 | 37.0 | 33.3 | 33.2 | 4.1 | 18.5 | 9.1 | 38.2 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 43.1 | 16.2 | 19.2 | 19.7 | 477.4 | 33.4 | 0.0 | 7.8 |
| Time, seconds (T) | 5.80 | 285.92 | 30.26 | 36.49 | 10.81 | 9.70 | 10.81 | 83.04 |

## camera (extra)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 6839 | 7017 | 7565 | 7074 | 20326 | 10881 | 20326 | 17068 |
| Irregular vertices (S) | 532 | 170 | 181 | 192 | 458 | 360 | 458 | 233 |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 4440 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 6.43 | 3.94 | 4.63 | 5.04 | 3.48 | 3.17 | 3.48 | 4.90 |
| Mean angle deviation, degrees (F) | 9.46 | 5.96 | 11.83 | 5.73 | 5.75 | 5.64 | 5.75 | 4.52 |
| Angle deviation, 95th percentile (F) | 38.06 | 23.18 | 72.49 | 23.37 | 21.00 | 22.13 | 21.00 | 18.16 |
| Mean aspect ratio (F) | 1.596 | 1.823 | 9.741 | 2.732 | 6.194 | 9.720 | 6.194 | 2.989 |
| Area variation (F) | 0.398 | 0.330 | 0.818 | 0.678 | 1.143 | 0.765 | 1.143 | 0.651 |
| Hausdorff, ‰ of the diagonal (H) | 36.49 | 36.30 | 20.69 | 20.69 | 7.91 | 17.13 | 7.91 | 34.81 |
| Mean distance, ‰ (H) | 1.944 | 1.189 | 1.166 | 1.360 | 0.334 | 0.577 | 0.334 | 0.654 |
| Thin parts: mean distance, % of a quad edge (V) | 21.3 | 11.1 | 12.5 | 14.1 | 2.8 | 4.7 | 3.9 | 17.1 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 14.5 | 12.3 | 5.4 | 11.6 | 154.1 | 36.0 | 0.0 | 16.0 |
| Time, seconds (T) | 2.76 | 146.04 | 65.35 | 68.58 | 7.25 | 6.14 | 7.25 | 261.96 |

## wolf (extra)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 7656 | 6579 | 6760 | 6901 | 28909 | 6170 | 28909 | 25241 |
| Irregular vertices (S) | 835 | 380 | 417 | 457 | 961 | 603 | 961 | 727 |
| … of which above the minimum | 730 | — | — | — | — | — | — | — |
| Base complex patches (C) | 7378 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 15.59 | 10.07 | 10.31 | 11.06 | 9.87 | 10.45 | 11.04 | 11.35 |
| Mean angle deviation, degrees (F) | 8.99 | 9.19 | 9.34 | 9.86 | 9.44 | 10.16 | 9.44 | 8.62 |
| Angle deviation, 95th percentile (F) | 30.80 | 26.78 | 26.93 | 28.71 | 27.87 | 30.40 | 27.87 | 25.31 |
| Mean aspect ratio (F) | 1.370 | 1.416 | 1.458 | 1.466 | 1.826 | 1.547 | 1.826 | 1.343 |
| Area variation (F) | 0.275 | 0.321 | 0.311 | 0.344 | 0.850 | 0.373 | 0.850 | 0.305 |
| Hausdorff, ‰ of the diagonal (H) | 31.17 | 13.40 | 12.56 | 12.56 | 9.82 | 39.39 | 9.82 | 8.60 |
| Mean distance, ‰ (H) | 1.125 | 0.922 | 0.688 | 0.677 | 0.330 | 1.288 | 0.330 | 0.207 |
| Thin parts: mean distance, % of a quad edge (V) | 74.2 | 59.6 | 34.0 | 33.4 | 20.6 | 67.3 | 46.3 | 39.9 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 4.3 | 17.8 | 15.5 | 13.7 | 261.4 | 22.9 | 0.0 | 12.7 |
| Time, seconds (T) | 1.15 | 117.14 | 150.90 | 147.36 | 41.01 | 41.67 | 41.01 | 338.94 |

## sphere (calibration)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 8190 | 7568 | 7984 | 8074 | 7088 | 7088 | 7088 | 7223 |
| Irregular vertices (S) | 193 | 26 | 26 | 26 | 8 | 8 | 8 | 18 |
| … of which above the minimum | 185 | 18 | 18 | 18 | 0 | 0 | 0 | 10 |
| Base complex patches (C) | 8190 | 7568 | 7984 | 8074 | 4992 | 4992 | 4992 | 7223 |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 3.49 | 6.08 | 4.34 | 4.52 | 7.65 | 7.65 | 7.65 | 5.94 |
| Angle deviation, 95th percentile (F) | 11.93 | 13.87 | 9.72 | 9.91 | 18.00 | 18.00 | 18.00 | 12.35 |
| Mean aspect ratio (F) | 1.097 | 1.149 | 1.130 | 1.134 | 1.187 | 1.187 | 1.187 | 1.198 |
| Area variation (F) | 0.122 | 0.153 | 0.174 | 0.168 | 0.391 | 0.391 | 0.391 | 0.198 |
| Hausdorff, ‰ of the diagonal (H) | 0.43 | 0.40 | 0.35 | 0.36 | 0.55 | 0.55 | 0.55 | 0.34 |
| Mean distance, ‰ (H) | 0.082 | 0.083 | 0.078 | 0.078 | 0.159 | 0.159 | 0.159 | 0.086 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.92 | 100.01 | 100.00 | 100.00 | 99.83 | 99.83 | 99.83 | 100.00 |
| Off budget, % (N) | 2.4 | 5.4 | 0.2 | 0.9 | 11.4 | 11.4 | 0.0 | 1.9 |
| Time, seconds (T) | 0.54 | 13.07 | 5.66 | 4.92 | 4.18 | 4.20 | 4.18 | 6.04 |

## torus (calibration)

| | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 8038 | 7280 | 8024 | 8024 | 7401 | 7401 | 7401 | 7467 |
| Irregular vertices (S) | 262 | 0 | 0 | 0 | 20 | 20 | 20 | 0 |
| … of which above the minimum | 262 | 0 | 0 | 0 | 20 | 20 | 20 | 0 |
| Base complex patches (C) | 7956 | 1 | 1 | 1 | 5983 | 5983 | 5983 | 1 |
| Alignment, degrees (A) | 6.29 | 0.61 | 0.64 | 0.64 | 4.25 | 4.25 | 4.25 | 0.62 |
| Mean angle deviation, degrees (F) | 5.12 | 0.91 | 0.91 | 0.91 | 3.68 | 3.68 | 3.68 | 0.91 |
| Angle deviation, 95th percentile (F) | 15.85 | 1.45 | 1.42 | 1.42 | 10.81 | 10.81 | 10.81 | 1.47 |
| Mean aspect ratio (F) | 1.139 | 1.340 | 1.321 | 1.319 | 1.233 | 1.233 | 1.233 | 1.321 |
| Area variation (F) | 0.149 | 0.281 | 0.299 | 0.300 | 0.225 | 0.225 | 0.225 | 0.299 |
| Hausdorff, ‰ of the diagonal (H) | 0.65 | 0.34 | 0.35 | 0.35 | 0.69 | 0.69 | 0.69 | 0.34 |
| Mean distance, ‰ (H) | 0.119 | 0.111 | 0.104 | 0.104 | 0.222 | 0.222 | 0.222 | 0.108 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.79 | 100.01 | 100.01 | 100.01 | 99.56 | 99.56 | 99.56 | 100.01 |
| Off budget, % (N) | 0.5 | 9.0 | 0.3 | 0.3 | 7.5 | 7.5 | 0.0 | 0.9 |
| Time, seconds (T) | 0.91 | 6.48 | 4.16 | 5.21 | 6.20 | 5.73 | 6.20 | 4.44 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 0 | 9 | 5 | 1 | 4 | 4 | 4 | 2 |
| Base complex patches (C) | 1 | 4 | 2 | 0 | 0 | 4 | 0 | 2 |
| Alignment, degrees (A) | 0 | 1 | 1 | 0 | 8 | 0 | 2 | 3 |
| Mean angle deviation, degrees (F) | 7 | 0 | 1 | 0 | 0 | 0 | 0 | 5 |
| Mean aspect ratio (F) | 12 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
| Valid meshes (V) | 50/65 | 8/13 | 13/13 | 12/13 | 11/13 | 9/13 | 11/13 | 12/13 |

### mechanical (4 models)

| Metric | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 0 | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 0 | 1 | 2 | 3 | 3 | 3 | 3 | 2 |
| Alignment, degrees (A) | 0 | 0 | 0 | 1 | 2 | 2 | 2 | 1 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 1 | 2 | 2 | 2 | 1 |
| Mean aspect ratio (F) | 1 | 0 | 2 | 0 | 1 | 1 | 1 | 0 |
| Valid meshes (V) | 19/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 |
