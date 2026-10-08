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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 99.81 | 99.80 |
| Off budget, % (N) | 0.9 | 0.3 |
| Time, seconds (T) | 6.03 | 6.83 |

## rock (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 116 | 134 |
| … of which above the minimum | 108 | 126 |
| Base complex patches (C) | 7943 | 7940 |
| Alignment, degrees (A) | 16.59 | 16.30 |
| Mean angle deviation, degrees (F) | 5.89 | 5.36 |
| Angle deviation, 95th percentile (F) | 13.90 | 13.03 |
| Mean aspect ratio (F) | 1.214 | 1.213 |
| Area variation (F) | 0.254 | 0.264 |
| Hausdorff, ‰ of the diagonal (H) | 3.12 | 3.35 |
| Mean distance, ‰ (H) | 0.426 | 0.417 |
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 99.89 | 99.90 |
| Off budget, % (N) | 0.7 | 0.8 |
| Time, seconds (T) | 6.67 | 5.27 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 99.75 | 99.77 |
| Off budget, % (N) | 1.5 | 2.2 |
| Time, seconds (T) | 13.20 | 15.59 |

## cage (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 48 | 58 |
| … of which above the minimum | 16 | 26 |
| Base complex patches (C) | 1654 | 275 |
| Alignment, degrees (A) | 2.47 | 1.94 |
| Mean angle deviation, degrees (F) | 2.12 | 2.26 |
| Angle deviation, 95th percentile (F) | 6.66 | 6.69 |
| Mean aspect ratio (F) | 1.197 | 1.235 |
| Area variation (F) | 0.214 | 0.328 |
| Hausdorff, ‰ of the diagonal (H) | 4.65 | 3.93 |
| Mean distance, ‰ (H) | 0.291 | 0.252 |
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 99.54 | 99.64 |
| Off budget, % (N) | 1.4 | 0.7 |
| Time, seconds (T) | 4.69 | 6.24 |

## suzanne (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 64 | 82 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 3543 | 4783 |
| Alignment, degrees (A) | 9.96 | 9.77 |
| Mean angle deviation, degrees (F) | 5.44 | 6.01 |
| Angle deviation, 95th percentile (F) | 15.15 | 15.25 |
| Mean aspect ratio (F) | 1.210 | 1.233 |
| Area variation (F) | 0.250 | 0.382 |
| Hausdorff, ‰ of the diagonal (H) | 4.99 | 4.22 |
| Mean distance, ‰ (H) | 0.273 | 0.217 |
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | — | — |
| Off budget, % (N) | 0.5 | 1.3 |
| Time, seconds (T) | 4.83 | 11.29 |

## ogre (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 202 | 240 |
| … of which above the minimum | 194 | 232 |
| Base complex patches (C) | 6252 | 6091 |
| Alignment, degrees (A) | 11.33 | 10.80 |
| Mean angle deviation, degrees (F) | 6.28 | 6.27 |
| Angle deviation, 95th percentile (F) | 16.97 | 16.67 |
| Mean aspect ratio (F) | 1.377 | 1.355 |
| Area variation (F) | 0.327 | 0.447 |
| Hausdorff, ‰ of the diagonal (H) | 12.97 | 9.55 |
| Mean distance, ‰ (H) | 0.433 | 0.409 |
| Thin parts: mean distance, % of a quad edge (V) | 15.4 | 6.2 |
| Volume kept, % of the source (V) | 99.54 | 99.57 |
| Off budget, % (N) | 0.4 | 0.6 |
| Time, seconds (T) | 10.80 | 14.25 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 99.75 | 99.75 |
| Off budget, % (N) | 0.4 | 1.2 |
| Time, seconds (T) | 11.18 | 13.71 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 99.75 | 99.75 |
| Off budget, % (N) | 0.4 | 0.2 |
| Time, seconds (T) | 9.02 | 8.38 |

## blub (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 48 | 56 |
| … of which above the minimum | 40 | 48 |
| Base complex patches (C) | 3338 | 4424 |
| Alignment, degrees (A) | 7.01 | 6.45 |
| Mean angle deviation, degrees (F) | 6.55 | 6.11 |
| Angle deviation, 95th percentile (F) | 18.56 | 16.89 |
| Mean aspect ratio (F) | 1.340 | 1.356 |
| Area variation (F) | 0.295 | 0.397 |
| Hausdorff, ‰ of the diagonal (H) | 4.70 | 3.28 |
| Mean distance, ‰ (H) | 0.134 | 0.117 |
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 99.69 | 99.74 |
| Off budget, % (N) | 0.1 | 0.1 |
| Time, seconds (T) | 7.32 | 7.76 |

