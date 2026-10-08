# Bench results

Requested budget: 8000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. Lower is better for every metric; "—" means it could not be measured.

## blob (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 277 | 24 | 24 | 24 | 24 | 24 |
| … of which above the minimum | 269 | 16 | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 8164 | 5660 | 6808 | 6819 | 4563 | 2919 |
| Alignment, degrees (A) | 15.05 | 5.88 | 5.81 | 5.46 | 4.08 | 4.45 |
| Mean angle deviation, degrees (F) | 4.85 | 7.41 | 6.29 | 5.78 | 5.52 | 5.55 |
| Angle deviation, 95th percentile (F) | 15.77 | 19.73 | 17.15 | 15.29 | 14.94 | 14.71 |
| Mean aspect ratio (F) | 1.142 | 1.317 | 1.309 | 1.290 | 1.414 | 1.323 |
| Area variation (F) | 0.153 | 0.316 | 0.340 | 0.360 | 0.395 | 0.331 |
| Hausdorff, ‰ of the diagonal (H) | 4.97 | 2.41 | 1.88 | 1.76 | 3.00 | 3.66 |
| Mean distance, ‰ (H) | 0.178 | 0.097 | 0.082 | 0.078 | 0.185 | 0.216 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.86 | 100.01 | 100.01 | 100.01 | 99.77 | 99.81 |
| Off budget, % (N) | 2.1 | 10.9 | 0.9 | 0.3 | 1.1 | 12.1 |
| Time, seconds (T) | 1.85 | 12.57 | 6.44 | 6.58 | 12.37 | 12.61 |

## rock (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 162 | 114 | 116 | 134 | 68 | 68 |
| … of which above the minimum | 154 | 106 | 108 | 126 | 60 | 60 |
| Base complex patches (C) | 7792 | 7393 | 7943 | 7940 | 7071 | 6767 |
| Alignment, degrees (A) | 22.82 | 16.04 | 16.60 | 16.32 | 17.28 | 17.32 |
| Mean angle deviation, degrees (F) | 4.13 | 9.24 | 5.93 | 5.42 | 9.80 | 9.89 |
| Angle deviation, 95th percentile (F) | 12.43 | 22.12 | 14.01 | 13.19 | 23.73 | 23.67 |
| Mean aspect ratio (F) | 1.115 | 1.246 | 1.216 | 1.215 | 1.350 | 1.340 |
| Area variation (F) | 0.128 | 0.204 | 0.254 | 0.264 | 0.287 | 0.281 |
| Hausdorff, ‰ of the diagonal (H) | 3.22 | 3.64 | 2.87 | 2.57 | 4.18 | 4.36 |
| Mean distance, ‰ (H) | 0.433 | 0.418 | 0.395 | 0.389 | 0.515 | 0.530 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.89 | 100.00 | 100.00 | 100.00 | 99.83 | 99.83 |
| Off budget, % (N) | 2.6 | 7.6 | 0.7 | 0.8 | 11.6 | 15.4 |
| Time, seconds (T) | 1.45 | 37.31 | 6.46 | 6.72 | 18.31 | 18.00 |

## creature (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 214 | 40 | 40 | 42 | 40 | 40 |
| … of which above the minimum | 206 | — | 32 | 34 | 32 | 32 |
| Base complex patches (C) | 7510 | 2490 | 2864 | 3225 | 3236 | 3467 |
| Alignment, degrees (A) | 11.81 | 6.40 | 6.02 | 5.63 | 6.64 | 6.66 |
| Mean angle deviation, degrees (F) | 3.91 | 8.75 | 6.71 | 5.60 | 6.13 | 6.16 |
| Angle deviation, 95th percentile (F) | 12.20 | 25.50 | 18.89 | 14.96 | 17.40 | 17.67 |
| Mean aspect ratio (F) | 1.116 | 1.352 | 1.345 | 1.322 | 1.277 | 1.278 |
| Area variation (F) | 0.127 | 0.334 | 0.364 | 0.470 | 0.277 | 0.275 |
| Hausdorff, ‰ of the diagonal (H) | 1.70 | 2.35 | 0.96 | 0.90 | 1.24 | 1.34 |
| Mean distance, ‰ (H) | 0.106 | 0.099 | 0.085 | 0.082 | 0.144 | 0.145 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.81 | — | 100.01 | 100.01 | 99.72 | 99.71 |
| Off budget, % (N) | 2.9 | 13.5 | 1.5 | 2.2 | 10.2 | 9.9 |
| Time, seconds (T) | 1.55 | 27.86 | 15.67 | 14.05 | 12.49 | 8.52 |

