# Bench results

Requested budget: 8000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. Lower is better for every metric; "—" means it could not be measured.

## blob (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 277 | 24 | 24 | 24 | 24 | 24 |
| … of which above the minimum | 269 | 16 | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 8164 | 5660 | 6808 | 6819 | 4563 | 2919 |
| Alignment, degrees (A) | 15.05 | 5.88 | 5.82 | 5.47 | 4.08 | 4.45 |
| Mean angle deviation, degrees (F) | 4.85 | 7.41 | 6.30 | 5.79 | 5.52 | 5.55 |
| Angle deviation, 95th percentile (F) | 15.77 | 19.76 | 17.21 | 15.38 | 14.94 | 14.71 |
| Mean aspect ratio (F) | 1.142 | 1.317 | 1.309 | 1.291 | 1.414 | 1.323 |
| Area variation (F) | 0.153 | 0.318 | 0.341 | 0.361 | 0.395 | 0.331 |
| Hausdorff, ‰ of the diagonal (H) | 4.97 | 3.05 | 2.49 | 2.21 | 3.00 | 3.66 |
| Mean distance, ‰ (H) | 0.178 | 0.204 | 0.177 | 0.174 | 0.185 | 0.216 |
| Off budget, % (N) | 2.1 | 10.9 | 0.9 | 0.3 | 1.1 | 12.1 |
| Time, seconds (T) | 1.58 | 13.05 | 4.67 | 4.37 | 12.37 | 12.61 |

## rock (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 162 | 114 | 116 | 130 | 68 | 68 |
| … of which above the minimum | 154 | 106 | 108 | 122 | 60 | 60 |
| Base complex patches (C) | 7792 | 7393 | 7943 | 7993 | 7071 | 6767 |
| Alignment, degrees (A) | 22.82 | 16.05 | 16.59 | 16.37 | 17.28 | 17.32 |
| Mean angle deviation, degrees (F) | 4.13 | 9.27 | 5.89 | 5.38 | 9.80 | 9.89 |
| Angle deviation, 95th percentile (F) | 12.43 | 22.21 | 13.90 | 12.91 | 23.73 | 23.67 |
| Mean aspect ratio (F) | 1.115 | 1.246 | 1.214 | 1.211 | 1.350 | 1.340 |
| Area variation (F) | 0.128 | 0.203 | 0.254 | 0.271 | 0.287 | 0.281 |
| Hausdorff, ‰ of the diagonal (H) | 3.22 | 3.88 | 3.12 | 3.01 | 4.18 | 4.36 |
| Mean distance, ‰ (H) | 0.433 | 0.453 | 0.426 | 0.416 | 0.515 | 0.530 |
| Off budget, % (N) | 2.6 | 7.6 | 0.7 | 0.1 | 11.6 | 15.4 |
| Time, seconds (T) | 1.07 | 38.83 | 4.59 | 4.57 | 18.31 | 18.00 |

## creature (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 214 | 40 | 40 | 42 | 40 | 40 |
| … of which above the minimum | 206 | — | 32 | 34 | 32 | 32 |
| Base complex patches (C) | 7510 | 2490 | 2864 | 3225 | 3236 | 3467 |
| Alignment, degrees (A) | 11.81 | 6.40 | 6.02 | 5.63 | 6.64 | 6.66 |
| Mean angle deviation, degrees (F) | 3.91 | 8.74 | 6.71 | 5.60 | 6.13 | 6.16 |
| Angle deviation, 95th percentile (F) | 12.20 | 25.48 | 18.89 | 14.96 | 17.40 | 17.67 |
| Mean aspect ratio (F) | 1.116 | 1.351 | 1.343 | 1.321 | 1.277 | 1.278 |
| Area variation (F) | 0.127 | 0.334 | 0.365 | 0.471 | 0.277 | 0.275 |
| Hausdorff, ‰ of the diagonal (H) | 1.70 | 2.35 | 1.48 | 1.54 | 1.24 | 1.34 |
| Mean distance, ‰ (H) | 0.106 | 0.143 | 0.124 | 0.118 | 0.144 | 0.145 |
| Off budget, % (N) | 2.9 | 13.5 | 1.5 | 2.2 | 10.2 | 9.9 |
| Time, seconds (T) | 0.82 | 24.73 | 9.72 | 11.17 | 12.49 | 8.52 |

