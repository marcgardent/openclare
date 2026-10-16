# Bench results

Requested budget: 3000 quads. Median over 5 seeds; Quad Remesher, AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

The last two columns compare at equal quad count: `quadremesher-as-made` is the mesh of `quadremesher` measured against the number of quads it has rather than the budget, and `openclare-matched` is `openclare` asked for that same number.

## blob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2781 | 1165 | 3090 | 2507 | 2980 | 3019 | 3370 | 2755 | 3370 | 3368 | — | — |
| Irregular vertices (S) | 38 | 26 | 143 | 26 | 24 | 24 | 26 | 24 | 26 | 24 | — | — |
| … of which above the minimum | 30 | — | 135 | 18 | 16 | 16 | 18 | 16 | 18 | 16 | — | — |
| Base complex patches (C) | 2697 | — | 3048 | 951 | 1322 | 765 | 1826 | 1350 | 1826 | 980 | — | — |
| Alignment, degrees (A) | 17.48 | 6.27 | 15.74 | 6.23 | 5.72 | 4.79 | 3.93 | 4.55 | 3.94 | 5.16 | — | — |
| Mean angle deviation, degrees (F) | 8.71 | 9.45 | 5.87 | 7.89 | 7.03 | 5.86 | 5.76 | 5.45 | 5.76 | 6.12 | — | — |
| Angle deviation, 95th percentile (F) | 27.30 | 30.58 | 18.26 | 21.16 | 18.76 | 16.28 | 17.21 | 15.61 | 17.21 | 16.14 | — | — |
| Mean aspect ratio (F) | 1.187 | 1.934 | 1.174 | 1.427 | 1.399 | 1.391 | 1.618 | 1.276 | 1.618 | 1.350 | — | — |
| Area variation (F) | 0.205 | 0.575 | 0.175 | 0.351 | 0.412 | 0.426 | 0.442 | 0.313 | 0.442 | 0.414 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 8.52 | 6.35 | 10.14 | 5.25 | 3.02 | 2.39 | 4.21 | 7.28 | 4.21 | 2.49 | — | — |
| Mean distance, ‰ (H) | 0.486 | 1.885 | 0.459 | 0.272 | 0.200 | 0.178 | 0.407 | 0.513 | 0.407 | 0.164 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.56 | 96.90 | 99.63 | 100.03 | 100.02 | 100.02 | 99.40 | 99.52 | 99.40 | 100.02 | — | — |
| Edges on closed loops, % (L) | 0.0 | 17.2 | — | — | 28.3 | 40.1 | 22.9 | — | — | 36.8 | — | — |
| Edges on spiralling loops, % (L) | 14.7 | 31.9 | — | — | 17.5 | 25.0 | 34.1 | — | — | 30.1 | — | — |
| Spiralling loops (L) | 3 | 7 | — | — | 3 | 2 | 7 | — | — | 6 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.3 | 50.9 | — | — | 54.2 | 34.9 | 43.0 | — | — | 33.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 321 | 120 | — | — | 421 | 243 | 274 | — | — | 408 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 7.3 | 61.2 | 3.0 | 16.4 | 0.7 | 0.6 | 12.3 | 8.2 | 0.0 | 0.1 | — | — |
| Time, seconds (T) | 2.17 | 0.74 | 1.71 | 4.09 | 2.50 | 3.50 | 16.38 | 12.28 | 16.38 | 3.78 | — | — |

## rock (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3036 | 2412 | 2898 | 2687 | 3007 | 2984 | 3539 | 2531 | 3539 | 3589 | — | — |
| Irregular vertices (S) | 8 | 20 | 91 | 50 | 50 | 66 | 114 | 101 | 114 | 76 | — | — |
| … of which above the minimum | 0 | 12 | 83 | 42 | 42 | 58 | 106 | 93 | 106 | 68 | — | — |
| Base complex patches (C) | 2855 | 2412 | 2896 | 2687 | 2923 | 2984 | 3539 | 2531 | 3539 | 3589 | — | — |
| Alignment, degrees (A) | 22.96 | 18.78 | 23.21 | 15.81 | 16.48 | 15.76 | 14.17 | 14.68 | 14.63 | 15.63 | — | — |
| Mean angle deviation, degrees (F) | 5.84 | 7.48 | 5.18 | 8.82 | 5.75 | 5.07 | 10.07 | 10.61 | 10.07 | 5.49 | — | — |
| Angle deviation, 95th percentile (F) | 16.05 | 17.91 | 14.78 | 22.11 | 15.28 | 13.33 | 24.88 | 26.21 | 24.88 | 14.78 | — | — |
| Mean aspect ratio (F) | 1.164 | 1.409 | 1.146 | 1.253 | 1.234 | 1.213 | 1.470 | 1.393 | 1.470 | 1.223 | — | — |
| Area variation (F) | 0.202 | 0.331 | 0.151 | 0.229 | 0.293 | 0.290 | 0.308 | 0.269 | 0.308 | 0.269 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.45 | 7.64 | 5.28 | 4.49 | 4.99 | 4.65 | 5.13 | 5.79 | 5.13 | 4.56 | — | — |
| Mean distance, ‰ (H) | 0.788 | 1.157 | 0.820 | 0.740 | 0.697 | 0.695 | 0.731 | 0.889 | 0.731 | 0.626 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.78 | 99.49 | 99.70 | 100.01 | 100.00 | 100.00 | 99.73 | 99.62 | 99.73 | 100.00 | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 1.4 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 56.9 | 27.1 | — | — | 10.3 | 2.5 | 10.3 | — | — | 7.2 | — | — |
| Spiralling loops (L) | 5 | 4 | — | — | 3 | 1 | 6 | — | — | 4 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 43.1 | 72.9 | — | — | 88.3 | 97.5 | 89.7 | — | — | 92.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 528 | 317 | — | — | 227 | 222 | 145 | — | — | 340 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 2.2 | 19.6 | 3.4 | 10.4 | 0.2 | 0.5 | 18.0 | 15.6 | 0.0 | 1.4 | — | — |
| Time, seconds (T) | 1.22 | 1.86 | 1.03 | 6.92 | 2.27 | 2.51 | 20.49 | 16.98 | 20.49 | 2.60 | — | — |

## creature (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2667 | 1991 | 3070 | 2597 | 3041 | 2975 | 2941 | 2719 | 2941 | 2961 | — | — |
| Irregular vertices (S) | 24 | 20 | 111 | 41 | 40 | 40 | 42 | 46 | 42 | 40 | — | — |
| … of which above the minimum | 16 | 12 | 103 | 33 | 32 | 32 | 34 | 38 | 34 | 32 | — | — |
| Base complex patches (C) | 2667 | 747 | 2856 | 913 | 1301 | 1049 | 1465 | 1301 | 1465 | 935 | — | — |
| Alignment, degrees (A) | 12.89 | 9.58 | 12.09 | 6.96 | 6.95 | 5.68 | 5.18 | 5.32 | 5.12 | 5.61 | — | — |
| Mean angle deviation, degrees (F) | 6.85 | 4.75 | 4.65 | 8.21 | 5.72 | 5.17 | 6.12 | 6.53 | 6.12 | 5.30 | — | — |
| Angle deviation, 95th percentile (F) | 18.98 | 13.44 | 14.73 | 24.05 | 15.03 | 14.24 | 17.13 | 18.98 | 17.13 | 15.16 | — | — |
| Mean aspect ratio (F) | 1.189 | 1.839 | 1.144 | 1.305 | 1.280 | 1.304 | 1.332 | 1.334 | 1.332 | 1.316 | — | — |
| Area variation (F) | 0.237 | 0.549 | 0.146 | 0.286 | 0.338 | 0.498 | 0.348 | 0.313 | 0.348 | 0.487 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 8.01 | 2.39 | 3.47 | 2.16 | 2.06 | 2.00 | 2.51 | 2.37 | 2.51 | 2.01 | — | — |
| Mean distance, ‰ (H) | 0.320 | 0.537 | 0.279 | 0.231 | 0.191 | 0.199 | 0.297 | 0.348 | 0.297 | 0.195 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.41 | 98.97 | 99.47 | 100.00 | 100.02 | 100.03 | 99.39 | 99.28 | 99.39 | 100.02 | — | — |
| Edges on closed loops, % (L) | 0.0 | 31.2 | — | — | 28.6 | 33.2 | 25.1 | — | — | 34.9 | — | — |
| Edges on spiralling loops, % (L) | 30.9 | 33.6 | — | — | 12.2 | 17.3 | 18.9 | — | — | 12.6 | — | — |
| Spiralling loops (L) | 4 | 6 | — | — | 6 | 6 | 8 | — | — | 3 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 69.1 | 35.2 | — | — | 59.2 | 49.5 | 56.0 | — | — | 52.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 377 | 166 | — | — | 396 | 308 | 331 | — | — | 229 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 11.1 | 33.6 | 2.3 | 13.4 | 1.4 | 0.8 | 2.0 | 9.4 | 0.0 | 0.7 | — | — |
| Time, seconds (T) | 1.35 | 1.49 | 0.93 | 6.08 | 2.67 | 2.74 | 11.24 | 9.11 | 11.24 | 2.74 | — | — |

## cage (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2891 | 4039 | 2944 | 2874 | 2996 | 2968 | 5809 | 2785 | 5809 | 5735 | — | — |
| Irregular vertices (S) | 66 | 97 | 200 | 41 | 48 | 48 | 44 | 43 | 44 | 52 | — | — |
| … of which above the minimum | 34 | — | 168 | 9 | 16 | 16 | 12 | 11 | 12 | 20 | — | — |
| Base complex patches (C) | 1943 | — | 2319 | 427 | 313 | 310 | 1092 | 1001 | 1092 | 186 | — | — |
| Alignment, degrees (A) | 8.10 | 8.92 | 7.97 | 3.38 | 3.13 | 2.35 | 2.04 | 2.57 | 2.08 | 2.10 | — | — |
| Mean angle deviation, degrees (F) | 5.86 | 9.02 | 6.39 | 4.00 | 2.82 | 3.37 | 3.91 | 4.30 | 3.91 | 2.48 | — | — |
| Angle deviation, 95th percentile (F) | 17.11 | 28.25 | 23.33 | 12.37 | 8.84 | 10.51 | 11.90 | 12.82 | 11.90 | 7.10 | — | — |
| Mean aspect ratio (F) | 1.183 | 2.030 | 1.252 | 1.202 | 1.189 | 1.239 | 1.993 | 1.318 | 1.993 | 1.191 | — | — |
| Area variation (F) | 0.147 | 0.496 | 0.203 | 0.193 | 0.208 | 0.329 | 0.314 | 0.238 | 0.314 | 0.357 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 11.45 | 13.32 | 14.46 | 11.74 | 10.74 | 10.56 | 5.74 | 9.66 | 5.74 | 3.11 | — | — |
| Mean distance, ‰ (H) | 0.734 | 0.800 | 0.860 | 0.814 | 0.834 | 0.495 | 0.693 | 1.276 | 0.693 | 0.264 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.99 | 99.63 | 98.70 | 99.89 | 99.93 | 99.98 | 99.00 | 97.85 | 99.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 18.2 | 4.2 | — | — | 67.5 | 61.5 | 55.0 | — | — | 82.3 | — | — |
| Edges on spiralling loops, % (L) | 38.5 | 29.9 | — | — | 9.6 | 18.0 | 28.2 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 20 | 24 | — | — | 3 | 5 | 10 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 41.8 | 65.8 | — | — | 22.8 | 20.5 | 16.8 | — | — | 17.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 86 | 110 | — | — | 77 | 69 | 92 | — | — | 61 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.6 | 34.6 | 1.9 | 4.2 | 0.1 | 1.1 | 93.6 | 7.2 | 0.0 | 1.3 | — | — |
| Time, seconds (T) | 1.26 | 2.38 | 0.34 | 8.42 | 2.15 | 2.46 | 4.21 | 3.92 | 4.21 | 5.68 | — | — |