## cage (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 250 | 40 | 48 | 58 | 62 | 64 |
| … of which above the minimum | 218 | 8 | 16 | 26 | 30 | 32 |
| Base complex patches (C) | 7125 | 766 | 1654 | 275 | 3227 | 1954 |
| Alignment, degrees (A) | 8.80 | 2.71 | 2.47 | 1.94 | 2.06 | 2.29 |
| Mean angle deviation, degrees (F) | 4.40 | 3.58 | 2.13 | 2.26 | 3.22 | 3.48 |
| Angle deviation, 95th percentile (F) | 14.23 | 10.77 | 6.74 | 6.75 | 9.04 | 10.22 |
| Mean aspect ratio (F) | 1.144 | 1.200 | 1.195 | 1.233 | 1.693 | 1.289 |
| Area variation (F) | 0.138 | 0.191 | 0.214 | 0.325 | 0.338 | 0.229 |
| Hausdorff, ‰ of the diagonal (H) | 9.87 | 4.82 | 3.02 | 2.86 | 4.53 | 5.93 |
| Mean distance, ‰ (H) | 0.344 | 0.275 | 0.232 | 0.206 | 0.451 | 0.614 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.48 | 99.98 | 99.99 | 100.00 | 99.38 | 99.02 |
| Off budget, % (N) | 0.6 | 2.9 | 1.4 | 0.7 | 37.6 | 6.1 |
| Time, seconds (T) | 0.57 | 30.63 | 5.42 | 8.97 | 5.76 | 4.92 |

## suzanne (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 346 | 67 | 64 | 82 | 78 | 72 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 7878 | 4267 | 3543 | 4783 | 4088 | 3340 |
| Alignment, degrees (A) | 16.64 | 10.01 | 10.00 | 9.80 | 8.58 | 9.86 |
| Mean angle deviation, degrees (F) | 5.17 | 7.60 | 5.45 | 5.99 | 7.04 | 6.91 |
| Angle deviation, 95th percentile (F) | 17.78 | 21.30 | 15.25 | 15.22 | 18.57 | 19.33 |
| Mean aspect ratio (F) | 1.169 | 1.223 | 1.211 | 1.231 | 1.470 | 1.246 |
| Area variation (F) | 0.164 | 0.218 | 0.251 | 0.377 | 0.366 | 0.248 |
| Hausdorff, ‰ of the diagonal (H) | 5.94 | 4.86 | 4.31 | 2.96 | 3.57 | 4.99 |
| Mean distance, ‰ (H) | 0.286 | 0.272 | 0.228 | 0.179 | 0.322 | 0.422 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Off budget, % (N) | 1.2 | 7.8 | 0.5 | 1.3 | 15.6 | 8.5 |
| Time, seconds (T) | 0.87 | 83.29 | 5.65 | 7.69 | 7.68 | 4.94 |

## ogre (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 455 | 169 | 202 | 240 | 338 | 322 |
| … of which above the minimum | 429 | — | 194 | 232 | 330 | — |
| Base complex patches (C) | 7656 | — | 6252 | 6091 | 9903 | — |
| Alignment, degrees (A) | 17.72 | 11.52 | 11.32 | 10.81 | 8.50 | 9.79 |
| Mean angle deviation, degrees (F) | 6.18 | 8.70 | 6.38 | 6.34 | 7.95 | 8.39 |
| Angle deviation, 95th percentile (F) | 20.84 | 23.69 | 17.23 | 16.92 | 22.55 | 22.69 |
| Mean aspect ratio (F) | 1.225 | 1.403 | 1.384 | 1.360 | 2.025 | 1.482 |
| Area variation (F) | 0.196 | 0.300 | 0.327 | 0.440 | 0.643 | 0.364 |
| Hausdorff, ‰ of the diagonal (H) | 30.74 | 22.74 | 11.29 | 8.94 | 5.23 | 25.57 |
| Mean distance, ‰ (H) | 0.601 | 0.402 | 0.321 | 0.299 | 0.346 | 0.548 |
| Thin parts: mean distance, % of a quad edge (V) | 35.7 | 24.7 | 12.7 | 4.5 | 7.4 | 18.6 |
| Volume kept, % of the source (V) | 99.41 | 99.94 | 99.99 | 100.00 | 99.59 | 99.39 |
| Off budget, % (N) | 2.1 | 12.1 | 0.4 | 0.6 | 59.0 | 15.5 |
| Time, seconds (T) | 2.51 | 36.75 | 12.12 | 12.27 | 23.50 | 24.03 |