## cage (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 250 | 40 | 48 | 48 | 62 | 64 |
| … of which above the minimum | 218 | 8 | 16 | 16 | 30 | 32 |
| Base complex patches (C) | 7125 | 766 | 1654 | 556 | 3227 | 1954 |
| Alignment, degrees (A) | 8.80 | 2.69 | 2.47 | 2.18 | 2.06 | 2.29 |
| Mean angle deviation, degrees (F) | 4.40 | 3.56 | 2.12 | 2.50 | 3.22 | 3.48 |
| Angle deviation, 95th percentile (F) | 14.23 | 10.68 | 6.66 | 7.09 | 9.04 | 10.22 |
| Mean aspect ratio (F) | 1.144 | 1.202 | 1.197 | 1.229 | 1.693 | 1.289 |
| Area variation (F) | 0.138 | 0.189 | 0.214 | 0.333 | 0.338 | 0.229 |
| Hausdorff, ‰ of the diagonal (H) | 9.87 | 6.72 | 4.65 | 3.75 | 4.53 | 5.93 |
| Mean distance, ‰ (H) | 0.344 | 0.349 | 0.291 | 0.253 | 0.451 | 0.614 |
| Off budget, % (N) | 0.6 | 2.9 | 1.4 | 0.4 | 37.6 | 6.1 |
| Time, seconds (T) | 0.51 | 25.83 | 3.77 | 4.23 | 5.76 | 4.92 |

## suzanne (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 346 | 67 | 64 | 78 | 78 | 72 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 7878 | 4267 | 3264 | 4489 | 4088 | 3340 |
| Alignment, degrees (A) | 16.64 | 9.92 | 9.96 | 9.99 | 8.58 | 9.86 |
| Mean angle deviation, degrees (F) | 5.17 | 7.61 | 5.34 | 5.94 | 7.04 | 6.91 |
| Angle deviation, 95th percentile (F) | 17.78 | 21.30 | 15.16 | 15.35 | 18.57 | 19.33 |
| Mean aspect ratio (F) | 1.169 | 1.225 | 1.197 | 1.231 | 1.470 | 1.246 |
| Area variation (F) | 0.164 | 0.219 | 0.262 | 0.383 | 0.366 | 0.248 |
| Hausdorff, ‰ of the diagonal (H) | 5.94 | 5.71 | 5.82 | 4.23 | 3.57 | 4.99 |
| Mean distance, ‰ (H) | 0.286 | 0.328 | 0.281 | 0.231 | 0.322 | 0.422 |
| Off budget, % (N) | 1.2 | 7.8 | 1.8 | 1.3 | 15.6 | 8.5 |
| Time, seconds (T) | 0.53 | 78.26 | 4.90 | 6.40 | 7.68 | 4.94 |

## ogre (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 455 | 169 | 202 | 228 | 338 | 322 |
| … of which above the minimum | 429 | — | 194 | 220 | 330 | — |
| Base complex patches (C) | 7656 | — | 6252 | 6062 | 9903 | — |
| Alignment, degrees (A) | 17.72 | 11.52 | 11.33 | 10.83 | 8.50 | 9.79 |
| Mean angle deviation, degrees (F) | 6.18 | 8.65 | 6.28 | 6.25 | 7.95 | 8.39 |
| Angle deviation, 95th percentile (F) | 20.84 | 23.63 | 16.97 | 16.59 | 22.55 | 22.69 |
| Mean aspect ratio (F) | 1.225 | 1.399 | 1.377 | 1.376 | 2.025 | 1.482 |
| Area variation (F) | 0.196 | 0.302 | 0.327 | 0.437 | 0.643 | 0.364 |
| Hausdorff, ‰ of the diagonal (H) | 30.74 | 22.53 | 12.97 | 7.86 | 5.23 | 25.57 |
| Mean distance, ‰ (H) | 0.601 | 0.535 | 0.433 | 0.398 | 0.346 | 0.548 |
| Off budget, % (N) | 2.1 | 12.1 | 0.4 | 0.7 | 59.0 | 15.5 |
| Time, seconds (T) | 2.81 | 34.15 | 8.72 | 10.52 | 23.50 | 24.03 |

