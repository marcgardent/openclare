# Bench results

Requested budget: 3000 quads. Median over 5 seeds; Quad Remesher, AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

The last two columns compare at equal quad count: `quadremesher-as-made` is the mesh of `quadremesher` measured against the number of quads it has rather than the budget, and `openclare-matched` is `openclare` asked for that same number.

## blob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2781 | 1165 | 3090 | 2507 | 3021 | 3005 | 3370 | 2755 | 3370 | 3392 |
| Irregular vertices (S) | 38 | 26 | 143 | 26 | 24 | 24 | 26 | 24 | 26 | 24 |
| … of which above the minimum | 30 | — | 135 | 18 | 16 | 16 | 18 | 16 | 18 | 16 |
| Base complex patches (C) | 2697 | — | 3048 | 951 | 2361 | 2045 | 1826 | 1350 | 1826 | 2274 |
| Alignment, degrees (A) | 17.48 | 6.27 | 15.74 | 6.23 | 5.35 | 4.89 | 3.93 | 4.55 | 3.94 | 4.94 |
| Mean angle deviation, degrees (F) | 8.71 | 9.45 | 5.87 | 7.89 | 6.14 | 5.67 | 5.76 | 5.45 | 5.76 | 5.64 |
| Angle deviation, 95th percentile (F) | 27.30 | 30.58 | 18.26 | 21.16 | 16.75 | 15.14 | 17.21 | 15.61 | 17.21 | 15.31 |
| Mean aspect ratio (F) | 1.187 | 1.934 | 1.174 | 1.427 | 1.413 | 1.369 | 1.618 | 1.276 | 1.618 | 1.351 |
| Area variation (F) | 0.205 | 0.575 | 0.175 | 0.351 | 0.374 | 0.394 | 0.442 | 0.313 | 0.442 | 0.393 |
| Hausdorff, ‰ of the diagonal (H) | 8.52 | 6.35 | 10.14 | 5.25 | 3.20 | 2.43 | 4.21 | 7.28 | 4.21 | 2.83 |
| Mean distance, ‰ (H) | 0.486 | 1.885 | 0.459 | 0.272 | 0.196 | 0.188 | 0.407 | 0.513 | 0.407 | 0.165 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.56 | 96.90 | 99.63 | 100.03 | 100.02 | 100.02 | 99.40 | 99.52 | 99.40 | 100.01 |
| Off budget, % (N) | 7.3 | 61.2 | 3.0 | 16.4 | 0.7 | 0.2 | 12.3 | 8.2 | 0.0 | 0.7 |
| Time, seconds (T) | 2.63 | 0.94 | 1.71 | 4.09 | 2.49 | 2.73 | 16.38 | 12.28 | 16.38 | 3.22 |

## rock (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 3036 | 2412 | 2898 | 2687 | 2968 | 2984 | 3539 | 2531 | 3539 | 3589 |
| Irregular vertices (S) | 8 | 20 | 91 | 50 | 50 | 66 | 114 | 101 | 114 | 76 |
| … of which above the minimum | 0 | 12 | 83 | 42 | 42 | 58 | 106 | 93 | 106 | 68 |
| Base complex patches (C) | 2855 | 2412 | 2896 | 2687 | 2968 | 2984 | 3539 | 2531 | 3539 | 3589 |
| Alignment, degrees (A) | 22.96 | 18.78 | 23.21 | 15.81 | 16.41 | 15.76 | 14.17 | 14.68 | 14.63 | 15.63 |
| Mean angle deviation, degrees (F) | 5.84 | 7.48 | 5.18 | 8.82 | 5.36 | 5.07 | 10.07 | 10.61 | 10.07 | 5.49 |
| Angle deviation, 95th percentile (F) | 16.05 | 17.91 | 14.78 | 22.11 | 13.98 | 13.33 | 24.88 | 26.21 | 24.88 | 14.78 |
| Mean aspect ratio (F) | 1.164 | 1.409 | 1.146 | 1.253 | 1.213 | 1.213 | 1.470 | 1.393 | 1.470 | 1.223 |
| Area variation (F) | 0.202 | 0.331 | 0.151 | 0.229 | 0.270 | 0.290 | 0.308 | 0.269 | 0.308 | 0.269 |
| Hausdorff, ‰ of the diagonal (H) | 5.45 | 7.64 | 5.28 | 4.49 | 4.60 | 4.65 | 5.13 | 5.79 | 5.13 | 4.56 |
| Mean distance, ‰ (H) | 0.788 | 1.157 | 0.820 | 0.740 | 0.698 | 0.695 | 0.731 | 0.889 | 0.731 | 0.626 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.78 | 99.49 | 99.70 | 100.01 | 100.00 | 100.00 | 99.73 | 99.62 | 99.73 | 100.00 |
| Off budget, % (N) | 2.2 | 19.6 | 3.4 | 10.4 | 1.1 | 0.5 | 18.0 | 15.6 | 0.0 | 1.4 |
| Time, seconds (T) | 1.41 | 0.85 | 1.03 | 6.92 | 1.84 | 2.26 | 20.49 | 16.98 | 20.49 | 3.68 |

## creature (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2667 | 1991 | 3070 | 2597 | 3041 | 2955 | 2941 | 2719 | 2941 | 2894 |
| Irregular vertices (S) | 24 | 20 | 111 | 41 | 40 | 40 | 42 | 46 | 42 | 40 |
| … of which above the minimum | 16 | 12 | 103 | 33 | 32 | 32 | 34 | 38 | 34 | 32 |
| Base complex patches (C) | 2667 | 747 | 2856 | 913 | 1301 | 2839 | 1465 | 1301 | 1465 | 1820 |
| Alignment, degrees (A) | 12.89 | 9.58 | 12.09 | 6.96 | 6.95 | 5.75 | 5.18 | 5.32 | 5.12 | 5.68 |
| Mean angle deviation, degrees (F) | 6.85 | 4.75 | 4.65 | 8.21 | 5.72 | 5.09 | 6.12 | 6.53 | 6.12 | 5.19 |
| Angle deviation, 95th percentile (F) | 18.98 | 13.44 | 14.73 | 24.05 | 15.03 | 13.57 | 17.13 | 18.98 | 17.13 | 14.22 |
| Mean aspect ratio (F) | 1.189 | 1.839 | 1.144 | 1.305 | 1.280 | 1.297 | 1.332 | 1.334 | 1.332 | 1.329 |
| Area variation (F) | 0.237 | 0.549 | 0.146 | 0.286 | 0.338 | 0.496 | 0.348 | 0.313 | 0.348 | 0.472 |
| Hausdorff, ‰ of the diagonal (H) | 8.01 | 2.39 | 3.47 | 2.16 | 2.06 | 2.05 | 2.51 | 2.37 | 2.51 | 1.90 |
| Mean distance, ‰ (H) | 0.320 | 0.537 | 0.279 | 0.231 | 0.191 | 0.201 | 0.297 | 0.348 | 0.297 | 0.205 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.41 | 98.97 | 99.47 | 100.00 | 100.02 | 100.03 | 99.39 | 99.28 | 99.39 | 100.02 |
| Off budget, % (N) | 11.1 | 33.6 | 2.3 | 13.4 | 1.4 | 1.5 | 2.0 | 9.4 | 0.0 | 1.6 |
| Time, seconds (T) | 1.69 | 1.19 | 0.93 | 6.08 | 2.75 | 2.65 | 11.24 | 9.11 | 11.24 | 2.30 |

## cage (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2891 | 4039 | 2944 | 2874 | 3040 | 2968 | 5809 | 2785 | 5809 | 5890 |
| Irregular vertices (S) | 66 | 97 | 200 | 41 | 48 | 48 | 44 | 43 | 44 | 52 |
| … of which above the minimum | 34 | — | 168 | 9 | 16 | 16 | 12 | 11 | 12 | 20 |
| Base complex patches (C) | 1943 | — | 2319 | 427 | 689 | 304 | 1092 | 1001 | 1092 | 498 |
| Alignment, degrees (A) | 8.10 | 8.92 | 7.97 | 3.38 | 3.16 | 2.31 | 2.04 | 2.57 | 2.08 | 2.11 |
| Mean angle deviation, degrees (F) | 5.86 | 9.02 | 6.39 | 4.00 | 2.94 | 3.22 | 3.91 | 4.30 | 3.91 | 2.59 |
| Angle deviation, 95th percentile (F) | 17.11 | 28.25 | 23.33 | 12.37 | 8.99 | 10.15 | 11.90 | 12.82 | 11.90 | 7.10 |
| Mean aspect ratio (F) | 1.183 | 2.030 | 1.252 | 1.202 | 1.184 | 1.227 | 1.993 | 1.318 | 1.993 | 1.165 |
| Area variation (F) | 0.147 | 0.496 | 0.203 | 0.193 | 0.205 | 0.332 | 0.314 | 0.238 | 0.314 | 0.377 |
| Hausdorff, ‰ of the diagonal (H) | 11.45 | 13.32 | 14.46 | 11.74 | 10.64 | 10.57 | 5.74 | 9.66 | 5.74 | 3.03 |
| Mean distance, ‰ (H) | 0.734 | 0.800 | 0.860 | 0.814 | 0.790 | 0.494 | 0.693 | 1.276 | 0.693 | 0.250 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.99 | 99.63 | 98.70 | 99.89 | 99.93 | 99.98 | 99.00 | 97.85 | 99.00 | 100.00 |
| Off budget, % (N) | 3.6 | 34.6 | 1.9 | 4.2 | 1.3 | 1.1 | 93.6 | 7.2 | 0.0 | 1.4 |
| Time, seconds (T) | 1.57 | 2.53 | 0.34 | 8.42 | 2.58 | 2.47 | 4.21 | 3.92 | 4.21 | 4.81 |