## spot (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 286 | 80 | 80 | 78 | 76 | 80 |
| … of which above the minimum | 278 | 72 | 72 | 70 | 68 | 72 |
| Base complex patches (C) | 7942 | 3692 | 5329 | 5007 | 4605 | 3862 |
| Alignment, degrees (A) | 16.86 | 8.49 | 7.93 | 7.55 | 7.56 | 8.42 |
| Mean angle deviation, degrees (F) | 4.47 | 9.07 | 6.47 | 6.49 | 5.56 | 6.01 |
| Angle deviation, 95th percentile (F) | 14.93 | 23.35 | 16.91 | 17.44 | 14.90 | 15.81 |
| Mean aspect ratio (F) | 1.136 | 1.348 | 1.326 | 1.331 | 1.342 | 1.289 |
| Area variation (F) | 0.144 | 0.296 | 0.330 | 0.429 | 0.301 | 0.256 |
| Hausdorff, ‰ of the diagonal (H) | 7.10 | 2.61 | 2.24 | 2.06 | 4.73 | 5.88 |
| Mean distance, ‰ (H) | 0.172 | 0.093 | 0.082 | 0.078 | 0.169 | 0.187 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.78 | 100.01 | 100.01 | 100.01 | 99.73 | 99.73 |
| Off budget, % (N) | 1.2 | 11.5 | 0.4 | 1.2 | 0.4 | 9.1 |
| Time, seconds (T) | 1.23 | 43.69 | 10.45 | 14.23 | 17.71 | 17.39 |

## bob (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 245 | 18 | 20 | 20 | 34 | 30 |
| … of which above the minimum | 245 | — | 20 | 20 | 34 | 30 |
| Base complex patches (C) | 8072 | 1541 | 1279 | 2898 | 4859 | 3789 |
| Alignment, degrees (A) | 20.15 | 4.10 | 3.64 | 3.75 | 5.48 | 5.82 |
| Mean angle deviation, degrees (F) | 4.31 | 4.94 | 3.64 | 3.76 | 6.43 | 6.86 |
| Angle deviation, 95th percentile (F) | 14.25 | 16.54 | 11.19 | 11.90 | 18.71 | 21.48 |
| Mean aspect ratio (F) | 1.124 | 1.407 | 1.402 | 1.380 | 1.407 | 1.407 |
| Area variation (F) | 0.139 | 0.356 | 0.374 | 0.433 | 0.369 | 0.373 |
| Hausdorff, ‰ of the diagonal (H) | 2.84 | 3.52 | 0.55 | 0.96 | 1.50 | 1.30 |
| Mean distance, ‰ (H) | 0.120 | 0.063 | 0.056 | 0.056 | 0.150 | 0.153 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.79 | — | 100.01 | 100.01 | 99.70 | 99.70 |
| Off budget, % (N) | 0.9 | 11.8 | 0.4 | 0.2 | 13.8 | 14.6 |
| Time, seconds (T) | 1.37 | 17.65 | 9.09 | 8.89 | 20.38 | 16.98 |

