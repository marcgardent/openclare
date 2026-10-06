# Bench results

Requested budget: 3000 quads. Median over 5 seeds; Quad Remesher has no seed and runs once. Lower is better for every metric; "—" means it could not be measured.

## blob (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 5/5 | 1/1 | 1/1 |
| Irregular vertices (S) | 38 | 143 | 26 | 24 |
| … of which above the minimum | 30 | 135 | 18 | 16 |
| Base complex patches (C) | 2697 | 3048 | 1826 | 1350 |
| Alignment, degrees (A) | 17.48 | 15.74 | 3.93 | 4.55 |
| Mean angle deviation, degrees (F) | 8.71 | 5.87 | 5.76 | 5.45 |
| Angle deviation, 95th percentile (F) | 27.30 | 18.26 | 17.21 | 15.61 |
| Mean aspect ratio (F) | 1.187 | 1.174 | 1.618 | 1.276 |
| Area variation (F) | 0.205 | 0.175 | 0.442 | 0.313 |
| Hausdorff, ‰ of the diagonal (H) | 8.52 | 10.14 | 4.21 | 7.28 |
| Mean distance, ‰ (H) | 0.486 | 0.459 | 0.407 | 0.513 |
| Off budget, % (N) | 7.3 | 3.0 | 12.3 | 8.2 |
| Time, seconds (T) | 1.98 | 1.65 | 16.38 | 12.28 |

## rock (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 91 | 114 | 101 |
| … of which above the minimum | 0 | 83 | 106 | 93 |
| Base complex patches (C) | 2855 | 2896 | 3539 | 2531 |
| Alignment, degrees (A) | 22.96 | 23.21 | 14.17 | 14.68 |
| Mean angle deviation, degrees (F) | 5.84 | 5.18 | 10.07 | 10.61 |
| Angle deviation, 95th percentile (F) | 16.05 | 14.78 | 24.88 | 26.21 |
| Mean aspect ratio (F) | 1.164 | 1.146 | 1.470 | 1.393 |
| Area variation (F) | 0.202 | 0.151 | 0.308 | 0.269 |
| Hausdorff, ‰ of the diagonal (H) | 5.45 | 5.28 | 5.13 | 5.79 |
| Mean distance, ‰ (H) | 0.788 | 0.820 | 0.731 | 0.889 |
| Off budget, % (N) | 2.2 | 3.4 | 18.0 | 15.6 |
| Time, seconds (T) | 1.37 | 0.82 | 20.49 | 16.98 |

## creature (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 |
| Irregular vertices (S) | 24 | 111 | 42 | 46 |
| … of which above the minimum | 16 | 103 | 34 | 38 |
| Base complex patches (C) | 2667 | 2856 | 1465 | 1301 |
| Alignment, degrees (A) | 12.89 | 12.09 | 5.18 | 5.32 |
| Mean angle deviation, degrees (F) | 6.85 | 4.65 | 6.12 | 6.53 |
| Angle deviation, 95th percentile (F) | 18.98 | 14.73 | 17.13 | 18.98 |
| Mean aspect ratio (F) | 1.189 | 1.144 | 1.332 | 1.334 |
| Area variation (F) | 0.237 | 0.146 | 0.348 | 0.313 |
| Hausdorff, ‰ of the diagonal (H) | 8.01 | 3.47 | 2.51 | 2.37 |
| Mean distance, ‰ (H) | 0.320 | 0.279 | 0.297 | 0.348 |
| Off budget, % (N) | 11.1 | 2.3 | 2.0 | 9.4 |
| Time, seconds (T) | 1.29 | 0.90 | 11.24 | 9.11 |