## suzanne (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 2/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | 3030 | 1255 | 3070 | 2757 | 2980 | 2986 | 4310 | 2628 | 4310 | 4306 |
| Irregular vertices (S) | 78 | 69 | 212 | 52 | 53 | 56 | 80 | 71 | 80 | 56 |
| … of which above the minimum | 54 | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2884 | — | 3002 | 1858 | 2472 | 2000 | 2513 | 1838 | 2513 | 1885 |
| Alignment, degrees (A) | 15.33 | 13.19 | 17.02 | 10.46 | 10.35 | 9.87 | 8.75 | 10.24 | 8.92 | 9.85 |
| Mean angle deviation, degrees (F) | 8.61 | 11.13 | 7.08 | 7.14 | 4.34 | 4.31 | 7.87 | 7.87 | 7.87 | 4.33 |
| Angle deviation, 95th percentile (F) | 26.08 | 41.63 | 25.30 | 18.16 | 11.83 | 12.01 | 21.85 | 20.46 | 21.85 | 11.09 |
| Mean aspect ratio (F) | 1.260 | 1.871 | 1.263 | 1.221 | 1.213 | 1.197 | 1.668 | 1.284 | 1.668 | 1.221 |
| Area variation (F) | 0.837 | 0.611 | 0.221 | 0.250 | 0.278 | 0.374 | 0.473 | 0.236 | 0.473 | 0.369 |
| Hausdorff, ‰ of the diagonal (H) | 42.26 | 33.04 | 20.15 | 8.61 | 9.76 | 7.83 | 6.77 | 9.47 | 6.77 | 5.52 |
| Mean distance, ‰ (H) | 0.834 | 2.167 | 0.906 | 0.738 | 0.575 | 0.477 | 0.603 | 1.053 | 0.603 | 0.338 |
| Thin parts: mean distance, % of a quad edge (V) | 16.4 | 28.9 | 21.4 | 16.0 | 8.6 | 6.8 | 5.1 | 17.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 1.0 | 58.2 | 2.3 | 8.1 | 0.7 | 0.5 | 43.7 | 12.4 | 0.0 | 0.1 |
| Time, seconds (T) | 3.22 | 0.84 | 0.28 | 8.24 | 2.07 | 2.74 | 3.44 | 3.21 | 3.44 | 3.06 |

## ogre (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 0/1 | 4/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 |
| Quads made | 2935 | 1081 | 2874 | 2465 | 3001 | 2949 | 6134 | 2417 | 6134 | 6179 |
| Irregular vertices (S) | 106 | 84 | 216 | 77 | 78 | 155 | 314 | 228 | 314 | 212 |
| … of which above the minimum | 98 | — | 202 | — | 70 | 147 | 306 | 220 | 306 | 204 |
| Base complex patches (C) | 2857 | — | 2802 | — | 2715 | 2405 | 5170 | 2009 | 5170 | 4911 |
| Alignment, degrees (A) | 19.31 | 16.24 | 18.57 | 12.24 | 12.24 | 11.43 | 8.42 | 10.81 | 9.14 | 10.80 |
| Mean angle deviation, degrees (F) | 12.01 | 10.37 | 7.49 | 9.03 | 6.52 | 6.33 | 9.10 | 11.45 | 9.10 | 6.10 |
| Angle deviation, 95th percentile (F) | 32.96 | 32.56 | 24.38 | 23.88 | 17.53 | 17.64 | 25.01 | 30.48 | 25.01 | 16.94 |
| Mean aspect ratio (F) | 1.403 | 1.826 | 1.283 | 1.546 | 1.510 | 1.436 | 2.364 | 1.612 | 2.364 | 1.367 |
| Area variation (F) | 0.324 | 0.590 | 0.224 | 0.389 | 0.387 | 0.499 | 0.716 | 0.393 | 0.716 | 0.460 |
| Hausdorff, ‰ of the diagonal (H) | 36.59 | 31.05 | 46.14 | 35.86 | 26.72 | 29.35 | 5.98 | 39.35 | 5.98 | 19.72 |
| Mean distance, ‰ (H) | 1.268 | 3.042 | 1.460 | 1.109 | 0.785 | 0.703 | 0.565 | 1.366 | 0.565 | 0.368 |
| Thin parts: mean distance, % of a quad edge (V) | 54.0 | 66.6 | 99.5 | 66.0 | 28.7 | 6.9 | 6.7 | 75.2 | 9.8 | 4.9 |
| Volume kept, % of the source (V) | 98.56 | — | 98.55 | 99.78 | 99.93 | 100.01 | 99.24 | 98.42 | 99.24 | 100.00 |
| Off budget, % (N) | 4.3 | 64.0 | 4.2 | 17.8 | 0.0 | 1.7 | 104.5 | 19.4 | 0.0 | 0.7 |
| Time, seconds (T) | 3.94 | 1.51 | 2.70 | 8.13 | 3.38 | 5.17 | 19.04 | 17.16 | 19.04 | 8.55 |

## spot (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2964 | 1284 | 3044 | 2659 | 2995 | 2951 | 3478 | 2686 | 3478 | 3436 |
| Irregular vertices (S) | 68 | 68 | 153 | 70 | 70 | 70 | 82 | 87 | 82 | 70 |
| … of which above the minimum | 60 | 60 | 145 | 62 | 62 | 62 | 74 | 79 | 74 | 62 |
| Base complex patches (C) | 2902 | 1022 | 2978 | 1987 | 1775 | 1649 | 2428 | 2118 | 2428 | 1864 |
| Alignment, degrees (A) | 18.12 | 12.08 | 17.17 | 9.37 | 8.78 | 7.85 | 6.29 | 7.56 | 6.35 | 7.82 |
| Mean angle deviation, degrees (F) | 7.25 | 8.87 | 5.59 | 9.51 | 5.60 | 5.28 | 6.35 | 7.05 | 6.35 | 5.67 |
| Angle deviation, 95th percentile (F) | 20.42 | 24.77 | 18.71 | 24.15 | 14.97 | 14.97 | 17.74 | 19.43 | 17.74 | 15.34 |
| Mean aspect ratio (F) | 1.194 | 1.451 | 1.179 | 1.344 | 1.347 | 1.340 | 1.460 | 1.318 | 1.460 | 1.330 |
| Area variation (F) | 0.177 | 0.666 | 0.172 | 0.289 | 0.306 | 0.426 | 0.392 | 0.268 | 0.392 | 0.435 |
| Hausdorff, ‰ of the diagonal (H) | 17.71 | 17.26 | 19.38 | 8.80 | 4.98 | 5.58 | 6.64 | 10.63 | 6.64 | 3.34 |
| Mean distance, ‰ (H) | 0.448 | 1.588 | 0.479 | 0.237 | 0.200 | 0.199 | 0.372 | 0.476 | 0.372 | 0.173 |
| Thin parts: mean distance, % of a quad edge (V) | 56.3 | 54.6 | 53.8 | 14.8 | 4.1 | 3.5 | 8.5 | 21.2 | — | — |
| Volume kept, % of the source (V) | 99.37 | 97.44 | 99.35 | 100.00 | 100.03 | 100.01 | 99.35 | 99.27 | 99.35 | 100.01 |
| Off budget, % (N) | 1.2 | 57.2 | 1.5 | 11.4 | 0.2 | 1.6 | 15.9 | 10.5 | 0.0 | 1.2 |
| Time, seconds (T) | 2.06 | 1.30 | 1.23 | 6.69 | 2.65 | 4.53 | 19.14 | 15.16 | 19.14 | 3.33 |