## pig (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 88 | 98 |
| … of which above the minimum | 80 | 90 |
| Base complex patches (C) | 5985 | 5403 |
| Alignment, degrees (A) | 9.39 | 8.09 |
| Mean angle deviation, degrees (F) | 6.99 | 5.65 |
| Angle deviation, 95th percentile (F) | 19.74 | 17.04 |
| Mean aspect ratio (F) | 1.338 | 1.326 |
| Area variation (F) | 0.332 | 0.503 |
| Hausdorff, ‰ of the diagonal (H) | 8.30 | 7.05 |
| Mean distance, ‰ (H) | 0.224 | 0.197 |
| Thin parts: mean distance, % of a quad edge (V) | 20.8 | 7.9 |
| Volume kept, % of the source (V) | 99.66 | 99.73 |
| Off budget, % (N) | 1.2 | 1.8 |
| Time, seconds (T) | 11.32 | 13.88 |

## jerry (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 166 | 273 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 5332 | 7075 |
| Alignment, degrees (A) | 12.91 | 12.41 |
| Mean angle deviation, degrees (F) | 5.26 | 5.91 |
| Angle deviation, 95th percentile (F) | 14.66 | 16.66 |
| Mean aspect ratio (F) | 1.250 | 1.275 |
| Area variation (F) | 0.316 | 0.532 |
| Hausdorff, ‰ of the diagonal (H) | 15.03 | 9.98 |
| Mean distance, ‰ (H) | 0.473 | 0.372 |
| Thin parts: mean distance, % of a quad edge (V) | 25.0 | 11.3 |
| Volume kept, % of the source (V) | — | — |
| Off budget, % (N) | 1.6 | 1.3 |
| Time, seconds (T) | 5.50 | 11.43 |

## rose (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 1/1 |
| Irregular vertices (S) | 1279 | 1723 |
| … of which above the minimum | 663 | 1107 |
| Base complex patches (C) | 7101 | 7316 |
| Alignment, degrees (A) | 11.98 | 11.46 |
| Mean angle deviation, degrees (F) | 10.60 | 12.53 |
| Angle deviation, 95th percentile (F) | 37.57 | 43.11 |
| Mean aspect ratio (F) | 2.328 | 2.581 |
| Area variation (F) | 0.540 | 0.852 |
| Hausdorff, ‰ of the diagonal (H) | 15.38 | 14.81 |
| Mean distance, ‰ (H) | 0.754 | 0.850 |
| Thin parts: mean distance, % of a quad edge (V) | 15.0 | 12.7 |
| Volume kept, % of the source (V) | 93.10 | 91.48 |
| Off budget, % (N) | 9.3 | 5.2 |
| Time, seconds (T) | 8.89 | 17.59 |

## bridge (organic)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 70 | 81 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 2987 | 2725 |
| Alignment, degrees (A) | 9.87 | 9.03 |
| Mean angle deviation, degrees (F) | 4.77 | 4.60 |
| Angle deviation, 95th percentile (F) | 12.30 | 12.04 |
| Mean aspect ratio (F) | 1.264 | 1.259 |
| Area variation (F) | 0.265 | 0.331 |
| Hausdorff, ‰ of the diagonal (H) | 6.70 | 6.45 |
| Mean distance, ‰ (H) | 0.277 | 0.268 |
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | — | — |
| Off budget, % (N) | 1.5 | 1.4 |
| Time, seconds (T) | 6.49 | 6.20 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 100.04 | 100.03 |
| Off budget, % (N) | 1.0 | 1.2 |
| Time, seconds (T) | 5.29 | 5.42 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 100.00 | 100.00 |
| Off budget, % (N) | 0.8 | 0.8 |
| Time, seconds (T) | 5.47 | 4.80 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 100.00 | 100.00 |
| Off budget, % (N) | 0.8 | 0.8 |
| Time, seconds (T) | 3.67 | 4.00 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 99.94 | 99.93 |
| Off budget, % (N) | 0.1 | 1.3 |
| Time, seconds (T) | 4.57 | 5.12 |