## blub (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 280 | 48 | 48 | 56 | 51 | 48 |
| … of which above the minimum | 272 | 40 | 40 | 48 | 43 | 40 |
| Base complex patches (C) | 7910 | 2997 | 3338 | 4424 | 6215 | 4909 |
| Alignment, degrees (A) | 14.61 | 7.16 | 6.93 | 6.43 | 6.32 | 7.75 |
| Mean angle deviation, degrees (F) | 4.50 | 8.89 | 6.56 | 6.11 | 6.44 | 7.53 |
| Angle deviation, 95th percentile (F) | 15.34 | 24.31 | 18.56 | 16.87 | 16.31 | 19.46 |
| Mean aspect ratio (F) | 1.139 | 1.332 | 1.340 | 1.351 | 1.595 | 1.351 |
| Area variation (F) | 0.144 | 0.270 | 0.297 | 0.394 | 0.365 | 0.270 |
| Hausdorff, ‰ of the diagonal (H) | 5.99 | 4.33 | 3.62 | 2.36 | 4.29 | 8.04 |
| Mean distance, ‰ (H) | 0.132 | 0.085 | 0.071 | 0.059 | 0.128 | 0.176 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.71 | 99.99 | 99.99 | 100.00 | 99.74 | 99.61 |
| Off budget, % (N) | 1.2 | 11.8 | 0.1 | 0.1 | 6.4 | 13.2 |
| Time, seconds (T) | 1.16 | 19.05 | 8.72 | 8.04 | 15.56 | 13.36 |

## pig (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 370 | 81 | 88 | 98 | 101 | 98 |
| … of which above the minimum | 362 | — | 80 | 90 | — | 90 |
| Base complex patches (C) | 7826 | 4511 | 5985 | 5403 | — | 4284 |
| Alignment, degrees (A) | 14.90 | 9.34 | 9.47 | 8.16 | 7.39 | 8.84 |
| Mean angle deviation, degrees (F) | 5.04 | 8.83 | 7.06 | 5.71 | 6.40 | 6.59 |
| Angle deviation, 95th percentile (F) | 17.35 | 25.07 | 20.08 | 17.19 | 18.75 | 19.80 |
| Mean aspect ratio (F) | 1.161 | 1.356 | 1.342 | 1.326 | 1.546 | 1.329 |
| Area variation (F) | 0.161 | 0.307 | 0.333 | 0.497 | 0.462 | 0.260 |
| Hausdorff, ‰ of the diagonal (H) | 21.49 | 7.72 | 7.22 | 6.09 | 9.11 | 9.71 |
| Mean distance, ‰ (H) | 0.238 | 0.164 | 0.142 | 0.119 | 0.222 | 0.301 |
| Thin parts: mean distance, % of a quad edge (V) | 11.6 | 16.5 | 15.9 | 5.9 | 3.7 | 15.8 |
| Volume kept, % of the source (V) | 99.70 | — | 99.99 | 100.00 | 99.74 | 99.55 |
| Off budget, % (N) | 1.5 | 10.1 | 1.2 | 1.8 | 22.8 | 11.6 |
| Time, seconds (T) | 1.42 | 35.83 | 8.46 | 15.99 | 16.84 | 16.13 |

## jerry (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 490 | 170 | 166 | 273 | 244 | 247 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 7797 | — | 5332 | 7075 | 11709 | — |
| Alignment, degrees (A) | 18.65 | 13.30 | 12.99 | 12.44 | 10.64 | 12.69 |
| Mean angle deviation, degrees (F) | 6.63 | 8.68 | 5.30 | 5.93 | 8.15 | 8.38 |
| Angle deviation, 95th percentile (F) | 23.27 | 23.16 | 14.93 | 16.78 | 22.38 | 23.18 |
| Mean aspect ratio (F) | 1.241 | 1.289 | 1.254 | 1.276 | 1.879 | 1.380 |
| Area variation (F) | 0.210 | 0.259 | 0.316 | 0.522 | 0.651 | 0.279 |
| Hausdorff, ‰ of the diagonal (H) | 29.67 | 18.78 | 15.05 | 9.97 | 4.22 | 10.31 |
| Mean distance, ‰ (H) | 0.629 | 0.462 | 0.361 | 0.279 | 0.310 | 0.598 |
| Thin parts: mean distance, % of a quad edge (V) | 38.5 | 36.0 | 22.8 | 9.3 | 3.9 | 14.2 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Off budget, % (N) | 0.8 | 8.2 | 1.6 | 1.3 | 71.4 | 13.2 |
| Time, seconds (T) | 0.92 | 36.40 | 5.71 | 7.40 | 9.49 | 8.46 |