## bob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2928 | 1583 | 3040 | 2642 | 3042 | 2950 | 2920 | 2696 | 2920 | 2947 |
| Irregular vertices (S) | 32 | 18 | 130 | 20 | 20 | 20 | 28 | 30 | 28 | 20 |
| … of which above the minimum | 32 | 18 | 130 | 20 | 20 | 20 | 28 | 30 | 28 | 20 |
| Base complex patches (C) | 2903 | 533 | 3040 | 714 | 1290 | 237 | 2390 | 2568 | 2390 | 908 |
| Alignment, degrees (A) | 19.37 | 6.64 | 20.80 | 4.37 | 4.01 | 3.61 | 4.85 | 5.15 | 4.84 | 3.98 |
| Mean angle deviation, degrees (F) | 6.41 | 7.07 | 5.33 | 4.82 | 3.29 | 3.25 | 5.52 | 5.94 | 5.52 | 3.53 |
| Angle deviation, 95th percentile (F) | 18.61 | 21.04 | 17.96 | 14.72 | 8.85 | 9.11 | 15.59 | 15.77 | 15.59 | 9.90 |
| Mean aspect ratio (F) | 1.175 | 2.090 | 1.158 | 1.409 | 1.385 | 1.370 | 1.368 | 1.360 | 1.368 | 1.346 |
| Area variation (F) | 0.159 | 0.474 | 0.161 | 0.339 | 0.374 | 0.433 | 0.360 | 0.324 | 0.360 | 0.435 |
| Hausdorff, ‰ of the diagonal (H) | 6.82 | 4.03 | 7.32 | 2.49 | 1.95 | 1.40 | 1.96 | 2.27 | 1.96 | 1.91 |
| Mean distance, ‰ (H) | 0.309 | 0.960 | 0.325 | 0.152 | 0.131 | 0.132 | 0.322 | 0.355 | 0.322 | 0.132 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.48 | 98.10 | 99.44 | 100.01 | 100.02 | 100.02 | 99.37 | 99.31 | 99.37 | 100.02 |
| Off budget, % (N) | 2.4 | 47.2 | 1.3 | 11.9 | 1.4 | 1.7 | 2.7 | 10.1 | 0.0 | 0.9 |
| Time, seconds (T) | 2.03 | 1.07 | 1.45 | 6.22 | 2.35 | 2.39 | 17.24 | 17.35 | 17.24 | 2.29 |

## blub (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2698 | 1343 | 3036 | 2680 | 2948 | 3030 | 3889 | 2703 | 3889 | 3911 |
| Irregular vertices (S) | 52 | 47 | 158 | 43 | 44 | 58 | 55 | 46 | 55 | 52 |
| … of which above the minimum | 44 | — | 150 | 35 | 36 | 50 | 47 | 38 | 47 | 44 |
| Base complex patches (C) | 2584 | — | 2922 | 902 | 886 | 1258 | 2003 | 2057 | 2003 | 1957 |
| Alignment, degrees (A) | 15.95 | 10.62 | 15.45 | 7.64 | 7.79 | 7.78 | 5.26 | 7.01 | 5.48 | 7.34 |
| Mean angle deviation, degrees (F) | 10.72 | 9.14 | 5.55 | 8.08 | 5.22 | 4.94 | 6.30 | 7.24 | 6.30 | 5.27 |
| Angle deviation, 95th percentile (F) | 28.70 | 28.42 | 19.72 | 22.68 | 16.04 | 14.85 | 16.74 | 18.88 | 16.74 | 15.44 |
| Mean aspect ratio (F) | 1.271 | 2.046 | 1.192 | 1.327 | 1.292 | 1.290 | 1.828 | 1.351 | 1.828 | 1.307 |
| Area variation (F) | 0.261 | 0.522 | 0.178 | 0.266 | 0.302 | 0.407 | 0.445 | 0.269 | 0.445 | 0.383 |
| Hausdorff, ‰ of the diagonal (H) | 40.70 | 17.71 | 15.68 | 12.42 | 8.81 | 8.13 | 5.19 | 11.32 | 5.19 | 4.71 |
| Mean distance, ‰ (H) | 0.588 | 1.082 | 0.390 | 0.254 | 0.190 | 0.187 | 0.230 | 0.392 | 0.230 | 0.123 |
| Thin parts: mean distance, % of a quad edge (V) | 46.5 | 17.7 | 24.8 | 19.4 | 9.5 | 7.9 | 7.7 | 20.0 | 10.8 | 4.4 |
| Volume kept, % of the source (V) | 98.93 | — | 99.17 | 99.88 | 99.96 | 99.96 | 99.55 | 99.13 | 99.55 | 99.99 |
| Off budget, % (N) | 10.1 | 55.2 | 1.2 | 10.7 | 1.7 | 1.0 | 29.6 | 9.9 | 0.0 | 0.6 |
| Time, seconds (T) | 1.82 | 0.93 | 1.25 | 6.03 | 2.85 | 4.05 | 14.34 | 12.32 | 14.34 | 3.67 |

## pig (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | 2675 | 1808 | 3016 | 2680 | 3047 | 3054 | 4698 | 2552 | 4698 | 4652 |
| Irregular vertices (S) | 122 | 99 | 209 | 82 | 78 | 104 | 106 | 96 | 106 | 104 |
| … of which above the minimum | 114 | — | 192 | — | 70 | 96 | 98 | 88 | 98 | 96 |
| Base complex patches (C) | 2612 | — | 2912 | — | 1955 | 1690 | 3004 | 1840 | 3004 | 3430 |
| Alignment, degrees (A) | 17.41 | 11.38 | 16.24 | 10.81 | 10.71 | 9.17 | 7.22 | 9.36 | 7.34 | 8.81 |
| Mean angle deviation, degrees (F) | 14.32 | 8.90 | 6.83 | 8.29 | 5.91 | 5.63 | 7.22 | 7.93 | 7.22 | 5.57 |
| Angle deviation, 95th percentile (F) | 41.52 | 29.13 | 23.86 | 23.45 | 17.32 | 17.28 | 21.68 | 23.91 | 21.68 | 16.01 |
| Mean aspect ratio (F) | 1.383 | 1.804 | 1.244 | 1.266 | 1.263 | 1.286 | 1.876 | 1.322 | 1.876 | 1.274 |
| Area variation (F) | 0.398 | 0.713 | 0.210 | 0.262 | 0.294 | 0.483 | 0.636 | 0.287 | 0.636 | 0.468 |
| Hausdorff, ‰ of the diagonal (H) | 44.59 | 52.50 | 48.32 | 25.35 | 16.51 | 12.40 | 10.22 | 12.72 | 10.22 | 8.02 |
| Mean distance, ‰ (H) | 1.056 | 2.185 | 1.012 | 0.532 | 0.400 | 0.310 | 0.375 | 0.724 | 0.375 | 0.205 |
| Thin parts: mean distance, % of a quad edge (V) | 67.8 | 29.1 | 26.2 | 25.0 | 16.6 | 5.5 | 3.7 | 16.4 | 6.3 | 7.0 |
| Volume kept, % of the source (V) | 98.78 | 97.54 | 98.84 | 99.69 | 99.84 | 99.97 | 99.54 | 98.93 | 99.54 | 99.98 |
| Off budget, % (N) | 10.8 | 39.7 | 1.1 | 10.7 | 1.6 | 1.8 | 56.6 | 14.9 | 0.0 | 1.0 |
| Time, seconds (T) | 1.94 | 2.15 | 1.15 | 9.77 | 2.21 | 3.42 | 14.59 | 16.02 | 14.59 | 6.22 |

## jerry (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 1/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | 3122 | 1102 | 2959 | 2595 | 3014 | 3049 | 6872 | 2573 | 6872 | 6903 |
| Irregular vertices (S) | 104 | 84 | 265 | 97 | 84 | 162 | 206 | 181 | 206 | 266 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2211 | — | 2959 | 1552 | 2007 | 2513 | — | 2088 | — | 5628 |
| Alignment, degrees (A) | 17.93 | 17.98 | 18.35 | 14.87 | 14.06 | 13.57 | 11.02 | 13.58 | 11.29 | 12.48 |
| Mean angle deviation, degrees (F) | 11.25 | 15.26 | 8.58 | 9.60 | 6.14 | 6.58 | 9.67 | 10.03 | 9.67 | 6.15 |
| Angle deviation, 95th percentile (F) | 29.73 | 53.91 | 29.29 | 24.87 | 16.28 | 21.17 | 26.54 | 28.28 | 26.54 | 17.35 |
| Mean aspect ratio (F) | 1.301 | 2.330 | 1.357 | 1.324 | 1.292 | 1.328 | 2.063 | 1.483 | 2.063 | 1.285 |
| Area variation (F) | 0.278 | 2.551 | 0.258 | 0.272 | 0.307 | 0.539 | 0.724 | 0.324 | 0.724 | 0.514 |
| Hausdorff, ‰ of the diagonal (H) | 38.82 | 200.69 | 49.21 | 48.31 | 32.58 | 21.64 | 6.74 | 15.07 | 6.74 | 13.26 |
| Mean distance, ‰ (H) | 1.467 | 5.546 | 2.158 | 1.786 | 1.092 | 0.831 | 0.529 | 1.488 | 0.529 | 0.332 |
| Thin parts: mean distance, % of a quad edge (V) | 23.4 | 76.1 | 40.3 | 57.3 | 29.9 | 15.3 | 4.8 | 19.2 | 6.8 | 9.7 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 4.1 | 63.3 | 1.4 | 13.5 | 0.5 | 1.6 | 129.1 | 14.2 | 0.0 | 0.5 |
| Time, seconds (T) | 2.10 | 1.29 | 0.59 | 7.59 | 1.94 | 3.13 | 6.74 | 6.79 | 6.74 | 5.95 |