## suzanne (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 2/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3030 | 1255 | 3070 | 2757 | 3025 | 2981 | 4310 | 2628 | 4310 | 4311 | — | — |
| Irregular vertices (S) | 78 | 69 | 212 | 52 | 52 | 56 | 80 | 71 | 80 | 56 | — | — |
| … of which above the minimum | 54 | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2884 | — | 3002 | 1858 | 1803 | 1459 | 2513 | 1838 | 2513 | 1867 | — | — |
| Alignment, degrees (A) | 15.33 | 13.19 | 17.02 | 10.46 | 10.39 | 9.94 | 8.75 | 10.24 | 8.92 | 9.87 | — | — |
| Mean angle deviation, degrees (F) | 8.61 | 11.13 | 7.08 | 7.14 | 4.76 | 4.52 | 7.87 | 7.87 | 7.87 | 4.81 | — | — |
| Angle deviation, 95th percentile (F) | 26.08 | 41.63 | 25.30 | 18.16 | 12.32 | 12.49 | 21.85 | 20.46 | 21.85 | 11.70 | — | — |
| Mean aspect ratio (F) | 1.260 | 1.871 | 1.263 | 1.221 | 1.220 | 1.227 | 1.668 | 1.284 | 1.668 | 1.212 | — | — |
| Area variation (F) | 0.837 | 0.611 | 0.221 | 0.250 | 0.288 | 0.383 | 0.473 | 0.236 | 0.473 | 0.384 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 42.26 | 33.04 | 20.15 | 8.61 | 8.11 | 6.44 | 6.77 | 9.47 | 6.77 | 6.06 | — | — |
| Mean distance, ‰ (H) | 0.834 | 2.167 | 0.906 | 0.738 | 0.547 | 0.477 | 0.603 | 1.053 | 0.603 | 0.345 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 16.4 | 28.9 | 21.4 | 16.0 | 8.1 | 7.3 | 5.1 | 17.1 | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.9 | 8.6 | — | — | 13.0 | 18.7 | 15.9 | — | — | 20.1 | — | — |
| Edges on spiralling loops, % (L) | 15.3 | 14.6 | — | — | 10.7 | 16.0 | 11.8 | — | — | 9.5 | — | — |
| Spiralling loops (L) | 12 | 8 | — | — | 4 | 6 | 7 | — | — | 4 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 80.2 | 76.8 | — | — | 66.2 | 54.7 | 64.9 | — | — | 57.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 233 | 82 | — | — | 138 | 119 | 207 | — | — | 211 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 10.1 | 10.6 | 7.4 | — | — | 13.3 | — | — |
| Off budget, % (N) | 1.0 | 58.2 | 2.3 | 8.1 | 0.8 | 0.6 | 43.7 | 12.4 | 0.0 | 0.0 | — | — |
| Time, seconds (T) | 3.31 | 0.83 | 0.28 | 8.24 | 1.96 | 2.67 | 3.44 | 3.21 | 3.44 | 3.58 | — | — |

## ogre (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 0/1 | 4/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2935 | 1081 | 2874 | 2465 | 3019 | 3021 | 6134 | 2417 | 6134 | 6178 | — | — |
| Irregular vertices (S) | 106 | 84 | 216 | 77 | 78 | 155 | 314 | 228 | 314 | 212 | — | — |
| … of which above the minimum | 98 | — | 202 | — | 70 | 147 | 306 | 220 | 306 | 204 | — | — |
| Base complex patches (C) | 2857 | — | 2802 | — | 2197 | 2345 | 5170 | 2009 | 5170 | 4340 | — | — |
| Alignment, degrees (A) | 19.31 | 16.24 | 18.57 | 12.24 | 12.33 | 11.60 | 8.42 | 10.81 | 9.14 | 10.80 | — | — |
| Mean angle deviation, degrees (F) | 12.01 | 10.37 | 7.49 | 9.03 | 6.71 | 6.69 | 9.10 | 11.45 | 9.10 | 6.32 | — | — |
| Angle deviation, 95th percentile (F) | 32.96 | 32.56 | 24.38 | 23.88 | 18.05 | 18.01 | 25.01 | 30.48 | 25.01 | 17.00 | — | — |
| Mean aspect ratio (F) | 1.403 | 1.826 | 1.283 | 1.546 | 1.530 | 1.426 | 2.364 | 1.612 | 2.364 | 1.378 | — | — |
| Area variation (F) | 0.324 | 0.590 | 0.224 | 0.389 | 0.385 | 0.528 | 0.716 | 0.393 | 0.716 | 0.453 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 36.59 | 31.05 | 46.14 | 35.86 | 26.72 | 28.19 | 5.98 | 39.35 | 5.98 | 19.53 | — | — |
| Mean distance, ‰ (H) | 1.268 | 3.042 | 1.460 | 1.109 | 0.799 | 0.694 | 0.565 | 1.366 | 0.565 | 0.368 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 54.0 | 66.6 | 99.5 | 66.0 | 28.6 | 6.6 | 6.7 | 75.2 | 9.8 | 6.3 | — | — |
| Volume kept, % of the source (V) | 98.56 | — | 98.55 | 99.78 | 99.93 | 100.00 | 99.24 | 98.42 | 99.24 | 100.00 | — | — |
| Edges on closed loops, % (L) | 1.7 | — | — | — | 13.6 | 11.2 | 7.9 | — | — | 14.9 | — | — |
| Edges on spiralling loops, % (L) | 9.6 | — | — | — | 17.4 | 15.3 | 15.1 | — | — | 7.6 | — | — |
| Spiralling loops (L) | 14 | — | — | — | 10 | 25 | 20 | — | — | 16 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 88.6 | — | — | — | 69.0 | 73.5 | 77.1 | — | — | 77.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 250 | — | — | — | 192 | 131 | 199 | — | — | 216 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 4.3 | 64.0 | 4.2 | 17.8 | 0.6 | 0.7 | 104.5 | 19.4 | 0.0 | 0.7 | — | — |
| Time, seconds (T) | 3.31 | 1.22 | 2.70 | 8.13 | 4.17 | 6.12 | 19.04 | 17.16 | 19.04 | 9.61 | — | — |

## human_male (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 854 | 2814 | 2710 | 3054 | 2997 | 5322 | 2598 | 5322 | 5271 | 10590 | 10709 |
| Irregular vertices (S) | — | 88 | 194 | 68 | 98 | 159 | 248 | 110 | 248 | 191 | 476 | 257 |
| … of which above the minimum | — | — | 178 | — | 90 | 151 | — | — | — | 183 | — | 249 |
| Base complex patches (C) | — | — | 2014 | — | 1474 | 1137 | — | — | — | 1767 | — | 6523 |
| Alignment, degrees (A) | — | 9.53 | 10.54 | 7.00 | 8.25 | 7.67 | 6.04 | 6.18 | 6.80 | 7.24 | 9.88 | 8.32 |
| Mean angle deviation, degrees (F) | — | 10.46 | 7.19 | 6.01 | 5.66 | 5.74 | 6.62 | 6.87 | 6.62 | 4.49 | 9.24 | 4.33 |
| Angle deviation, 95th percentile (F) | — | 37.89 | 25.75 | 21.11 | 20.14 | 19.42 | 22.36 | 21.99 | 22.36 | 14.48 | 26.27 | 13.76 |
| Mean aspect ratio (F) | — | 2.541 | 1.301 | 1.355 | 1.578 | 1.716 | 1.879 | 1.436 | 1.879 | 1.307 | 1.616 | 1.312 |
| Area variation (F) | — | 0.757 | 0.248 | 0.337 | 0.410 | 0.643 | 0.568 | 0.327 | 0.568 | 0.538 | 1.417 | 0.461 |
| Hausdorff, ‰ of the diagonal (H) | — | 31.67 | 47.23 | 38.14 | 15.86 | 9.71 | 5.29 | 48.08 | 5.29 | 12.54 | 6.47 | 4.93 |
| Mean distance, ‰ (H) | — | 5.764 | 4.380 | 2.749 | 1.410 | 0.879 | 0.488 | 3.106 | 0.488 | 0.624 | 0.172 | 0.193 |
| Thin parts: mean distance, % of a quad edge (V) | — | 79.0 | 84.2 | 57.3 | 21.8 | 12.2 | 5.3 | 64.6 | 7.3 | 9.8 | 1.5 | 6.9 |
| Volume kept, % of the source (V) | — | — | 97.16 | 99.70 | 99.90 | 99.97 | 98.88 | — | 98.88 | 100.01 | 98.80 | 100.01 |
| Edges on closed loops, % (L) | — | — | 14.8 | 35.4 | 26.0 | 31.1 | 28.7 | — | 28.7 | 35.0 | 31.6 | 19.5 |
| Edges on spiralling loops, % (L) | — | — | 7.8 | 4.4 | 16.4 | 4.2 | 5.9 | — | 5.9 | 2.7 | 2.5 | 18.1 |
| Spiralling loops (L) | — | — | 8 | 8 | 14 | 12 | 7 | — | 7 | 14 | 3 | 18 |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 76.8 | 60.3 | 57.6 | 64.7 | 65.4 | — | 65.4 | 62.3 | 65.8 | 62.4 |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 86 | 242 | 224 | 145 | 177 | — | 177 | 245 | 274 | 521 |
| Edges on loops from a border to a border, % (L) | — | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 71.5 | 6.2 | 9.7 | 1.8 | 0.1 | 77.4 | 13.4 | 0.0 | 1.0 | 0.0 | 1.1 |
| Time, seconds (T) | — | 4.48 | 22.16 | 18.09 | 16.11 | 17.41 | 17.66 | 17.14 | 17.66 | 22.48 | — | 30.45 |

Failures: quadriflow: cancelled.