## cage (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 |
| Irregular vertices (S) | 66 | 200 | 44 | 43 |
| … of which above the minimum | 34 | 168 | 12 | 11 |
| Base complex patches (C) | 1943 | 2319 | 1092 | 1001 |
| Alignment, degrees (A) | 8.10 | 7.97 | 2.04 | 2.57 |
| Mean angle deviation, degrees (F) | 5.86 | 6.39 | 3.91 | 4.30 |
| Angle deviation, 95th percentile (F) | 17.11 | 23.33 | 11.90 | 12.82 |
| Mean aspect ratio (F) | 1.183 | 1.252 | 1.993 | 1.318 |
| Area variation (F) | 0.147 | 0.203 | 0.314 | 0.238 |
| Hausdorff, ‰ of the diagonal (H) | 11.45 | 14.46 | 5.74 | 9.66 |
| Mean distance, ‰ (H) | 0.734 | 0.860 | 0.693 | 1.276 |
| Off budget, % (N) | 3.6 | 1.9 | 93.6 | 7.2 |
| Time, seconds (T) | 1.47 | 0.47 | 4.21 | 3.92 |

## suzanne (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 2/5 | 0/1 | 1/1 |
| Irregular vertices (S) | 78 | 212 | 80 | 71 |
| … of which above the minimum | 54 | — | — | — |
| Base complex patches (C) | 2884 | 3002 | 2513 | 1838 |
| Alignment, degrees (A) | 15.33 | 17.02 | 8.75 | 10.24 |
| Mean angle deviation, degrees (F) | 8.61 | 7.08 | 7.87 | 7.87 |
| Angle deviation, 95th percentile (F) | 26.08 | 25.30 | 21.85 | 20.46 |
| Mean aspect ratio (F) | 1.260 | 1.263 | 1.668 | 1.284 |
| Area variation (F) | 0.837 | 0.221 | 0.473 | 0.236 |
| Hausdorff, ‰ of the diagonal (H) | 42.26 | 20.15 | 6.77 | 9.47 |
| Mean distance, ‰ (H) | 0.834 | 0.906 | 0.603 | 1.053 |
| Off budget, % (N) | 1.0 | 2.3 | 43.7 | 12.4 |
| Time, seconds (T) | 3.32 | 0.24 | 3.44 | 3.21 |

## ogre (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 4/5 | 1/1 | 0/1 |
| Irregular vertices (S) | 106 | 216 | 314 | 228 |
| … of which above the minimum | 98 | 202 | 306 | 220 |
| Base complex patches (C) | 2857 | 2802 | 5170 | 2009 |
| Alignment, degrees (A) | 19.31 | 18.57 | 8.42 | 10.81 |
| Mean angle deviation, degrees (F) | 12.01 | 7.49 | 9.10 | 11.45 |
| Angle deviation, 95th percentile (F) | 32.96 | 24.38 | 25.01 | 30.48 |
| Mean aspect ratio (F) | 1.403 | 1.283 | 2.364 | 1.612 |
| Area variation (F) | 0.324 | 0.224 | 0.716 | 0.393 |
| Hausdorff, ‰ of the diagonal (H) | 36.59 | 46.14 | 5.98 | 39.35 |
| Mean distance, ‰ (H) | 1.268 | 1.460 | 0.565 | 1.366 |
| Off budget, % (N) | 4.3 | 4.2 | 104.5 | 19.4 |
| Time, seconds (T) | 3.31 | 2.38 | 19.04 | 17.16 |

## spot (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 |
| Irregular vertices (S) | 68 | 153 | 82 | 87 |
| … of which above the minimum | 60 | 145 | 74 | 79 |
| Base complex patches (C) | 2902 | 2978 | 2428 | 2118 |
| Alignment, degrees (A) | 18.12 | 17.17 | 6.29 | 7.56 |
| Mean angle deviation, degrees (F) | 7.25 | 5.59 | 6.35 | 7.05 |
| Angle deviation, 95th percentile (F) | 20.42 | 18.71 | 17.74 | 19.43 |
| Mean aspect ratio (F) | 1.194 | 1.179 | 1.460 | 1.318 |
| Area variation (F) | 0.177 | 0.172 | 0.392 | 0.268 |
| Hausdorff, ‰ of the diagonal (H) | 17.71 | 19.38 | 6.64 | 10.63 |
| Mean distance, ‰ (H) | 0.448 | 0.479 | 0.372 | 0.476 |
| Off budget, % (N) | 1.2 | 1.5 | 15.9 | 10.5 |
| Time, seconds (T) | 1.71 | 1.25 | 19.14 | 15.16 |