## rose (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 |
| Irregular vertices (S) | 1126 | 557 | 1279 | 1723 | 1899 | 1164 |
| … of which above the minimum | — | — | 663 | 1107 | — | — |
| Base complex patches (C) | 6434 | — | 7101 | 7316 | — | — |
| Alignment, degrees (A) | 18.49 | 14.07 | 12.15 | 11.60 | 9.05 | 12.40 |
| Mean angle deviation, degrees (F) | 12.06 | 10.17 | 10.74 | 12.56 | 8.61 | 11.61 |
| Angle deviation, 95th percentile (F) | 40.69 | 29.46 | 36.99 | 41.93 | 24.64 | 31.66 |
| Mean aspect ratio (F) | 1.595 | 1.400 | 2.317 | 2.574 | 1.839 | 1.518 |
| Area variation (F) | 0.359 | 0.321 | 0.535 | 0.840 | 1.220 | 0.433 |
| Hausdorff, ‰ of the diagonal (H) | 72.22 | 73.55 | 15.38 | 14.81 | 73.68 | 75.13 |
| Mean distance, ‰ (H) | 1.750 | 1.315 | 0.615 | 0.683 | 0.689 | 1.593 |
| Thin parts: mean distance, % of a quad edge (V) | 47.5 | 38.9 | 12.5 | 10.8 | 22.2 | 42.9 |
| Volume kept, % of the source (V) | — | — | 97.87 | 97.38 | — | — |
| Off budget, % (N) | 13.4 | 17.9 | 9.3 | 5.2 | 573.4 | 13.8 |
| Time, seconds (T) | 1.79 | 141.47 | 10.43 | 16.95 | 9.67 | 7.85 |

## bridge (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 258 | 66 | 70 | 81 | 82 | 79 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 5201 | 1629 | 2987 | 2725 | 2942 | — |
| Alignment, degrees (A) | 13.88 | 9.82 | 9.87 | 9.02 | 8.33 | 9.16 |
| Mean angle deviation, degrees (F) | 4.48 | 6.93 | 4.82 | 4.65 | 6.17 | 6.74 |
| Angle deviation, 95th percentile (F) | 14.92 | 19.61 | 12.45 | 12.17 | 17.33 | 19.99 |
| Mean aspect ratio (F) | 1.145 | 1.269 | 1.271 | 1.263 | 1.480 | 1.386 |
| Area variation (F) | 0.140 | 0.230 | 0.265 | 0.329 | 0.354 | 0.285 |
| Hausdorff, ‰ of the diagonal (H) | 9.03 | 6.82 | 6.83 | 6.75 | 6.69 | 6.45 |
| Mean distance, ‰ (H) | 0.281 | 0.238 | 0.220 | 0.215 | 0.264 | 0.325 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Off budget, % (N) | 0.5 | 7.6 | 1.5 | 1.4 | 10.1 | 11.8 |
| Time, seconds (T) | 2.16 | 28.64 | 5.85 | 7.51 | 15.51 | 11.76 |

## part (mechanical)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 337 | 24 | 24 | 24 | 24 | 24 |
| … of which above the minimum | 329 | 16 | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 4678 | 128 | 429 | 92 | 730 | 730 |
| Alignment, degrees (A) | 13.52 | 11.84 | 11.83 | 10.33 | 10.87 | 10.87 |
| Mean angle deviation, degrees (F) | 4.52 | 1.66 | 1.25 | 1.00 | 3.63 | 3.63 |
| Angle deviation, 95th percentile (F) | 18.21 | 7.02 | 3.80 | 3.80 | 14.81 | 14.81 |
| Mean aspect ratio (F) | 1.178 | 1.151 | 1.093 | 1.129 | 1.301 | 1.301 |
| Area variation (F) | 0.166 | 0.105 | 0.126 | 0.328 | 0.244 | 0.244 |
| Hausdorff, ‰ of the diagonal (H) | 5.05 | 0.32 | 0.31 | 0.25 | 0.68 | 0.68 |
| Mean distance, ‰ (H) | 0.023 | 0.021 | 0.022 | 0.024 | 0.040 | 0.040 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.06 | 100.00 | 100.00 | 100.00 | 100.06 | 100.06 |
| Off budget, % (N) | 3.4 | 2.2 | 1.0 | 1.2 | 8.1 | 8.1 |
| Time, seconds (T) | 2.22 | 10.78 | 6.89 | 7.53 | 2.65 | 2.72 |