## human_female (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 689 | 2837 | 2766 | 2947 | 2977 | 5391 | 2635 | 5391 | 5320 | 10590 | 10656 |
| Irregular vertices (S) | — | 77 | 204 | 92 | 110 | 171 | 248 | 142 | 248 | 224 | 476 | 278 |
| … of which above the minimum | — | — | 196 | — | 102 | 163 | — | — | — | 216 | — | 270 |
| Base complex patches (C) | — | — | 2366 | — | 1131 | 1797 | — | — | — | 3134 | — | 4118 |
| Alignment, degrees (A) | — | 11.80 | 10.59 | 6.68 | 7.40 | 7.76 | 5.43 | 6.33 | 6.12 | 7.12 | 9.43 | 7.47 |
| Mean angle deviation, degrees (F) | — | 14.87 | 7.68 | 6.60 | 4.82 | 5.69 | 6.40 | 7.65 | 6.40 | 5.07 | 8.82 | 4.18 |
| Angle deviation, 95th percentile (F) | — | 40.05 | 25.19 | 21.54 | 17.16 | 17.91 | 21.59 | 25.61 | 21.59 | 16.98 | 25.47 | 14.56 |
| Mean aspect ratio (F) | — | 2.191 | 1.291 | 1.348 | 1.514 | 1.333 | 1.900 | 1.425 | 1.900 | 1.330 | 1.657 | 1.277 |
| Area variation (F) | — | 0.762 | 0.239 | 0.327 | 0.407 | 0.637 | 0.585 | 0.374 | 0.585 | 0.570 | 1.432 | 0.488 |
| Hausdorff, ‰ of the diagonal (H) | — | 40.87 | 56.57 | 39.14 | 20.79 | 13.93 | 7.43 | 43.64 | 7.43 | 6.75 | 4.44 | 4.97 |
| Mean distance, ‰ (H) | — | 6.121 | 4.562 | 2.685 | 1.593 | 1.039 | 0.431 | 2.663 | 0.431 | 0.434 | 0.156 | 0.189 |
| Thin parts: mean distance, % of a quad edge (V) | — | 87.6 | 97.9 | 57.7 | 29.5 | 16.7 | 5.3 | 56.1 | 7.5 | 8.3 | 1.4 | 6.8 |
| Volume kept, % of the source (V) | — | 85.52 | 96.97 | 99.64 | 99.88 | 100.03 | 98.76 | 97.05 | 98.76 | 100.03 | 98.78 | 100.02 |
| Edges on closed loops, % (L) | — | 11.9 | 8.2 | 24.7 | 30.9 | 19.8 | 29.4 | 32.3 | 29.4 | 20.5 | 31.6 | 30.7 |
| Edges on spiralling loops, % (L) | — | 20.2 | 9.7 | 17.5 | 5.7 | 13.5 | 1.4 | 3.6 | 1.4 | 13.9 | 2.5 | 4.0 |
| Spiralling loops (L) | — | 18 | 18 | 15 | 15 | 13 | 5 | 13 | 5 | 20 | 3 | 14 |
| Edges on loops that stop at an irregular vertex, % (L) | — | 67.9 | 80.5 | 57.8 | 63.3 | 66.7 | 69.2 | 64.1 | 69.2 | 65.6 | 65.8 | 65.3 |
| Longest loop that stops at an irregular vertex, edges (L) | — | 46 | 108 | 188 | 221 | 166 | 211 | 185 | 211 | 291 | 274 | 389 |
| Edges on loops from a border to a border, % (L) | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 77.0 | 5.4 | 7.8 | 1.8 | 0.8 | 79.7 | 12.2 | 0.0 | 1.3 | 0.0 | 0.6 |
| Time, seconds (T) | — | 4.77 | 20.88 | 17.37 | 13.91 | 16.19 | 21.63 | 19.65 | 21.63 | 21.59 | — | 34.88 |

Failures: quadriflow: cancelled.

## spot (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2964 | 1284 | 3044 | 2659 | 3002 | 2951 | 3478 | 2686 | 3478 | 3482 | — | — |
| Irregular vertices (S) | 68 | 68 | 153 | 70 | 70 | 70 | 82 | 87 | 82 | 70 | — | — |
| … of which above the minimum | 60 | 60 | 145 | 62 | 62 | 62 | 74 | 79 | 74 | 62 | — | — |
| Base complex patches (C) | 2902 | 1022 | 2978 | 1987 | 1566 | 1649 | 2428 | 2118 | 2428 | 1794 | — | — |
| Alignment, degrees (A) | 18.12 | 12.08 | 17.17 | 9.37 | 8.93 | 7.85 | 6.29 | 7.56 | 6.35 | 7.79 | — | — |
| Mean angle deviation, degrees (F) | 7.25 | 8.87 | 5.59 | 9.51 | 5.54 | 5.28 | 6.35 | 7.05 | 6.35 | 5.57 | — | — |
| Angle deviation, 95th percentile (F) | 20.42 | 24.77 | 18.71 | 24.15 | 15.26 | 14.97 | 17.74 | 19.43 | 17.74 | 15.38 | — | — |
| Mean aspect ratio (F) | 1.194 | 1.451 | 1.179 | 1.344 | 1.363 | 1.340 | 1.460 | 1.318 | 1.460 | 1.350 | — | — |
| Area variation (F) | 0.177 | 0.666 | 0.172 | 0.289 | 0.305 | 0.426 | 0.392 | 0.268 | 0.392 | 0.429 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 17.71 | 17.26 | 19.38 | 8.80 | 5.81 | 5.58 | 6.64 | 10.63 | 6.64 | 3.85 | — | — |
| Mean distance, ‰ (H) | 0.448 | 1.588 | 0.479 | 0.237 | 0.208 | 0.199 | 0.372 | 0.476 | 0.372 | 0.172 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 56.3 | 54.6 | 53.8 | 14.8 | 4.1 | 3.5 | 8.5 | 21.2 | — | — | — | — |
| Volume kept, % of the source (V) | 99.37 | 97.44 | 99.35 | 100.00 | 100.03 | 100.01 | 99.35 | 99.27 | 99.35 | 100.01 | — | — |
| Edges on closed loops, % (L) | 0.0 | 10.2 | — | — | 24.1 | 22.2 | 15.5 | — | — | 24.4 | — | — |
| Edges on spiralling loops, % (L) | 22.5 | 5.6 | — | — | 7.5 | 15.0 | 16.7 | — | — | 13.4 | — | — |
| Spiralling loops (L) | 10 | 3 | — | — | 4 | 5 | 7 | — | — | 5 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 77.5 | 84.2 | — | — | 68.4 | 62.8 | 67.8 | — | — | 62.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 299 | 90 | — | — | 273 | 176 | 156 | — | — | 248 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 1.2 | 57.2 | 1.5 | 11.4 | 0.1 | 1.6 | 15.9 | 10.5 | 0.0 | 0.1 | — | — |
| Time, seconds (T) | 1.80 | 0.94 | 1.23 | 6.69 | 3.69 | 4.10 | 19.14 | 15.16 | 19.14 | 4.60 | — | — |

## bob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2928 | 1583 | 3040 | 2642 | 2966 | 2950 | 2920 | 2696 | 2920 | 2887 | — | — |
| Irregular vertices (S) | 32 | 18 | 130 | 20 | 20 | 20 | 28 | 30 | 28 | 20 | — | — |
| … of which above the minimum | 32 | 18 | 130 | 20 | 20 | 20 | 28 | 30 | 28 | 20 | — | — |
| Base complex patches (C) | 2903 | 533 | 3040 | 714 | 268 | 237 | 2390 | 2568 | 2390 | 256 | — | — |
| Alignment, degrees (A) | 19.37 | 6.64 | 20.80 | 4.37 | 3.86 | 3.61 | 4.85 | 5.15 | 4.84 | 3.82 | — | — |
| Mean angle deviation, degrees (F) | 6.41 | 7.07 | 5.33 | 4.82 | 3.32 | 3.25 | 5.52 | 5.94 | 5.52 | 3.59 | — | — |
| Angle deviation, 95th percentile (F) | 18.61 | 21.04 | 17.96 | 14.72 | 9.02 | 9.11 | 15.59 | 15.77 | 15.59 | 10.21 | — | — |
| Mean aspect ratio (F) | 1.175 | 2.090 | 1.158 | 1.409 | 1.379 | 1.370 | 1.368 | 1.360 | 1.368 | 1.358 | — | — |
| Area variation (F) | 0.159 | 0.474 | 0.161 | 0.339 | 0.397 | 0.433 | 0.360 | 0.324 | 0.360 | 0.443 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.82 | 4.03 | 7.32 | 2.49 | 2.01 | 1.40 | 1.96 | 2.27 | 1.96 | 2.09 | — | — |
| Mean distance, ‰ (H) | 0.309 | 0.960 | 0.325 | 0.152 | 0.134 | 0.132 | 0.322 | 0.355 | 0.322 | 0.140 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.48 | 98.10 | 99.44 | 100.01 | 100.02 | 100.02 | 99.37 | 99.31 | 99.37 | 100.02 | — | — |
| Edges on closed loops, % (L) | 0.6 | 45.2 | — | — | 72.0 | 71.1 | 9.3 | — | — | 71.4 | — | — |
| Edges on spiralling loops, % (L) | 35.4 | 11.1 | — | — | 1.9 | 3.7 | 45.9 | — | — | 5.5 | — | — |
| Spiralling loops (L) | 9 | 3 | — | — | 1 | 1 | 12 | — | — | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 64.6 | 43.7 | — | — | 26.1 | 25.2 | 44.8 | — | — | 23.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 309 | 172 | — | — | 126 | 118 | 218 | — | — | 115 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 2.4 | 47.2 | 1.3 | 11.9 | 1.1 | 1.7 | 2.7 | 10.1 | 0.0 | 1.1 | — | — |
| Time, seconds (T) | 1.72 | 1.14 | 1.45 | 6.22 | 2.48 | 2.69 | 17.24 | 17.35 | 17.24 | 3.54 | — | — |

## blub (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2698 | 1343 | 3036 | 2680 | 3009 | 3030 | 3889 | 2703 | 3889 | 3871 | — | — |
| Irregular vertices (S) | 52 | 47 | 158 | 43 | 44 | 58 | 55 | 46 | 55 | 52 | — | — |
| … of which above the minimum | 44 | — | 150 | 35 | 36 | 50 | 47 | 38 | 47 | 44 | — | — |
| Base complex patches (C) | 2584 | — | 2922 | 902 | 717 | 1258 | 2003 | 2057 | 2003 | 1287 | — | — |
| Alignment, degrees (A) | 15.95 | 10.62 | 15.45 | 7.64 | 7.77 | 7.78 | 5.26 | 7.01 | 5.48 | 7.31 | — | — |
| Mean angle deviation, degrees (F) | 10.72 | 9.14 | 5.55 | 8.08 | 5.17 | 4.94 | 6.30 | 7.24 | 6.30 | 5.16 | — | — |
| Angle deviation, 95th percentile (F) | 28.70 | 28.42 | 19.72 | 22.68 | 15.78 | 14.85 | 16.74 | 18.88 | 16.74 | 15.68 | — | — |
| Mean aspect ratio (F) | 1.271 | 2.046 | 1.192 | 1.327 | 1.300 | 1.290 | 1.828 | 1.351 | 1.828 | 1.316 | — | — |
| Area variation (F) | 0.261 | 0.522 | 0.178 | 0.266 | 0.292 | 0.407 | 0.445 | 0.269 | 0.445 | 0.385 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 40.70 | 17.71 | 15.68 | 12.42 | 8.81 | 8.13 | 5.19 | 11.32 | 5.19 | 5.21 | — | — |
| Mean distance, ‰ (H) | 0.588 | 1.082 | 0.390 | 0.254 | 0.182 | 0.187 | 0.230 | 0.392 | 0.230 | 0.124 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 46.5 | 17.7 | 24.8 | 19.4 | 9.7 | 7.9 | 7.7 | 20.0 | 10.8 | 4.3 | — | — |
| Volume kept, % of the source (V) | 98.93 | — | 99.17 | 99.88 | 99.97 | 99.96 | 99.55 | 99.13 | 99.55 | 99.99 | — | — |
| Edges on closed loops, % (L) | 0.0 | — | — | — | 45.4 | 35.8 | 25.2 | — | — | 41.2 | — | — |
| Edges on spiralling loops, % (L) | 32.5 | — | — | — | 5.8 | 12.9 | 20.5 | — | — | 15.6 | — | — |
| Spiralling loops (L) | 13 | — | — | — | 2 | 8 | 6 | — | — | 6 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 64.2 | — | — | — | 48.8 | 51.3 | 54.3 | — | — | 43.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 225 | — | — | — | 178 | 170 | 201 | — | — | 183 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 10.1 | 55.2 | 1.2 | 10.7 | 0.3 | 1.0 | 29.6 | 9.9 | 0.0 | 0.5 | — | — |
| Time, seconds (T) | 1.68 | 1.23 | 1.25 | 6.03 | 2.42 | 3.73 | 14.34 | 12.32 | 14.34 | 3.85 | — | — |

