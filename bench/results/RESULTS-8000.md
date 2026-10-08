# Bench results

Requested budget: 8000 quads. Median over 5 seeds; Quad Remesher and integer-grid have no seed and run once. Lower is better for every metric; "—" means it could not be measured.

## blob (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 24 | 24 |
| … of which above the minimum | 16 | 16 |
| Base complex patches (C) | 6808 | 6819 |
| Alignment, degrees (A) | 5.82 | 5.47 |
| Mean angle deviation, degrees (F) | 6.30 | 5.79 |
| Angle deviation, 95th percentile (F) | 17.21 | 15.38 |
| Mean aspect ratio (F) | 1.309 | 1.291 |
| Area variation (F) | 0.341 | 0.361 |
| Hausdorff, ‰ of the diagonal (H) | 2.49 | 2.21 |
| Mean distance, ‰ (H) | 0.177 | 0.174 |
| Off budget, % (N) | 0.9 | 0.3 |
| Time, seconds (T) | 4.73 | 4.86 |

## rock (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 116 | 130 |
| … of which above the minimum | 108 | 122 |
| Base complex patches (C) | 7943 | 7993 |
| Alignment, degrees (A) | 16.59 | 16.37 |
| Mean angle deviation, degrees (F) | 5.89 | 5.38 |
| Angle deviation, 95th percentile (F) | 13.90 | 12.91 |
| Mean aspect ratio (F) | 1.214 | 1.211 |
| Area variation (F) | 0.254 | 0.271 |
| Hausdorff, ‰ of the diagonal (H) | 3.12 | 3.01 |
| Mean distance, ‰ (H) | 0.426 | 0.416 |
| Off budget, % (N) | 0.7 | 0.1 |
| Time, seconds (T) | 4.92 | 5.82 |

## creature (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 40 | 42 |
| … of which above the minimum | 32 | 34 |
| Base complex patches (C) | 2864 | 3225 |
| Alignment, degrees (A) | 6.02 | 5.63 |
| Mean angle deviation, degrees (F) | 6.71 | 5.60 |
| Angle deviation, 95th percentile (F) | 18.89 | 14.96 |
| Mean aspect ratio (F) | 1.343 | 1.321 |
| Area variation (F) | 0.365 | 0.471 |
| Hausdorff, ‰ of the diagonal (H) | 1.48 | 1.54 |
| Mean distance, ‰ (H) | 0.124 | 0.118 |
| Off budget, % (N) | 1.5 | 2.2 |
| Time, seconds (T) | 10.60 | 13.09 |

## cage (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 48 | 48 |
| … of which above the minimum | 16 | 16 |
| Base complex patches (C) | 1654 | 556 |
| Alignment, degrees (A) | 2.47 | 2.18 |
| Mean angle deviation, degrees (F) | 2.12 | 2.50 |
| Angle deviation, 95th percentile (F) | 6.66 | 7.09 |
| Mean aspect ratio (F) | 1.197 | 1.229 |
| Area variation (F) | 0.214 | 0.333 |
| Hausdorff, ‰ of the diagonal (H) | 4.65 | 3.75 |
| Mean distance, ‰ (H) | 0.291 | 0.253 |
| Off budget, % (N) | 1.4 | 0.4 |
| Time, seconds (T) | 5.44 | 5.34 |

## suzanne (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 64 | 76 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 3543 | 5461 |
| Alignment, degrees (A) | 9.96 | 9.87 |
| Mean angle deviation, degrees (F) | 5.44 | 5.94 |
| Angle deviation, 95th percentile (F) | 15.15 | 15.31 |
| Mean aspect ratio (F) | 1.210 | 1.232 |
| Area variation (F) | 0.250 | 0.382 |
| Hausdorff, ‰ of the diagonal (H) | 4.99 | 4.41 |
| Mean distance, ‰ (H) | 0.273 | 0.225 |
| Off budget, % (N) | 0.5 | 1.7 |
| Time, seconds (T) | 4.55 | 9.52 |

## ogre (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 202 | 228 |
| … of which above the minimum | 194 | 220 |
| Base complex patches (C) | 6252 | 6062 |
| Alignment, degrees (A) | 11.33 | 10.83 |
| Mean angle deviation, degrees (F) | 6.28 | 6.25 |
| Angle deviation, 95th percentile (F) | 16.97 | 16.59 |
| Mean aspect ratio (F) | 1.377 | 1.376 |
| Area variation (F) | 0.327 | 0.437 |
| Hausdorff, ‰ of the diagonal (H) | 12.97 | 7.86 |
| Mean distance, ‰ (H) | 0.433 | 0.398 |
| Off budget, % (N) | 0.4 | 0.7 |
| Time, seconds (T) | 9.26 | 11.41 |

