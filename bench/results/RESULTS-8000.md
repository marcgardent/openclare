# Bench results

Requested budget: 8000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. Lower is better for every metric; "—" means it could not be measured.

## blob (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 40 | 277 | 24 | 24 | 24 | 24 | 24 |
| … of which above the minimum | 32 | 269 | 16 | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 7412 | 8164 | 5660 | 6808 | 6819 | 4563 | 2919 |
| Alignment, degrees (A) | 14.77 | 15.05 | 5.88 | 5.82 | 5.47 | 4.08 | 4.45 |
| Mean angle deviation, degrees (F) | 7.77 | 4.85 | 7.41 | 6.30 | 5.79 | 5.52 | 5.55 |
| Angle deviation, 95th percentile (F) | 19.95 | 15.77 | 19.76 | 17.21 | 15.38 | 14.94 | 14.71 |
| Mean aspect ratio (F) | 1.152 | 1.142 | 1.317 | 1.309 | 1.291 | 1.414 | 1.323 |
| Area variation (F) | 0.177 | 0.153 | 0.318 | 0.341 | 0.361 | 0.395 | 0.331 |
| Hausdorff, ‰ of the diagonal (H) | 5.18 | 4.97 | 3.05 | 2.49 | 2.21 | 3.00 | 3.66 |
| Mean distance, ‰ (H) | 0.171 | 0.178 | 0.204 | 0.177 | 0.174 | 0.185 | 0.216 |
| Off budget, % (N) | 5.1 | 2.1 | 10.9 | 0.9 | 0.3 | 1.1 | 12.1 |
| Time, seconds (T) | 3.90 | 1.58 | 13.05 | 5.20 | 4.63 | 12.37 | 12.61 |

## rock (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 10 | 162 | 114 | 116 | 130 | 68 | 68 |
| … of which above the minimum | 2 | 154 | 106 | 108 | 122 | 60 | 60 |
| Base complex patches (C) | 7342 | 7792 | 7393 | 7943 | 7993 | 7071 | 6767 |
| Alignment, degrees (A) | 22.36 | 22.82 | 16.05 | 16.59 | 16.37 | 17.28 | 17.32 |
| Mean angle deviation, degrees (F) | 6.22 | 4.13 | 9.27 | 5.89 | 5.38 | 9.80 | 9.89 |
| Angle deviation, 95th percentile (F) | 18.32 | 12.43 | 22.21 | 13.90 | 12.91 | 23.73 | 23.67 |
| Mean aspect ratio (F) | 1.158 | 1.115 | 1.246 | 1.214 | 1.211 | 1.350 | 1.340 |
| Area variation (F) | 0.244 | 0.128 | 0.203 | 0.254 | 0.271 | 0.287 | 0.281 |
| Hausdorff, ‰ of the diagonal (H) | 6.95 | 3.22 | 3.88 | 3.12 | 3.01 | 4.18 | 4.36 |
| Mean distance, ‰ (H) | 0.478 | 0.433 | 0.453 | 0.426 | 0.416 | 0.515 | 0.530 |
| Off budget, % (N) | 8.2 | 2.6 | 7.6 | 0.7 | 0.1 | 11.6 | 15.4 |
| Time, seconds (T) | 3.68 | 1.07 | 38.83 | 4.63 | 4.63 | 18.31 | 18.00 |