## pig (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2675 | 1808 | 3016 | 2680 | 2949 | 3054 | 4698 | 2552 | 4698 | 4684 | — | — |
| Irregular vertices (S) | 122 | 99 | 209 | 82 | 78 | 104 | 106 | 96 | 106 | 104 | — | — |
| … of which above the minimum | 114 | — | 192 | — | 70 | 96 | 98 | 88 | 98 | 96 | — | — |
| Base complex patches (C) | 2612 | — | 2912 | — | 1571 | 1690 | 3004 | 1840 | 3004 | 2876 | — | — |
| Alignment, degrees (A) | 17.41 | 11.38 | 16.24 | 10.81 | 10.92 | 9.17 | 7.22 | 9.36 | 7.34 | 8.78 | — | — |
| Mean angle deviation, degrees (F) | 14.32 | 8.90 | 6.83 | 8.29 | 5.78 | 5.63 | 7.22 | 7.93 | 7.22 | 5.56 | — | — |
| Angle deviation, 95th percentile (F) | 41.52 | 29.13 | 23.86 | 23.45 | 17.12 | 17.28 | 21.68 | 23.91 | 21.68 | 16.30 | — | — |
| Mean aspect ratio (F) | 1.383 | 1.804 | 1.244 | 1.266 | 1.260 | 1.286 | 1.876 | 1.322 | 1.876 | 1.278 | — | — |
| Area variation (F) | 0.398 | 0.713 | 0.210 | 0.262 | 0.287 | 0.483 | 0.636 | 0.287 | 0.636 | 0.472 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 44.59 | 52.50 | 48.32 | 25.35 | 23.42 | 12.40 | 10.22 | 12.72 | 10.22 | 8.02 | — | — |
| Mean distance, ‰ (H) | 1.056 | 2.185 | 1.012 | 0.532 | 0.475 | 0.310 | 0.375 | 0.724 | 0.375 | 0.201 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 67.8 | 29.1 | 26.2 | 25.0 | 22.3 | 5.5 | 3.7 | 16.4 | 6.3 | 5.8 | — | — |
| Volume kept, % of the source (V) | 98.78 | 97.54 | 98.84 | 99.69 | 99.74 | 99.97 | 99.54 | 98.93 | 99.54 | 99.99 | — | — |
| Edges on closed loops, % (L) | 0.4 | 10.6 | — | — | 25.9 | 24.8 | 18.0 | — | — | 20.2 | — | — |
| Edges on spiralling loops, % (L) | 9.5 | 5.7 | — | — | 16.1 | 9.0 | 24.2 | — | — | 19.9 | — | — |
| Spiralling loops (L) | 12 | 5 | — | — | 9 | 9 | 20 | — | — | 14 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 90.5 | 83.7 | — | — | 58.0 | 66.2 | 57.8 | — | — | 59.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 218 | 113 | — | — | 162 | 145 | 191 | — | — | 326 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 10.8 | 39.7 | 1.1 | 10.7 | 1.7 | 1.8 | 56.6 | 14.9 | 0.0 | 0.3 | — | — |
| Time, seconds (T) | 1.65 | 1.53 | 1.15 | 9.77 | 2.85 | 4.39 | 14.59 | 16.02 | 14.59 | 8.10 | — | — |

## jerry (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 1/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3122 | 1102 | 2959 | 2595 | 3013 | 2969 | 6872 | 2573 | 6872 | 6849 | — | — |
| Irregular vertices (S) | 104 | 84 | 265 | 97 | 84 | 162 | 206 | 181 | 206 | 266 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2211 | — | 2959 | 1552 | 1686 | 2397 | — | 2088 | — | 5678 | — | — |
| Alignment, degrees (A) | 17.93 | 17.98 | 18.35 | 14.87 | 13.82 | 13.67 | 11.02 | 13.58 | 11.29 | 12.45 | — | — |
| Mean angle deviation, degrees (F) | 11.25 | 15.26 | 8.58 | 9.60 | 6.12 | 6.75 | 9.67 | 10.03 | 9.67 | 6.21 | — | — |
| Angle deviation, 95th percentile (F) | 29.73 | 53.91 | 29.29 | 24.87 | 16.27 | 21.79 | 26.54 | 28.28 | 26.54 | 17.42 | — | — |
| Mean aspect ratio (F) | 1.301 | 2.330 | 1.357 | 1.324 | 1.296 | 1.354 | 2.063 | 1.483 | 2.063 | 1.302 | — | — |
| Area variation (F) | 0.278 | 2.551 | 0.258 | 0.272 | 0.315 | 0.555 | 0.724 | 0.324 | 0.724 | 0.522 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 38.82 | 200.69 | 49.21 | 48.31 | 31.53 | 21.30 | 6.74 | 15.07 | 6.74 | 12.40 | — | — |
| Mean distance, ‰ (H) | 1.467 | 5.546 | 2.158 | 1.786 | 1.178 | 0.857 | 0.529 | 1.488 | 0.529 | 0.339 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 23.4 | 76.1 | 40.3 | 57.3 | 28.6 | 15.9 | 4.8 | 19.2 | 6.8 | 10.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 6.8 | 3.1 | 3.3 | — | — | 2.5 | — | — |
| Edges on spiralling loops, % (L) | 0.7 | 13.4 | — | — | 2.4 | 2.3 | 6.4 | — | — | 0.9 | — | — |
| Spiralling loops (L) | 1 | 6 | — | — | 1 | 5 | 10 | — | — | 5 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 82.0 | 86.6 | — | — | 72.1 | 87.6 | 85.0 | — | — | 89.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 155 | 64 | — | — | 126 | 111 | 282 | — | — | 188 | — | — |
| Edges on loops from a border to a border, % (L) | 17.0 | 0.0 | — | — | 18.7 | 7.0 | 5.3 | — | — | 7.3 | — | — |
| Off budget, % (N) | 4.1 | 63.3 | 1.4 | 13.5 | 0.4 | 1.0 | 129.1 | 14.2 | 0.0 | 0.3 | — | — |
| Time, seconds (T) | 1.74 | 1.38 | 0.59 | 7.59 | 2.53 | 3.52 | 6.74 | 6.79 | 6.74 | 8.19 | — | — |

## rose (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 2085 | 636 | 2226 | 1923 | 3364 | 3585 | 35874 | 2689 | 35874 | 36253 | — | — |
| Irregular vertices (S) | 539 | 243 | 499 | 252 | 861 | 1051 | 1772 | 763 | 1772 | 2375 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | 1759 | — | — |
| Base complex patches (C) | 1872 | — | 1954 | — | — | — | — | — | — | 25991 | — | — |
| Alignment, degrees (A) | 18.30 | 19.39 | 19.07 | 16.12 | 14.84 | 14.98 | 8.98 | 14.12 | 11.16 | 9.33 | — | — |
| Mean angle deviation, degrees (F) | 16.45 | 31.06 | 15.58 | 13.29 | 19.34 | 18.76 | 9.06 | 13.63 | 9.06 | 6.99 | — | — |
| Angle deviation, 95th percentile (F) | 48.25 | 82.38 | 48.45 | 38.66 | 62.94 | 59.34 | 26.38 | 39.32 | 26.38 | 22.00 | — | — |
| Mean aspect ratio (F) | 1.647 | 5.541 | 1.752 | 1.809 | 8.828 | 6.675 | 1.790 | 1.653 | 1.790 | 1.453 | — | — |
| Area variation (F) | 0.587 | 1.193 | 0.465 | 0.441 | 1.353 | 1.652 | 1.388 | 0.599 | 1.388 | 0.521 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 75.61 | 108.28 | 79.25 | 72.60 | 29.23 | 31.55 | 74.46 | 78.76 | 74.46 | 7.95 | — | — |
| Mean distance, ‰ (H) | 3.765 | 21.969 | 4.183 | 3.665 | 2.802 | 3.787 | 0.842 | 3.206 | 0.842 | 0.160 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 39.1 | 157.4 | 46.1 | 40.1 | 27.0 | 35.8 | 11.9 | 35.3 | 116.7 | 9.7 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 78.81 | — | — | — | — | 99.65 | — | — |
| Edges on closed loops, % (L) | 3.2 | — | — | — | 11.5 | — | — | — | — | 14.6 | — | — |
| Edges on spiralling loops, % (L) | 17.6 | — | — | — | 30.3 | — | — | — | — | 12.9 | — | — |
| Spiralling loops (L) | 82 | — | — | — | 409 | — | — | — | — | 278 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 79.7 | — | — | — | 58.2 | — | — | — | — | 72.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 38 | — | — | — | 45 | — | — | — | — | 193 | — | — |
| Edges on loops from a border to a border, % (L) | 0.5 | — | — | — | 0.0 | — | — | — | — | 0.0 | — | — |
| Off budget, % (N) | 30.5 | 78.8 | 25.8 | 35.9 | 12.1 | 19.5 | 1095.8 | 10.4 | 0.0 | 1.1 | — | — |
| Time, seconds (T) | 2.21 | 1.16 | 2.12 | 16.37 | 2.54 | 6.40 | 9.53 | 10.86 | 9.53 | 28.08 | — | — |