## rose (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 2085 | 636 | 2226 | 1923 | 3831 | 5672 | 35874 | 2689 | 35874 | 37852 |
| Irregular vertices (S) | 539 | 243 | 499 | 252 | 933 | 1232 | 1772 | 763 | 1772 | 2409 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | 1793 |
| Base complex patches (C) | 1872 | — | 1954 | — | — | — | — | — | — | 29294 |
| Alignment, degrees (A) | 18.39 | 19.39 | 19.07 | 16.12 | 14.13 | 14.30 | 8.98 | 14.12 | 11.16 | 9.24 |
| Mean angle deviation, degrees (F) | 16.45 | 31.06 | 15.58 | 13.29 | 18.55 | 16.22 | 9.06 | 13.63 | 9.06 | 6.87 |
| Angle deviation, 95th percentile (F) | 48.25 | 82.38 | 48.45 | 38.66 | 60.88 | 49.81 | 26.38 | 39.32 | 26.38 | 21.56 |
| Mean aspect ratio (F) | 1.647 | 5.541 | 1.752 | 1.809 | 5.653 | 2.716 | 1.790 | 1.653 | 1.790 | 1.441 |
| Area variation (F) | 0.587 | 1.193 | 0.465 | 0.441 | 1.092 | 1.391 | 1.388 | 0.599 | 1.388 | 0.512 |
| Hausdorff, ‰ of the diagonal (H) | 75.61 | 108.28 | 79.25 | 72.60 | 39.04 | 25.08 | 74.46 | 78.76 | 74.46 | 7.95 |
| Mean distance, ‰ (H) | 3.772 | 21.969 | 4.183 | 3.665 | 2.880 | 1.864 | 0.842 | 3.206 | 0.842 | 0.154 |
| Thin parts: mean distance, % of a quad edge (V) | 39.1 | 157.4 | 46.1 | 40.1 | 26.8 | 18.7 | 11.9 | 35.3 | 116.7 | 9.5 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | 99.66 |
| Off budget, % (N) | 30.5 | 78.8 | 25.8 | 35.9 | 27.7 | 89.1 | 1095.8 | 10.4 | 0.0 | 5.5 |
| Time, seconds (T) | 2.77 | 1.32 | 2.12 | 16.37 | 3.38 | 4.29 | 9.53 | 10.86 | 9.53 | 20.00 |

## bridge (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 |
| Quads made | — | 1893 | 2903 | 2750 | 3019 | 2950 | 4533 | 2726 | 4533 | 4467 |
| Irregular vertices (S) | — | 63 | 151 | 53 | 50 | 44 | 81 | 64 | 81 | 54 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1879 | 520 | 1196 | 703 | — | — | — | 1557 |
| Alignment, degrees (A) | — | 9.17 | 12.58 | 8.69 | 7.96 | 7.44 | 6.49 | 7.93 | 7.33 | 8.02 |
| Mean angle deviation, degrees (F) | — | 7.60 | 6.12 | 7.12 | 3.86 | 4.03 | 6.65 | 7.20 | 6.65 | 4.60 |
| Angle deviation, 95th percentile (F) | — | 22.56 | 20.51 | 20.62 | 11.22 | 11.93 | 18.96 | 19.72 | 18.96 | 12.98 |
| Mean aspect ratio (F) | — | 1.778 | 1.215 | 1.305 | 1.258 | 1.260 | 1.753 | 1.413 | 1.753 | 1.276 |
| Area variation (F) | — | 0.765 | 0.174 | 0.232 | 0.291 | 0.435 | 0.421 | 0.275 | 0.421 | 0.391 |
| Hausdorff, ‰ of the diagonal (H) | — | 14.76 | 28.19 | 7.13 | 7.15 | 6.65 | 6.05 | 7.00 | 6.05 | 6.60 |
| Mean distance, ‰ (H) | — | 1.042 | 0.638 | 0.500 | 0.452 | 0.447 | 0.414 | 0.670 | 0.414 | 0.338 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 36.9 | 3.2 | 8.3 | 0.6 | 1.7 | 51.1 | 9.1 | 0.0 | 1.5 |
| Time, seconds (T) | — | 1.40 | 2.98 | 10.79 | 5.45 | 4.11 | 11.41 | 11.88 | 11.41 | 5.16 |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2890 | 1855 | 2938 | 2976 | 3049 | 2978 | 2815 | 2815 | 2815 | 2828 |
| Irregular vertices (S) | 28 | 117 | 204 | 24 | 24 | 24 | 24 | 24 | 24 | 24 |
| … of which above the minimum | 20 | — | 196 | 16 | 16 | 16 | 16 | 16 | 16 | 16 |
| Base complex patches (C) | 619 | — | 1782 | 78 | 78 | 285 | 501 | 501 | 501 | 182 |
| Alignment, degrees (A) | 12.97 | 15.75 | 13.96 | 11.65 | 11.96 | 10.47 | 10.90 | 10.90 | 10.90 | 10.52 |
| Mean angle deviation, degrees (F) | 2.66 | 12.33 | 6.10 | 2.61 | 1.35 | 2.25 | 3.02 | 3.02 | 3.02 | 1.81 |
| Angle deviation, 95th percentile (F) | 10.11 | 37.42 | 25.03 | 10.79 | 4.82 | 6.23 | 12.04 | 12.04 | 12.04 | 6.62 |
| Mean aspect ratio (F) | 1.116 | 2.476 | 1.233 | 1.163 | 1.158 | 1.161 | 1.326 | 1.326 | 1.326 | 1.162 |
| Area variation (F) | 0.100 | 0.672 | 0.199 | 0.126 | 0.113 | 0.364 | 0.196 | 0.196 | 0.196 | 0.381 |
| Hausdorff, ‰ of the diagonal (H) | 5.54 | 17.76 | 9.41 | 0.80 | 0.92 | 0.47 | 0.89 | 0.89 | 0.89 | 0.54 |
| Mean distance, ‰ (H) | 0.114 | 1.229 | 0.067 | 0.054 | 0.052 | 0.056 | 0.068 | 0.068 | 0.068 | 0.062 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.15 | — | 100.14 | 100.00 | 100.00 | 100.00 | 100.12 | 100.12 | 100.12 | 100.00 |
| Off budget, % (N) | 3.7 | 38.2 | 2.1 | 0.8 | 1.6 | 0.7 | 6.2 | 6.2 | 0.0 | 0.5 |
| Time, seconds (T) | 2.91 | 1.62 | 0.71 | 4.01 | 2.36 | 2.17 | 2.83 | 2.88 | 2.83 | 1.86 |

## cube (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2904 | 1260 | 2974 | 2992 | 2992 | 2992 | 3082 | 3082 | 3082 | 3082 |
| Irregular vertices (S) | 8 | 8 | 24 | 8 | 8 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 0 | 16 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 6 | 36 | 274 | 1102 | 6 | 6 | 6 | 6 | 6 | 6 |
| Alignment, degrees (A) | 0.29 | 2.29 | 1.97 | 0.75 | 0.14 | 0.36 | 0.13 | 0.13 | 0.13 | 0.37 |
| Mean angle deviation, degrees (F) | 0.18 | 3.61 | 1.65 | 0.75 | 0.01 | 0.36 | 0.00 | 0.00 | 0.00 | 0.36 |
| Angle deviation, 95th percentile (F) | 0.43 | 11.71 | 4.89 | 2.01 | 0.02 | 0.85 | 0.00 | 0.00 | 0.00 | 0.93 |
| Mean aspect ratio (F) | 1.005 | 2.174 | 1.062 | 1.039 | 1.031 | 1.088 | 1.030 | 1.030 | 1.030 | 1.088 |
| Area variation (F) | 0.004 | 0.662 | 0.090 | 0.045 | 0.021 | 0.098 | 0.021 | 0.021 | 0.021 | 0.098 |
| Hausdorff, ‰ of the diagonal (H) | 4.80 | 10.77 | 0.00 | 8.84 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.017 | 0.439 | 0.000 | 0.067 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.89 | 100.00 | 99.99 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 3.2 | 58.0 | 1.0 | 0.3 | 0.3 | 0.3 | 2.7 | 2.7 | 0.0 | 0.0 |
| Time, seconds (T) | 0.77 | 0.49 | 0.20 | 5.18 | 1.46 | 1.46 | 2.97 | 3.00 | 2.97 | 1.51 |