## creature (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 22 | 214 | 40 | 40 | 42 | 40 | 40 |
| … of which above the minimum | 14 | 206 | — | 32 | 34 | 32 | 32 |
| Base complex patches (C) | 8170 | 7510 | 2490 | 2864 | 3225 | 3236 | 3467 |
| Alignment, degrees (A) | 13.70 | 11.81 | 6.40 | 6.02 | 5.63 | 6.64 | 6.66 |
| Mean angle deviation, degrees (F) | 6.02 | 3.91 | 8.74 | 6.71 | 5.60 | 6.13 | 6.16 |
| Angle deviation, 95th percentile (F) | 15.98 | 12.20 | 25.48 | 18.89 | 14.96 | 17.40 | 17.67 |
| Mean aspect ratio (F) | 1.154 | 1.116 | 1.351 | 1.343 | 1.321 | 1.277 | 1.278 |
| Area variation (F) | 0.157 | 0.127 | 0.334 | 0.365 | 0.471 | 0.277 | 0.275 |
| Hausdorff, ‰ of the diagonal (H) | 3.19 | 1.70 | 2.35 | 1.48 | 1.54 | 1.24 | 1.34 |
| Mean distance, ‰ (H) | 0.100 | 0.106 | 0.143 | 0.124 | 0.118 | 0.144 | 0.145 |
| Off budget, % (N) | 2.1 | 2.9 | 13.5 | 1.5 | 2.2 | 10.2 | 9.9 |
| Time, seconds (T) | 5.87 | 0.82 | 24.73 | 12.74 | 14.26 | 12.49 | 8.52 |

## cage (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 72 | 250 | 40 | 48 | 48 | 62 | 64 |
| … of which above the minimum | 40 | 218 | 8 | 16 | 16 | 30 | 32 |
| Base complex patches (C) | 7283 | 7125 | 766 | 1654 | 556 | 3227 | 1954 |
| Alignment, degrees (A) | 8.22 | 8.80 | 2.69 | 2.47 | 2.18 | 2.06 | 2.29 |
| Mean angle deviation, degrees (F) | 5.83 | 4.40 | 3.56 | 2.12 | 2.50 | 3.22 | 3.48 |
| Angle deviation, 95th percentile (F) | 16.15 | 14.23 | 10.68 | 6.66 | 7.09 | 9.04 | 10.22 |
| Mean aspect ratio (F) | 1.140 | 1.144 | 1.202 | 1.197 | 1.229 | 1.693 | 1.289 |
| Area variation (F) | 0.139 | 0.138 | 0.189 | 0.214 | 0.333 | 0.338 | 0.229 |
| Hausdorff, ‰ of the diagonal (H) | 6.65 | 9.87 | 6.72 | 4.65 | 3.75 | 4.53 | 5.93 |
| Mean distance, ‰ (H) | 0.294 | 0.344 | 0.349 | 0.291 | 0.253 | 0.451 | 0.614 |
| Off budget, % (N) | 0.9 | 0.6 | 2.9 | 1.4 | 0.4 | 37.6 | 6.1 |
| Time, seconds (T) | 4.81 | 0.51 | 25.83 | 6.79 | 5.08 | 5.76 | 4.92 |

## suzanne (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 55 | 346 | 67 | 64 | 78 | 78 | 72 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | 6194 | 7878 | 4267 | 3543 | 4489 | 4088 | 3340 |
| Alignment, degrees (A) | 14.27 | 16.64 | 9.92 | 10.01 | 10.01 | 8.58 | 9.86 |
| Mean angle deviation, degrees (F) | 6.09 | 5.17 | 7.61 | 5.50 | 5.96 | 7.04 | 6.91 |
| Angle deviation, 95th percentile (F) | 18.06 | 17.78 | 21.30 | 15.47 | 15.39 | 18.57 | 19.33 |
| Mean aspect ratio (F) | 1.208 | 1.169 | 1.225 | 1.213 | 1.232 | 1.470 | 1.246 |
| Area variation (F) | 0.208 | 0.164 | 0.219 | 0.256 | 0.382 | 0.366 | 0.248 |
| Hausdorff, ‰ of the diagonal (H) | 6.87 | 5.94 | 5.71 | 4.99 | 4.23 | 3.57 | 4.99 |
| Mean distance, ‰ (H) | 0.267 | 0.286 | 0.328 | 0.280 | 0.232 | 0.322 | 0.422 |
| Off budget, % (N) | 0.9 | 1.2 | 7.8 | 0.4 | 1.3 | 15.6 | 8.5 |
| Time, seconds (T) | 6.70 | 0.53 | 78.26 | 5.61 | 6.90 | 7.68 | 4.94 |