## bridge (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 1893 | 2903 | 2750 | 3054 | 3023 | 4533 | 2726 | 4533 | 4570 | — | — |
| Irregular vertices (S) | — | 63 | 151 | 53 | 50 | 44 | 81 | 64 | 81 | 54 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1879 | 520 | 1015 | 667 | — | — | — | 1171 | — | — |
| Alignment, degrees (A) | — | 9.17 | 12.58 | 8.69 | 8.00 | 7.41 | 6.49 | 7.93 | 7.33 | 7.94 | — | — |
| Mean angle deviation, degrees (F) | — | 7.60 | 6.12 | 7.12 | 4.06 | 4.22 | 6.65 | 7.20 | 6.65 | 4.47 | — | — |
| Angle deviation, 95th percentile (F) | — | 22.56 | 20.51 | 20.62 | 11.51 | 12.21 | 18.96 | 19.72 | 18.96 | 12.48 | — | — |
| Mean aspect ratio (F) | — | 1.778 | 1.215 | 1.305 | 1.260 | 1.271 | 1.753 | 1.413 | 1.753 | 1.302 | — | — |
| Area variation (F) | — | 0.765 | 0.174 | 0.232 | 0.287 | 0.434 | 0.421 | 0.275 | 0.421 | 0.397 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 14.76 | 28.19 | 7.13 | 7.02 | 6.65 | 6.05 | 7.00 | 6.05 | 7.09 | — | — |
| Mean distance, ‰ (H) | — | 1.042 | 0.638 | 0.500 | 0.454 | 0.438 | 0.414 | 0.670 | 0.414 | 0.334 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 19.4 | — | — | 19.5 | 27.3 | 15.1 | — | — | 23.8 | — | — |
| Edges on spiralling loops, % (L) | — | 8.2 | — | — | 4.5 | 0.0 | 7.8 | — | — | 0.9 | — | — |
| Spiralling loops (L) | — | 4 | — | — | 4 | 0 | 7 | — | — | 2 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 62.1 | — | — | 53.9 | 50.5 | 60.0 | — | — | 51.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 70 | — | — | 108 | 125 | 145 | — | — | 153 | — | — |
| Edges on loops from a border to a border, % (L) | — | 10.3 | — | — | 22.2 | 22.2 | 17.1 | — | — | 23.7 | — | — |
| Off budget, % (N) | — | 36.9 | 3.2 | 8.3 | 1.8 | 0.8 | 51.1 | 9.1 | 0.0 | 0.8 | — | — |
| Time, seconds (T) | — | 1.21 | 2.98 | 10.79 | 3.77 | 5.07 | 11.41 | 11.88 | 11.41 | 5.30 | — | — |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2890 | 1855 | 2938 | 2976 | 3049 | 2978 | 2815 | 2815 | 2815 | 2780 | — | — |
| Irregular vertices (S) | 28 | 117 | 204 | 24 | 24 | 24 | 24 | 24 | 24 | 24 | — | — |
| … of which above the minimum | 20 | — | 196 | 16 | 16 | 16 | 16 | 16 | 16 | 16 | — | — |
| Base complex patches (C) | 619 | — | 1782 | 78 | 78 | 78 | 501 | 501 | 501 | 92 | — | — |
| Alignment, degrees (A) | 12.97 | 15.75 | 13.96 | 11.65 | 11.96 | 10.49 | 10.90 | 10.90 | 10.90 | 10.37 | — | — |
| Mean angle deviation, degrees (F) | 2.66 | 12.33 | 6.10 | 2.61 | 1.35 | 1.59 | 3.02 | 3.02 | 3.02 | 1.56 | — | — |
| Angle deviation, 95th percentile (F) | 10.11 | 37.42 | 25.03 | 10.79 | 4.82 | 6.04 | 12.04 | 12.04 | 12.04 | 6.33 | — | — |
| Mean aspect ratio (F) | 1.116 | 2.476 | 1.233 | 1.163 | 1.158 | 1.164 | 1.326 | 1.326 | 1.326 | 1.138 | — | — |
| Area variation (F) | 0.100 | 0.672 | 0.199 | 0.126 | 0.113 | 0.365 | 0.196 | 0.196 | 0.196 | 0.400 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.54 | 17.76 | 9.41 | 0.80 | 0.92 | 0.47 | 0.89 | 0.89 | 0.89 | 0.51 | — | — |
| Mean distance, ‰ (H) | 0.114 | 1.229 | 0.067 | 0.054 | 0.052 | 0.056 | 0.068 | 0.068 | 0.068 | 0.061 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.15 | — | 100.14 | 100.00 | 100.00 | 100.00 | 100.12 | 100.12 | 100.12 | 100.00 | — | — |
| Edges on closed loops, % (L) | 51.3 | — | — | — | 83.8 | 84.0 | 55.7 | — | — | 82.0 | — | — |
| Edges on spiralling loops, % (L) | 26.0 | — | — | — | 0.0 | 0.0 | 24.0 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 7 | — | — | — | 0 | 0 | 7 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 21.7 | — | — | — | 16.2 | 16.0 | 20.3 | — | — | 18.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 90 | — | — | — | 57 | 48 | 91 | — | — | 45 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.7 | 38.2 | 2.1 | 0.8 | 1.6 | 0.7 | 6.2 | 6.2 | 0.0 | 1.2 | — | — |
| Time, seconds (T) | 2.72 | 1.14 | 0.71 | 4.01 | 3.17 | 2.23 | 2.83 | 2.88 | 2.83 | 2.29 | — | — |

## cube (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2904 | 1260 | 2974 | 2992 | 2992 | 2992 | 3082 | 3082 | 3082 | 3082 | — | — |
| Irregular vertices (S) | 8 | 8 | 24 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | 0 | 16 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 6 | 36 | 274 | 1102 | 6 | 6 | 6 | 6 | 6 | 6 | — | — |
| Alignment, degrees (A) | 0.29 | 2.29 | 1.97 | 0.75 | 0.14 | 0.36 | 0.13 | 0.13 | 0.13 | 0.37 | — | — |
| Mean angle deviation, degrees (F) | 0.18 | 3.61 | 1.65 | 0.75 | 0.01 | 0.36 | 0.00 | 0.00 | 0.00 | 0.36 | — | — |
| Angle deviation, 95th percentile (F) | 0.43 | 11.71 | 4.89 | 2.01 | 0.02 | 0.85 | 0.00 | 0.00 | 0.00 | 0.93 | — | — |
| Mean aspect ratio (F) | 1.005 | 2.174 | 1.062 | 1.039 | 1.031 | 1.088 | 1.030 | 1.030 | 1.030 | 1.088 | — | — |
| Area variation (F) | 0.004 | 0.662 | 0.090 | 0.045 | 0.021 | 0.098 | 0.021 | 0.021 | 0.021 | 0.098 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 4.80 | 10.77 | 0.00 | 8.84 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.017 | 0.439 | 0.000 | 0.067 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.89 | 100.00 | 99.99 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 95.5 | 82.9 | — | — | 95.5 | 95.5 | 95.6 | — | — | 95.6 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 0 | 0 | — | — | 0 | 0 | 0 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 4.5 | 17.1 | — | — | 4.5 | 4.5 | 4.4 | — | — | 4.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 22 | 58 | — | — | 23 | 23 | 23 | — | — | 23 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.2 | 58.0 | 1.0 | 0.3 | 0.3 | 0.3 | 2.7 | 2.7 | 0.0 | 0.0 | — | — |
| Time, seconds (T) | 1.21 | 0.63 | 0.20 | 5.18 | 1.58 | 1.91 | 2.97 | 3.00 | 2.97 | 1.67 | — | — |

## lblock (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2928 | 1328 | 2780 | 2800 | 3060 | 2972 | 3030 | 3030 | 3030 | 2972 | — | — |
| Irregular vertices (S) | 12 | 12 | 24 | 12 | 12 | 12 | 12 | 12 | 12 | 12 | — | — |
| … of which above the minimum | 4 | 4 | 16 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | — | — |
| Base complex patches (C) | 86 | 628 | 464 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | — | — |
| Alignment, degrees (A) | 1.09 | 3.78 | 1.54 | 0.59 | 0.52 | 0.86 | 0.63 | 0.63 | 0.63 | 0.86 | — | — |
| Mean angle deviation, degrees (F) | 0.80 | 5.83 | 1.56 | 0.23 | 0.01 | 0.53 | 0.37 | 0.37 | 0.37 | 0.53 | — | — |
| Angle deviation, 95th percentile (F) | 2.44 | 18.79 | 4.54 | 0.98 | 0.05 | 1.42 | 1.43 | 1.43 | 1.43 | 1.42 | — | — |
| Mean aspect ratio (F) | 1.047 | 1.994 | 1.051 | 1.022 | 1.030 | 1.098 | 1.037 | 1.037 | 1.037 | 1.097 | — | — |
| Area variation (F) | 0.039 | 0.572 | 0.063 | 0.023 | 0.033 | 0.106 | 0.021 | 0.021 | 0.021 | 0.106 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 10.33 | 11.65 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.034 | 0.454 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.83 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 78.3 | 26.4 | — | — | 93.2 | 93.1 | 93.2 | — | — | 93.1 | — | — |
| Edges on spiralling loops, % (L) | 13.8 | 62.2 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 1 | 5 | — | — | 0 | 0 | 0 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 7.9 | 11.4 | — | — | 6.8 | 6.9 | 6.8 | — | — | 6.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 86 | 58 | — | — | 45 | 43 | 45 | — | — | 43 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 2.4 | 55.7 | 7.3 | 6.7 | 2.0 | 0.9 | 1.0 | 1.0 | 0.0 | 1.9 | — | — |
| Time, seconds (T) | 1.19 | 0.82 | 0.36 | 23.07 | 1.62 | 1.38 | 2.73 | 2.90 | 2.73 | 1.41 | — | — |

## cylinder (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2900 | 1306 | 3100 | 2533 | 2988 | 3040 | 2458 | 2458 | 2458 | 2496 | — | — |
| Irregular vertices (S) | 8 | 8 | 91 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | 0 | 83 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 1003 | 68 | 3052 | 643 | 726 | 546 | 408 | 408 | 408 | 446 | — | — |
| Alignment, degrees (A) | 4.18 | 4.72 | 5.46 | 1.56 | 1.50 | 1.72 | 1.52 | 1.52 | 1.64 | 1.43 | — | — |
| Mean angle deviation, degrees (F) | 4.32 | 8.13 | 4.18 | 4.16 | 3.60 | 2.88 | 3.22 | 3.22 | 3.22 | 2.58 | — | — |
| Angle deviation, 95th percentile (F) | 17.78 | 26.13 | 15.60 | 17.67 | 14.03 | 8.47 | 11.87 | 11.87 | 11.87 | 8.86 | — | — |
| Mean aspect ratio (F) | 1.087 | 2.019 | 1.132 | 1.355 | 1.387 | 1.563 | 1.511 | 1.511 | 1.511 | 1.645 | — | — |
| Area variation (F) | 0.103 | 0.640 | 0.140 | 0.374 | 0.367 | 0.245 | 0.403 | 0.403 | 0.403 | 0.286 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 2.07 | 9.51 | 3.74 | 1.00 | 0.93 | 0.91 | 1.43 | 1.43 | 1.43 | 1.05 | — | — |
| Mean distance, ‰ (H) | 0.144 | 0.790 | 0.121 | 0.145 | 0.117 | 0.140 | 0.369 | 0.369 | 0.369 | 0.180 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.91 | 99.21 | 99.90 | 100.00 | 100.00 | 100.00 | 99.56 | 99.56 | 99.56 | 100.00 | — | — |
| Edges on closed loops, % (L) | 33.0 | 74.5 | — | — | 37.9 | 41.0 | 41.7 | — | — | 41.1 | — | — |
| Edges on spiralling loops, % (L) | 55.4 | 0.0 | — | — | 26.5 | 49.1 | 39.4 | — | — | 35.6 | — | — |
| Spiralling loops (L) | 6 | 0 | — | — | 1 | 5 | 2 | — | — | 3 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 11.6 | 25.5 | — | — | 35.7 | 9.9 | 18.9 | — | — | 23.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 207 | 178 | — | — | 739 | 180 | 345 | — | — | 227 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.3 | 56.5 | 3.3 | 15.6 | 0.4 | 1.3 | 18.1 | 18.1 | 0.0 | 1.5 | — | — |
| Time, seconds (T) | 1.05 | 0.65 | 0.17 | 2.92 | 1.74 | 2.05 | 2.68 | 2.49 | 2.68 | 1.37 | — | — |