## spot (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 80 | 78 |
| … of which above the minimum | 72 | 70 |
| Base complex patches (C) | 5329 | 5007 |
| Alignment, degrees (A) | 7.92 | 7.55 |
| Mean angle deviation, degrees (F) | 6.47 | 6.50 |
| Angle deviation, 95th percentile (F) | 16.91 | 17.46 |
| Mean aspect ratio (F) | 1.326 | 1.330 |
| Area variation (F) | 0.332 | 0.431 |
| Hausdorff, ‰ of the diagonal (H) | 3.01 | 2.40 |
| Mean distance, ‰ (H) | 0.162 | 0.155 |
| Off budget, % (N) | 0.4 | 1.2 |
| Time, seconds (T) | 10.82 | 11.64 |

## bob (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 20 | 20 |
| … of which above the minimum | 20 | 20 |
| Base complex patches (C) | 1279 | 2898 |
| Alignment, degrees (A) | 3.64 | 3.75 |
| Mean angle deviation, degrees (F) | 3.64 | 3.76 |
| Angle deviation, 95th percentile (F) | 11.16 | 11.87 |
| Mean aspect ratio (F) | 1.402 | 1.380 |
| Area variation (F) | 0.374 | 0.432 |
| Hausdorff, ‰ of the diagonal (H) | 0.91 | 1.28 |
| Mean distance, ‰ (H) | 0.126 | 0.126 |
| Off budget, % (N) | 0.4 | 0.2 |
| Time, seconds (T) | 7.73 | 6.22 |

## blub (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 48 | 56 |
| … of which above the minimum | 40 | 48 |
| Base complex patches (C) | 3338 | 2980 |
| Alignment, degrees (A) | 7.01 | 6.42 |
| Mean angle deviation, degrees (F) | 6.55 | 6.03 |
| Angle deviation, 95th percentile (F) | 18.56 | 16.78 |
| Mean aspect ratio (F) | 1.340 | 1.354 |
| Area variation (F) | 0.295 | 0.401 |
| Hausdorff, ‰ of the diagonal (H) | 4.70 | 3.28 |
| Mean distance, ‰ (H) | 0.134 | 0.116 |
| Off budget, % (N) | 0.1 | 0.6 |
| Time, seconds (T) | 6.42 | 6.76 |

## pig (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 88 | 94 |
| … of which above the minimum | 80 | 86 |
| Base complex patches (C) | 5985 | 4984 |
| Alignment, degrees (A) | 9.39 | 8.10 |
| Mean angle deviation, degrees (F) | 6.99 | 5.57 |
| Angle deviation, 95th percentile (F) | 19.74 | 17.39 |
| Mean aspect ratio (F) | 1.338 | 1.302 |
| Area variation (F) | 0.332 | 0.499 |
| Hausdorff, ‰ of the diagonal (H) | 8.30 | 7.02 |
| Mean distance, ‰ (H) | 0.224 | 0.209 |
| Off budget, % (N) | 1.2 | 1.8 |
| Time, seconds (T) | 10.84 | 9.87 |

## jerry (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 166 | 182 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 5332 | 7034 |
| Alignment, degrees (A) | 12.91 | 12.33 |
| Mean angle deviation, degrees (F) | 5.26 | 5.67 |
| Angle deviation, 95th percentile (F) | 14.66 | 15.37 |
| Mean aspect ratio (F) | 1.250 | 1.256 |
| Area variation (F) | 0.316 | 0.483 |
| Hausdorff, ‰ of the diagonal (H) | 15.03 | 11.51 |
| Mean distance, ‰ (H) | 0.473 | 0.381 |
| Off budget, % (N) | 1.6 | 1.0 |
| Time, seconds (T) | 4.92 | 5.68 |