## ogre (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 162 | 455 | 169 | 202 | 228 | 338 | 322 |
| … of which above the minimum | 154 | 429 | — | 194 | 220 | 330 | — |
| Base complex patches (C) | 7352 | 7656 | — | 6252 | 6062 | 9903 | — |
| Alignment, degrees (A) | 17.78 | 17.72 | 11.52 | 11.33 | 10.83 | 8.50 | 9.79 |
| Mean angle deviation, degrees (F) | 11.59 | 6.18 | 8.65 | 6.28 | 6.25 | 7.95 | 8.39 |
| Angle deviation, 95th percentile (F) | 33.33 | 20.84 | 23.63 | 16.97 | 16.59 | 22.55 | 22.69 |
| Mean aspect ratio (F) | 1.339 | 1.225 | 1.399 | 1.377 | 1.376 | 2.025 | 1.482 |
| Area variation (F) | 0.312 | 0.196 | 0.302 | 0.327 | 0.437 | 0.643 | 0.364 |
| Hausdorff, ‰ of the diagonal (H) | 27.28 | 30.74 | 22.53 | 12.97 | 7.86 | 5.23 | 25.57 |
| Mean distance, ‰ (H) | 0.521 | 0.601 | 0.535 | 0.433 | 0.398 | 0.346 | 0.548 |
| Off budget, % (N) | 3.0 | 2.1 | 12.1 | 0.4 | 0.7 | 59.0 | 15.5 |
| Time, seconds (T) | 4.53 | 2.81 | 34.15 | 10.62 | 11.23 | 23.50 | 24.03 |

## spot (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 76 | 286 | 80 | 80 | 78 | 76 | 80 |
| … of which above the minimum | 68 | 278 | 72 | 72 | 70 | 68 | 72 |
| Base complex patches (C) | 7772 | 7942 | 3692 | 5329 | 5007 | 4605 | 3862 |
| Alignment, degrees (A) | 16.85 | 16.86 | 8.48 | 7.92 | 7.55 | 7.56 | 8.42 |
| Mean angle deviation, degrees (F) | 6.23 | 4.47 | 9.08 | 6.47 | 6.50 | 5.56 | 6.01 |
| Angle deviation, 95th percentile (F) | 16.10 | 14.93 | 23.37 | 16.91 | 17.46 | 14.90 | 15.81 |
| Mean aspect ratio (F) | 1.177 | 1.136 | 1.348 | 1.326 | 1.330 | 1.342 | 1.289 |
| Area variation (F) | 0.178 | 0.144 | 0.299 | 0.332 | 0.431 | 0.301 | 0.256 |
| Hausdorff, ‰ of the diagonal (H) | 5.30 | 7.10 | 3.60 | 3.01 | 2.40 | 4.73 | 5.88 |
| Mean distance, ‰ (H) | 0.161 | 0.172 | 0.184 | 0.162 | 0.155 | 0.169 | 0.187 |
| Off budget, % (N) | 1.6 | 1.2 | 11.5 | 0.4 | 1.2 | 0.4 | 9.1 |
| Time, seconds (T) | 3.24 | 1.12 | 38.48 | 9.63 | 11.70 | 17.71 | 17.39 |