## venturi (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 4046 | 2489 | 2412 | 2721 | 3050 | 14193 | 9910 | 14193 | 14233 | — | — |
| Irregular vertices (S) | — | 1527 | 432 | 169 | 206 | 409 | 613 | 613 | 613 | 840 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1464 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 11.84 | 11.84 | 11.55 | 11.37 | 13.56 | 7.08 | 6.76 | 8.23 | 10.39 | — | — |
| Mean angle deviation, degrees (F) | — | 48.87 | 13.84 | 8.20 | 8.58 | 11.45 | 9.06 | 9.35 | 9.06 | 6.70 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.70 | 52.08 | 25.25 | 30.13 | 42.20 | 41.05 | 48.55 | 41.05 | 25.36 | — | — |
| Mean aspect ratio (F) | — | 12.801 | 1.928 | 1.344 | 1.373 | 1.773 | 2.237 | 2.425 | 2.237 | 1.427 | — | — |
| Area variation (F) | — | 2.373 | 0.475 | 0.291 | 0.347 | 0.479 | 1.573 | 1.464 | 1.573 | 0.335 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 71.10 | 40.69 | 38.37 | 44.81 | 44.81 | 14.05 | 18.60 | 14.05 | 23.34 | — | — |
| Mean distance, ‰ (H) | — | 5.233 | 4.298 | 5.725 | 4.794 | 4.304 | 0.337 | 0.555 | 0.337 | 1.137 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 24.5 | 28.2 | 36.2 | 29.9 | 26.5 | 1.7 | 2.6 | 3.0 | 23.9 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 14.8 | 9.8 | 5.9 | — | — | 15.1 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 38.3 | 25.4 | 3.4 | — | — | 26.7 | — | — |
| Spiralling loops (L) | — | — | — | — | 86 | 71 | 18 | — | — | 33 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 46.9 | 64.8 | 29.5 | — | — | 57.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 119 | 119 | 112 | — | — | 324 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | 61.3 | — | — | 0.5 | — | — |
| Off budget, % (N) | — | 34.9 | 17.0 | 19.6 | 9.3 | 1.7 | 373.1 | 230.3 | 0.0 | 0.3 | — | — |
| Time, seconds (T) | — | 1.67 | 1.05 | 22.47 | 12.87 | 18.45 | 5.47 | 4.22 | 5.47 | 110.62 | — | — |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 3/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 1283 | 2836 | 2515 | 2978 | 2998 | 6269 | 2409 | 6269 | 6234 | — | — |
| Irregular vertices (S) | — | 87 | 223 | 85 | 88 | 194 | 275 | 210 | 275 | 314 | — | — |
| … of which above the minimum | — | — | 219 | — | 80 | 186 | 267 | — | 267 | 306 | — | — |
| Base complex patches (C) | — | — | 2718 | — | 2124 | 2078 | 4923 | — | 4923 | 4878 | — | — |
| Alignment, degrees (A) | — | 16.86 | 18.44 | 13.66 | 13.67 | 12.42 | 9.30 | 11.90 | 9.51 | 10.62 | — | — |
| Mean angle deviation, degrees (F) | — | 10.34 | 7.84 | 9.13 | 6.24 | 6.60 | 9.35 | 10.70 | 9.35 | 6.74 | — | — |
| Angle deviation, 95th percentile (F) | — | 29.60 | 25.71 | 24.63 | 18.17 | 19.86 | 24.49 | 28.47 | 24.49 | 19.62 | — | — |
| Mean aspect ratio (F) | — | 1.786 | 1.295 | 1.502 | 1.514 | 1.411 | 2.322 | 1.600 | 2.322 | 1.383 | — | — |
| Area variation (F) | — | 0.654 | 0.236 | 0.339 | 0.372 | 0.785 | 0.763 | 0.383 | 0.763 | 0.901 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 30.78 | 61.14 | 38.53 | 27.54 | 16.61 | 6.85 | 42.33 | 6.85 | 8.73 | — | — |
| Mean distance, ‰ (H) | — | 2.902 | 1.839 | 1.177 | 0.775 | 0.749 | 0.574 | 1.440 | 0.574 | 0.438 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 57.6 | 145.0 | 75.2 | 25.6 | 5.1 | 4.0 | 74.8 | 6.9 | 1.9 | — | — |
| Volume kept, % of the source (V) | — | 95.87 | 98.43 | 99.70 | 99.92 | 100.02 | 99.17 | — | 99.17 | 100.01 | — | — |
| Edges on closed loops, % (L) | — | 10.2 | — | — | 14.3 | 15.8 | 10.7 | — | — | 11.1 | — | — |
| Edges on spiralling loops, % (L) | — | 16.6 | — | — | 12.1 | 7.5 | 13.9 | — | — | 12.1 | — | — |
| Spiralling loops (L) | — | 12 | — | — | 8 | 14 | 18 | — | — | 24 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 73.3 | — | — | 73.5 | 76.7 | 75.3 | — | — | 76.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 51 | — | — | 239 | 135 | 182 | — | — | 168 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 57.2 | 5.5 | 16.2 | 0.7 | 0.1 | 109.0 | 19.7 | 0.0 | 0.6 | — | — |
| Time, seconds (T) | — | 13.40 | 89.83 | 57.56 | 51.99 | 106.69 | 36.00 | 36.17 | 36.00 | 68.37 | — | — |

Failures: quadriflow: cancelled.

## watermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 2675 | 709 | 1784 | 2260 | 2189 | 2193 | 22110 | 2850 | 22110 | — | — | — |
| Irregular vertices (S) | 302 | 129 | 307 | 226 | 169 | 163 | 1250 | 484 | 1250 | — | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2588 | — | 1676 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 14.64 | 19.69 | 16.44 | 15.73 | 14.71 | 14.75 | 10.86 | 15.13 | 12.09 | — | — | — |
| Mean angle deviation, degrees (F) | 11.57 | 21.19 | 17.11 | 11.91 | 10.51 | 10.54 | 8.01 | 11.64 | 8.01 | — | — | — |
| Angle deviation, 95th percentile (F) | 39.87 | 71.32 | 57.84 | 47.77 | 38.20 | 38.51 | 24.75 | 41.56 | 24.75 | — | — | — |
| Mean aspect ratio (F) | 1.472 | 2.842 | 1.945 | 1.601 | 1.595 | 1.602 | 1.720 | 1.617 | 1.720 | — | — | — |
| Area variation (F) | 0.335 | 0.833 | 0.490 | 0.562 | 0.519 | 0.521 | 0.834 | 0.372 | 0.834 | — | — | — |
| Hausdorff, ‰ of the diagonal (H) | 86.07 | 187.44 | 74.17 | 77.11 | 81.69 | 81.69 | 33.67 | 57.63 | 33.67 | — | — | — |
| Mean distance, ‰ (H) | 5.115 | 23.533 | 7.799 | 6.793 | 6.948 | 6.950 | 1.298 | 4.420 | 1.298 | — | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 31.4 | 104.7 | 43.1 | 39.8 | 40.1 | 40.1 | 12.7 | 26.0 | 35.7 | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | 99.75 | — | 99.75 | — | — | — |
| Edges on closed loops, % (L) | 2.2 | — | — | — | — | — | 5.2 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 6.9 | — | — | — | — | — | 9.1 | — | — | — | — | — |
| Spiralling loops (L) | 27 | — | — | — | — | — | 75 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.4 | — | — | — | — | — | 85.7 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 76 | — | — | — | — | — | 228 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 2.6 | — | — | — | — | — | 0.0 | — | — | — | — | — |
| Off budget, % (N) | 10.8 | 76.4 | 40.5 | 24.7 | 27.0 | 26.9 | 637.0 | 5.0 | 0.0 | — | — | — |
| Time, seconds (T) | 2.98 | 1.89 | 0.55 | 23.13 | 344.92 | 660.09 | 413.72 | 382.00 | 413.72 | — | — | — |

Failures: openclare-matched: timed out after 1800 s.

## towermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 2724 | 1088 | 1904 | 2395 | 2969 | 2393 | 16107 | 2897 | 16107 | 12835 | — | — |
| Irregular vertices (S) | 378 | 194 | 370 | 147 | 329 | 142 | 813 | 375 | 813 | 412 | — | — |
| … of which above the minimum | — | — | — | — | 25 | — | — | — | — | — | — | — |
| Base complex patches (C) | 2661 | — | 1899 | — | 2837 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 13.28 | 16.22 | 16.31 | 12.77 | 14.40 | 12.77 | 10.58 | 11.73 | 11.77 | 12.24 | — | — |
| Mean angle deviation, degrees (F) | 11.93 | 19.35 | 19.44 | 9.80 | 14.26 | 9.80 | 8.97 | 9.30 | 8.97 | 7.46 | — | — |
| Angle deviation, 95th percentile (F) | 38.83 | 64.57 | 61.40 | 35.38 | 46.85 | 35.38 | 27.41 | 33.36 | 27.41 | 24.75 | — | — |
| Mean aspect ratio (F) | 1.501 | 2.846 | 2.098 | 1.446 | 2.498 | 1.446 | 2.149 | 1.547 | 2.149 | 1.310 | — | — |
| Area variation (F) | 0.329 | 0.711 | 0.515 | 0.322 | 0.721 | 0.320 | 0.715 | 0.327 | 0.715 | 0.297 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 56.00 | 98.83 | 77.09 | 63.94 | 65.07 | 63.94 | 27.59 | 44.98 | 27.59 | 29.36 | — | — |
| Mean distance, ‰ (H) | 3.285 | 12.802 | 6.169 | 4.799 | 3.529 | 4.800 | 1.555 | 3.480 | 1.555 | 1.173 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 16.0 | 56.9 | 34.1 | 27.6 | 21.1 | 27.6 | 11.3 | 17.6 | 25.1 | 27.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 93.99 | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.6 | — | — | — | 2.3 | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 10.8 | — | — | — | 10.1 | — | — | — | — | — | — | — |
| Spiralling loops (L) | 20 | — | — | — | 41 | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 88.2 | — | — | — | 87.7 | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 61 | — | — | — | 59 | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 0.4 | — | — | — | 0.0 | — | — | — | — | — | — | — |
| Off budget, % (N) | 9.2 | 63.7 | 36.5 | 20.2 | 1.0 | 20.2 | 436.9 | 3.4 | 0.0 | 20.3 | — | — |
| Time, seconds (T) | 3.14 | 1.83 | 0.47 | 12.81 | 36.44 | 24.51 | 106.09 | 101.48 | 106.09 | 143.25 | — | — |

## stonemill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 0/1 | 1/5 | 0/1 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2635 | 1809 | 2664 | 2538 | 2951 | 2974 | 14140 | 2859 | 14140 | 14060 | — | — |
| Irregular vertices (S) | 242 | 99 | 287 | 105 | 128 | 229 | 779 | 368 | 779 | 389 | — | — |
| … of which above the minimum | 206 | — | 247 | — | 64 | 165 | — | — | — | 325 | — | — |
| Base complex patches (C) | 2605 | — | 2586 | — | 2222 | 2872 | — | — | — | 12249 | — | — |
| Alignment, degrees (A) | 13.78 | 14.83 | 14.29 | 13.89 | 13.09 | 13.02 | 11.44 | 13.23 | 12.79 | 12.31 | — | — |
| Mean angle deviation, degrees (F) | 9.37 | 11.26 | 9.56 | 8.82 | 5.64 | 7.16 | 7.97 | 8.50 | 7.97 | 5.21 | — | — |
| Angle deviation, 95th percentile (F) | 28.53 | 33.83 | 32.90 | 24.82 | 18.13 | 24.75 | 24.83 | 29.72 | 24.83 | 16.12 | — | — |
| Mean aspect ratio (F) | 1.324 | 2.004 | 1.440 | 1.264 | 1.268 | 1.414 | 1.797 | 1.392 | 1.797 | 1.235 | — | — |
| Area variation (F) | 0.239 | 0.710 | 0.288 | 0.245 | 0.259 | 0.459 | 0.725 | 0.278 | 0.725 | 0.315 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 42.66 | 106.61 | 66.01 | 59.45 | 20.36 | 20.03 | 21.42 | 39.68 | 21.42 | 11.17 | — | — |
| Mean distance, ‰ (H) | 2.229 | 4.106 | 2.742 | 2.632 | 1.950 | 1.792 | 1.214 | 2.515 | 1.214 | 0.709 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 40.9 | 155.1 | 65.7 | 77.5 | 20.6 | 16.0 | 19.0 | 45.6 | 62.7 | 19.4 | — | — |
| Volume kept, % of the source (V) | 98.96 | — | 98.49 | — | 99.91 | 99.98 | 98.92 | — | 98.92 | 100.03 | — | — |
| Edges on closed loops, % (L) | 2.5 | — | — | — | 13.1 | 1.9 | 1.8 | — | — | 7.0 | — | — |
| Edges on spiralling loops, % (L) | 5.2 | — | — | — | 8.4 | 7.7 | 11.3 | — | — | 9.3 | — | — |
| Spiralling loops (L) | 12 | — | — | — | 15 | 12 | 37 | — | — | 17 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 92.4 | — | — | — | 78.5 | 90.4 | 86.8 | — | — | 83.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 92 | — | — | — | 170 | 105 | 189 | — | — | 281 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 12.2 | 39.7 | 11.2 | 15.4 | 1.6 | 0.9 | 371.3 | 4.7 | 0.0 | 0.6 | — | — |
| Time, seconds (T) | 2.73 | 0.95 | 0.53 | 9.68 | 3.30 | 5.31 | 37.80 | 33.37 | 37.80 | 16.82 | — | — |