## spot (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 286 | 80 | 80 | 78 | 76 | 80 |
| … of which above the minimum | 278 | 72 | 72 | 70 | 68 | 72 |
| Base complex patches (C) | 7942 | 3692 | 5329 | 5007 | 4605 | 3862 |
| Alignment, degrees (A) | 16.86 | 8.48 | 7.92 | 7.55 | 7.56 | 8.42 |
| Mean angle deviation, degrees (F) | 4.47 | 9.08 | 6.47 | 6.50 | 5.56 | 6.01 |
| Angle deviation, 95th percentile (F) | 14.93 | 23.37 | 16.91 | 17.46 | 14.90 | 15.81 |
| Mean aspect ratio (F) | 1.136 | 1.348 | 1.326 | 1.330 | 1.342 | 1.289 |
| Area variation (F) | 0.144 | 0.299 | 0.332 | 0.431 | 0.301 | 0.256 |
| Hausdorff, ‰ of the diagonal (H) | 7.10 | 3.60 | 3.01 | 2.40 | 4.73 | 5.88 |
| Mean distance, ‰ (H) | 0.172 | 0.184 | 0.162 | 0.155 | 0.169 | 0.187 |
| Off budget, % (N) | 1.2 | 11.5 | 0.4 | 1.2 | 0.4 | 9.1 |
| Time, seconds (T) | 1.12 | 38.48 | 9.60 | 12.57 | 17.71 | 17.39 |

## bob (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 245 | 18 | 20 | 20 | 34 | 30 |
| … of which above the minimum | 245 | — | 20 | 20 | 34 | 30 |
| Base complex patches (C) | 8072 | 1541 | 1279 | 2898 | 4859 | 3789 |
| Alignment, degrees (A) | 20.15 | 4.10 | 3.64 | 3.75 | 5.48 | 5.82 |
| Mean angle deviation, degrees (F) | 4.31 | 4.94 | 3.64 | 3.76 | 6.43 | 6.86 |
| Angle deviation, 95th percentile (F) | 14.25 | 16.53 | 11.16 | 11.87 | 18.71 | 21.48 |
| Mean aspect ratio (F) | 1.124 | 1.407 | 1.402 | 1.380 | 1.407 | 1.407 |
| Area variation (F) | 0.139 | 0.356 | 0.374 | 0.432 | 0.369 | 0.373 |
| Hausdorff, ‰ of the diagonal (H) | 2.84 | 3.52 | 0.91 | 1.28 | 1.50 | 1.30 |
| Mean distance, ‰ (H) | 0.120 | 0.143 | 0.126 | 0.126 | 0.150 | 0.153 |
| Off budget, % (N) | 0.9 | 11.8 | 0.4 | 0.2 | 13.8 | 14.6 |
| Time, seconds (T) | 0.97 | 20.35 | 8.19 | 5.57 | 20.38 | 16.98 |

## blub (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 280 | 48 | 48 | 56 | 51 | 48 |
| … of which above the minimum | 272 | 40 | 40 | 48 | 43 | 40 |
| Base complex patches (C) | 7910 | 2997 | 3338 | 2980 | 6215 | 4909 |
| Alignment, degrees (A) | 14.61 | 7.27 | 7.01 | 6.42 | 6.32 | 7.75 |
| Mean angle deviation, degrees (F) | 4.50 | 8.90 | 6.55 | 6.03 | 6.44 | 7.53 |
| Angle deviation, 95th percentile (F) | 15.34 | 24.31 | 18.56 | 16.78 | 16.31 | 19.46 |
| Mean aspect ratio (F) | 1.139 | 1.333 | 1.340 | 1.354 | 1.595 | 1.351 |
| Area variation (F) | 0.144 | 0.268 | 0.295 | 0.401 | 0.365 | 0.270 |
| Hausdorff, ‰ of the diagonal (H) | 5.99 | 5.60 | 4.70 | 3.28 | 4.29 | 8.04 |
| Mean distance, ‰ (H) | 0.132 | 0.156 | 0.134 | 0.116 | 0.128 | 0.176 |
| Off budget, % (N) | 1.2 | 11.8 | 0.1 | 0.6 | 6.4 | 13.2 |
| Time, seconds (T) | 1.37 | 16.96 | 6.58 | 6.48 | 15.56 | 13.36 |