## bob (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 5/5 | 0/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 36 | 245 | 18 | 20 | 20 | 34 | 30 |
| … of which above the minimum | 36 | 245 | — | 20 | 20 | 34 | 30 |
| Base complex patches (C) | 8263 | 8072 | 1541 | 1279 | 2898 | 4859 | 3789 |
| Alignment, degrees (A) | 17.31 | 20.15 | 4.10 | 3.64 | 3.75 | 5.48 | 5.82 |
| Mean angle deviation, degrees (F) | 6.39 | 4.31 | 4.94 | 3.64 | 3.76 | 6.43 | 6.86 |
| Angle deviation, 95th percentile (F) | 17.71 | 14.25 | 16.53 | 11.16 | 11.87 | 18.71 | 21.48 |
| Mean aspect ratio (F) | 1.142 | 1.124 | 1.407 | 1.402 | 1.380 | 1.407 | 1.407 |
| Area variation (F) | 0.159 | 0.139 | 0.356 | 0.374 | 0.432 | 0.369 | 0.373 |
| Hausdorff, ‰ of the diagonal (H) | 3.23 | 2.84 | 3.52 | 0.91 | 1.28 | 1.50 | 1.30 |
| Mean distance, ‰ (H) | 0.104 | 0.120 | 0.143 | 0.126 | 0.126 | 0.150 | 0.153 |
| Off budget, % (N) | 3.3 | 0.9 | 11.8 | 0.4 | 0.2 | 13.8 | 14.6 |
| Time, seconds (T) | 3.67 | 0.97 | 20.35 | 8.53 | 5.61 | 20.38 | 16.98 |

## blub (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 70 | 280 | 48 | 48 | 56 | 51 | 48 |
| … of which above the minimum | 62 | 272 | 40 | 40 | 48 | 43 | 40 |
| Base complex patches (C) | 6275 | 7910 | 2997 | 3338 | 2980 | 6215 | 4909 |
| Alignment, degrees (A) | 15.23 | 14.61 | 7.27 | 7.01 | 6.42 | 6.32 | 7.75 |
| Mean angle deviation, degrees (F) | 10.30 | 4.50 | 8.90 | 6.55 | 6.03 | 6.44 | 7.53 |
| Angle deviation, 95th percentile (F) | 28.50 | 15.34 | 24.31 | 18.56 | 16.78 | 16.31 | 19.46 |
| Mean aspect ratio (F) | 1.267 | 1.139 | 1.333 | 1.340 | 1.354 | 1.595 | 1.351 |
| Area variation (F) | 0.371 | 0.144 | 0.268 | 0.295 | 0.401 | 0.365 | 0.270 |
| Hausdorff, ‰ of the diagonal (H) | 21.90 | 5.99 | 5.60 | 4.70 | 3.28 | 4.29 | 8.04 |
| Mean distance, ‰ (H) | 0.213 | 0.132 | 0.156 | 0.134 | 0.116 | 0.128 | 0.176 |
| Off budget, % (N) | 9.6 | 1.2 | 11.8 | 0.1 | 0.6 | 6.4 | 13.2 |
| Time, seconds (T) | 2.92 | 1.37 | 16.96 | 6.31 | 6.75 | 15.56 | 13.36 |

## pig (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 4/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Irregular vertices (S) | 136 | 370 | 81 | 88 | 94 | 101 | 98 |
| … of which above the minimum | 128 | 362 | — | 80 | 86 | — | 90 |
| Base complex patches (C) | 6841 | 7826 | 4511 | 5985 | 4984 | — | 4284 |
| Alignment, degrees (A) | 15.84 | 14.90 | 9.14 | 9.39 | 8.10 | 7.39 | 8.84 |
| Mean angle deviation, degrees (F) | 11.44 | 5.04 | 8.76 | 6.99 | 5.57 | 6.40 | 6.59 |
| Angle deviation, 95th percentile (F) | 34.02 | 17.35 | 24.69 | 19.74 | 17.39 | 18.75 | 19.80 |
| Mean aspect ratio (F) | 1.339 | 1.161 | 1.356 | 1.338 | 1.302 | 1.546 | 1.329 |
| Area variation (F) | 0.383 | 0.161 | 0.312 | 0.332 | 0.499 | 0.462 | 0.260 |
| Hausdorff, ‰ of the diagonal (H) | 56.23 | 21.49 | 8.71 | 8.30 | 7.02 | 9.11 | 9.71 |
| Mean distance, ‰ (H) | 0.610 | 0.238 | 0.258 | 0.224 | 0.209 | 0.222 | 0.301 |
| Off budget, % (N) | 12.8 | 1.5 | 10.1 | 1.2 | 1.8 | 22.8 | 11.6 |
| Time, seconds (T) | 4.25 | 1.17 | 35.75 | 8.96 | 11.91 | 16.84 | 16.13 |