## windmill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 1003 | 1886 | — | 207 | 349 | 17287 | 2787 | 17287 | 753 | — | — |
| Irregular vertices (S) | — | 145 | 312 | — | 28 | 81 | 905 | 382 | 905 | 201 | — | — |
| … of which above the minimum | — | — | — | — | 4 | 57 | — | — | — | 177 | — | — |
| Base complex patches (C) | — | — | 1778 | — | 199 | 349 | — | — | — | 745 | — | — |
| Alignment, degrees (A) | — | 14.39 | 15.52 | — | 19.17 | 18.79 | 9.49 | 11.63 | 11.73 | 17.04 | — | — |
| Mean angle deviation, degrees (F) | — | 20.17 | 15.16 | — | 29.40 | 26.04 | 8.30 | 10.62 | 8.30 | 19.15 | — | — |
| Angle deviation, 95th percentile (F) | — | 69.91 | 53.30 | — | 68.85 | 65.83 | 25.71 | 37.24 | 25.71 | 53.10 | — | — |
| Mean aspect ratio (F) | — | 2.697 | 1.760 | — | 8.010 | 2.972 | 1.847 | 1.540 | 1.847 | 3.134 | — | — |
| Area variation (F) | — | 0.807 | 0.437 | — | 1.419 | 1.252 | 0.787 | 0.335 | 0.787 | 1.275 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 193.85 | 69.08 | — | 532.08 | 530.74 | 16.02 | 36.49 | 16.02 | 528.56 | — | — |
| Mean distance, ‰ (H) | — | 18.937 | 5.814 | — | 248.299 | 236.760 | 0.917 | 2.648 | 0.917 | 216.191 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 122.8 | 42.9 | — | 1176.4 | 1162.3 | 9.2 | 17.8 | 19.1 | 2478.2 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 1.41 | 1.39 | 99.26 | — | 99.26 | 1.49 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 1.9 | 0.0 | 7.5 | — | — | 0.5 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 6.5 | 11.0 | 10.7 | — | — | 4.2 | — | — |
| Spiralling loops (L) | — | — | — | — | 5 | 8 | 46 | — | — | 4 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 91.5 | 89.0 | 81.8 | — | — | 95.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 38 | 18 | 180 | — | — | 32 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 66.6 | 37.1 | — | 93.1 | 88.4 | 476.2 | 7.1 | 0.0 | 95.6 | — | — |
| Time, seconds (T) | — | 1.35 | 0.52 | — | 0.94 | 1.61 | 171.59 | 233.11 | 171.59 | 7.40 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 2 non-manifold or inconsistently oriented edges.

## terrain (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2717 | 919 | 1753 | 2502 | 2968 | 3052 | 4677 | 2597 | 4677 | 4686 | — | — |
| Irregular vertices (S) | 112 | 45 | 115 | 55 | 56 | 99 | 231 | 167 | 231 | 141 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 826 | — | 1209 | 997 | 1681 | 2135 | — | 1662 | — | 3912 | — | — |
| Alignment, degrees (A) | 20.16 | 20.27 | 21.17 | 18.74 | 18.68 | 18.57 | 18.30 | 18.61 | 18.51 | 18.20 | — | — |
| Mean angle deviation, degrees (F) | 6.49 | 6.70 | 8.24 | 9.36 | 6.25 | 6.35 | 9.57 | 8.13 | 9.57 | 7.03 | — | — |
| Angle deviation, 95th percentile (F) | 23.46 | 24.56 | 27.33 | 27.14 | 17.87 | 19.81 | 26.86 | 23.75 | 26.86 | 19.20 | — | — |
| Mean aspect ratio (F) | 1.209 | 1.423 | 1.291 | 1.338 | 1.291 | 1.343 | 1.806 | 1.429 | 1.806 | 1.333 | — | — |
| Area variation (F) | 0.187 | 0.413 | 0.215 | 0.276 | 0.313 | 0.450 | 0.552 | 0.247 | 0.552 | 0.514 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 23.91 | 34.95 | 143.82 | 24.92 | 20.22 | 18.44 | 12.55 | 25.16 | 12.55 | 13.04 | — | — |
| Mean distance, ‰ (H) | 1.851 | 3.051 | 2.384 | 1.715 | 1.360 | 1.142 | 0.967 | 1.780 | 0.967 | 0.886 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 32.8 | 56.8 | 36.9 | 35.4 | 26.3 | 20.3 | 14.4 | 32.3 | 18.9 | 19.3 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 1.0 | 0.0 | — | — | 1.9 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 1.7 | 0.0 | — | — | 4.8 | — | — |
| Spiralling loops (L) | 0 | 0 | — | — | 0 | 1 | 0 | — | — | 4 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 53.0 | 67.6 | — | — | 74.3 | 83.0 | 84.8 | — | — | 86.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 46 | 27 | — | — | 80 | 127 | 105 | — | — | 123 | — | — |
| Edges on loops from a border to a border, % (L) | 47.0 | 32.4 | — | — | 25.7 | 14.2 | 15.2 | — | — | 7.0 | — | — |
| Off budget, % (N) | 9.4 | 69.4 | 41.6 | 16.6 | 1.1 | 1.7 | 55.9 | 13.4 | 0.0 | 0.2 | — | — |
| Time, seconds (T) | 19.92 | 1.90 | 8.92 | 14.15 | 8.60 | 8.83 | 21.17 | 19.56 | 21.17 | 11.36 | — | — |

## mahaveer (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 1325 | 1699 | 2080 | 2721 | 3236 | 46202 | 4324 | 46202 | 45336 | — | — |
| Irregular vertices (S) | — | 422 | 175 | 64 | 476 | 483 | 3076 | 1079 | 3076 | 1438 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1227 | — | — | 1130 | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 9.34 | 9.97 | 7.13 | 9.53 | 7.47 | 13.54 | 11.62 | 13.50 | 5.78 | — | — |
| Mean angle deviation, degrees (F) | — | 32.26 | 9.66 | 7.58 | 18.30 | 23.19 | 12.36 | 12.56 | 12.36 | 5.57 | — | — |
| Angle deviation, 95th percentile (F) | — | 85.28 | 34.02 | 26.60 | 83.44 | 82.43 | 39.42 | 48.70 | 39.42 | 21.80 | — | — |
| Mean aspect ratio (F) | — | 5.235 | 1.583 | 1.656 | 127.364 | 160.276 | 1.824 | 1.786 | 1.824 | 9.447 | — | — |
| Area variation (F) | — | 1.443 | 0.344 | 0.570 | 2.650 | 2.917 | 2.664 | 0.975 | 2.664 | 0.899 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 44.66 | 52.26 | 26.46 | 49.79 | 49.79 | 13.60 | 23.83 | 13.60 | 13.55 | — | — |
| Mean distance, ‰ (H) | — | 5.890 | 7.890 | 3.785 | 4.084 | 4.040 | 0.929 | 2.320 | 0.929 | 0.613 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 46.8 | 75.1 | 35.8 | 34.7 | 34.5 | 11.6 | 21.9 | 49.3 | 35.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 22.4 | 20.1 | — | — | — | 22.6 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 61.2 | 64.6 | — | — | — | 23.2 | — | — |
| Spiralling loops (L) | — | — | — | — | 556 | 591 | — | — | — | 632 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 12.4 | 12.0 | — | — | — | 43.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 7 | 66 | — | — | — | 785 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 4.0 | 3.3 | — | — | — | 10.7 | — | — |
| Off budget, % (N) | — | 55.8 | 43.4 | 30.7 | 9.3 | 7.9 | 1440.1 | 44.1 | 0.0 | 1.9 | — | — |
| Time, seconds (T) | — | 1.33 | 7.16 | 18.77 | 5.95 | 7.50 | 14.10 | 14.84 | 14.10 | 31.88 | — | — |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 870 | 1225 | — | 9101 | 8883 | 214018 | 11340 | 214018 | 201494 | — | — |
| Irregular vertices (S) | — | 858 | 616 | — | 6076 | 6004 | 10315 | 4939 | 10315 | 11571 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1136 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 18.06 | 17.78 | — | 12.38 | 12.41 | 10.18 | 13.63 | 12.25 | 8.54 | — | — |
| Mean angle deviation, degrees (F) | — | 52.53 | 23.80 | — | 21.55 | 21.11 | 9.86 | 17.03 | 9.86 | 5.14 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.72 | 60.14 | — | 68.29 | 66.71 | 29.30 | 51.38 | 29.30 | 19.26 | — | — |
| Mean aspect ratio (F) | — | 14.155 | 2.545 | — | 4.472 | 4.027 | 1.580 | 1.889 | 1.580 | 1.369 | — | — |
| Area variation (F) | — | 7.748 | 0.789 | — | 3.479 | 3.450 | 2.386 | 2.005 | 2.386 | 0.561 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 9.99 | 7.35 | — | 7.06 | 6.26 | 1.80 | 3.03 | 1.80 | 3.00 | — | — |
| Mean distance, ‰ (H) | — | 2.873 | 1.806 | — | 0.446 | 0.457 | 0.123 | 0.419 | 0.123 | 0.037 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 87.1 | 57.0 | — | 12.6 | 12.9 | 7.6 | 14.6 | 59.1 | 18.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | — | — | — | 34.7 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | — | — | — | 15.4 | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | — | — | — | 1430 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | — | — | — | 43.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | — | — | — | 355 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | — | — | — | 6.4 | — | — |
| Off budget, % (N) | — | 71.0 | 59.2 | — | 203.4 | 196.1 | 7033.9 | 278.0 | 0.0 | 5.9 | — | — |
| Time, seconds (T) | — | 0.99 | 4.00 | — | 13.23 | 16.40 | 12.21 | 10.92 | 12.21 | 44.86 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 8 non-manifold or inconsistently oriented edges.