## lblock (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2928 | 1328 | 2780 | 2800 | 3060 | 2972 | 3030 | 3030 | 3030 | 2972 |
| Irregular vertices (S) | 12 | 12 | 24 | 12 | 12 | 12 | 12 | 12 | 12 | 12 |
| … of which above the minimum | 4 | 4 | 16 | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 86 | 628 | 464 | 14 | 14 | 14 | 14 | 14 | 14 | 14 |
| Alignment, degrees (A) | 1.09 | 3.78 | 1.54 | 0.59 | 0.52 | 0.86 | 0.63 | 0.63 | 0.63 | 0.86 |
| Mean angle deviation, degrees (F) | 0.80 | 5.83 | 1.56 | 0.23 | 0.01 | 0.53 | 0.37 | 0.37 | 0.37 | 0.53 |
| Angle deviation, 95th percentile (F) | 2.44 | 18.79 | 4.54 | 0.98 | 0.05 | 1.42 | 1.43 | 1.43 | 1.43 | 1.42 |
| Mean aspect ratio (F) | 1.047 | 1.994 | 1.051 | 1.022 | 1.030 | 1.098 | 1.037 | 1.037 | 1.037 | 1.097 |
| Area variation (F) | 0.039 | 0.572 | 0.063 | 0.023 | 0.033 | 0.106 | 0.021 | 0.021 | 0.021 | 0.106 |
| Hausdorff, ‰ of the diagonal (H) | 10.33 | 11.65 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Mean distance, ‰ (H) | 0.034 | 0.454 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.83 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 |
| Off budget, % (N) | 2.4 | 55.7 | 7.3 | 6.7 | 2.0 | 0.9 | 1.0 | 1.0 | 0.0 | 1.9 |
| Time, seconds (T) | 1.39 | 0.40 | 0.36 | 23.07 | 1.23 | 1.32 | 2.73 | 2.90 | 2.73 | 1.41 |

## cylinder (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2900 | 1306 | 3100 | 2533 | 2988 | 3040 | 2458 | 2458 | 2458 | 2496 |
| Irregular vertices (S) | 8 | 8 | 91 | 8 | 8 | 8 | 8 | 8 | 8 | 8 |
| … of which above the minimum | 0 | 0 | 83 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Base complex patches (C) | 1003 | 68 | 3052 | 643 | 726 | 546 | 408 | 408 | 408 | 446 |
| Alignment, degrees (A) | 4.18 | 4.72 | 5.46 | 1.56 | 1.50 | 1.72 | 1.52 | 1.52 | 1.64 | 1.43 |
| Mean angle deviation, degrees (F) | 4.32 | 8.13 | 4.18 | 4.16 | 3.60 | 2.89 | 3.22 | 3.22 | 3.22 | 2.59 |
| Angle deviation, 95th percentile (F) | 17.78 | 26.13 | 15.60 | 17.67 | 14.03 | 8.49 | 11.87 | 11.87 | 11.87 | 8.88 |
| Mean aspect ratio (F) | 1.087 | 2.019 | 1.132 | 1.355 | 1.387 | 1.563 | 1.511 | 1.511 | 1.511 | 1.645 |
| Area variation (F) | 0.103 | 0.640 | 0.140 | 0.374 | 0.367 | 0.245 | 0.403 | 0.403 | 0.403 | 0.286 |
| Hausdorff, ‰ of the diagonal (H) | 2.07 | 9.51 | 3.74 | 1.00 | 0.93 | 0.91 | 1.43 | 1.43 | 1.43 | 1.05 |
| Mean distance, ‰ (H) | 0.144 | 0.790 | 0.121 | 0.145 | 0.117 | 0.140 | 0.369 | 0.369 | 0.369 | 0.180 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.91 | 99.21 | 99.90 | 100.00 | 100.00 | 100.00 | 99.56 | 99.56 | 99.56 | 100.00 |
| Off budget, % (N) | 3.3 | 56.5 | 3.3 | 15.6 | 0.4 | 1.3 | 18.1 | 18.1 | 0.0 | 1.5 |
| Time, seconds (T) | 1.79 | 0.99 | 0.17 | 2.92 | 1.32 | 1.50 | 2.68 | 2.49 | 2.68 | 1.57 |

## venturi (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 4046 | 2489 | 2412 | 2721 | 3049 | 14193 | 9910 | 14193 | 14282 |
| Irregular vertices (S) | — | 1527 | 432 | 169 | 206 | 409 | 613 | 613 | 613 | 840 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1464 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 11.84 | 11.84 | 11.55 | 11.37 | 13.53 | 7.08 | 6.76 | 8.23 | 10.44 |
| Mean angle deviation, degrees (F) | — | 48.87 | 13.84 | 8.20 | 8.58 | 11.41 | 9.06 | 9.35 | 9.06 | 6.75 |
| Angle deviation, 95th percentile (F) | — | 89.70 | 52.08 | 25.25 | 30.13 | 42.32 | 41.05 | 48.55 | 41.05 | 25.65 |
| Mean aspect ratio (F) | — | 12.801 | 1.928 | 1.344 | 1.373 | 1.770 | 2.237 | 2.425 | 2.237 | 1.432 |
| Area variation (F) | — | 2.373 | 0.475 | 0.291 | 0.347 | 0.478 | 1.573 | 1.464 | 1.573 | 0.334 |
| Hausdorff, ‰ of the diagonal (H) | — | 71.10 | 40.69 | 38.37 | 44.81 | 44.81 | 14.05 | 18.60 | 14.05 | 23.34 |
| Mean distance, ‰ (H) | — | 5.233 | 4.298 | 5.725 | 4.794 | 4.313 | 0.337 | 0.555 | 0.337 | 1.124 |
| Thin parts: mean distance, % of a quad edge (V) | — | 24.5 | 28.2 | 36.2 | 29.9 | 26.6 | 1.7 | 2.6 | 3.0 | 23.5 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 34.9 | 17.0 | 19.6 | 9.3 | 1.6 | 373.1 | 230.3 | 0.0 | 0.6 |
| Time, seconds (T) | — | 1.11 | 1.05 | 22.47 | 11.00 | 19.98 | 5.47 | 4.22 | 5.47 | 104.98 |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 3/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 |
| Quads made | — | 1283 | 2836 | 2515 | 2964 | 2954 | 6269 | 2409 | 6269 | 6213 |
| Irregular vertices (S) | — | 87 | 223 | 85 | 88 | 194 | 275 | 210 | 275 | 314 |
| … of which above the minimum | — | — | 219 | — | 80 | 186 | 267 | — | 267 | 306 |
| Base complex patches (C) | — | — | 2718 | — | 2764 | 2408 | 4923 | — | 4923 | 5043 |
| Alignment, degrees (A) | — | 16.86 | 18.44 | 13.66 | 13.77 | 12.42 | 9.30 | 11.90 | 9.51 | 10.68 |
| Mean angle deviation, degrees (F) | — | 10.34 | 7.84 | 9.13 | 6.21 | 6.68 | 9.35 | 10.70 | 9.35 | 6.52 |
| Angle deviation, 95th percentile (F) | — | 29.60 | 25.71 | 24.63 | 18.44 | 19.89 | 24.49 | 28.47 | 24.49 | 19.28 |
| Mean aspect ratio (F) | — | 1.786 | 1.295 | 1.502 | 1.509 | 1.407 | 2.322 | 1.600 | 2.322 | 1.371 |
| Area variation (F) | — | 0.654 | 0.236 | 0.339 | 0.370 | 0.798 | 0.763 | 0.383 | 0.763 | 0.898 |
| Hausdorff, ‰ of the diagonal (H) | — | 30.78 | 61.14 | 38.53 | 26.12 | 22.94 | 6.85 | 42.33 | 6.85 | 11.08 |
| Mean distance, ‰ (H) | — | 2.902 | 1.839 | 1.177 | 0.779 | 0.776 | 0.574 | 1.440 | 0.574 | 0.437 |
| Thin parts: mean distance, % of a quad edge (V) | — | 57.6 | 145.0 | 75.2 | 24.5 | 5.4 | 4.0 | 74.8 | 6.9 | 1.9 |
| Volume kept, % of the source (V) | — | 95.87 | 98.43 | 99.70 | 99.92 | 100.02 | 99.17 | — | 99.17 | 100.02 |
| Off budget, % (N) | — | 57.2 | 5.5 | 16.2 | 1.2 | 1.5 | 109.0 | 19.7 | 0.0 | 0.9 |
| Time, seconds (T) | — | 16.40 | 89.83 | 57.56 | 56.71 | 111.83 | 36.00 | 36.17 | 36.00 | 69.06 |

Failures: quadriflow: cancelled.

## watermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 2675 | 709 | 1784 | 2260 | 2189 | 2193 | 22110 | 2850 | 22110 | — |
| Irregular vertices (S) | 302 | 129 | 307 | 226 | 169 | 163 | 1250 | 484 | 1250 | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2588 | — | 1676 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 14.64 | 19.69 | 16.44 | 15.73 | 14.71 | 14.75 | 10.86 | 15.13 | 12.09 | — |
| Mean angle deviation, degrees (F) | 11.57 | 21.19 | 17.11 | 11.91 | 10.51 | 10.54 | 8.01 | 11.64 | 8.01 | — |
| Angle deviation, 95th percentile (F) | 39.87 | 71.32 | 57.84 | 47.77 | 38.20 | 38.51 | 24.75 | 41.56 | 24.75 | — |
| Mean aspect ratio (F) | 1.472 | 2.842 | 1.945 | 1.601 | 1.595 | 1.602 | 1.720 | 1.617 | 1.720 | — |
| Area variation (F) | 0.335 | 0.833 | 0.490 | 0.562 | 0.519 | 0.521 | 0.834 | 0.372 | 0.834 | — |
| Hausdorff, ‰ of the diagonal (H) | 86.07 | 187.44 | 74.17 | 77.11 | 81.69 | 81.69 | 33.67 | 57.63 | 33.67 | — |
| Mean distance, ‰ (H) | 5.115 | 23.533 | 7.799 | 6.793 | 6.948 | 6.950 | 1.298 | 4.420 | 1.298 | — |
| Thin parts: mean distance, % of a quad edge (V) | 31.4 | 104.7 | 43.1 | 39.8 | 40.1 | 40.1 | 12.7 | 26.0 | 35.7 | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | 99.75 | — | 99.75 | — |
| Off budget, % (N) | 10.8 | 76.4 | 40.5 | 24.7 | 27.0 | 26.9 | 637.0 | 5.0 | 0.0 | — |
| Time, seconds (T) | 3.14 | 1.85 | 0.55 | 23.13 | 134.83 | 751.56 | 413.72 | 382.00 | 413.72 | — |

Failures: openclare-matched: timed out after 1800 s.

## towermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | 2724 | 1088 | 1904 | 2395 | 2969 | 2393 | 16107 | 2897 | 16107 | 12835 |
| Irregular vertices (S) | 378 | 194 | 370 | 147 | 329 | 142 | 813 | 375 | 813 | 412 |
| … of which above the minimum | — | — | — | — | 25 | — | — | — | — | — |
| Base complex patches (C) | 2661 | — | 1899 | — | 2837 | — | — | — | — | — |
| Alignment, degrees (A) | 13.28 | 16.22 | 16.31 | 12.77 | 14.40 | 12.77 | 10.58 | 11.73 | 11.77 | 12.24 |
| Mean angle deviation, degrees (F) | 11.93 | 19.35 | 19.44 | 9.80 | 14.26 | 9.80 | 8.97 | 9.30 | 8.97 | 7.46 |
| Angle deviation, 95th percentile (F) | 38.83 | 64.57 | 61.40 | 35.38 | 46.85 | 35.38 | 27.41 | 33.36 | 27.41 | 24.75 |
| Mean aspect ratio (F) | 1.501 | 2.846 | 2.098 | 1.446 | 2.498 | 1.446 | 2.149 | 1.547 | 2.149 | 1.310 |
| Area variation (F) | 0.329 | 0.711 | 0.515 | 0.322 | 0.721 | 0.320 | 0.715 | 0.327 | 0.715 | 0.297 |
| Hausdorff, ‰ of the diagonal (H) | 56.00 | 98.83 | 77.09 | 63.94 | 65.07 | 63.94 | 27.59 | 44.98 | 27.59 | 29.36 |
| Mean distance, ‰ (H) | 3.285 | 12.802 | 6.169 | 4.799 | 3.529 | 4.800 | 1.555 | 3.480 | 1.555 | 1.173 |
| Thin parts: mean distance, % of a quad edge (V) | 16.0 | 56.9 | 34.1 | 27.6 | 21.1 | 27.6 | 11.3 | 17.6 | 25.1 | 27.6 |
| Volume kept, % of the source (V) | — | — | — | — | 93.99 | — | — | — | — | — |
| Off budget, % (N) | 9.2 | 63.7 | 36.5 | 20.2 | 1.0 | 20.2 | 436.9 | 3.4 | 0.0 | 20.3 |
| Time, seconds (T) | 2.48 | 1.50 | 0.47 | 12.81 | 12.82 | 22.37 | 106.09 | 101.48 | 106.09 | 149.70 |

## stonemill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 0/1 | 1/5 | 0/1 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 | 1/1 |
| Quads made | 2635 | 1809 | 2664 | 2538 | 3008 | 2974 | 14140 | 2859 | 14140 | 14370 |
| Irregular vertices (S) | 242 | 99 | 287 | 105 | 128 | 229 | 779 | 368 | 779 | 389 |
| … of which above the minimum | 206 | — | 247 | — | 64 | 165 | — | — | — | 325 |
| Base complex patches (C) | 2605 | — | 2586 | — | 2904 | 2872 | — | — | — | 13636 |
| Alignment, degrees (A) | 13.78 | 14.83 | 14.29 | 13.89 | 13.07 | 13.02 | 11.44 | 13.23 | 12.79 | 12.17 |
| Mean angle deviation, degrees (F) | 9.37 | 11.26 | 9.56 | 8.82 | 5.45 | 7.16 | 7.97 | 8.50 | 7.97 | 4.98 |
| Angle deviation, 95th percentile (F) | 28.53 | 33.83 | 32.90 | 24.82 | 18.55 | 24.75 | 24.83 | 29.72 | 24.83 | 15.66 |
| Mean aspect ratio (F) | 1.324 | 2.004 | 1.440 | 1.264 | 1.260 | 1.414 | 1.797 | 1.392 | 1.797 | 1.214 |
| Area variation (F) | 0.239 | 0.710 | 0.288 | 0.245 | 0.261 | 0.459 | 0.725 | 0.278 | 0.725 | 0.329 |
| Hausdorff, ‰ of the diagonal (H) | 42.66 | 106.61 | 66.01 | 59.45 | 20.37 | 20.03 | 21.42 | 39.68 | 21.42 | 11.17 |
| Mean distance, ‰ (H) | 2.229 | 4.106 | 2.742 | 2.632 | 1.906 | 1.792 | 1.214 | 2.515 | 1.214 | 0.691 |
| Thin parts: mean distance, % of a quad edge (V) | 40.9 | 155.1 | 65.7 | 77.5 | 20.5 | 16.0 | 19.0 | 45.6 | 62.7 | 19.2 |
| Volume kept, % of the source (V) | 98.96 | — | 98.49 | — | 99.86 | 99.98 | 98.92 | — | 98.92 | 100.02 |
| Off budget, % (N) | 12.2 | 39.7 | 11.2 | 15.4 | 0.3 | 0.9 | 371.3 | 4.7 | 0.0 | 1.6 |
| Time, seconds (T) | 2.36 | 1.06 | 0.53 | 9.68 | 2.58 | 3.94 | 37.80 | 33.37 | 37.80 | 14.57 |

## windmill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 1003 | 1886 | — | 207 | 349 | 17287 | 2787 | 17287 | 753 |
| Irregular vertices (S) | — | 145 | 312 | — | 28 | 81 | 905 | 382 | 905 | 201 |
| … of which above the minimum | — | — | — | — | 4 | 57 | — | — | — | 177 |
| Base complex patches (C) | — | — | 1778 | — | 199 | 349 | — | — | — | 745 |
| Alignment, degrees (A) | — | 14.39 | 15.52 | — | 19.17 | 18.79 | 9.49 | 11.63 | 11.73 | 17.04 |
| Mean angle deviation, degrees (F) | — | 20.17 | 15.16 | — | 29.40 | 26.04 | 8.30 | 10.62 | 8.30 | 19.15 |
| Angle deviation, 95th percentile (F) | — | 69.91 | 53.30 | — | 68.85 | 65.83 | 25.71 | 37.24 | 25.71 | 53.10 |
| Mean aspect ratio (F) | — | 2.697 | 1.760 | — | 8.010 | 2.972 | 1.847 | 1.540 | 1.847 | 3.134 |
| Area variation (F) | — | 0.807 | 0.437 | — | 1.419 | 1.252 | 0.787 | 0.335 | 0.787 | 1.275 |
| Hausdorff, ‰ of the diagonal (H) | — | 193.85 | 69.08 | — | 532.08 | 530.74 | 16.02 | 36.49 | 16.02 | 528.56 |
| Mean distance, ‰ (H) | — | 18.937 | 5.814 | — | 248.299 | 236.760 | 0.917 | 2.648 | 0.917 | 216.191 |
| Thin parts: mean distance, % of a quad edge (V) | — | 122.8 | 42.9 | — | 1176.4 | 1162.3 | 9.2 | 17.8 | 19.1 | 2478.2 |
| Volume kept, % of the source (V) | — | — | — | — | 1.41 | 1.39 | 99.26 | — | 99.26 | 1.49 |
| Off budget, % (N) | — | 66.6 | 37.1 | — | 93.1 | 88.4 | 476.2 | 7.1 | 0.0 | 95.6 |
| Time, seconds (T) | — | 1.35 | 0.52 | — | 1.03 | 1.65 | 171.59 | 233.11 | 171.59 | 8.76 |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 2 non-manifold or inconsistently oriented edges.