## bob (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 |
| Irregular vertices (S) | 32 | 130 | 28 | 30 |
| … of which above the minimum | 32 | 130 | 28 | 30 |
| Base complex patches (C) | 2903 | 3040 | 2390 | 2568 |
| Alignment, degrees (A) | 19.37 | 20.80 | 4.85 | 5.15 |
| Mean angle deviation, degrees (F) | 6.41 | 5.33 | 5.52 | 5.94 |
| Angle deviation, 95th percentile (F) | 18.61 | 17.96 | 15.59 | 15.77 |
| Mean aspect ratio (F) | 1.175 | 1.158 | 1.368 | 1.360 |
| Area variation (F) | 0.159 | 0.161 | 0.360 | 0.324 |
| Hausdorff, ‰ of the diagonal (H) | 6.82 | 7.32 | 1.96 | 2.27 |
| Mean distance, ‰ (H) | 0.309 | 0.325 | 0.322 | 0.355 |
| Off budget, % (N) | 2.4 | 1.3 | 2.7 | 10.1 |
| Time, seconds (T) | 1.59 | 1.24 | 17.24 | 17.35 |

## blub (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 5/5 | 1/1 | 1/1 |
| Irregular vertices (S) | 52 | 158 | 55 | 46 |
| … of which above the minimum | 44 | 150 | 47 | 38 |
| Base complex patches (C) | 2584 | 2922 | 2003 | 2057 |
| Alignment, degrees (A) | 15.95 | 15.45 | 5.26 | 7.01 |
| Mean angle deviation, degrees (F) | 10.72 | 5.55 | 6.30 | 7.24 |
| Angle deviation, 95th percentile (F) | 28.70 | 19.72 | 16.74 | 18.88 |
| Mean aspect ratio (F) | 1.271 | 1.192 | 1.828 | 1.351 |
| Area variation (F) | 0.261 | 0.178 | 0.445 | 0.269 |
| Hausdorff, ‰ of the diagonal (H) | 40.70 | 15.68 | 5.19 | 11.32 |
| Mean distance, ‰ (H) | 0.588 | 0.390 | 0.230 | 0.392 |
| Off budget, % (N) | 10.1 | 1.2 | 29.6 | 9.9 |
| Time, seconds (T) | 1.77 | 1.11 | 14.34 | 12.32 |

## pig (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 1/1 |
| Irregular vertices (S) | 122 | 209 | 106 | 96 |
| … of which above the minimum | 114 | 192 | 98 | 88 |
| Base complex patches (C) | 2612 | 2912 | 3004 | 1840 |
| Alignment, degrees (A) | 17.41 | 16.24 | 7.22 | 9.36 |
| Mean angle deviation, degrees (F) | 14.32 | 6.83 | 7.22 | 7.93 |
| Angle deviation, 95th percentile (F) | 41.52 | 23.86 | 21.68 | 23.91 |
| Mean aspect ratio (F) | 1.383 | 1.244 | 1.876 | 1.322 |
| Area variation (F) | 0.398 | 0.210 | 0.636 | 0.287 |
| Hausdorff, ‰ of the diagonal (H) | 44.59 | 48.32 | 10.22 | 12.72 |
| Mean distance, ‰ (H) | 1.056 | 1.012 | 0.375 | 0.724 |
| Off budget, % (N) | 10.8 | 1.1 | 56.6 | 14.9 |
| Time, seconds (T) | 1.73 | 1.23 | 14.59 | 16.02 |