## f16 (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 2/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 4122 | 1500 | 1333 | 4417 | 4320 | 36038 | 6315 | 36038 | 33553 | — | — |
| Irregular vertices (S) | — | 2217 | 403 | 242 | 1884 | 1782 | 3062 | 1343 | 3062 | 1970 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1329 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 14.85 | 12.61 | 8.71 | 6.14 | 6.22 | 5.01 | 7.09 | 4.91 | 3.96 | — | — |
| Mean angle deviation, degrees (F) | — | 41.32 | 19.76 | 15.35 | 16.88 | 17.31 | 10.99 | 14.23 | 10.99 | 7.06 | — | — |
| Angle deviation, 95th percentile (F) | — | 87.07 | 64.46 | 49.46 | 66.97 | 68.60 | 41.48 | 46.16 | 41.48 | 29.87 | — | — |
| Mean aspect ratio (F) | — | 7.957 | 2.173 | 1.844 | 106.037 | 125.642 | 2.408 | 3.370 | 2.408 | 15.347 | — | — |
| Area variation (F) | — | 2.547 | 0.539 | 0.650 | 2.392 | 2.279 | 1.937 | 1.320 | 1.937 | 0.957 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 87.83 | 95.94 | 63.38 | 38.88 | 61.00 | 8.24 | 47.42 | 8.24 | 80.32 | — | — |
| Mean distance, ‰ (H) | — | 9.067 | 20.293 | 13.479 | 1.433 | 1.840 | 0.363 | 2.336 | 0.363 | 0.868 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 62.6 | 143.8 | 98.2 | 9.2 | 11.1 | 3.4 | 18.2 | 11.6 | 8.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 37.4 | 50.0 | 55.6 | 47.2 | 44.0 | 1101.3 | 110.5 | 0.0 | 6.9 | — | — |
| Time, seconds (T) | — | 2.15 | 5.15 | 88.05 | 11.81 | 12.63 | 9.36 | 9.51 | 9.36 | 123.64 | — | — |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2222 | 2216 | 1109 | 3105 | 3427 | 12621 | 5458 | 12621 | 12542 | — | — |
| Irregular vertices (S) | — | 338 | 265 | 70 | 179 | 210 | 443 | 322 | 443 | 300 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1675 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 7.28 | 8.44 | 8.56 | 5.23 | 5.44 | 4.33 | 3.81 | 3.56 | 3.36 | — | — |
| Mean angle deviation, degrees (F) | — | 22.14 | 11.96 | 12.59 | 17.03 | 20.66 | 7.26 | 8.23 | 7.26 | 10.33 | — | — |
| Angle deviation, 95th percentile (F) | — | 84.37 | 45.38 | 46.79 | 72.03 | 73.58 | 26.15 | 33.02 | 26.15 | 52.59 | — | — |
| Mean aspect ratio (F) | — | 4.205 | 1.771 | 2.901 | 15.611 | 26.877 | 8.875 | 18.163 | 8.875 | 16.946 | — | — |
| Area variation (F) | — | 2.439 | 0.430 | 0.902 | 1.298 | 1.418 | 1.511 | 0.973 | 1.511 | 0.814 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 50.21 | 35.63 | 45.56 | 26.86 | 26.86 | 13.66 | 28.78 | 13.66 | 19.79 | — | — |
| Mean distance, ‰ (H) | — | 5.701 | 4.253 | 5.359 | 2.526 | 2.320 | 0.560 | 1.134 | 0.560 | 0.633 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 25.0 | 19.2 | 22.4 | 11.8 | 11.6 | 2.4 | 5.0 | 4.6 | 11.7 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 55.6 | 50.2 | — | — | — | 48.4 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 8.7 | 9.0 | — | — | — | 7.6 | — | — |
| Spiralling loops (L) | — | — | — | — | 92 | 95 | — | — | — | 71 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 23.6 | 29.8 | — | — | — | 21.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 50 | 52 | — | — | — | 158 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 12.0 | 11.0 | — | — | — | 22.3 | — | — |
| Off budget, % (N) | — | 25.9 | 26.1 | 63.0 | 3.5 | 14.2 | 320.7 | 81.9 | 0.0 | 0.6 | — | — |
| Time, seconds (T) | — | 0.92 | 1.12 | 70.22 | 10.37 | 3.20 | 5.03 | 4.34 | 5.03 | 8.57 | — | — |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 625 | 2774 | 2420 | 2419 | 2466 | 14984 | 2308 | 14984 | 13011 | — | — |
| Irregular vertices (S) | — | 125 | 430 | 185 | 230 | 225 | 775 | 331 | 775 | 612 | — | — |
| … of which above the minimum | — | — | 387 | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 2517 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 14.57 | 14.83 | 10.13 | 10.60 | 11.08 | 9.19 | 9.89 | 10.20 | 10.66 | — | — |
| Mean angle deviation, degrees (F) | — | 16.08 | 11.44 | 9.70 | 10.49 | 10.98 | 9.40 | 11.17 | 9.40 | 9.79 | — | — |
| Angle deviation, 95th percentile (F) | — | 44.86 | 37.21 | 27.88 | 30.89 | 33.10 | 27.92 | 33.61 | 27.92 | 28.34 | — | — |
| Mean aspect ratio (F) | — | 2.516 | 1.550 | 1.474 | 1.694 | 1.692 | 1.735 | 1.574 | 1.735 | 1.420 | — | — |
| Area variation (F) | — | 0.933 | 0.323 | 0.335 | 0.350 | 0.371 | 0.938 | 0.374 | 0.938 | 0.330 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 89.72 | 44.20 | 33.89 | 37.13 | 37.13 | 21.30 | 37.66 | 21.30 | 10.23 | — | — |
| Mean distance, ‰ (H) | — | 9.165 | 3.465 | 2.989 | 2.505 | 2.414 | 0.702 | 3.072 | 0.702 | 0.381 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 125.4 | 88.0 | 69.3 | 56.5 | 54.2 | 17.5 | 65.3 | 44.1 | 36.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 17.4 | 16.9 | 15.8 | — | — | 10.0 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 12.3 | 12.9 | 7.2 | — | — | 25.1 | — | — |
| Spiralling loops (L) | — | — | — | — | 48 | 52 | 70 | — | — | 104 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 70.3 | 70.2 | 76.9 | — | — | 64.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 82 | 82 | 170 | — | — | 192 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 79.2 | 7.5 | 19.3 | 19.4 | 17.8 | 399.5 | 23.1 | 0.0 | 13.2 | — | — |
| Time, seconds (T) | — | 0.74 | 0.34 | 22.46 | 25.36 | 30.27 | 26.98 | 30.23 | 26.98 | 507.50 | — | — |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3107 | 2086 | 3090 | 3000 | 3000 | 3000 | 2740 | 2740 | 2740 | 2706 | — | — |
| Irregular vertices (S) | 8 | 10 | 97 | 32 | 32 | 32 | 10 | 10 | 10 | 32 | — | — |
| … of which above the minimum | 0 | 2 | 89 | 24 | 24 | 24 | 2 | 2 | 2 | 24 | — | — |
| Base complex patches (C) | 2730 | 2004 | 3090 | 30 | 30 | 30 | 1794 | 1794 | 1794 | 30 | — | — |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.49 | 7.65 | 4.19 | 8.38 | 3.97 | 3.97 | 6.09 | 6.09 | 6.09 | 3.87 | — | — |
| Angle deviation, 95th percentile (F) | 12.67 | 19.88 | 14.49 | 17.72 | 7.49 | 7.49 | 15.49 | 15.49 | 15.49 | 7.86 | — | — |
| Mean aspect ratio (F) | 1.125 | 1.326 | 1.124 | 1.081 | 1.083 | 1.083 | 1.185 | 1.185 | 1.185 | 1.118 | — | — |
| Area variation (F) | 0.135 | 0.341 | 0.136 | 0.190 | 0.302 | 0.302 | 0.312 | 0.312 | 0.312 | 0.309 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.63 | 1.54 | 0.72 | 0.42 | 0.49 | 0.49 | 0.82 | 0.82 | 0.82 | 0.63 | — | — |
| Mean distance, ‰ (H) | 0.216 | 0.408 | 0.221 | 0.168 | 0.161 | 0.161 | 0.258 | 0.258 | 0.258 | 0.178 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.81 | 99.59 | 99.79 | 100.01 | 100.01 | 100.01 | 99.74 | 99.74 | 99.74 | 100.01 | — | — |
| Edges on closed loops, % (L) | 6.1 | 2.0 | — | — | 90.0 | 90.0 | 17.3 | — | — | 89.5 | — | — |
| Edges on spiralling loops, % (L) | 70.1 | 74.4 | — | — | 0.0 | 0.0 | 69.2 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 5 | 6 | — | — | 0 | 0 | 6 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 15.4 | 23.6 | — | — | 10.0 | 10.0 | 13.5 | — | — | 10.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 276 | 298 | — | — | 10 | 10 | 158 | — | — | 10 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.6 | 30.5 | 3.0 | 0.0 | 0.0 | 0.0 | 8.7 | 8.7 | 0.0 | 1.2 | — | — |
| Time, seconds (T) | 1.81 | 0.40 | 0.34 | 1.94 | 1.44 | 1.40 | 3.56 | 3.39 | 3.56 | 1.57 | — | — |

## torus (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2976 | 2672 | 3078 | 2720 | 2952 | 2952 | 2824 | 2824 | 2824 | 2800 | — | — |
| Irregular vertices (S) | 30 | 0 | 125 | 0 | 0 | 0 | 10 | 10 | 10 | 0 | — | — |
| … of which above the minimum | 30 | 0 | 125 | 0 | 0 | 0 | 10 | 10 | 10 | 0 | — | — |
| Base complex patches (C) | 2976 | 1 | 3078 | 1 | 1 | 1 | 2264 | 2264 | 2264 | 1 | — | — |
| Alignment, degrees (A) | 10.14 | 10.37 | 14.28 | 0.65 | 0.62 | 0.62 | 6.90 | 6.90 | 6.90 | 0.62 | — | — |
| Mean angle deviation, degrees (F) | 10.25 | 4.17 | 5.57 | 1.45 | 1.44 | 1.44 | 5.23 | 5.23 | 5.23 | 1.48 | — | — |
| Angle deviation, 95th percentile (F) | 26.29 | 11.07 | 18.30 | 2.89 | 2.36 | 2.36 | 14.40 | 14.40 | 14.40 | 2.40 | — | — |
| Mean aspect ratio (F) | 1.239 | 2.545 | 1.163 | 1.351 | 1.322 | 1.322 | 1.213 | 1.213 | 1.213 | 1.320 | — | — |
| Area variation (F) | 0.223 | 0.256 | 0.163 | 0.281 | 0.308 | 0.308 | 0.233 | 0.233 | 0.233 | 0.310 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 1.86 | 1.50 | 1.38 | 0.59 | 0.59 | 0.59 | 1.71 | 1.71 | 1.71 | 0.62 | — | — |
| Mean distance, ‰ (H) | 0.299 | 0.465 | 0.298 | 0.208 | 0.191 | 0.191 | 0.518 | 0.518 | 0.518 | 0.202 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.46 | 99.09 | 99.44 | 100.02 | 100.02 | 100.02 | 98.94 | 98.94 | 98.94 | 100.02 | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 100.0 | 100.0 | 9.9 | — | — | 100.0 | — | — |
| Edges on spiralling loops, % (L) | 43.5 | 100.0 | — | — | 0.0 | 0.0 | 80.9 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 7 | 6 | — | — | 0 | 0 | 10 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 56.5 | 0.0 | — | — | 0.0 | 0.0 | 9.2 | — | — | 0.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 411 | 0 | — | — | 0 | 0 | 108 | — | — | 0 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 0.8 | 10.9 | 2.6 | 9.3 | 1.6 | 1.6 | 5.9 | 5.9 | 0.0 | 0.8 | — | — |
| Time, seconds (T) | 1.56 | 0.53 | 0.22 | 2.01 | 1.91 | 1.76 | 4.71 | 2.89 | 4.71 | 3.07 | — | — |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (15 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 6 | 0 | 5 | 4 | 2 | 0 | 1 | 0 | 1 | 0 | 0 |
| Base complex patches (C) | 1 | 3 | 0 | 2 | 3 | 4 | 0 | 1 | 0 | 1 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 0 | 1 | 13 | 0 | 1 | 0 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 1 | 0 | 2 | 5 | 0 | 1 | 0 | 4 | 0 | 2 |
| Mean aspect ratio (F) | 1 | 0 | 10 | 0 | 1 | 0 | 0 | 0 | 0 | 2 | 0 | 1 |
| Valid meshes (V) | 36/75 | 4/15 | 48/75 | 10/15 | 14/15 | 14/15 | 8/15 | 10/15 | 8/15 | 14/15 | 0/2 | 2/2 |

### mechanical (4 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 3 | 0 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 0 | 0 |
| Base complex patches (C) | 1 | 1 | 0 | 2 | 3 | 3 | 2 | 2 | 2 | 2 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 1 | 1 | 2 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 0 | 2 | 0 | 1 | 1 | 1 | 1 | 0 | 0 |
| Mean aspect ratio (F) | 3 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Valid meshes (V) | 18/20 | 3/4 | 20/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 0/0 | 0/0 |