## terrain (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | 2717 | 919 | 1753 | 2502 | 2968 | 2967 | 4677 | 2597 | 4677 | 4619 |
| Irregular vertices (S) | 112 | 45 | 115 | 55 | 56 | 99 | 231 | 167 | 231 | 141 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 826 | — | 1209 | 997 | 1681 | 2021 | — | 1662 | — | 3878 |
| Alignment, degrees (A) | 20.17 | 20.27 | 21.17 | 18.74 | 18.68 | 18.58 | 18.30 | 18.61 | 18.51 | 18.17 |
| Mean angle deviation, degrees (F) | 6.49 | 6.70 | 8.24 | 9.36 | 6.25 | 6.57 | 9.57 | 8.13 | 9.57 | 7.05 |
| Angle deviation, 95th percentile (F) | 23.46 | 24.56 | 27.33 | 27.14 | 17.87 | 20.09 | 26.86 | 23.75 | 26.86 | 19.90 |
| Mean aspect ratio (F) | 1.209 | 1.423 | 1.291 | 1.338 | 1.291 | 1.344 | 1.806 | 1.429 | 1.806 | 1.347 |
| Area variation (F) | 0.187 | 0.413 | 0.215 | 0.276 | 0.313 | 0.449 | 0.552 | 0.247 | 0.552 | 0.492 |
| Hausdorff, ‰ of the diagonal (H) | 23.91 | 34.95 | 143.82 | 24.92 | 20.22 | 18.45 | 12.55 | 25.16 | 12.55 | 13.13 |
| Mean distance, ‰ (H) | 1.851 | 3.051 | 2.384 | 1.715 | 1.360 | 1.156 | 0.967 | 1.780 | 0.967 | 0.892 |
| Thin parts: mean distance, % of a quad edge (V) | 32.8 | 56.8 | 36.9 | 35.4 | 26.3 | 20.3 | 14.4 | 32.3 | 18.9 | 19.3 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | 9.4 | 69.4 | 41.6 | 16.6 | 1.1 | 1.1 | 55.9 | 13.4 | 0.0 | 1.2 |
| Time, seconds (T) | 22.93 | 2.34 | 8.92 | 14.15 | 7.44 | 8.07 | 21.17 | 19.56 | 21.17 | 9.85 |

## mahaveer (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 1325 | 1699 | 2080 | 5131 | 5437 | 46202 | 4324 | 46202 | 55041 |
| Irregular vertices (S) | — | 422 | 175 | 64 | 485 | 487 | 3076 | 1079 | 3076 | 1186 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1227 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 9.34 | 9.97 | 7.13 | 14.65 | 14.76 | 13.54 | 11.62 | 13.50 | 6.13 |
| Mean angle deviation, degrees (F) | — | 32.26 | 9.66 | 7.58 | 23.32 | 19.55 | 12.36 | 12.56 | 12.36 | 5.93 |
| Angle deviation, 95th percentile (F) | — | 85.28 | 34.02 | 26.60 | 69.73 | 67.55 | 39.42 | 48.70 | 39.42 | 22.41 |
| Mean aspect ratio (F) | — | 5.235 | 1.583 | 1.656 | 44.461 | 64.963 | 1.824 | 1.786 | 1.824 | 7.544 |
| Area variation (F) | — | 1.443 | 0.344 | 0.570 | 2.238 | 2.329 | 2.664 | 0.975 | 2.664 | 0.732 |
| Hausdorff, ‰ of the diagonal (H) | — | 44.66 | 52.26 | 26.46 | 48.05 | 48.05 | 13.60 | 23.83 | 13.60 | 13.19 |
| Mean distance, ‰ (H) | — | 5.890 | 7.890 | 3.785 | 3.135 | 3.140 | 0.929 | 2.320 | 0.929 | 0.573 |
| Thin parts: mean distance, % of a quad edge (V) | — | 46.8 | 75.1 | 35.8 | 28.1 | 28.1 | 11.6 | 21.9 | 49.3 | 35.6 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 55.8 | 43.4 | 30.7 | 71.0 | 81.2 | 1440.1 | 44.1 | 0.0 | 19.1 |
| Time, seconds (T) | — | 1.33 | 7.16 | 18.77 | 7.80 | 6.84 | 14.10 | 14.84 | 14.10 | 69.13 |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 870 | 1225 | — | 11505 | 13732 | 214018 | 11340 | 214018 | 226774 |
| Irregular vertices (S) | — | 858 | 616 | — | 6111 | 6059 | 10315 | 4939 | 10315 | 11588 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1136 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 18.06 | 17.78 | — | 11.55 | 11.25 | 10.18 | 13.63 | 12.25 | 8.37 |
| Mean angle deviation, degrees (F) | — | 52.53 | 23.80 | — | 17.36 | 14.27 | 9.86 | 17.03 | 9.86 | 4.90 |
| Angle deviation, 95th percentile (F) | — | 89.72 | 60.14 | — | 65.07 | 59.74 | 29.30 | 51.38 | 29.30 | 18.22 |
| Mean aspect ratio (F) | — | 14.155 | 2.545 | — | 3.829 | 2.657 | 1.580 | 1.889 | 1.580 | 1.323 |
| Area variation (F) | — | 7.748 | 0.789 | — | 3.418 | 3.876 | 2.386 | 2.005 | 2.386 | 0.526 |
| Hausdorff, ‰ of the diagonal (H) | — | 9.99 | 7.35 | — | 7.06 | 6.60 | 1.80 | 3.03 | 1.80 | 3.00 |
| Mean distance, ‰ (H) | — | 2.873 | 1.806 | — | 0.423 | 0.432 | 0.123 | 0.419 | 0.123 | 0.033 |
| Thin parts: mean distance, % of a quad edge (V) | — | 87.1 | 57.0 | — | 12.3 | 12.9 | 7.6 | 14.6 | 59.1 | 17.0 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 71.0 | 59.2 | — | 283.5 | 357.7 | 7033.9 | 278.0 | 0.0 | 6.0 |
| Time, seconds (T) | — | 1.33 | 4.00 | — | 11.92 | 16.57 | 12.21 | 10.92 | 12.21 | 45.21 |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 8 non-manifold or inconsistently oriented edges.