## pig (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 370 | 81 | 88 | 94 | 101 | 98 |
| … of which above the minimum | 362 | — | 80 | 86 | — | 90 |
| Base complex patches (C) | 7826 | 4511 | 5985 | 4984 | — | 4284 |
| Alignment, degrees (A) | 14.90 | 9.14 | 9.39 | 8.10 | 7.39 | 8.84 |
| Mean angle deviation, degrees (F) | 5.04 | 8.76 | 6.99 | 5.57 | 6.40 | 6.59 |
| Angle deviation, 95th percentile (F) | 17.35 | 24.69 | 19.74 | 17.39 | 18.75 | 19.80 |
| Mean aspect ratio (F) | 1.161 | 1.356 | 1.338 | 1.302 | 1.546 | 1.329 |
| Area variation (F) | 0.161 | 0.312 | 0.332 | 0.499 | 0.462 | 0.260 |
| Hausdorff, ‰ of the diagonal (H) | 21.49 | 8.71 | 8.30 | 7.02 | 9.11 | 9.71 |
| Mean distance, ‰ (H) | 0.238 | 0.258 | 0.224 | 0.209 | 0.222 | 0.301 |
| Off budget, % (N) | 1.5 | 10.1 | 1.2 | 1.8 | 22.8 | 11.6 |
| Time, seconds (T) | 1.17 | 35.75 | 9.44 | 11.05 | 16.84 | 16.13 |

## jerry (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 490 | 170 | 166 | 182 | 244 | 247 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 7797 | — | 5332 | 7034 | 11709 | — |
| Alignment, degrees (A) | 18.65 | 13.30 | 12.91 | 12.33 | 10.64 | 12.69 |
| Mean angle deviation, degrees (F) | 6.63 | 8.69 | 5.26 | 5.67 | 8.15 | 8.38 |
| Angle deviation, 95th percentile (F) | 23.27 | 23.17 | 14.66 | 15.37 | 22.38 | 23.18 |
| Mean aspect ratio (F) | 1.241 | 1.291 | 1.250 | 1.256 | 1.879 | 1.380 |
| Area variation (F) | 0.210 | 0.261 | 0.316 | 0.483 | 0.651 | 0.279 |
| Hausdorff, ‰ of the diagonal (H) | 29.67 | 18.98 | 15.03 | 11.51 | 4.22 | 10.31 |
| Mean distance, ‰ (H) | 0.629 | 0.601 | 0.473 | 0.381 | 0.310 | 0.598 |
| Off budget, % (N) | 0.8 | 8.2 | 1.6 | 1.0 | 71.4 | 13.2 |
| Time, seconds (T) | 0.67 | 46.08 | 5.34 | 6.81 | 9.49 | 8.46 |

## rose (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | 1126 | 557 | 557 | 557 | 1899 | 1164 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 6434 | — | — | — | — | — |
| Alignment, degrees (A) | 18.49 | 14.24 | 14.24 | 14.24 | 9.05 | 12.40 |
| Mean angle deviation, degrees (F) | 12.06 | 10.21 | 10.21 | 10.21 | 8.61 | 11.61 |
| Angle deviation, 95th percentile (F) | 40.69 | 29.73 | 29.73 | 29.73 | 24.64 | 31.66 |
| Mean aspect ratio (F) | 1.595 | 1.413 | 1.413 | 1.413 | 1.839 | 1.518 |
| Area variation (F) | 0.359 | 0.331 | 0.331 | 0.331 | 1.220 | 0.433 |
| Hausdorff, ‰ of the diagonal (H) | 72.22 | 74.98 | 74.98 | 74.98 | 73.68 | 75.13 |
| Mean distance, ‰ (H) | 1.750 | 1.548 | 1.548 | 1.548 | 0.689 | 1.593 |
| Off budget, % (N) | 13.4 | 17.9 | 17.9 | 17.9 | 573.4 | 13.8 |
| Time, seconds (T) | 1.60 | 160.12 | 152.10 | 124.82 | 9.67 | 7.85 |

## bridge (organic)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 258 | 66 | 70 | 86 | 82 | 79 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 5201 | 1629 | 2987 | 2286 | 2942 | — |
| Alignment, degrees (A) | 13.88 | 9.82 | 9.87 | 8.88 | 8.33 | 9.16 |
| Mean angle deviation, degrees (F) | 4.48 | 6.90 | 4.77 | 4.33 | 6.17 | 6.74 |
| Angle deviation, 95th percentile (F) | 14.92 | 19.61 | 12.29 | 11.70 | 17.33 | 19.99 |
| Mean aspect ratio (F) | 1.145 | 1.265 | 1.264 | 1.254 | 1.480 | 1.386 |
| Area variation (F) | 0.140 | 0.231 | 0.265 | 0.342 | 0.354 | 0.285 |
| Hausdorff, ‰ of the diagonal (H) | 9.03 | 6.77 | 6.70 | 6.54 | 6.69 | 6.45 |
| Mean distance, ‰ (H) | 0.281 | 0.301 | 0.277 | 0.274 | 0.264 | 0.325 |
| Off budget, % (N) | 0.5 | 7.6 | 1.5 | 1.8 | 10.1 | 11.8 |
| Time, seconds (T) | 1.86 | 24.78 | 4.90 | 4.17 | 15.51 | 11.76 |