## cube (mechanical)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 18 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 10 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 376 | 8028 | 6 | 6 | 6 | 6 |
| Alignment, degrees (A) | 0.96 | 0.68 | 0.11 | 0.27 | 0.11 | 0.11 |
| Mean angle deviation, degrees (F) | 0.91 | 0.95 | 0.01 | 0.20 | 0.00 | 0.00 |
| Angle deviation, 95th percentile (F) | 2.60 | 2.23 | 0.02 | 0.74 | 0.00 | 0.00 |
| Mean aspect ratio (F) | 1.030 | 1.032 | 1.018 | 1.074 | 1.019 | 1.019 |
| Area variation (F) | 0.045 | 0.044 | 0.013 | 0.093 | 0.013 | 0.013 |
| Hausdorff, ‰ of the diagonal (H) | 0.00 | 5.40 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.000 | 0.046 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.99 | 100.00 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 2.5 | 0.4 | 0.8 | 0.8 | 1.0 | 1.0 |
| Time, seconds (T) | 0.78 | 79.70 | 4.99 | 4.25 | 3.50 | 2.90 |

## lblock (mechanical)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 24 | 12 | 12 | 12 | 12 | 12 |
| … of which above the minimum | 16 | 4 | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 706 | 14 | 14 | 14 | 14 | 14 |
| Alignment, degrees (A) | 1.17 | 0.57 | 0.61 | 0.92 | 0.54 | 0.54 |
| Mean angle deviation, degrees (F) | 0.88 | 0.01 | 0.00 | 0.49 | 0.00 | 0.00 |
| Angle deviation, 95th percentile (F) | 2.62 | 0.03 | 0.01 | 1.48 | 0.00 | 0.00 |
| Mean aspect ratio (F) | 1.025 | 1.000 | 1.000 | 1.078 | 1.000 | 1.000 |
| Area variation (F) | 0.041 | 0.000 | 0.000 | 0.096 | 0.000 | 0.000 |
| Hausdorff, ‰ of the diagonal (H) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 0.1 | 0.8 | 0.8 | 0.8 | 0.8 | 0.8 |
| Time, seconds (T) | 0.34 | 7.50 | 4.68 | 4.67 | 3.10 | 3.18 |

## cylinder (mechanical)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 165 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 157 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 4940 | 1346 | 182 | 1399 | 850 | 850 |
| Alignment, degrees (A) | 3.47 | 1.30 | 1.38 | 1.27 | 1.73 | 1.73 |
| Mean angle deviation, degrees (F) | 3.05 | 2.50 | 2.41 | 2.26 | 4.41 | 4.41 |
| Angle deviation, 95th percentile (F) | 11.70 | 13.39 | 10.19 | 10.24 | 18.99 | 18.99 |
| Mean aspect ratio (F) | 1.097 | 1.283 | 1.289 | 1.400 | 1.349 | 1.349 |
| Area variation (F) | 0.115 | 0.291 | 0.297 | 0.208 | 0.322 | 0.322 |
| Hausdorff, ‰ of the diagonal (H) | 2.07 | 0.54 | 0.53 | 0.55 | 0.73 | 0.73 |
| Mean distance, ‰ (H) | 0.045 | 0.070 | 0.064 | 0.073 | 0.147 | 0.147 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.96 | 100.00 | 100.00 | 100.00 | 99.83 | 99.83 |
| Off budget, % (N) | 0.5 | 8.5 | 0.1 | 1.3 | 13.5 | 13.5 |
| Time, seconds (T) | 0.57 | 8.05 | 5.47 | 6.17 | 4.42 | 4.47 |