## jerry (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | 112 | 490 | 170 | 166 | 182 | 244 | 247 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | 4991 | 7797 | — | 5332 | 7034 | 11709 | — |
| Alignment, degrees (A) | 17.56 | 18.65 | 13.30 | 12.91 | 12.32 | 10.64 | 12.69 |
| Mean angle deviation, degrees (F) | 7.80 | 6.63 | 8.69 | 5.26 | 5.68 | 8.15 | 8.38 |
| Angle deviation, 95th percentile (F) | 22.58 | 23.27 | 23.17 | 14.66 | 15.36 | 22.38 | 23.18 |
| Mean aspect ratio (F) | 1.257 | 1.241 | 1.291 | 1.250 | 1.257 | 1.879 | 1.380 |
| Area variation (F) | 0.245 | 0.210 | 0.261 | 0.316 | 0.473 | 0.651 | 0.279 |
| Hausdorff, ‰ of the diagonal (H) | 31.29 | 29.67 | 18.98 | 15.03 | 11.51 | 4.22 | 10.31 |
| Mean distance, ‰ (H) | 0.581 | 0.629 | 0.601 | 0.473 | 0.385 | 0.310 | 0.598 |
| Off budget, % (N) | 2.7 | 0.8 | 8.2 | 1.6 | 1.0 | 71.4 | 13.2 |
| Time, seconds (T) | 5.41 | 0.67 | 46.08 | 6.86 | 7.43 | 9.49 | 8.46 |

## rose (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Irregular vertices (S) | 917 | 1126 | 557 | 557 | 557 | 1899 | 1164 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | 6524 | 6434 | — | — | — | — | — |
| Alignment, degrees (A) | 18.20 | 18.49 | 14.24 | 14.24 | 14.24 | 9.05 | 12.40 |
| Mean angle deviation, degrees (F) | 14.82 | 12.06 | 10.21 | 10.21 | 10.21 | 8.61 | 11.61 |
| Angle deviation, 95th percentile (F) | 41.38 | 40.69 | 29.73 | 29.73 | 29.73 | 24.64 | 31.66 |
| Mean aspect ratio (F) | 1.514 | 1.595 | 1.413 | 1.413 | 1.413 | 1.839 | 1.518 |
| Area variation (F) | 0.407 | 0.359 | 0.331 | 0.331 | 0.331 | 1.220 | 0.433 |
| Hausdorff, ‰ of the diagonal (H) | 74.30 | 72.22 | 74.98 | 74.98 | 74.98 | 73.68 | 75.13 |
| Mean distance, ‰ (H) | 1.489 | 1.750 | 1.548 | 1.548 | 1.548 | 0.689 | 1.593 |
| Off budget, % (N) | 15.5 | 13.4 | 17.9 | 17.9 | 17.9 | 573.4 | 13.8 |
| Time, seconds (T) | 3.22 | 1.60 | 160.12 | 145.35 | 146.23 | 9.67 | 7.85 |