## f16 (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 2/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 4122 | 1500 | 1333 | 7642 | 7177 | 36038 | 6315 | 36038 | 43487 |
| Irregular vertices (S) | — | 2217 | 403 | 242 | 1828 | 2154 | 3062 | 1343 | 3062 | 2377 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1329 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 14.85 | 12.61 | 8.71 | 4.53 | 4.76 | 5.01 | 7.09 | 4.91 | 4.15 |
| Mean angle deviation, degrees (F) | — | 41.32 | 19.76 | 15.35 | 12.92 | 11.10 | 10.99 | 14.23 | 10.99 | 6.09 |
| Angle deviation, 95th percentile (F) | — | 87.07 | 64.46 | 49.46 | 63.24 | 56.42 | 41.48 | 46.16 | 41.48 | 22.92 |
| Mean aspect ratio (F) | — | 7.957 | 2.173 | 1.844 | 57.869 | 26.730 | 2.408 | 3.370 | 2.408 | 6.995 |
| Area variation (F) | — | 2.547 | 0.539 | 0.650 | 1.854 | 1.782 | 1.937 | 1.320 | 1.937 | 0.760 |
| Hausdorff, ‰ of the diagonal (H) | — | 87.83 | 95.94 | 63.38 | 17.14 | 23.90 | 8.24 | 47.42 | 8.24 | 9.62 |
| Mean distance, ‰ (H) | — | 9.067 | 20.293 | 13.479 | 0.729 | 0.977 | 0.363 | 2.336 | 0.363 | 0.213 |
| Thin parts: mean distance, % of a quad edge (V) | — | 62.6 | 143.8 | 98.2 | 5.4 | 6.9 | 3.4 | 18.2 | 11.6 | 6.9 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 37.4 | 50.0 | 55.6 | 154.7 | 139.2 | 1101.3 | 110.5 | 0.0 | 20.7 |
| Time, seconds (T) | — | 2.61 | 5.15 | 88.05 | 9.78 | 23.28 | 9.36 | 9.51 | 9.36 | 140.81 |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 2222 | 2216 | 1109 | 3536 | 3409 | 12621 | 5458 | 12621 | 12776 |
| Irregular vertices (S) | — | 338 | 265 | 70 | 191 | 221 | 443 | 322 | 443 | 393 |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1675 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 7.28 | 8.44 | 8.56 | 5.00 | 4.17 | 4.33 | 3.81 | 3.56 | 3.31 |
| Mean angle deviation, degrees (F) | — | 22.14 | 11.96 | 12.59 | 16.32 | 12.93 | 7.26 | 8.23 | 7.26 | 8.45 |
| Angle deviation, 95th percentile (F) | — | 84.37 | 45.38 | 46.79 | 71.51 | 59.61 | 26.15 | 33.02 | 26.15 | 44.29 |
| Mean aspect ratio (F) | — | 4.205 | 1.771 | 2.901 | 15.727 | 37.192 | 8.875 | 18.163 | 8.875 | 24.361 |
| Area variation (F) | — | 2.439 | 0.430 | 0.902 | 1.171 | 1.057 | 1.511 | 0.973 | 1.511 | 0.690 |
| Hausdorff, ‰ of the diagonal (H) | — | 50.21 | 35.63 | 45.56 | 27.82 | 27.78 | 13.66 | 28.78 | 13.66 | 19.89 |
| Mean distance, ‰ (H) | — | 5.701 | 4.253 | 5.359 | 2.217 | 1.941 | 0.560 | 1.134 | 0.560 | 0.732 |
| Thin parts: mean distance, % of a quad edge (V) | — | 25.0 | 19.2 | 22.4 | 10.4 | 8.8 | 2.4 | 5.0 | 4.6 | 13.0 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 25.9 | 26.1 | 63.0 | 17.9 | 13.6 | 320.7 | 81.9 | 0.0 | 1.2 |
| Time, seconds (T) | — | 0.62 | 1.12 | 70.22 | 4.13 | 10.40 | 5.03 | 4.34 | 5.03 | 21.21 |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 |
| Quads made | — | 625 | 2774 | 2420 | 2419 | 2466 | 14984 | 2308 | 14984 | 13011 |
| Irregular vertices (S) | — | 125 | 430 | 185 | 230 | 225 | 775 | 331 | 775 | 612 |
| … of which above the minimum | — | — | 387 | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 2517 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 14.57 | 14.83 | 10.13 | 10.60 | 11.08 | 9.19 | 9.89 | 10.20 | 10.66 |
| Mean angle deviation, degrees (F) | — | 16.08 | 11.44 | 9.70 | 10.49 | 10.98 | 9.40 | 11.17 | 9.40 | 9.79 |
| Angle deviation, 95th percentile (F) | — | 44.86 | 37.21 | 27.88 | 30.89 | 33.10 | 27.92 | 33.61 | 27.92 | 28.34 |
| Mean aspect ratio (F) | — | 2.516 | 1.550 | 1.474 | 1.694 | 1.692 | 1.735 | 1.574 | 1.735 | 1.420 |
| Area variation (F) | — | 0.933 | 0.323 | 0.335 | 0.350 | 0.371 | 0.938 | 0.374 | 0.938 | 0.330 |
| Hausdorff, ‰ of the diagonal (H) | — | 89.72 | 44.20 | 33.89 | 37.13 | 37.13 | 21.30 | 37.66 | 21.30 | 10.23 |
| Mean distance, ‰ (H) | — | 9.165 | 3.465 | 2.989 | 2.505 | 2.414 | 0.702 | 3.072 | 0.702 | 0.381 |
| Thin parts: mean distance, % of a quad edge (V) | — | 125.4 | 88.0 | 69.3 | 56.5 | 54.2 | 17.5 | 65.3 | 44.1 | 36.1 |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 79.2 | 7.5 | 19.3 | 19.4 | 17.8 | 399.5 | 23.1 | 0.0 | 13.2 |
| Time, seconds (T) | — | 1.05 | 0.34 | 22.46 | 23.86 | 30.74 | 26.98 | 30.23 | 26.98 | 555.29 |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 3107 | 2086 | 3090 | 3000 | 3000 | 3000 | 2740 | 2740 | 2740 | 2706 |
| Irregular vertices (S) | 8 | 10 | 97 | 32 | 32 | 32 | 10 | 10 | 10 | 32 |
| … of which above the minimum | 0 | 2 | 89 | 24 | 24 | 24 | 2 | 2 | 2 | 24 |
| Base complex patches (C) | 2730 | 2004 | 3090 | 30 | 30 | 30 | 1794 | 1794 | 1794 | 30 |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.49 | 7.65 | 4.19 | 8.38 | 3.97 | 3.97 | 6.09 | 6.09 | 6.09 | 3.87 |
| Angle deviation, 95th percentile (F) | 12.67 | 19.88 | 14.49 | 17.72 | 7.49 | 7.49 | 15.49 | 15.49 | 15.49 | 7.86 |
| Mean aspect ratio (F) | 1.125 | 1.326 | 1.124 | 1.081 | 1.083 | 1.083 | 1.185 | 1.185 | 1.185 | 1.118 |
| Area variation (F) | 0.135 | 0.341 | 0.136 | 0.190 | 0.302 | 0.302 | 0.312 | 0.312 | 0.312 | 0.309 |
| Hausdorff, ‰ of the diagonal (H) | 0.63 | 1.54 | 0.72 | 0.42 | 0.49 | 0.49 | 0.82 | 0.82 | 0.82 | 0.63 |
| Mean distance, ‰ (H) | 0.216 | 0.408 | 0.221 | 0.168 | 0.161 | 0.161 | 0.258 | 0.258 | 0.258 | 0.178 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.81 | 99.59 | 99.79 | 100.01 | 100.01 | 100.01 | 99.74 | 99.74 | 99.74 | 100.01 |
| Off budget, % (N) | 3.6 | 30.5 | 3.0 | 0.0 | 0.0 | 0.0 | 8.7 | 8.7 | 0.0 | 1.2 |
| Time, seconds (T) | 1.80 | 0.63 | 0.34 | 1.94 | 1.66 | 1.36 | 3.56 | 3.39 | 3.56 | 1.66 |

## torus (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 |
| Quads made | 2976 | 2672 | 3078 | 2720 | 2952 | 2952 | 2824 | 2824 | 2824 | 2800 |
| Irregular vertices (S) | 30 | 0 | 125 | 0 | 0 | 0 | 10 | 10 | 10 | 0 |
| … of which above the minimum | 30 | 0 | 125 | 0 | 0 | 0 | 10 | 10 | 10 | 0 |
| Base complex patches (C) | 2976 | 1 | 3078 | 1 | 1 | 1 | 2264 | 2264 | 2264 | 1 |
| Alignment, degrees (A) | 10.14 | 10.37 | 14.28 | 0.65 | 0.62 | 0.62 | 6.90 | 6.90 | 6.90 | 0.62 |
| Mean angle deviation, degrees (F) | 10.25 | 4.17 | 5.57 | 1.45 | 1.44 | 1.44 | 5.23 | 5.23 | 5.23 | 1.48 |
| Angle deviation, 95th percentile (F) | 26.29 | 11.07 | 18.30 | 2.89 | 2.36 | 2.36 | 14.40 | 14.40 | 14.40 | 2.40 |
| Mean aspect ratio (F) | 1.239 | 2.545 | 1.163 | 1.351 | 1.322 | 1.322 | 1.213 | 1.213 | 1.213 | 1.320 |
| Area variation (F) | 0.223 | 0.256 | 0.163 | 0.281 | 0.308 | 0.308 | 0.233 | 0.233 | 0.233 | 0.310 |
| Hausdorff, ‰ of the diagonal (H) | 1.86 | 1.50 | 1.38 | 0.59 | 0.59 | 0.59 | 1.71 | 1.71 | 1.71 | 0.62 |
| Mean distance, ‰ (H) | 0.299 | 0.465 | 0.298 | 0.208 | 0.191 | 0.191 | 0.518 | 0.518 | 0.518 | 0.202 |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.46 | 99.09 | 99.44 | 100.02 | 100.02 | 100.02 | 98.94 | 98.94 | 98.94 | 100.02 |
| Off budget, % (N) | 0.8 | 10.9 | 2.6 | 9.3 | 1.6 | 1.6 | 5.9 | 5.9 | 0.0 | 0.8 |
| Time, seconds (T) | 1.55 | 0.95 | 0.22 | 2.01 | 2.39 | 1.95 | 4.71 | 2.89 | 4.71 | 1.86 |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (13 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 5 | 0 | 4 | 3 | 2 | 0 | 1 | 0 | 1 |
| Base complex patches (C) | 1 | 3 | 0 | 3 | 1 | 3 | 0 | 2 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 0 | 1 | 11 | 0 | 1 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 1 | 0 | 2 | 5 | 0 | 1 | 0 | 4 |
| Mean aspect ratio (F) | 0 | 0 | 9 | 0 | 0 | 1 | 0 | 0 | 0 | 3 |
| Valid meshes (V) | 36/65 | 4/13 | 47/65 | 10/13 | 12/13 | 12/13 | 8/13 | 10/13 | 8/13 | 12/13 |

### mechanical (4 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched |
|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 3 | 0 | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| Base complex patches (C) | 1 | 1 | 0 | 2 | 3 | 2 | 2 | 2 | 2 | 2 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 1 | 1 | 1 | 1 | 1 | 1 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 0 | 2 | 0 | 1 | 1 | 1 | 1 |
| Mean aspect ratio (F) | 3 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| Valid meshes (V) | 18/20 | 3/4 | 20/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 |