## part (mechanical)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 337 | 24 | 24 | 24 | 24 | 24 |
| … of which above the minimum | 329 | 16 | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 4678 | 128 | 78 | 92 | 730 | 730 |
| Alignment, degrees (A) | 13.52 | 11.85 | 11.90 | 10.36 | 10.87 | 10.87 |
| Mean angle deviation, degrees (F) | 4.52 | 1.66 | 1.09 | 1.03 | 3.63 | 3.63 |
| Angle deviation, 95th percentile (F) | 18.21 | 7.02 | 3.79 | 3.87 | 14.81 | 14.81 |
| Mean aspect ratio (F) | 1.178 | 1.151 | 1.102 | 1.136 | 1.301 | 1.301 |
| Area variation (F) | 0.166 | 0.105 | 0.117 | 0.321 | 0.244 | 0.244 |
| Hausdorff, ‰ of the diagonal (H) | 5.05 | 0.42 | 0.44 | 0.36 | 0.68 | 0.68 |
| Mean distance, ‰ (H) | 0.023 | 0.021 | 0.021 | 0.021 | 0.040 | 0.040 |
| Off budget, % (N) | 3.4 | 2.2 | 1.4 | 0.7 | 8.1 | 8.1 |
| Time, seconds (T) | 2.42 | 15.27 | 4.41 | 5.12 | 2.65 | 2.72 |

## cube (mechanical)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 18 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 10 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 376 | 8028 | 6 | 6 | 6 | 6 |
| Alignment, degrees (A) | 0.96 | 0.67 | 0.11 | 0.27 | 0.11 | 0.11 |
| Mean angle deviation, degrees (F) | 0.91 | 0.95 | 0.01 | 0.20 | 0.00 | 0.00 |
| Angle deviation, 95th percentile (F) | 2.60 | 2.22 | 0.02 | 0.74 | 0.00 | 0.00 |
| Mean aspect ratio (F) | 1.030 | 1.034 | 1.018 | 1.074 | 1.019 | 1.019 |
| Area variation (F) | 0.045 | 0.047 | 0.013 | 0.093 | 0.013 | 0.013 |
| Hausdorff, ‰ of the diagonal (H) | 0.00 | 6.86 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.000 | 0.030 | 0.000 | 0.000 | 0.000 | 0.000 |
| Off budget, % (N) | 2.5 | 0.4 | 0.8 | 0.8 | 1.0 | 1.0 |
| Time, seconds (T) | 0.86 | 71.95 | 4.46 | 3.90 | 3.50 | 2.90 |

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
| Off budget, % (N) | 0.1 | 0.8 | 0.8 | 0.8 | 0.8 | 0.8 |
| Time, seconds (T) | 1.34 | 4.68 | 3.65 | 3.16 | 3.10 | 3.18 |

## cylinder (mechanical)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 165 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 157 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 4940 | 1346 | 1478 | 1399 | 850 | 850 |
| Alignment, degrees (A) | 3.47 | 1.31 | 1.34 | 1.28 | 1.73 | 1.73 |
| Mean angle deviation, degrees (F) | 3.05 | 2.50 | 2.28 | 2.26 | 4.41 | 4.41 |
| Angle deviation, 95th percentile (F) | 11.70 | 13.39 | 10.13 | 10.19 | 18.99 | 18.99 |
| Mean aspect ratio (F) | 1.097 | 1.283 | 1.289 | 1.400 | 1.349 | 1.349 |
| Area variation (F) | 0.115 | 0.291 | 0.297 | 0.209 | 0.322 | 0.322 |
| Hausdorff, ‰ of the diagonal (H) | 2.07 | 0.54 | 0.52 | 0.56 | 0.73 | 0.73 |
| Mean distance, ‰ (H) | 0.045 | 0.062 | 0.055 | 0.059 | 0.147 | 0.147 |
| Off budget, % (N) | 0.5 | 8.5 | 0.1 | 1.3 | 13.5 | 13.5 |
| Time, seconds (T) | 0.79 | 9.27 | 4.74 | 6.01 | 4.42 | 4.47 |