## rose (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 |
| Irregular vertices (S) | 1279 | 1304 |
| … of which above the minimum | 663 | — |
| Base complex patches (C) | 7101 | — |
| Alignment, degrees (A) | 11.98 | 11.62 |
| Mean angle deviation, degrees (F) | 10.60 | 10.22 |
| Angle deviation, 95th percentile (F) | 37.57 | 35.76 |
| Mean aspect ratio (F) | 2.328 | 3.287 |
| Area variation (F) | 0.540 | 0.552 |
| Hausdorff, ‰ of the diagonal (H) | 15.38 | 23.41 |
| Mean distance, ‰ (H) | 0.754 | 0.764 |
| Off budget, % (N) | 9.3 | 7.7 |
| Time, seconds (T) | 9.82 | 9.29 |

## bridge (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 70 | 86 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 2987 | 2286 |
| Alignment, degrees (A) | 9.87 | 8.88 |
| Mean angle deviation, degrees (F) | 4.77 | 4.33 |
| Angle deviation, 95th percentile (F) | 12.30 | 11.70 |
| Mean aspect ratio (F) | 1.264 | 1.254 |
| Area variation (F) | 0.265 | 0.342 |
| Hausdorff, ‰ of the diagonal (H) | 6.70 | 6.54 |
| Mean distance, ‰ (H) | 0.277 | 0.274 |
| Off budget, % (N) | 1.5 | 1.8 |
| Time, seconds (T) | 5.05 | 4.73 |

## part (mechanical)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 24 | 24 |
| … of which above the minimum | 16 | 16 |
| Base complex patches (C) | 429 | 92 |
| Alignment, degrees (A) | 11.84 | 10.35 |
| Mean angle deviation, degrees (F) | 1.25 | 1.00 |
| Angle deviation, 95th percentile (F) | 3.80 | 3.80 |
| Mean aspect ratio (F) | 1.093 | 1.130 |
| Area variation (F) | 0.126 | 0.329 |
| Hausdorff, ‰ of the diagonal (H) | 0.42 | 0.36 |
| Mean distance, ‰ (H) | 0.021 | 0.021 |
| Off budget, % (N) | 1.0 | 1.2 |
| Time, seconds (T) | 5.60 | 5.38 |

## cube (mechanical)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 8 |
| … of which above the minimum | 0 | 0 |
| Base complex patches (C) | 6 | 6 |
| Alignment, degrees (A) | 0.11 | 0.27 |
| Mean angle deviation, degrees (F) | 0.01 | 0.20 |
| Angle deviation, 95th percentile (F) | 0.02 | 0.74 |
| Mean aspect ratio (F) | 1.018 | 1.074 |
| Area variation (F) | 0.013 | 0.093 |
| Hausdorff, ‰ of the diagonal (H) | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.000 | 0.000 |
| Off budget, % (N) | 0.8 | 0.8 |
| Time, seconds (T) | 3.38 | 3.51 |

## lblock (mechanical)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 12 | 12 |
| … of which above the minimum | 4 | 4 |
| Base complex patches (C) | 14 | 14 |
| Alignment, degrees (A) | 0.61 | 0.92 |
| Mean angle deviation, degrees (F) | 0.00 | 0.49 |
| Angle deviation, 95th percentile (F) | 0.01 | 1.48 |
| Mean aspect ratio (F) | 1.000 | 1.078 |
| Area variation (F) | 0.000 | 0.096 |
| Hausdorff, ‰ of the diagonal (H) | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.000 | 0.000 |
| Off budget, % (N) | 0.8 | 0.8 |
| Time, seconds (T) | 3.47 | 3.55 |

## cylinder (mechanical)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 8 |
| … of which above the minimum | 0 | 0 |
| Base complex patches (C) | 182 | 1399 |
| Alignment, degrees (A) | 1.38 | 1.28 |
| Mean angle deviation, degrees (F) | 2.41 | 2.26 |
| Angle deviation, 95th percentile (F) | 10.19 | 10.24 |
| Mean aspect ratio (F) | 1.288 | 1.400 |
| Area variation (F) | 0.296 | 0.208 |
| Hausdorff, ‰ of the diagonal (H) | 0.55 | 0.56 |
| Mean distance, ‰ (H) | 0.055 | 0.061 |
| Off budget, % (N) | 0.1 | 1.3 |
| Time, seconds (T) | 4.05 | 4.06 |