## terrain (extra)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 273 | 123 | 124 | 225 | 296 | 253 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 3587 | 4449 | 5354 | 6757 | 8228 | 4351 |
| Alignment, degrees (A) | 21.43 | 17.91 | 17.81 | 17.56 | 18.10 | 18.39 |
| Mean angle deviation, degrees (F) | 7.45 | 9.64 | 6.73 | 7.17 | 8.69 | 7.98 |
| Angle deviation, 95th percentile (F) | 25.48 | 25.48 | 18.70 | 19.36 | 25.15 | 22.11 |
| Mean aspect ratio (F) | 1.264 | 1.329 | 1.295 | 1.313 | 1.668 | 1.342 |
| Area variation (F) | 0.215 | 0.290 | 0.328 | 0.485 | 0.518 | 0.237 |
| Hausdorff, ‰ of the diagonal (H) | 130.45 | 17.07 | 11.35 | 8.83 | 10.70 | 16.84 |
| Mean distance, ‰ (H) | 1.477 | 0.969 | 0.741 | 0.633 | 0.719 | 1.071 |
| Thin parts: mean distance, % of a quad edge (V) | 41.7 | 38.1 | 23.5 | 17.9 | 20.8 | 34.8 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Off budget, % (N) | 39.9 | 14.1 | 0.5 | 1.2 | 47.7 | 10.2 |
| Time, seconds (T) | 10.05 | 36.19 | 10.62 | 13.35 | 24.96 | 24.23 |

## mahaveer (extra)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | 420 | 173 | 552 | 651 | 3155 | 1146 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 4008 | — | — | — | — | — |
| Alignment, degrees (A) | 9.69 | 6.65 | 12.35 | 12.12 | 11.56 | 8.99 |
| Mean angle deviation, degrees (F) | 8.33 | 8.03 | 12.92 | 13.42 | 11.13 | 8.88 |
| Angle deviation, 95th percentile (F) | 32.25 | 25.50 | 59.15 | 61.85 | 36.88 | 32.39 |
| Mean aspect ratio (F) | 1.365 | 1.631 | 32.668 | 49.598 | 1.978 | 1.569 |
| Area variation (F) | 0.299 | 0.396 | 0.739 | 0.922 | 1.998 | 0.626 |
| Hausdorff, ‰ of the diagonal (H) | 32.21 | 15.22 | 15.73 | 15.73 | 13.01 | 18.76 |
| Mean distance, ‰ (H) | 3.712 | 2.734 | 1.145 | 1.061 | 0.784 | 1.567 |
| Thin parts: mean distance, % of a quad edge (V) | 60.8 | 45.7 | 18.5 | 16.9 | 17.5 | 26.0 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Off budget, % (N) | 33.2 | 27.8 | 1.8 | 3.6 | 589.8 | 9.9 |
| Time, seconds (T) | 6.80 | 95.64 | 16.38 | 19.06 | 13.92 | 11.71 |

## marine (extra)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | 1524 | — | 5777 | 6073 | 10419 | 5525 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 3685 | — | — | — | — | — |
| Alignment, degrees (A) | 17.92 | — | 12.86 | 13.98 | 10.30 | 13.42 |
| Mean angle deviation, degrees (F) | 20.55 | — | 21.66 | 22.47 | 8.58 | 15.13 |
| Angle deviation, 95th percentile (F) | 58.40 | — | 70.24 | 68.62 | 25.88 | 47.93 |
| Mean aspect ratio (F) | 3.745 | — | 5.844 | 7.404 | 1.549 | 1.767 |
| Area variation (F) | 0.689 | — | 1.173 | 1.308 | 1.812 | 1.232 |
| Hausdorff, ‰ of the diagonal (H) | 6.21 | — | 3.41 | 3.62 | 2.08 | 2.57 |
| Mean distance, ‰ (H) | 1.017 | — | 0.311 | 0.330 | 0.091 | 0.324 |
| Thin parts: mean distance, % of a quad edge (V) | 58.2 | — | 15.4 | 16.3 | 10.1 | 19.7 |
| Volume kept, % of the source (V) | — | — | — | — | — | — |
| Off budget, % (N) | 48.2 | — | 12.2 | 15.1 | 2975.8 | 86.4 |
| Time, seconds (T) | 4.87 | — | 43.38 | 57.07 | 13.22 | 13.40 |