## bridge (organic)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 0/1 |
| Irregular vertices (S) | — | 258 | 66 | 70 | 86 | 82 | 79 |
| … of which above the minimum | — | — | — | — | — | — | — |
| Base complex patches (C) | — | 5201 | 1629 | 2987 | 2286 | 2942 | — |
| Alignment, degrees (A) | — | 13.88 | 9.82 | 9.87 | 8.89 | 8.33 | 9.16 |
| Mean angle deviation, degrees (F) | — | 4.48 | 6.90 | 4.77 | 4.33 | 6.17 | 6.74 |
| Angle deviation, 95th percentile (F) | — | 14.92 | 19.61 | 12.30 | 11.70 | 17.33 | 19.99 |
| Mean aspect ratio (F) | — | 1.145 | 1.265 | 1.273 | 1.254 | 1.480 | 1.386 |
| Area variation (F) | — | 0.140 | 0.231 | 0.265 | 0.342 | 0.354 | 0.285 |
| Hausdorff, ‰ of the diagonal (H) | — | 9.03 | 6.77 | 6.71 | 6.98 | 6.69 | 6.45 |
| Mean distance, ‰ (H) | — | 0.281 | 0.301 | 0.277 | 0.274 | 0.264 | 0.325 |
| Off budget, % (N) | — | 0.5 | 7.6 | 1.5 | 1.8 | 10.1 | 11.8 |
| Time, seconds (T) | — | 1.86 | 24.78 | 6.38 | 4.51 | 15.51 | 11.76 |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 38 | 337 | 24 | 24 | 24 | 24 | 24 |
| … of which above the minimum | 30 | 329 | 16 | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 1171 | 4678 | 128 | 78 | 92 | 730 | 730 |
| Alignment, degrees (A) | 13.31 | 13.52 | 11.85 | 11.89 | 10.47 | 10.87 | 10.87 |
| Mean angle deviation, degrees (F) | 2.56 | 4.52 | 1.66 | 1.25 | 1.16 | 3.63 | 3.63 |
| Angle deviation, 95th percentile (F) | 9.55 | 18.21 | 7.02 | 3.94 | 4.59 | 14.81 | 14.81 |
| Mean aspect ratio (F) | 1.074 | 1.178 | 1.151 | 1.107 | 1.133 | 1.301 | 1.301 |
| Area variation (F) | 0.087 | 0.166 | 0.105 | 0.124 | 0.329 | 0.244 | 0.244 |
| Hausdorff, ‰ of the diagonal (H) | 5.14 | 5.05 | 0.42 | 3.90 | 5.02 | 0.68 | 0.68 |
| Mean distance, ‰ (H) | 0.034 | 0.023 | 0.021 | 0.022 | 0.030 | 0.040 | 0.040 |
| Off budget, % (N) | 0.3 | 3.4 | 2.2 | 1.1 | 0.5 | 8.1 | 8.1 |
| Time, seconds (T) | 6.71 | 2.42 | 15.27 | 4.84 | 5.52 | 2.65 | 2.72 |

## cube (mechanical)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 18 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 10 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 6 | 376 | 8028 | 6 | 6 | 6 | 6 |
| Alignment, degrees (A) | 0.46 | 0.96 | 0.67 | 0.22 | 0.29 | 0.11 | 0.11 |
| Mean angle deviation, degrees (F) | 0.36 | 0.91 | 0.95 | 0.45 | 0.21 | 0.00 | 0.00 |
| Angle deviation, 95th percentile (F) | 1.18 | 2.60 | 2.22 | 1.27 | 0.74 | 0.00 | 0.00 |
| Mean aspect ratio (F) | 1.020 | 1.030 | 1.034 | 1.028 | 1.075 | 1.019 | 1.019 |
| Area variation (F) | 0.023 | 0.045 | 0.047 | 0.027 | 0.094 | 0.013 | 0.013 |
| Hausdorff, ‰ of the diagonal (H) | 0.85 | 0.00 | 6.86 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.007 | 0.000 | 0.030 | 0.000 | 0.000 | 0.000 | 0.000 |
| Off budget, % (N) | 1.0 | 2.5 | 0.4 | 0.8 | 1.0 | 1.0 | 1.0 |
| Time, seconds (T) | 3.89 | 0.86 | 71.95 | 4.64 | 3.77 | 3.50 | 2.90 |