## terrain (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 1/1 | 1/1 |
| Irregular vertices (S) | 124 | 225 |
| … of which above the minimum | — | — |
| Base complex patches (C) | 5354 | 6757 |
| Alignment, degrees (A) | 17.76 | 17.57 |
| Mean angle deviation, degrees (F) | 6.62 | 7.06 |
| Angle deviation, 95th percentile (F) | 18.39 | 18.95 |
| Mean aspect ratio (F) | 1.292 | 1.313 |
| Area variation (F) | 0.333 | 0.495 |
| Hausdorff, ‰ of the diagonal (H) | 11.92 | 9.22 |
| Mean distance, ‰ (H) | 0.872 | 0.739 |
| Thin parts: mean distance, % of a quad edge (V) | 26.8 | 20.7 |
| Volume kept, % of the source (V) | — | — |
| Off budget, % (N) | 0.5 | 1.2 |
| Time, seconds (T) | 10.61 | 13.82 |

## mahaveer (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 |
| Irregular vertices (S) | 552 | 651 |
| … of which above the minimum | — | — |
| Base complex patches (C) | — | — |
| Alignment, degrees (A) | 11.80 | 11.62 |
| Mean angle deviation, degrees (F) | 13.35 | 13.71 |
| Angle deviation, 95th percentile (F) | 61.70 | 63.49 |
| Mean aspect ratio (F) | 35.141 | 51.943 |
| Area variation (F) | 0.744 | 0.911 |
| Hausdorff, ‰ of the diagonal (H) | 15.73 | 15.73 |
| Mean distance, ‰ (H) | 1.284 | 1.199 |
| Thin parts: mean distance, % of a quad edge (V) | 20.4 | 18.5 |
| Volume kept, % of the source (V) | — | — |
| Off budget, % (N) | 1.8 | 3.6 |
| Time, seconds (T) | 15.46 | 19.16 |

## marine (extra)

| | quad-layout | quad-layout-adaptive |
|---|---|---|
| **Valid meshes (V)** | 0/1 | 0/1 |
| Irregular vertices (S) | 5777 | 6073 |
| … of which above the minimum | — | — |
| Base complex patches (C) | — | — |
| Alignment, degrees (A) | 12.65 | 13.35 |
| Mean angle deviation, degrees (F) | 22.00 | 22.83 |
| Angle deviation, 95th percentile (F) | 70.80 | 69.34 |
| Mean aspect ratio (F) | 5.985 | 7.529 |
| Area variation (F) | 1.198 | 1.322 |
| Hausdorff, ‰ of the diagonal (H) | 3.53 | 3.75 |
| Mean distance, ‰ (H) | 0.352 | 0.368 |
| Thin parts: mean distance, % of a quad edge (V) | 17.7 | 18.3 |
| Volume kept, % of the source (V) | — | — |
| Off budget, % (N) | 12.2 | 15.1 |
| Time, seconds (T) | 40.16 | 58.40 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 99.92 | 99.92 |
| Off budget, % (N) | 0.4 | 0.4 |
| Time, seconds (T) | 4.81 | 7.61 |

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
| Thin parts: mean distance, % of a quad edge (V) | — | — |
| Volume kept, % of the source (V) | 99.77 | 99.77 |
| Off budget, % (N) | 0.3 | 0.3 |
| Time, seconds (T) | 3.62 | 3.97 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quad-layout | quad-layout-adaptive |
|---|---|---|
| Irregular vertices (S) | 12 | 3 |
| Base complex patches (C) | 7 | 6 |
| Alignment, degrees (A) | 1 | 12 |
| Mean angle deviation, degrees (F) | 6 | 7 |
| Mean aspect ratio (F) | 6 | 7 |
| Valid meshes (V) | 12/13 | 13/13 |

### mechanical (4 models)

| Metric | quad-layout | quad-layout-adaptive |
|---|---|---|
| Irregular vertices (S) | 4 | 4 |
| Base complex patches (C) | 3 | 3 |
| Alignment, degrees (A) | 2 | 2 |
| Mean angle deviation, degrees (F) | 2 | 2 |
| Mean aspect ratio (F) | 4 | 0 |
| Valid meshes (V) | 4/4 | 4/4 |