## terrain (extra)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 273 | 123 | 124 | 148 | 296 | 253 |
| … of which above the minimum | — | — | — | — | — | — |
| Base complex patches (C) | 3587 | 4449 | 5354 | 6466 | 8228 | 4351 |
| Alignment, degrees (A) | 21.43 | 17.78 | 17.76 | 17.69 | 18.10 | 18.39 |
| Mean angle deviation, degrees (F) | 7.45 | 9.69 | 6.62 | 6.61 | 8.69 | 7.98 |
| Angle deviation, 95th percentile (F) | 25.48 | 25.63 | 18.39 | 17.50 | 25.15 | 22.11 |
| Mean aspect ratio (F) | 1.264 | 1.334 | 1.292 | 1.299 | 1.668 | 1.342 |
| Area variation (F) | 0.215 | 0.298 | 0.333 | 0.476 | 0.518 | 0.237 |
| Hausdorff, ‰ of the diagonal (H) | 130.45 | 17.98 | 11.92 | 11.03 | 10.70 | 16.84 |
| Mean distance, ‰ (H) | 1.477 | 1.192 | 0.872 | 0.766 | 0.719 | 1.071 |
| Off budget, % (N) | 39.9 | 14.1 | 0.5 | 1.8 | 47.7 | 10.2 |
| Time, seconds (T) | 8.99 | 34.78 | 11.12 | 11.96 | 24.96 | 24.23 |

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
| Hausdorff, ‰ of the diagonal (H) | 0.43 | 0.64 | 0.47 | 0.47 | 0.55 | 0.55 |
| Mean distance, ‰ (H) | 0.082 | 0.090 | 0.084 | 0.084 | 0.159 | 0.159 |
| Off budget, % (N) | 2.4 | 5.9 | 0.4 | 0.4 | 11.4 | 11.4 |
| Time, seconds (T) | 0.36 | 14.63 | 4.49 | 4.42 | 4.18 | 4.20 |

## torus (calibration)

| | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 262 | 0 | 0 | 0 | 20 | 20 |
| … of which above the minimum | 262 | 0 | 0 | 0 | 20 | 20 |
| Base complex patches (C) | 7956 | 1 | 1 | 1 | 5983 | 5983 |
| Alignment, degrees (A) | 6.29 | 0.61 | 0.61 | 0.61 | 4.25 | 4.25 |
| Mean angle deviation, degrees (F) | 5.12 | 0.91 | 0.91 | 0.91 | 3.68 | 3.68 |
| Angle deviation, 95th percentile (F) | 15.85 | 1.44 | 1.44 | 1.44 | 10.81 | 10.81 |
| Mean aspect ratio (F) | 1.139 | 1.340 | 1.340 | 1.340 | 1.233 | 1.233 |
| Area variation (F) | 0.149 | 0.280 | 0.280 | 0.280 | 0.225 | 0.225 |
| Hausdorff, ‰ of the diagonal (H) | 0.65 | 0.61 | 0.61 | 0.61 | 0.69 | 0.69 |
| Mean distance, ‰ (H) | 0.119 | 0.133 | 0.133 | 0.133 | 0.222 | 0.222 |
| Off budget, % (N) | 0.5 | 9.0 | 9.0 | 9.0 | 7.5 | 7.5 |
| Time, seconds (T) | 0.75 | 6.61 | 3.65 | 3.44 | 6.20 | 5.73 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| Irregular vertices (S) | 0 | 9 | 6 | 2 | 4 | 4 |
| Base complex patches (C) | 1 | 3 | 3 | 3 | 0 | 3 |
| Alignment, degrees (A) | 0 | 1 | 1 | 2 | 9 | 0 |
| Mean angle deviation, degrees (F) | 8 | 0 | 3 | 1 | 1 | 0 |
| Mean aspect ratio (F) | 12 | 1 | 1 | 1 | 0 | 0 |
| Valid meshes (V) | 50/65 | 7/13 | 12/13 | 12/13 | 11/13 | 9/13 |

### mechanical (4 models)

| Metric | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|
| Irregular vertices (S) | 0 | 4 | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 0 | 1 | 3 | 2 | 3 | 3 |
| Alignment, degrees (A) | 0 | 0 | 0 | 2 | 2 | 2 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 2 | 2 | 2 |
| Mean aspect ratio (F) | 1 | 0 | 2 | 0 | 1 | 1 |
| Valid meshes (V) | 19/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 |