## jerry (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/5 | 0/1 | 1/1 |
| Irregular vertices (S) | 104 | 265 | 206 | 181 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | 2211 | 2959 | — | 2088 |
| Alignment, degrees (A) | 17.93 | 18.35 | 11.02 | 13.58 |
| Mean angle deviation, degrees (F) | 11.25 | 8.58 | 9.67 | 10.03 |
| Angle deviation, 95th percentile (F) | 29.73 | 29.29 | 26.54 | 28.28 |
| Mean aspect ratio (F) | 1.301 | 1.357 | 2.063 | 1.483 |
| Area variation (F) | 0.278 | 0.258 | 0.724 | 0.324 |
| Hausdorff, ‰ of the diagonal (H) | 38.82 | 49.21 | 6.74 | 15.07 |
| Mean distance, ‰ (H) | 1.467 | 2.158 | 0.529 | 1.488 |
| Off budget, % (N) | 4.1 | 1.4 | 129.1 | 14.2 |
| Time, seconds (T) | 1.72 | 0.67 | 6.74 | 6.79 |

## rose (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/5 | 0/1 | 0/1 |
| Irregular vertices (S) | 539 | 499 | 1772 | 763 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | 1872 | 1954 | — | — |
| Alignment, degrees (A) | 18.31 | 19.07 | 8.98 | 14.12 |
| Mean angle deviation, degrees (F) | 16.45 | 15.58 | 9.06 | 13.63 |
| Angle deviation, 95th percentile (F) | 48.37 | 48.45 | 26.38 | 39.32 |
| Mean aspect ratio (F) | 1.648 | 1.752 | 1.790 | 1.653 |
| Area variation (F) | 0.587 | 0.465 | 1.388 | 0.599 |
| Hausdorff, ‰ of the diagonal (H) | 75.61 | 79.25 | 74.46 | 78.76 |
| Mean distance, ‰ (H) | 3.761 | 4.183 | 0.842 | 3.206 |
| Off budget, % (N) | 30.5 | 25.8 | 1095.8 | 10.4 |
| Time, seconds (T) | 2.15 | 1.61 | 9.53 | 10.86 |

## bridge (organic)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 5/5 | 0/1 | 0/1 |
| Irregular vertices (S) | — | 151 | 81 | 64 |
| … of which above the minimum | — | — | — | — |
| Base complex patches (C) | — | 1879 | — | — |
| Alignment, degrees (A) | — | 12.58 | 6.49 | 7.93 |
| Mean angle deviation, degrees (F) | — | 6.12 | 6.65 | 7.20 |
| Angle deviation, 95th percentile (F) | — | 20.51 | 18.96 | 19.72 |
| Mean aspect ratio (F) | — | 1.215 | 1.753 | 1.413 |
| Area variation (F) | — | 0.174 | 0.421 | 0.275 |
| Hausdorff, ‰ of the diagonal (H) | — | 28.19 | 6.05 | 7.00 |
| Mean distance, ‰ (H) | — | 0.638 | 0.414 | 0.670 |
| Off budget, % (N) | — | 3.2 | 51.1 | 9.1 |
| Time, seconds (T) | — | 1.96 | 11.41 | 11.88 |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 5/5 | 1/1 | 1/1 |
| Irregular vertices (S) | 28 | 204 | 24 | 24 |
| … of which above the minimum | 20 | 196 | 16 | 16 |
| Base complex patches (C) | 619 | 1782 | 501 | 501 |
| Alignment, degrees (A) | 12.97 | 13.96 | 10.90 | 10.90 |
| Mean angle deviation, degrees (F) | 2.66 | 6.10 | 3.02 | 3.02 |
| Angle deviation, 95th percentile (F) | 10.11 | 25.03 | 12.04 | 12.04 |
| Mean aspect ratio (F) | 1.116 | 1.233 | 1.326 | 1.326 |
| Area variation (F) | 0.100 | 0.199 | 0.196 | 0.196 |
| Hausdorff, ‰ of the diagonal (H) | 5.54 | 9.41 | 0.89 | 0.89 |
| Mean distance, ‰ (H) | 0.114 | 0.067 | 0.068 | 0.068 |
| Off budget, % (N) | 3.7 | 2.1 | 6.2 | 6.2 |
| Time, seconds (T) | 2.39 | 0.87 | 2.83 | 2.88 |