Failures: integer-grid: error: invalid mesh: work mesh has 547 non-manifold or inconsistently oriented edges.

## sphere (calibration)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 193 | 24 | 28 | 28 | 8 | 8 |
| … of which above the minimum | 185 | 16 | 20 | 20 | 0 | 0 |
| Base complex patches (C) | 8190 | 7528 | 8035 | 8031 | 4992 | 4992 |
| Alignment, degrees (A) | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 3.49 | 7.08 | 4.96 | 5.09 | 7.65 | 7.65 |
| Angle deviation, 95th percentile (F) | 11.93 | 15.76 | 10.99 | 11.34 | 18.00 | 18.00 |
| Mean aspect ratio (F) | 1.097 | 1.156 | 1.145 | 1.141 | 1.187 | 1.187 |
| Area variation (F) | 0.122 | 0.155 | 0.171 | 0.175 | 0.391 | 0.391 |
| Hausdorff, ‰ of the diagonal (H) | 0.43 | 0.42 | 0.35 | 0.36 | 0.55 | 0.55 |
| Mean distance, ‰ (H) | 0.082 | 0.083 | 0.078 | 0.078 | 0.159 | 0.159 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.92 | 100.01 | 100.00 | 100.00 | 99.83 | 99.83 |
| Off budget, % (N) | 2.4 | 5.9 | 0.4 | 0.4 | 11.4 | 11.4 |
| Time, seconds (T) | 0.54 | 18.34 | 5.22 | 6.05 | 4.18 | 4.20 |

## torus (calibration)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 262 | 0 | 0 | 0 | 20 | 20 |
| … of which above the minimum | 262 | 0 | 0 | 0 | 20 | 20 |
| Base complex patches (C) | 7956 | 1 | 1 | 1 | 5983 | 5983 |
| Alignment, degrees (A) | 6.29 | 0.61 | 0.64 | 0.64 | 4.25 | 4.25 |
| Mean angle deviation, degrees (F) | 5.12 | 0.91 | 0.92 | 0.92 | 3.68 | 3.68 |
| Angle deviation, 95th percentile (F) | 15.85 | 1.44 | 1.40 | 1.40 | 10.81 | 10.81 |
| Mean aspect ratio (F) | 1.139 | 1.340 | 1.321 | 1.319 | 1.233 | 1.233 |
| Area variation (F) | 0.149 | 0.281 | 0.299 | 0.300 | 0.225 | 0.225 |
| Hausdorff, ‰ of the diagonal (H) | 0.65 | 0.34 | 0.35 | 0.35 | 0.69 | 0.69 |
| Mean distance, ‰ (H) | 0.119 | 0.111 | 0.104 | 0.104 | 0.222 | 0.222 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.79 | 100.01 | 100.01 | 100.01 | 99.56 | 99.56 |
| Off budget, % (N) | 0.5 | 9.0 | 0.3 | 0.3 | 7.5 | 7.5 |
| Time, seconds (T) | 0.91 | 5.85 | 4.31 | 4.88 | 6.20 | 5.73 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| Irregular vertices (S) | 0 | 9 | 5 | 1 | 4 | 4 |
| Base complex patches (C) | 1 | 4 | 2 | 2 | 0 | 4 |
| Alignment, degrees (A) | 0 | 1 | 1 | 3 | 8 | 0 |
| Mean angle deviation, degrees (F) | 9 | 0 | 3 | 0 | 1 | 0 |
| Mean aspect ratio (F) | 12 | 1 | 0 | 0 | 0 | 0 |
| Valid meshes (V) | 50/65 | 7/13 | 13/13 | 13/13 | 11/13 | 9/13 |

### mechanical (4 models)

| Metric | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| Irregular vertices (S) | 0 | 4 | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 0 | 1 | 3 | 3 | 2 | 2 |
| Alignment, degrees (A) | 0 | 0 | 0 | 2 | 2 | 2 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 2 | 2 | 2 |
| Mean aspect ratio (F) | 1 | 0 | 2 | 0 | 1 | 1 |
| Valid meshes (V) | 19/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 |