## terrain (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 124 | 148 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 5354 | 6466 |
| Alignment, degrees (A) | 17.76 | 17.69 |
| Mean angle deviation, degrees (F) | 6.62 | 6.61 |
| Angle deviation, 95th percentile (F) | 18.39 | 17.50 |
| Mean aspect ratio (F) | 1.292 | 1.299 |
| Area variation (F) | 0.333 | 0.476 |
| Hausdorff, ‰ of the diagonal (H) | 11.92 | 11.03 |
| Mean distance, ‰ (H) | 0.872 | 0.766 |
| Off budget, % (N) | 0.5 | 1.8 |
| Time, seconds (T) | 9.57 | 9.15 |

## mahaveer (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 |
| Irregular vertices (S) | 552 | 560 |
| … of which above the minimum | — | — |
| Base complex patches (C) | — | — |
| Alignment, degrees (A) | 11.80 | 11.58 |
| Mean angle deviation, degrees (F) | 13.35 | 13.59 |
| Angle deviation, 95th percentile (F) | 61.70 | 62.12 |
| Mean aspect ratio (F) | 35.141 | 31.912 |
| Area variation (F) | 0.744 | 0.775 |
| Hausdorff, ‰ of the diagonal (H) | 15.73 | 15.73 |
| Mean distance, ‰ (H) | 1.284 | 1.276 |
| Off budget, % (N) | 1.8 | 1.8 |
| Time, seconds (T) | 15.94 | 14.73 |

## marine (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 |
| Irregular vertices (S) | 5777 | 5768 |
| … of which above the minimum | — | — |
| Base complex patches (C) | — | — |
| Alignment, degrees (A) | 12.65 | 12.56 |
| Mean angle deviation, degrees (F) | 22.00 | 21.96 |
| Angle deviation, 95th percentile (F) | 70.80 | 70.93 |
| Mean aspect ratio (F) | 5.985 | 5.989 |
| Area variation (F) | 1.198 | 1.207 |
| Hausdorff, ‰ of the diagonal (H) | 3.53 | 3.53 |
| Mean distance, ‰ (H) | 0.352 | 0.355 |
| Off budget, % (N) | 12.2 | 13.2 |
| Time, seconds (T) | 40.40 | 35.29 |

## sphere (calibration)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 28 | 28 |
| … of which above the minimum | 20 | 20 |
| Base complex patches (C) | 8035 | 8031 |
| Alignment, degrees (A) | — | — |
| Mean angle deviation, degrees (F) | 4.96 | 5.09 |
| Angle deviation, 95th percentile (F) | 10.99 | 11.34 |
| Mean aspect ratio (F) | 1.145 | 1.141 |
| Area variation (F) | 0.171 | 0.175 |
| Hausdorff, ‰ of the diagonal (H) | 0.47 | 0.47 |
| Mean distance, ‰ (H) | 0.084 | 0.084 |
| Off budget, % (N) | 0.4 | 0.4 |
| Time, seconds (T) | 4.48 | 4.33 |

## torus (calibration)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 0 | 0 |
| … of which above the minimum | 0 | 0 |
| Base complex patches (C) | 1 | 1 |
| Alignment, degrees (A) | 0.64 | 0.64 |
| Mean angle deviation, degrees (F) | 0.92 | 0.91 |
| Angle deviation, 95th percentile (F) | 1.40 | 1.40 |
| Mean aspect ratio (F) | 1.321 | 1.319 |
| Area variation (F) | 0.298 | 0.300 |
| Hausdorff, ‰ of the diagonal (H) | 0.60 | 0.60 |
| Mean distance, ‰ (H) | 0.120 | 0.120 |
| Off budget, % (N) | 0.3 | 0.3 |
| Time, seconds (T) | 3.07 | 3.80 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quad-layout | quad-layout-adaptive |
|---|---|---|
| Irregular vertices (S) | 12 | 4 |
| Base complex patches (C) | 7 | 6 |
| Alignment, degrees (A) | 1 | 12 |
| Mean angle deviation, degrees (F) | 5 | 8 |
| Mean aspect ratio (F) | 6 | 7 |
| Valid meshes (V) | 12/13 | 12/13 |

### mechanical (4 models)

| Metric | quad-layout | quad-layout-adaptive |
|---|---|---|
| Irregular vertices (S) | 4 | 4 |
| Base complex patches (C) | 3 | 3 |
| Alignment, degrees (A) | 2 | 2 |
| Mean angle deviation, degrees (F) | 2 | 2 |
| Mean aspect ratio (F) | 4 | 0 |
| Valid meshes (V) | 4/4 | 4/4 |