## cube (mechanical)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 24 | 8 | 8 |
| … of which above the minimum | 0 | 16 | 0 | 0 |
| Base complex patches (C) | 6 | 274 | 6 | 6 |
| Alignment, degrees (A) | 0.29 | 1.97 | 0.13 | 0.13 |
| Mean angle deviation, degrees (F) | 0.18 | 1.65 | 0.00 | 0.00 |
| Angle deviation, 95th percentile (F) | 0.43 | 4.89 | 0.00 | 0.00 |
| Mean aspect ratio (F) | 1.005 | 1.062 | 1.030 | 1.030 |
| Area variation (F) | 0.004 | 0.090 | 0.021 | 0.021 |
| Hausdorff, ‰ of the diagonal (H) | 4.80 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.017 | 0.000 | 0.000 | 0.000 |
| Off budget, % (N) | 3.2 | 1.0 | 2.7 | 2.7 |
| Time, seconds (T) | 1.03 | 0.10 | 2.97 | 3.00 |

## sphere (calibration)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 5/5 | 1/1 | 1/1 |
| Irregular vertices (S) | 8 | 97 | 10 | 10 |
| … of which above the minimum | 0 | 89 | 2 | 2 |
| Base complex patches (C) | 2730 | 3090 | 1794 | 1794 |
| Alignment, degrees (A) | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.49 | 4.19 | 6.09 | 6.09 |
| Angle deviation, 95th percentile (F) | 12.67 | 14.49 | 15.49 | 15.49 |
| Mean aspect ratio (F) | 1.125 | 1.124 | 1.185 | 1.185 |
| Area variation (F) | 0.135 | 0.136 | 0.312 | 0.312 |
| Hausdorff, ‰ of the diagonal (H) | 0.63 | 0.72 | 0.82 | 0.82 |
| Mean distance, ‰ (H) | 0.216 | 0.221 | 0.258 | 0.258 |
| Off budget, % (N) | 3.6 | 3.0 | 8.7 | 8.7 |
| Time, seconds (T) | 1.73 | 0.29 | 3.56 | 3.39 |

## torus (calibration)

| | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 5/5 | 1/1 | 1/1 |
| Irregular vertices (S) | 30 | 125 | 10 | 10 |
| … of which above the minimum | 30 | 125 | 10 | 10 |
| Base complex patches (C) | 2976 | 3078 | 2264 | 2264 |
| Alignment, degrees (A) | 10.14 | 14.28 | 6.90 | 6.90 |
| Mean angle deviation, degrees (F) | 10.25 | 5.57 | 5.23 | 5.23 |
| Angle deviation, 95th percentile (F) | 26.29 | 18.30 | 14.40 | 14.40 |
| Mean aspect ratio (F) | 1.239 | 1.163 | 1.213 | 1.213 |
| Area variation (F) | 0.223 | 0.163 | 0.233 | 0.233 |
| Hausdorff, ‰ of the diagonal (H) | 1.86 | 1.38 | 1.71 | 1.71 |
| Mean distance, ‰ (H) | 0.299 | 0.298 | 0.518 | 0.518 |
| Off budget, % (N) | 0.8 | 2.6 | 5.9 | 5.9 |
| Time, seconds (T) | 1.40 | 0.30 | 4.71 | 2.89 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| Irregular vertices (S) | 5 | 1 | 1 | 6 |
| Base complex patches (C) | 1 | 1 | 2 | 9 |
| Alignment, degrees (A) | 0 | 0 | 13 | 0 |
| Mean angle deviation, degrees (F) | 0 | 10 | 2 | 1 |
| Mean aspect ratio (F) | 4 | 9 | 0 | 0 |
| Valid meshes (V) | 36/65 | 47/65 | 8/13 | 10/13 |

### mechanical (2 models)

| Metric | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| Irregular vertices (S) | 1 | 0 | 2 | 2 |
| Base complex patches (C) | 1 | 0 | 2 | 2 |
| Alignment, degrees (A) | 0 | 0 | 2 | 2 |
| Mean angle deviation, degrees (F) | 1 | 0 | 1 | 1 |
| Mean aspect ratio (F) | 2 | 0 | 0 | 0 |
| Valid meshes (V) | 9/10 | 10/10 | 2/2 | 2/2 |
