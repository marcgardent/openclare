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

### organic (6 models)

| Metric | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| Irregular vertices (S) | 3 | 0 | 0 | 3 |
| Base complex patches (C) | 0 | 0 | 0 | 6 |
| Alignment, degrees (A) | 0 | 0 | 6 | 0 |
| Mean angle deviation, degrees (F) | 0 | 4 | 1 | 1 |
| Mean aspect ratio (F) | 2 | 4 | 0 | 0 |
| Valid meshes (V) | 19/30 | 26/30 | 5/6 | 5/6 |

### mechanical (2 models)

| Metric | quadriflow | instant-meshes | quadremesher | quadremesher-uniform |
|---|---|---|---|---|
| Irregular vertices (S) | 1 | 0 | 2 | 2 |
| Base complex patches (C) | 1 | 0 | 2 | 2 |
| Alignment, degrees (A) | 0 | 0 | 2 | 2 |
| Mean angle deviation, degrees (F) | 1 | 0 | 1 | 1 |
| Mean aspect ratio (F) | 2 | 0 | 0 | 0 |
| Valid meshes (V) | 9/10 | 10/10 | 2/2 | 2/2 |