## sphere (calibration)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 12 | 193 | 24 | 28 | 28 | 8 | 8 |
| … of which above the minimum | 4 | 185 | 16 | 20 | 20 | 0 | 0 |
| Base complex patches (C) | 7729 | 8190 | 7528 | 8035 | 8031 | 4992 | 4992 |
| Alignment, degrees (A) | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.93 | 3.49 | 7.08 | 4.96 | 5.09 | 7.65 | 7.65 |
| Angle deviation, 95th percentile (F) | 12.52 | 11.93 | 15.76 | 10.99 | 11.34 | 18.00 | 18.00 |
| Mean aspect ratio (F) | 1.109 | 1.097 | 1.156 | 1.145 | 1.141 | 1.187 | 1.187 |
| Area variation (F) | 0.134 | 0.122 | 0.155 | 0.171 | 0.175 | 0.391 | 0.391 |
| Hausdorff, ‰ of the diagonal (H) | 0.42 | 0.43 | 0.64 | 0.47 | 0.47 | 0.55 | 0.55 |
| Mean distance, ‰ (H) | 0.085 | 0.082 | 0.090 | 0.084 | 0.084 | 0.159 | 0.159 |
| Off budget, % (N) | 3.3 | 2.4 | 5.9 | 0.4 | 0.4 | 11.4 | 11.4 |
| Time, seconds (T) | 4.79 | 0.36 | 14.63 | 5.48 | 5.56 | 4.18 | 4.20 |

## torus (calibration)

| | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Irregular vertices (S) | 22 | 262 | 0 | 0 | 0 | 20 | 20 |
| … of which above the minimum | 22 | 262 | 0 | 0 | 0 | 20 | 20 |
| Base complex patches (C) | 9070 | 7956 | 1 | 1 | 1 | 5983 | 5983 |
| Alignment, degrees (A) | 11.41 | 6.29 | 0.61 | 0.61 | 0.61 | 4.25 | 4.25 |
| Mean angle deviation, degrees (F) | 7.33 | 5.12 | 0.91 | 0.91 | 0.91 | 3.68 | 3.68 |
| Angle deviation, 95th percentile (F) | 19.57 | 15.85 | 1.44 | 1.44 | 1.44 | 10.81 | 10.81 |
| Mean aspect ratio (F) | 1.180 | 1.139 | 1.340 | 1.340 | 1.340 | 1.233 | 1.233 |
| Area variation (F) | 0.189 | 0.149 | 0.280 | 0.280 | 0.280 | 0.225 | 0.225 |
| Hausdorff, ‰ of the diagonal (H) | 0.60 | 0.65 | 0.61 | 0.61 | 0.61 | 0.69 | 0.69 |
| Mean distance, ‰ (H) | 0.109 | 0.119 | 0.133 | 0.133 | 0.133 | 0.222 | 0.222 |
| Off budget, % (N) | 14.3 | 0.5 | 9.0 | 9.0 | 9.0 | 7.5 | 7.5 |
| Time, seconds (T) | 6.46 | 0.75 | 6.61 | 4.56 | 4.54 | 6.20 | 5.73 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 6 | 0 | 7 | 3 | 2 | 2 | 2 |
| Base complex patches (C) | 1 | 1 | 3 | 1 | 3 | 0 | 4 |
| Alignment, degrees (A) | 0 | 0 | 1 | 1 | 2 | 9 | 0 |
| Mean angle deviation, degrees (F) | 0 | 8 | 0 | 3 | 1 | 1 | 0 |
| Mean aspect ratio (F) | 1 | 11 | 1 | 1 | 1 | 0 | 0 |
| Valid meshes (V) | 33/65 | 50/65 | 7/13 | 12/13 | 12/13 | 11/13 | 9/13 |

### mechanical (2 models)

| Metric | quadriflow | instant-meshes | integer-grid | quad-layout | quad-layout-adaptive | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 1 | 0 | 2 | 2 | 2 | 2 | 2 |
| Base complex patches (C) | 1 | 0 | 0 | 2 | 1 | 1 | 1 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 1 | 1 | 1 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 0 | 1 | 1 | 1 |
| Mean aspect ratio (F) | 1 | 0 | 0 | 0 | 0 | 1 | 1 |
| Valid meshes (V) | 8/10 | 10/10 | 2/2 | 2/2 | 2/2 | 2/2 | 2/2 |
