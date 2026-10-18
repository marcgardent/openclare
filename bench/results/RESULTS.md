# Bench results

Requested budget: 3000 quads. Median over 5 seeds; AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

## blob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2781 | 1165 | 3090 | 2507 | 2976 | 3047 | — | — |
| Irregular vertices (S) | 38 | 26 | 143 | 26 | 24 | 24 | — | — |
| … of which above the minimum | 30 | — | 135 | 18 | 16 | 16 | — | — |
| Base complex patches (C) | 2697 | — | 3048 | 951 | 1084 | 849 | — | — |
| Alignment, degrees (A) | 17.48 | 6.27 | 15.74 | 6.23 | 5.37 | 4.86 | — | — |
| Mean angle deviation, degrees (F) | 8.71 | 9.45 | 5.87 | 7.89 | 6.68 | 6.15 | — | — |
| Angle deviation, 95th percentile (F) | 27.30 | 30.58 | 18.26 | 21.16 | 20.94 | 16.71 | — | — |
| Mean aspect ratio (F) | 1.187 | 1.934 | 1.174 | 1.427 | 1.448 | 1.421 | — | — |
| Area variation (F) | 0.205 | 0.575 | 0.175 | 0.351 | 0.433 | 0.460 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 8.52 | 6.35 | 10.14 | 5.25 | 3.26 | 2.61 | — | — |
| Mean distance, ‰ (H) | 0.486 | 1.885 | 0.459 | 0.272 | 0.198 | 0.188 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.56 | 96.90 | 99.63 | 100.03 | 100.02 | 100.02 | — | — |
| Edges on closed loops, % (L) | 0.0 | 17.2 | — | — | 31.8 | 40.3 | — | — |
| Edges on spiralling loops, % (L) | 14.7 | 31.9 | — | — | 19.6 | 11.6 | — | — |
| Spiralling loops (L) | 3 | 7 | — | — | 4 | 2 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.3 | 50.9 | — | — | 48.6 | 48.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 321 | 120 | — | — | 382 | 324 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 7.3 | 61.2 | 3.0 | 16.4 | 0.8 | 1.6 | — | — |
| Time, seconds (T) | 2.17 | 0.74 | 1.71 | 4.09 | 3.47 | 4.03 | — | — |

## rock (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3036 | 2412 | 2898 | 2687 | 3039 | 3010 | — | — |
| Irregular vertices (S) | 8 | 20 | 91 | 50 | 54 | 64 | — | — |
| … of which above the minimum | 0 | 12 | 83 | 42 | 46 | 56 | — | — |
| Base complex patches (C) | 2855 | 2412 | 2896 | 2687 | 3039 | 3010 | — | — |
| Alignment, degrees (A) | 22.96 | 18.78 | 23.21 | 15.81 | 16.23 | 15.75 | — | — |
| Mean angle deviation, degrees (F) | 5.84 | 7.48 | 5.18 | 8.82 | 5.35 | 5.10 | — | — |
| Angle deviation, 95th percentile (F) | 16.05 | 17.91 | 14.78 | 22.11 | 13.54 | 13.37 | — | — |
| Mean aspect ratio (F) | 1.164 | 1.409 | 1.146 | 1.253 | 1.211 | 1.203 | — | — |
| Area variation (F) | 0.202 | 0.331 | 0.151 | 0.229 | 0.270 | 0.282 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.45 | 7.64 | 5.28 | 4.49 | 4.50 | 4.53 | — | — |
| Mean distance, ‰ (H) | 0.788 | 1.157 | 0.820 | 0.740 | 0.694 | 0.696 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.78 | 99.49 | 99.70 | 100.01 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 56.9 | 27.1 | — | — | 11.0 | 9.3 | — | — |
| Spiralling loops (L) | 5 | 4 | — | — | 3 | 3 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 43.1 | 72.9 | — | — | 89.0 | 90.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 528 | 317 | — | — | 238 | 207 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 2.2 | 19.6 | 3.4 | 10.4 | 1.3 | 0.3 | — | — |
| Time, seconds (T) | 1.22 | 1.86 | 1.03 | 6.92 | 5.08 | 6.12 | — | — |

## creature (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2667 | 1991 | 3070 | 2597 | 2946 | 3041 | — | — |
| Irregular vertices (S) | 24 | 20 | 111 | 41 | 36 | 40 | — | — |
| … of which above the minimum | 16 | 12 | 103 | 33 | 28 | 32 | — | — |
| Base complex patches (C) | 2667 | 747 | 2856 | 913 | 1124 | 889 | — | — |
| Alignment, degrees (A) | 12.89 | 9.58 | 12.09 | 6.96 | 7.41 | 5.65 | — | — |
| Mean angle deviation, degrees (F) | 6.85 | 4.75 | 4.65 | 8.21 | 5.63 | 5.08 | — | — |
| Angle deviation, 95th percentile (F) | 18.98 | 13.44 | 14.73 | 24.05 | 16.63 | 14.22 | — | — |
| Mean aspect ratio (F) | 1.189 | 1.839 | 1.144 | 1.305 | 1.272 | 1.309 | — | — |
| Area variation (F) | 0.237 | 0.549 | 0.146 | 0.286 | 0.378 | 0.460 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 8.01 | 2.39 | 3.47 | 2.16 | 1.88 | 2.12 | — | — |
| Mean distance, ‰ (H) | 0.320 | 0.537 | 0.279 | 0.231 | 0.193 | 0.196 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.41 | 98.97 | 99.47 | 100.00 | 100.02 | 100.03 | — | — |
| Edges on closed loops, % (L) | 0.0 | 31.2 | — | — | 31.7 | 38.4 | — | — |
| Edges on spiralling loops, % (L) | 30.9 | 33.6 | — | — | 4.8 | 3.2 | — | — |
| Spiralling loops (L) | 4 | 6 | — | — | 2 | 2 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 69.1 | 35.2 | — | — | 63.5 | 58.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 377 | 166 | — | — | 318 | 275 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 11.1 | 33.6 | 2.3 | 13.4 | 1.8 | 1.4 | — | — |
| Time, seconds (T) | 1.35 | 1.49 | 0.93 | 6.08 | 3.80 | 3.75 | — | — |

## cage (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2891 | 4039 | 2944 | 2874 | 2981 | 3021 | — | — |
| Irregular vertices (S) | 66 | 97 | 200 | 41 | 48 | 48 | — | — |
| … of which above the minimum | 34 | — | 168 | 9 | 16 | 16 | — | — |
| Base complex patches (C) | 1943 | — | 2319 | 427 | 225 | 571 | — | — |
| Alignment, degrees (A) | 8.10 | 8.92 | 7.97 | 3.38 | 3.12 | 2.45 | — | — |
| Mean angle deviation, degrees (F) | 5.86 | 9.02 | 6.39 | 4.00 | 2.77 | 3.44 | — | — |
| Angle deviation, 95th percentile (F) | 17.11 | 28.25 | 23.33 | 12.37 | 8.90 | 10.56 | — | — |
| Mean aspect ratio (F) | 1.183 | 2.030 | 1.252 | 1.202 | 1.175 | 1.240 | — | — |
| Area variation (F) | 0.147 | 0.496 | 0.203 | 0.193 | 0.200 | 0.332 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 11.45 | 13.32 | 14.46 | 11.74 | 11.02 | 10.21 | — | — |
| Mean distance, ‰ (H) | 0.734 | 0.800 | 0.860 | 0.814 | 0.795 | 0.535 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.99 | 99.63 | 98.70 | 99.89 | 99.93 | 99.97 | — | — |
| Edges on closed loops, % (L) | 18.2 | 4.2 | — | — | 71.2 | 54.5 | — | — |
| Edges on spiralling loops, % (L) | 38.5 | 29.9 | — | — | 9.2 | 22.2 | — | — |
| Spiralling loops (L) | 20 | 24 | — | — | 3 | 6 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 41.8 | 65.8 | — | — | 19.6 | 23.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 86 | 110 | — | — | 40 | 69 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.6 | 34.6 | 1.9 | 4.2 | 0.6 | 0.7 | — | — |
| Time, seconds (T) | 1.26 | 2.38 | 0.34 | 8.42 | 2.38 | 3.01 | — | — |

## suzanne (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 2/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3030 | 1255 | 3070 | 2757 | 2982 | 2987 | — | — |
| Irregular vertices (S) | 78 | 69 | 212 | 52 | 49 | 54 | — | — |
| … of which above the minimum | 54 | — | — | — | — | — | — | — |
| Base complex patches (C) | 2884 | — | 3002 | 1858 | 1453 | 1507 | — | — |
| Alignment, degrees (A) | 15.33 | 13.19 | 17.02 | 10.46 | 10.48 | 9.97 | — | — |
| Mean angle deviation, degrees (F) | 8.61 | 11.13 | 7.08 | 7.14 | 4.41 | 4.34 | — | — |
| Angle deviation, 95th percentile (F) | 26.08 | 41.63 | 25.30 | 18.16 | 11.22 | 11.59 | — | — |
| Mean aspect ratio (F) | 1.260 | 1.871 | 1.263 | 1.221 | 1.178 | 1.193 | — | — |
| Area variation (F) | 0.837 | 0.611 | 0.221 | 0.250 | 0.280 | 0.372 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 42.26 | 33.04 | 20.15 | 8.61 | 7.95 | 7.71 | — | — |
| Mean distance, ‰ (H) | 0.834 | 2.167 | 0.906 | 0.738 | 0.597 | 0.473 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 16.4 | 28.9 | 21.4 | 16.0 | 9.7 | 6.4 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.9 | 8.6 | — | — | 19.3 | 19.2 | — | — |
| Edges on spiralling loops, % (L) | 15.3 | 14.6 | — | — | 4.1 | 10.2 | — | — |
| Spiralling loops (L) | 12 | 8 | — | — | 1 | 3 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 80.2 | 76.8 | — | — | 65.7 | 60.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 233 | 82 | — | — | 203 | 170 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 11.0 | 9.9 | — | — |
| Off budget, % (N) | 1.0 | 58.2 | 2.3 | 8.1 | 0.6 | 0.4 | — | — |
| Time, seconds (T) | 3.31 | 0.83 | 0.28 | 8.24 | 3.93 | 5.71 | — | — |

## ogre (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 0/1 | 4/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2935 | 1081 | 2874 | 2465 | 3053 | 2965 | — | — |
| Irregular vertices (S) | 106 | 84 | 216 | 77 | 78 | 159 | — | — |
| … of which above the minimum | 98 | — | 202 | — | 70 | 151 | — | — |
| Base complex patches (C) | 2857 | — | 2802 | — | 1653 | 2021 | — | — |
| Alignment, degrees (A) | 19.31 | 16.24 | 18.57 | 12.24 | 12.47 | 11.66 | — | — |
| Mean angle deviation, degrees (F) | 12.01 | 10.37 | 7.49 | 9.03 | 7.06 | 7.20 | — | — |
| Angle deviation, 95th percentile (F) | 32.96 | 32.56 | 24.38 | 23.88 | 19.74 | 19.95 | — | — |
| Mean aspect ratio (F) | 1.403 | 1.826 | 1.283 | 1.546 | 1.553 | 1.451 | — | — |
| Area variation (F) | 0.324 | 0.590 | 0.224 | 0.389 | 0.428 | 0.476 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 36.59 | 31.05 | 46.14 | 35.86 | 27.51 | 26.69 | — | — |
| Mean distance, ‰ (H) | 1.268 | 3.042 | 1.460 | 1.109 | 0.774 | 0.730 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 54.0 | 66.6 | 99.5 | 66.0 | 25.1 | 8.4 | — | — |
| Volume kept, % of the source (V) | 98.56 | — | 98.55 | 99.78 | 99.96 | 100.02 | — | — |
| Edges on closed loops, % (L) | 1.7 | — | — | — | 22.9 | 15.9 | — | — |
| Edges on spiralling loops, % (L) | 9.6 | — | — | — | 8.7 | 7.6 | — | — |
| Spiralling loops (L) | 14 | — | — | — | 5 | 19 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 88.6 | — | — | — | 68.4 | 76.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 250 | — | — | — | 203 | 135 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 4.3 | 64.0 | 4.2 | 17.8 | 1.8 | 1.2 | — | — |
| Time, seconds (T) | 3.31 | 1.22 | 2.70 | 8.13 | 9.43 | 12.51 | — | — |

## human_male (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 854 | 2814 | 2710 | 3013 | 2979 | 10590 | 9426 |
| Irregular vertices (S) | — | 88 | 194 | 68 | 98 | 158 | 476 | 141 |
| … of which above the minimum | — | — | 178 | — | 90 | 150 | — | 133 |
| Base complex patches (C) | — | — | 2014 | — | 1005 | 1159 | — | 5382 |
| Alignment, degrees (A) | — | 9.53 | 10.54 | 7.00 | 7.96 | 7.82 | 9.88 | 8.45 |
| Mean angle deviation, degrees (F) | — | 10.46 | 7.19 | 6.01 | 5.54 | 6.04 | 9.24 | 6.33 |
| Angle deviation, 95th percentile (F) | — | 37.89 | 25.75 | 21.11 | 19.18 | 20.29 | 26.27 | 18.39 |
| Mean aspect ratio (F) | — | 2.541 | 1.301 | 1.355 | 1.583 | 1.400 | 1.616 | 1.333 |
| Area variation (F) | — | 0.757 | 0.248 | 0.337 | 0.425 | 0.648 | 1.417 | 0.294 |
| Hausdorff, ‰ of the diagonal (H) | — | 31.67 | 47.23 | 38.14 | 15.53 | 12.14 | 6.47 | 8.89 |
| Mean distance, ‰ (H) | — | 5.764 | 4.380 | 2.749 | 1.425 | 0.908 | 0.172 | 0.557 |
| Thin parts: mean distance, % of a quad edge (V) | — | 79.0 | 84.2 | 57.3 | 22.0 | 11.3 | 1.5 | 25.0 |
| Volume kept, % of the source (V) | — | — | 97.16 | 99.70 | 99.91 | 99.97 | 98.80 | 99.93 |
| Edges on closed loops, % (L) | — | — | 14.8 | 35.4 | 33.3 | 30.5 | 31.6 | 22.1 |
| Edges on spiralling loops, % (L) | — | — | 7.8 | 4.4 | 6.9 | 4.6 | 2.5 | 32.6 |
| Spiralling loops (L) | — | — | 8 | 8 | 13 | 8 | 3 | 44 |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 76.8 | 60.3 | 59.8 | 64.8 | 65.8 | 45.2 |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 86 | 242 | 314 | 121 | 274 | 467 |
| Edges on loops from a border to a border, % (L) | — | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 71.5 | 6.2 | 9.7 | 0.4 | 0.7 | 0.0 | 11.0 |
| Time, seconds (T) | — | 4.48 | 22.16 | 18.09 | 16.34 | 16.96 | — | 75.16 |

Failures: quadriflow: cancelled.

## human_female (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 689 | 2837 | 2766 | 2981 | 3040 | 10590 | 10602 |
| Irregular vertices (S) | — | 77 | 204 | 92 | 110 | 172 | 476 | 280 |
| … of which above the minimum | — | — | 196 | — | 102 | 164 | — | 272 |
| Base complex patches (C) | — | — | 2366 | — | 1399 | 1282 | — | 3904 |
| Alignment, degrees (A) | — | 11.80 | 10.59 | 6.68 | 7.37 | 7.80 | 9.43 | 7.20 |
| Mean angle deviation, degrees (F) | — | 14.87 | 7.68 | 6.60 | 4.75 | 5.31 | 8.82 | 4.44 |
| Angle deviation, 95th percentile (F) | — | 40.05 | 25.19 | 21.54 | 16.79 | 18.78 | 25.47 | 14.29 |
| Mean aspect ratio (F) | — | 2.191 | 1.291 | 1.348 | 1.473 | 1.389 | 1.657 | 1.281 |
| Area variation (F) | — | 0.762 | 0.239 | 0.327 | 0.408 | 0.643 | 1.432 | 0.482 |
| Hausdorff, ‰ of the diagonal (H) | — | 40.87 | 56.57 | 39.14 | 17.10 | 13.94 | 4.44 | 4.92 |
| Mean distance, ‰ (H) | — | 6.121 | 4.562 | 2.685 | 1.416 | 1.074 | 0.156 | 0.181 |
| Thin parts: mean distance, % of a quad edge (V) | — | 87.6 | 97.9 | 57.7 | 27.1 | 17.6 | 1.4 | 6.1 |
| Volume kept, % of the source (V) | — | 85.52 | 96.97 | 99.64 | 99.90 | 100.01 | 98.78 | 100.02 |
| Edges on closed loops, % (L) | — | 11.9 | 8.2 | 24.7 | 26.7 | 28.9 | 31.6 | 31.6 |
| Edges on spiralling loops, % (L) | — | 20.2 | 9.7 | 17.5 | 7.5 | 4.4 | 2.5 | 3.5 |
| Spiralling loops (L) | — | 18 | 18 | 15 | 12 | 15 | 3 | 8 |
| Edges on loops that stop at an irregular vertex, % (L) | — | 67.9 | 80.5 | 57.8 | 65.8 | 66.7 | 65.8 | 64.9 |
| Longest loop that stops at an irregular vertex, edges (L) | — | 46 | 108 | 188 | 223 | 187 | 274 | 325 |
| Edges on loops from a border to a border, % (L) | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 77.0 | 5.4 | 7.8 | 0.6 | 1.3 | 0.0 | 0.1 |
| Time, seconds (T) | — | 4.77 | 20.88 | 17.37 | 17.65 | 17.88 | — | 33.99 |

Failures: quadriflow: cancelled.

## spot (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2964 | 1284 | 3044 | 2659 | 2976 | 3008 | — | — |
| Irregular vertices (S) | 68 | 68 | 153 | 70 | 70 | 70 | — | — |
| … of which above the minimum | 60 | 60 | 145 | 62 | 62 | 62 | — | — |
| Base complex patches (C) | 2902 | 1022 | 2978 | 1987 | 1280 | 1210 | — | — |
| Alignment, degrees (A) | 18.12 | 12.08 | 17.17 | 9.37 | 9.05 | 8.13 | — | — |
| Mean angle deviation, degrees (F) | 7.25 | 8.87 | 5.59 | 9.51 | 6.48 | 5.46 | — | — |
| Angle deviation, 95th percentile (F) | 20.42 | 24.77 | 18.71 | 24.15 | 18.01 | 16.01 | — | — |
| Mean aspect ratio (F) | 1.194 | 1.451 | 1.179 | 1.344 | 1.367 | 1.358 | — | — |
| Area variation (F) | 0.177 | 0.666 | 0.172 | 0.289 | 0.353 | 0.439 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 17.71 | 17.26 | 19.38 | 8.80 | 4.21 | 3.61 | — | — |
| Mean distance, ‰ (H) | 0.448 | 1.588 | 0.479 | 0.237 | 0.204 | 0.200 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 56.3 | 54.6 | 53.8 | 14.8 | 4.3 | 3.9 | — | — |
| Volume kept, % of the source (V) | 99.37 | 97.44 | 99.35 | 100.00 | 100.03 | 100.02 | — | — |
| Edges on closed loops, % (L) | 0.0 | 10.2 | — | — | 30.2 | 32.3 | — | — |
| Edges on spiralling loops, % (L) | 22.5 | 5.6 | — | — | 2.3 | 0.0 | — | — |
| Spiralling loops (L) | 10 | 3 | — | — | 1 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 77.5 | 84.2 | — | — | 67.5 | 67.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 299 | 90 | — | — | 182 | 239 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 1.2 | 57.2 | 1.5 | 11.4 | 0.8 | 0.3 | — | — |
| Time, seconds (T) | 1.80 | 0.94 | 1.23 | 6.69 | 4.20 | 5.00 | — | — |

## bob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2928 | 1583 | 3040 | 2642 | 2967 | 2968 | — | — |
| Irregular vertices (S) | 32 | 18 | 130 | 20 | 20 | 20 | — | — |
| … of which above the minimum | 32 | 18 | 130 | 20 | 20 | 20 | — | — |
| Base complex patches (C) | 2903 | 533 | 3040 | 714 | 219 | 231 | — | — |
| Alignment, degrees (A) | 19.37 | 6.64 | 20.80 | 4.37 | 3.97 | 3.88 | — | — |
| Mean angle deviation, degrees (F) | 6.41 | 7.07 | 5.33 | 4.82 | 3.39 | 3.48 | — | — |
| Angle deviation, 95th percentile (F) | 18.61 | 21.04 | 17.96 | 14.72 | 9.41 | 10.08 | — | — |
| Mean aspect ratio (F) | 1.175 | 2.090 | 1.158 | 1.409 | 1.362 | 1.345 | — | — |
| Area variation (F) | 0.159 | 0.474 | 0.161 | 0.339 | 0.401 | 0.438 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.82 | 4.03 | 7.32 | 2.49 | 2.24 | 1.62 | — | — |
| Mean distance, ‰ (H) | 0.309 | 0.960 | 0.325 | 0.152 | 0.136 | 0.137 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.48 | 98.10 | 99.44 | 100.01 | 100.02 | 100.02 | — | — |
| Edges on closed loops, % (L) | 0.6 | 45.2 | — | — | 75.3 | 74.2 | — | — |
| Edges on spiralling loops, % (L) | 35.4 | 11.1 | — | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 9 | 3 | — | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 64.6 | 43.7 | — | — | 24.7 | 25.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 309 | 172 | — | — | 112 | 122 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 2.4 | 47.2 | 1.3 | 11.9 | 1.1 | 1.1 | — | — |
| Time, seconds (T) | 1.72 | 1.14 | 1.45 | 6.22 | 2.49 | 3.72 | — | — |

## blub (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2698 | 1343 | 3036 | 2680 | 2963 | 2981 | — | — |
| Irregular vertices (S) | 52 | 47 | 158 | 43 | 46 | 56 | — | — |
| … of which above the minimum | 44 | — | 150 | 35 | 38 | 48 | — | — |
| Base complex patches (C) | 2584 | — | 2922 | 902 | 699 | 1127 | — | — |
| Alignment, degrees (A) | 15.95 | 10.62 | 15.45 | 7.64 | 8.00 | 7.52 | — | — |
| Mean angle deviation, degrees (F) | 10.72 | 9.14 | 5.55 | 8.08 | 5.56 | 5.04 | — | — |
| Angle deviation, 95th percentile (F) | 28.70 | 28.42 | 19.72 | 22.68 | 18.27 | 15.06 | — | — |
| Mean aspect ratio (F) | 1.271 | 2.046 | 1.192 | 1.327 | 1.312 | 1.294 | — | — |
| Area variation (F) | 0.261 | 0.522 | 0.178 | 0.266 | 0.297 | 0.405 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 40.70 | 17.71 | 15.68 | 12.42 | 12.15 | 9.51 | — | — |
| Mean distance, ‰ (H) | 0.588 | 1.082 | 0.390 | 0.254 | 0.226 | 0.171 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 46.5 | 17.7 | 24.8 | 19.4 | 16.1 | 9.6 | — | — |
| Volume kept, % of the source (V) | 98.93 | — | 99.17 | 99.88 | 99.91 | 99.98 | — | — |
| Edges on closed loops, % (L) | 0.0 | — | — | — | 46.3 | 34.6 | — | — |
| Edges on spiralling loops, % (L) | 32.5 | — | — | — | 10.4 | 11.4 | — | — |
| Spiralling loops (L) | 13 | — | — | — | 3 | 4 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 64.2 | — | — | — | 43.2 | 54.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 225 | — | — | — | 193 | 164 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 10.1 | 55.2 | 1.2 | 10.7 | 1.2 | 0.6 | — | — |
| Time, seconds (T) | 1.68 | 1.23 | 1.25 | 6.03 | 5.91 | 4.83 | — | — |

## pig (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2675 | 1808 | 3016 | 2680 | 2990 | 3027 | — | — |
| Irregular vertices (S) | 122 | 99 | 209 | 82 | 78 | 104 | — | — |
| … of which above the minimum | 114 | — | 192 | — | 70 | 96 | — | — |
| Base complex patches (C) | 2612 | — | 2912 | — | 1282 | 1857 | — | — |
| Alignment, degrees (A) | 17.41 | 11.38 | 16.24 | 10.81 | 10.83 | 9.00 | — | — |
| Mean angle deviation, degrees (F) | 14.32 | 8.90 | 6.83 | 8.29 | 6.21 | 5.39 | — | — |
| Angle deviation, 95th percentile (F) | 41.52 | 29.13 | 23.86 | 23.45 | 17.09 | 16.12 | — | — |
| Mean aspect ratio (F) | 1.383 | 1.804 | 1.244 | 1.266 | 1.259 | 1.284 | — | — |
| Area variation (F) | 0.398 | 0.713 | 0.210 | 0.262 | 0.291 | 0.459 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 44.59 | 52.50 | 48.32 | 25.35 | 20.45 | 12.18 | — | — |
| Mean distance, ‰ (H) | 1.056 | 2.185 | 1.012 | 0.532 | 0.417 | 0.288 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 67.8 | 29.1 | 26.2 | 25.0 | 16.7 | 5.3 | — | — |
| Volume kept, % of the source (V) | 98.78 | 97.54 | 98.84 | 99.69 | 99.82 | 99.98 | — | — |
| Edges on closed loops, % (L) | 0.4 | 10.6 | — | — | 32.3 | 20.7 | — | — |
| Edges on spiralling loops, % (L) | 9.5 | 5.7 | — | — | 5.6 | 4.4 | — | — |
| Spiralling loops (L) | 12 | 5 | — | — | 2 | 6 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 90.5 | 83.7 | — | — | 62.1 | 74.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 218 | 113 | — | — | 225 | 170 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 10.8 | 39.7 | 1.1 | 10.7 | 0.3 | 0.9 | — | — |
| Time, seconds (T) | 1.65 | 1.53 | 1.15 | 9.77 | 4.58 | 6.22 | — | — |

## jerry (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 1/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3122 | 1102 | 2959 | 2595 | 3045 | 3012 | — | — |
| Irregular vertices (S) | 104 | 84 | 265 | 97 | 86 | 170 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2211 | — | 2959 | 1552 | 1723 | 2270 | — | — |
| Alignment, degrees (A) | 17.93 | 17.98 | 18.35 | 14.87 | 14.41 | 14.49 | — | — |
| Mean angle deviation, degrees (F) | 11.25 | 15.26 | 8.58 | 9.60 | 6.48 | 6.86 | — | — |
| Angle deviation, 95th percentile (F) | 29.73 | 53.91 | 29.29 | 24.87 | 16.84 | 21.27 | — | — |
| Mean aspect ratio (F) | 1.301 | 2.330 | 1.357 | 1.324 | 1.277 | 1.329 | — | — |
| Area variation (F) | 0.278 | 2.551 | 0.258 | 0.272 | 0.295 | 0.487 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 38.82 | 200.69 | 49.21 | 48.31 | 22.98 | 21.24 | — | — |
| Mean distance, ‰ (H) | 1.467 | 5.546 | 2.158 | 1.786 | 1.217 | 0.913 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 23.4 | 76.1 | 40.3 | 57.3 | 28.6 | 14.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 6.6 | 3.0 | — | — |
| Edges on spiralling loops, % (L) | 0.7 | 13.4 | — | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 1 | 6 | — | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 82.0 | 86.6 | — | — | 75.7 | 86.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 155 | 64 | — | — | 140 | 103 | — | — |
| Edges on loops from a border to a border, % (L) | 17.0 | 0.0 | — | — | 17.7 | 10.3 | — | — |
| Off budget, % (N) | 4.1 | 63.3 | 1.4 | 13.5 | 1.5 | 0.4 | — | — |
| Time, seconds (T) | 1.74 | 1.38 | 0.59 | 7.59 | 3.11 | 6.29 | — | — |

## rose (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 2085 | 636 | 2226 | 1923 | 3268 | 3624 | — | — |
| Irregular vertices (S) | 539 | 243 | 499 | 252 | 853 | 1038 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 1872 | — | 1954 | — | — | — | — | — |
| Alignment, degrees (A) | 18.30 | 19.39 | 19.07 | 16.12 | 14.93 | 14.51 | — | — |
| Mean angle deviation, degrees (F) | 16.45 | 31.06 | 15.58 | 13.29 | 18.71 | 18.71 | — | — |
| Angle deviation, 95th percentile (F) | 48.25 | 82.38 | 48.45 | 38.66 | 60.71 | 58.96 | — | — |
| Mean aspect ratio (F) | 1.647 | 5.541 | 1.752 | 1.809 | 4.514 | 68.483 | — | — |
| Area variation (F) | 0.587 | 1.193 | 0.465 | 0.441 | 1.298 | 1.651 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 75.61 | 108.28 | 79.25 | 72.60 | 35.08 | 33.88 | — | — |
| Mean distance, ‰ (H) | 3.765 | 21.969 | 4.183 | 3.665 | 2.769 | 3.574 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 39.1 | 157.4 | 46.1 | 40.1 | 25.7 | 33.9 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 79.48 | — | — | — |
| Edges on closed loops, % (L) | 3.2 | — | — | — | 12.0 | — | — | — |
| Edges on spiralling loops, % (L) | 17.6 | — | — | — | 26.5 | — | — | — |
| Spiralling loops (L) | 82 | — | — | — | 417 | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 79.7 | — | — | — | 61.5 | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 38 | — | — | — | 61 | — | — | — |
| Edges on loops from a border to a border, % (L) | 0.5 | — | — | — | 0.0 | — | — | — |
| Off budget, % (N) | 30.5 | 78.8 | 25.8 | 35.9 | 8.9 | 20.8 | — | — |
| Time, seconds (T) | 2.21 | 1.16 | 2.12 | 16.37 | 4.20 | 10.12 | — | — |

## bridge (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 1893 | 2903 | 2750 | 2997 | 2971 | — | — |
| Irregular vertices (S) | — | 63 | 151 | 53 | 38 | 40 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1879 | 520 | 572 | 500 | — | — |
| Alignment, degrees (A) | — | 9.17 | 12.58 | 8.69 | 8.30 | 7.54 | — | — |
| Mean angle deviation, degrees (F) | — | 7.60 | 6.12 | 7.12 | 3.52 | 4.31 | — | — |
| Angle deviation, 95th percentile (F) | — | 22.56 | 20.51 | 20.62 | 11.86 | 12.84 | — | — |
| Mean aspect ratio (F) | — | 1.778 | 1.215 | 1.305 | 1.271 | 1.268 | — | — |
| Area variation (F) | — | 0.765 | 0.174 | 0.232 | 0.279 | 0.418 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 14.76 | 28.19 | 7.13 | 6.90 | 9.33 | — | — |
| Mean distance, ‰ (H) | — | 1.042 | 0.638 | 0.500 | 0.466 | 0.459 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 19.4 | — | — | 21.4 | 26.1 | — | — |
| Edges on spiralling loops, % (L) | — | 8.2 | — | — | 0.0 | 0.4 | — | — |
| Spiralling loops (L) | — | 4 | — | — | 0 | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 62.1 | — | — | 44.2 | 39.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 70 | — | — | 94 | 94 | — | — |
| Edges on loops from a border to a border, % (L) | — | 10.3 | — | — | 34.5 | 33.7 | — | — |
| Off budget, % (N) | — | 36.9 | 3.2 | 8.3 | 0.1 | 1.0 | — | — |
| Time, seconds (T) | — | 1.21 | 2.98 | 10.79 | 5.64 | 5.15 | — | — |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2890 | 1855 | 2938 | 2976 | 2945 | 2999 | — | — |
| Irregular vertices (S) | 28 | 117 | 204 | 24 | 24 | 24 | — | — |
| … of which above the minimum | 20 | — | 196 | 16 | 16 | 16 | — | — |
| Base complex patches (C) | 619 | — | 1782 | 78 | 78 | 78 | — | — |
| Alignment, degrees (A) | 12.97 | 15.75 | 13.96 | 11.65 | 12.29 | 10.64 | — | — |
| Mean angle deviation, degrees (F) | 2.66 | 12.33 | 6.10 | 2.61 | 1.38 | 1.53 | — | — |
| Angle deviation, 95th percentile (F) | 10.11 | 37.42 | 25.03 | 10.79 | 5.07 | 5.98 | — | — |
| Mean aspect ratio (F) | 1.116 | 2.476 | 1.233 | 1.163 | 1.133 | 1.140 | — | — |
| Area variation (F) | 0.100 | 0.672 | 0.199 | 0.126 | 0.126 | 0.334 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.54 | 17.76 | 9.41 | 0.80 | 11.57 | 0.49 | — | — |
| Mean distance, ‰ (H) | 0.114 | 1.229 | 0.067 | 0.054 | 0.068 | 0.059 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.15 | — | 100.14 | 100.00 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 51.3 | — | — | — | 83.5 | 84.0 | — | — |
| Edges on spiralling loops, % (L) | 26.0 | — | — | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 7 | — | — | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 21.7 | — | — | — | 16.5 | 16.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 90 | — | — | — | 56 | 50 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.7 | 38.2 | 2.1 | 0.8 | 1.8 | 0.0 | — | — |
| Time, seconds (T) | 2.72 | 1.14 | 0.71 | 4.01 | 3.11 | 2.96 | — | — |

## cube (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2904 | 1260 | 2974 | 2992 | 2992 | 2992 | — | — |
| Irregular vertices (S) | 8 | 8 | 24 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | 0 | 16 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 6 | 36 | 274 | 1102 | 6 | 6 | — | — |
| Alignment, degrees (A) | 0.29 | 2.29 | 1.97 | 0.75 | 0.14 | 0.36 | — | — |
| Mean angle deviation, degrees (F) | 0.18 | 3.61 | 1.65 | 0.75 | 0.01 | 0.37 | — | — |
| Angle deviation, 95th percentile (F) | 0.43 | 11.71 | 4.89 | 2.01 | 0.01 | 0.86 | — | — |
| Mean aspect ratio (F) | 1.005 | 2.174 | 1.062 | 1.039 | 1.031 | 1.088 | — | — |
| Area variation (F) | 0.004 | 0.662 | 0.090 | 0.045 | 0.021 | 0.098 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 4.80 | 10.77 | 0.00 | 8.84 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.017 | 0.439 | 0.000 | 0.067 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.89 | 100.00 | 99.99 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 95.5 | 82.9 | — | — | 95.5 | 95.5 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 0 | 0 | — | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 4.5 | 17.1 | — | — | 4.5 | 4.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 22 | 58 | — | — | 23 | 23 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.2 | 58.0 | 1.0 | 0.3 | 0.3 | 0.3 | — | — |
| Time, seconds (T) | 1.21 | 0.63 | 0.20 | 5.18 | 2.76 | 2.95 | — | — |

## lblock (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2928 | 1328 | 2780 | 2800 | 3060 | 2972 | — | — |
| Irregular vertices (S) | 12 | 12 | 24 | 12 | 12 | 12 | — | — |
| … of which above the minimum | 4 | 4 | 16 | 4 | 4 | 4 | — | — |
| Base complex patches (C) | 86 | 628 | 464 | 14 | 14 | 14 | — | — |
| Alignment, degrees (A) | 1.09 | 3.78 | 1.54 | 0.59 | 0.52 | 0.86 | — | — |
| Mean angle deviation, degrees (F) | 0.80 | 5.83 | 1.56 | 0.23 | 0.01 | 0.54 | — | — |
| Angle deviation, 95th percentile (F) | 2.44 | 18.79 | 4.54 | 0.98 | 0.05 | 1.42 | — | — |
| Mean aspect ratio (F) | 1.047 | 1.994 | 1.051 | 1.022 | 1.030 | 1.098 | — | — |
| Area variation (F) | 0.039 | 0.572 | 0.063 | 0.023 | 0.033 | 0.106 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 10.33 | 11.65 | 0.00 | 0.00 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.034 | 0.454 | 0.000 | 0.000 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.83 | 100.00 | 100.00 | 100.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 78.3 | 26.4 | — | — | 93.2 | 93.1 | — | — |
| Edges on spiralling loops, % (L) | 13.8 | 62.2 | — | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 1 | 5 | — | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 7.9 | 11.4 | — | — | 6.8 | 6.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 86 | 58 | — | — | 45 | 43 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 2.4 | 55.7 | 7.3 | 6.7 | 2.0 | 0.9 | — | — |
| Time, seconds (T) | 1.19 | 0.82 | 0.36 | 23.07 | 3.28 | 2.38 | — | — |

## cylinder (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2900 | 1306 | 3100 | 2533 | 2992 | 2952 | — | — |
| Irregular vertices (S) | 8 | 8 | 91 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | 0 | 83 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 1003 | 68 | 3052 | 643 | 6 | 6 | — | — |
| Alignment, degrees (A) | 4.18 | 4.72 | 5.46 | 1.56 | 1.52 | 4.33 | — | — |
| Mean angle deviation, degrees (F) | 4.32 | 8.13 | 4.18 | 4.16 | 2.45 | 3.95 | — | — |
| Angle deviation, 95th percentile (F) | 17.78 | 26.13 | 15.60 | 17.67 | 6.67 | 9.59 | — | — |
| Mean aspect ratio (F) | 1.087 | 2.019 | 1.132 | 1.355 | 1.166 | 1.212 | — | — |
| Area variation (F) | 0.103 | 0.640 | 0.140 | 0.374 | 0.171 | 0.143 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 2.07 | 9.51 | 3.74 | 1.00 | 0.78 | 0.81 | — | — |
| Mean distance, ‰ (H) | 0.144 | 0.790 | 0.121 | 0.145 | 0.132 | 0.118 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.91 | 99.21 | 99.90 | 100.00 | 100.00 | 100.01 | — | — |
| Edges on closed loops, % (L) | 33.0 | 74.5 | — | — | 95.4 | 95.4 | — | — |
| Edges on spiralling loops, % (L) | 55.4 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 6 | 0 | — | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 11.6 | 25.5 | — | — | 4.6 | 4.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 207 | 178 | — | — | 34 | 32 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.3 | 56.5 | 3.3 | 15.6 | 0.3 | 1.6 | — | — |
| Time, seconds (T) | 1.05 | 0.65 | 0.17 | 2.92 | 2.58 | 3.41 | — | — |

## venturi (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 4046 | 2489 | 2412 | 2700 | 3828 | — | — |
| Irregular vertices (S) | — | 1527 | 432 | 169 | 210 | 659 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1464 | — | — | — | — | — |
| Alignment, degrees (A) | — | 11.84 | 11.84 | 11.55 | 11.42 | 14.88 | — | — |
| Mean angle deviation, degrees (F) | — | 48.87 | 13.84 | 8.20 | 8.86 | 13.27 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.70 | 52.08 | 25.25 | 30.79 | 43.53 | — | — |
| Mean aspect ratio (F) | — | 12.801 | 1.928 | 1.344 | 1.419 | 1.976 | — | — |
| Area variation (F) | — | 2.373 | 0.475 | 0.291 | 0.347 | 0.708 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 71.10 | 40.69 | 38.37 | 40.05 | 40.05 | — | — |
| Mean distance, ‰ (H) | — | 5.233 | 4.298 | 5.725 | 4.535 | 3.845 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 24.5 | 28.2 | 36.2 | 27.8 | 23.8 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 24.2 | 11.0 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 26.9 | 15.2 | — | — |
| Spiralling loops (L) | — | — | — | — | 55 | 46 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 48.9 | 73.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 104 | 104 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | — | 34.9 | 17.0 | 19.6 | 10.0 | 27.6 | — | — |
| Time, seconds (T) | — | 1.67 | 1.05 | 22.47 | 15.74 | 22.37 | — | — |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 3/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 1283 | 2836 | 2515 | 2979 | 3048 | — | — |
| Irregular vertices (S) | — | 87 | 223 | 85 | 90 | 190 | — | — |
| … of which above the minimum | — | — | 219 | — | 82 | 182 | — | — |
| Base complex patches (C) | — | — | 2718 | — | 1853 | 1854 | — | — |
| Alignment, degrees (A) | — | 16.86 | 18.44 | 13.66 | 13.48 | 12.42 | — | — |
| Mean angle deviation, degrees (F) | — | 10.34 | 7.84 | 9.13 | 6.21 | 7.02 | — | — |
| Angle deviation, 95th percentile (F) | — | 29.60 | 25.71 | 24.63 | 17.38 | 20.27 | — | — |
| Mean aspect ratio (F) | — | 1.786 | 1.295 | 1.502 | 1.562 | 1.433 | — | — |
| Area variation (F) | — | 0.654 | 0.236 | 0.339 | 0.369 | 0.780 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 30.78 | 61.14 | 38.53 | 27.24 | 24.08 | — | — |
| Mean distance, ‰ (H) | — | 2.902 | 1.839 | 1.177 | 0.766 | 0.745 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 57.6 | 145.0 | 75.2 | 23.4 | 5.3 | — | — |
| Volume kept, % of the source (V) | — | 95.87 | 98.43 | 99.70 | 99.94 | 99.99 | — | — |
| Edges on closed loops, % (L) | — | 10.2 | — | — | 18.9 | 20.7 | — | — |
| Edges on spiralling loops, % (L) | — | 16.6 | — | — | 5.8 | 3.7 | — | — |
| Spiralling loops (L) | — | 12 | — | — | 6 | 6 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 73.3 | — | — | 75.3 | 75.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 51 | — | — | 211 | 105 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | — | 57.2 | 5.5 | 16.2 | 0.7 | 1.6 | — | — |
| Time, seconds (T) | — | 13.40 | 89.83 | 57.56 | 58.37 | 127.46 | — | — |

Failures: quadriflow: cancelled.

## watermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 2675 | 709 | 1784 | 2260 | 2249 | 2242 | — | — |
| Irregular vertices (S) | 302 | 129 | 307 | 226 | 209 | 189 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2588 | — | 1676 | — | — | — | — | — |
| Alignment, degrees (A) | 14.64 | 19.69 | 16.44 | 15.73 | 15.67 | 15.72 | — | — |
| Mean angle deviation, degrees (F) | 11.57 | 21.19 | 17.11 | 11.91 | 11.32 | 11.35 | — | — |
| Angle deviation, 95th percentile (F) | 39.87 | 71.32 | 57.84 | 47.77 | 46.08 | 46.08 | — | — |
| Mean aspect ratio (F) | 1.472 | 2.842 | 1.945 | 1.601 | 1.648 | 1.652 | — | — |
| Area variation (F) | 0.335 | 0.833 | 0.490 | 0.562 | 0.756 | 0.755 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 86.07 | 187.44 | 74.17 | 77.11 | 75.48 | 75.48 | — | — |
| Mean distance, ‰ (H) | 5.115 | 23.533 | 7.799 | 6.793 | 6.539 | 6.553 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 31.4 | 104.7 | 43.1 | 39.8 | 37.2 | 37.3 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 2.2 | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 6.9 | — | — | — | — | — | — | — |
| Spiralling loops (L) | 27 | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.4 | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 76 | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 2.6 | — | — | — | — | — | — | — |
| Off budget, % (N) | 10.8 | 76.4 | 40.5 | 24.7 | 25.0 | 25.3 | — | — |
| Time, seconds (T) | 2.98 | 1.89 | 0.55 | 23.13 | 122.35 | 1332.03 | — | — |

## towermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 2724 | 1088 | 1904 | 2395 | 3060 | 2304 | — | — |
| Irregular vertices (S) | 378 | 194 | 370 | 147 | 327 | 158 | — | — |
| … of which above the minimum | — | — | — | — | 23 | — | — | — |
| Base complex patches (C) | 2661 | — | 1899 | — | 2868 | — | — | — |
| Alignment, degrees (A) | 13.28 | 16.22 | 16.31 | 12.77 | 14.60 | 12.61 | — | — |
| Mean angle deviation, degrees (F) | 11.93 | 19.35 | 19.44 | 9.80 | 13.53 | 10.16 | — | — |
| Angle deviation, 95th percentile (F) | 38.83 | 64.57 | 61.40 | 35.38 | 43.31 | 35.17 | — | — |
| Mean aspect ratio (F) | 1.501 | 2.846 | 2.098 | 1.446 | 2.271 | 1.480 | — | — |
| Area variation (F) | 0.329 | 0.711 | 0.515 | 0.322 | 0.670 | 0.330 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 56.00 | 98.83 | 77.09 | 63.94 | 47.87 | 55.25 | — | — |
| Mean distance, ‰ (H) | 3.285 | 12.802 | 6.169 | 4.799 | 3.316 | 4.940 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 16.0 | 56.9 | 34.1 | 27.6 | 20.0 | 28.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 94.56 | — | — | — |
| Edges on closed loops, % (L) | 0.6 | — | — | — | 3.2 | — | — | — |
| Edges on spiralling loops, % (L) | 10.8 | — | — | — | 9.3 | — | — | — |
| Spiralling loops (L) | 20 | — | — | — | 43 | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 88.2 | — | — | — | 87.5 | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 61 | — | — | — | 58 | — | — | — |
| Edges on loops from a border to a border, % (L) | 0.4 | — | — | — | 0.0 | — | — | — |
| Off budget, % (N) | 9.2 | 63.7 | 36.5 | 20.2 | 2.0 | 23.2 | — | — |
| Time, seconds (T) | 3.14 | 1.83 | 0.47 | 12.81 | 36.90 | 26.92 | — | — |

## stonemill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 0/1 | 1/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2635 | 1809 | 2664 | 2538 | 2976 | 3008 | — | — |
| Irregular vertices (S) | 242 | 99 | 287 | 105 | 126 | 233 | — | — |
| … of which above the minimum | 206 | — | 247 | — | 62 | 169 | — | — |
| Base complex patches (C) | 2605 | — | 2586 | — | 2061 | 2822 | — | — |
| Alignment, degrees (A) | 13.78 | 14.83 | 14.29 | 13.89 | 13.17 | 13.00 | — | — |
| Mean angle deviation, degrees (F) | 9.37 | 11.26 | 9.56 | 8.82 | 6.74 | 7.25 | — | — |
| Angle deviation, 95th percentile (F) | 28.53 | 33.83 | 32.90 | 24.82 | 20.03 | 22.82 | — | — |
| Mean aspect ratio (F) | 1.324 | 2.004 | 1.440 | 1.264 | 1.316 | 1.481 | — | — |
| Area variation (F) | 0.239 | 0.710 | 0.288 | 0.245 | 0.294 | 0.411 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 42.66 | 106.61 | 66.01 | 59.45 | 19.62 | 17.54 | — | — |
| Mean distance, ‰ (H) | 2.229 | 4.106 | 2.742 | 2.632 | 1.896 | 1.801 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 40.9 | 155.1 | 65.7 | 77.5 | 20.3 | 16.9 | — | — |
| Volume kept, % of the source (V) | 98.96 | — | 98.49 | — | 99.95 | 99.96 | — | — |
| Edges on closed loops, % (L) | 2.5 | — | — | — | 16.9 | 3.4 | — | — |
| Edges on spiralling loops, % (L) | 5.2 | — | — | — | 6.3 | 2.2 | — | — |
| Spiralling loops (L) | 12 | — | — | — | 12 | 6 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 92.4 | — | — | — | 76.8 | 94.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 92 | — | — | — | 138 | 156 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 12.2 | 39.7 | 11.2 | 15.4 | 0.8 | 0.3 | — | — |
| Time, seconds (T) | 2.73 | 0.95 | 0.53 | 9.68 | 4.81 | 8.13 | — | — |

## windmill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 1003 | 1886 | — | 135 | 432 | — | — |
| Irregular vertices (S) | — | 145 | 312 | — | 28 | 124 | — | — |
| … of which above the minimum | — | — | — | — | 4 | 100 | — | — |
| Base complex patches (C) | — | — | 1778 | — | 135 | 432 | — | — |
| Alignment, degrees (A) | — | 14.39 | 15.52 | — | 18.13 | 18.56 | — | — |
| Mean angle deviation, degrees (F) | — | 20.17 | 15.16 | — | 28.28 | 19.29 | — | — |
| Angle deviation, 95th percentile (F) | — | 69.91 | 53.30 | — | 72.51 | 53.63 | — | — |
| Mean aspect ratio (F) | — | 2.697 | 1.760 | — | 6.328 | 2.508 | — | — |
| Area variation (F) | — | 0.807 | 0.437 | — | 1.107 | 1.202 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 193.85 | 69.08 | — | 535.07 | 529.38 | — | — |
| Mean distance, ‰ (H) | — | 18.937 | 5.814 | — | 256.264 | 232.365 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 122.8 | 42.9 | — | 1191.0 | 1163.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 1.13 | 1.44 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 0.0 | 0.0 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 34.4 | 3.7 | — | — |
| Spiralling loops (L) | — | — | — | — | 7 | 8 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 65.6 | 96.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 13 | 17 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | — | 66.6 | 37.1 | — | 95.5 | 85.6 | — | — |
| Time, seconds (T) | — | 1.35 | 0.52 | — | 1.46 | 5.22 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 2 non-manifold or inconsistently oriented edges.

## terrain (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2717 | 919 | 1753 | 2502 | 3029 | 3032 | — | — |
| Irregular vertices (S) | 112 | 45 | 115 | 55 | 58 | 97 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 826 | — | 1209 | 997 | 1498 | 2229 | — | — |
| Alignment, degrees (A) | 20.16 | 20.27 | 21.17 | 18.74 | 18.13 | 18.43 | — | — |
| Mean angle deviation, degrees (F) | 6.49 | 6.70 | 8.24 | 9.36 | 6.49 | 6.48 | — | — |
| Angle deviation, 95th percentile (F) | 23.46 | 24.56 | 27.33 | 27.14 | 18.44 | 19.56 | — | — |
| Mean aspect ratio (F) | 1.209 | 1.423 | 1.291 | 1.338 | 1.277 | 1.337 | — | — |
| Area variation (F) | 0.187 | 0.413 | 0.215 | 0.276 | 0.308 | 0.433 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 23.91 | 34.95 | 143.82 | 24.92 | 19.27 | 15.98 | — | — |
| Mean distance, ‰ (H) | 1.851 | 3.051 | 2.384 | 1.715 | 1.374 | 1.100 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 32.8 | 56.8 | 36.9 | 35.4 | 26.5 | 18.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 1.8 | — | — |
| Spiralling loops (L) | 0 | 0 | — | — | 0 | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 53.0 | 67.6 | — | — | 69.2 | 83.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 46 | 27 | — | — | 98 | 110 | — | — |
| Edges on loops from a border to a border, % (L) | 47.0 | 32.4 | — | — | 30.8 | 14.9 | — | — |
| Off budget, % (N) | 9.4 | 69.4 | 41.6 | 16.6 | 1.0 | 1.1 | — | — |
| Time, seconds (T) | 19.92 | 1.90 | 8.92 | 14.15 | 10.09 | 8.99 | — | — |

## mahaveer (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 1325 | 1699 | 2080 | 2838 | 3322 | — | — |
| Irregular vertices (S) | — | 422 | 175 | 64 | 476 | 488 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1227 | — | 804 | — | — | — |
| Alignment, degrees (A) | — | 9.34 | 9.97 | 7.13 | 9.25 | 7.20 | — | — |
| Mean angle deviation, degrees (F) | — | 32.26 | 9.66 | 7.58 | 18.19 | 22.37 | — | — |
| Angle deviation, 95th percentile (F) | — | 85.28 | 34.02 | 26.60 | 82.06 | 82.64 | — | — |
| Mean aspect ratio (F) | — | 5.235 | 1.583 | 1.656 | 93.141 | 126.926 | — | — |
| Area variation (F) | — | 1.443 | 0.344 | 0.570 | 2.670 | 2.913 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 44.66 | 52.26 | 26.46 | 50.03 | 50.03 | — | — |
| Mean distance, ‰ (H) | — | 5.890 | 7.890 | 3.785 | 4.005 | 3.962 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 46.8 | 75.1 | 35.8 | 34.1 | 33.8 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 21.5 | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 56.3 | — | — | — |
| Spiralling loops (L) | — | — | — | — | 401 | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 18.8 | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 10 | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 3.5 | — | — | — |
| Off budget, % (N) | — | 55.8 | 43.4 | 30.7 | 5.4 | 10.7 | — | — |
| Time, seconds (T) | — | 1.33 | 7.16 | 18.77 | 9.45 | 12.02 | — | — |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 870 | 1225 | — | 8941 | 9054 | — | — |
| Irregular vertices (S) | — | 858 | 616 | — | 6050 | 6012 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1136 | — | — | — | — | — |
| Alignment, degrees (A) | — | 18.06 | 17.78 | — | 12.74 | 12.04 | — | — |
| Mean angle deviation, degrees (F) | — | 52.53 | 23.80 | — | 21.75 | 21.23 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.72 | 60.14 | — | 68.55 | 66.73 | — | — |
| Mean aspect ratio (F) | — | 14.155 | 2.545 | — | 5.235 | 4.238 | — | — |
| Area variation (F) | — | 7.748 | 0.789 | — | 3.361 | 3.360 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 9.99 | 7.35 | — | 7.04 | 5.99 | — | — |
| Mean distance, ‰ (H) | — | 2.873 | 1.806 | — | 0.448 | 0.448 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 87.1 | 57.0 | — | 12.5 | 12.8 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 71.0 | 59.2 | — | 198.0 | 201.8 | — | — |
| Time, seconds (T) | — | 0.99 | 4.00 | — | 14.68 | 21.31 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 8 non-manifold or inconsistently oriented edges.

## f16 (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 2/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 4122 | 1500 | 1333 | 4505 | 4395 | — | — |
| Irregular vertices (S) | — | 2217 | 403 | 242 | 1830 | 1758 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1329 | — | — | — | — | — |
| Alignment, degrees (A) | — | 14.85 | 12.61 | 8.71 | 6.11 | 5.89 | — | — |
| Mean angle deviation, degrees (F) | — | 41.32 | 19.76 | 15.35 | 18.12 | 16.99 | — | — |
| Angle deviation, 95th percentile (F) | — | 87.07 | 64.46 | 49.46 | 68.74 | 66.31 | — | — |
| Mean aspect ratio (F) | — | 7.957 | 2.173 | 1.844 | 104.856 | 116.397 | — | — |
| Area variation (F) | — | 2.547 | 0.539 | 0.650 | 2.769 | 2.422 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 87.83 | 95.94 | 63.38 | 33.11 | 39.00 | — | — |
| Mean distance, ‰ (H) | — | 9.067 | 20.293 | 13.479 | 1.231 | 1.499 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 62.6 | 143.8 | 98.2 | 7.7 | 9.7 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 37.4 | 50.0 | 55.6 | 50.2 | 46.5 | — | — |
| Time, seconds (T) | — | 2.15 | 5.15 | 88.05 | 11.24 | 11.72 | — | — |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2222 | 2216 | 1109 | 3276 | 3445 | — | — |
| Irregular vertices (S) | — | 338 | 265 | 70 | 204 | 202 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1675 | — | — | — | — | — |
| Alignment, degrees (A) | — | 7.28 | 8.44 | 8.56 | 5.33 | 5.21 | — | — |
| Mean angle deviation, degrees (F) | — | 22.14 | 11.96 | 12.59 | 18.06 | 20.25 | — | — |
| Angle deviation, 95th percentile (F) | — | 84.37 | 45.38 | 46.79 | 73.19 | 74.42 | — | — |
| Mean aspect ratio (F) | — | 4.205 | 1.771 | 2.901 | 16.052 | 49.040 | — | — |
| Area variation (F) | — | 2.439 | 0.430 | 0.902 | 1.348 | 1.421 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 50.21 | 35.63 | 45.56 | 26.86 | 26.86 | — | — |
| Mean distance, ‰ (H) | — | 5.701 | 4.253 | 5.359 | 2.485 | 2.309 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 25.0 | 19.2 | 22.4 | 11.5 | 11.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 52.5 | 50.1 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 8.0 | 6.2 | — | — |
| Spiralling loops (L) | — | — | — | — | 85 | 82 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 27.9 | 32.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 50 | 72 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 11.5 | 11.1 | — | — |
| Off budget, % (N) | — | 25.9 | 26.1 | 63.0 | 9.2 | 14.8 | — | — |
| Time, seconds (T) | — | 0.92 | 1.12 | 70.22 | 9.82 | 3.11 | — | — |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 625 | 2774 | 2420 | 2390 | 2394 | — | — |
| Irregular vertices (S) | — | 125 | 430 | 185 | 221 | 217 | — | — |
| … of which above the minimum | — | — | 387 | — | — | — | — | — |
| Base complex patches (C) | — | — | 2517 | — | — | — | — | — |
| Alignment, degrees (A) | — | 14.57 | 14.83 | 10.13 | 11.04 | 10.94 | — | — |
| Mean angle deviation, degrees (F) | — | 16.08 | 11.44 | 9.70 | 10.21 | 10.22 | — | — |
| Angle deviation, 95th percentile (F) | — | 44.86 | 37.21 | 27.88 | 29.69 | 30.09 | — | — |
| Mean aspect ratio (F) | — | 2.516 | 1.550 | 1.474 | 1.628 | 1.678 | — | — |
| Area variation (F) | — | 0.933 | 0.323 | 0.335 | 0.358 | 0.357 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 89.72 | 44.20 | 33.89 | 43.77 | 43.77 | — | — |
| Mean distance, ‰ (H) | — | 9.165 | 3.465 | 2.989 | 2.662 | 2.679 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 125.4 | 88.0 | 69.3 | 60.7 | 61.2 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 20.3 | 20.4 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 11.1 | 10.7 | — | — |
| Spiralling loops (L) | — | — | — | — | 44 | 40 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 68.4 | 68.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 86 | 86 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.2 | 0.2 | — | — |
| Off budget, % (N) | — | 79.2 | 7.5 | 19.3 | 20.3 | 20.2 | — | — |
| Time, seconds (T) | — | 0.74 | 0.34 | 22.46 | 23.60 | 23.65 | — | — |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3107 | 2086 | 3090 | 3000 | 2990 | 2990 | — | — |
| Irregular vertices (S) | 8 | 10 | 97 | 32 | 8 | 8 | — | — |
| … of which above the minimum | 0 | 2 | 89 | 24 | 0 | 0 | — | — |
| Base complex patches (C) | 2730 | 2004 | 3090 | 30 | 6 | 6 | — | — |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.49 | 7.65 | 4.19 | 8.38 | 1.23 | 1.23 | — | — |
| Angle deviation, 95th percentile (F) | 12.67 | 19.88 | 14.49 | 17.72 | 3.57 | 3.58 | — | — |
| Mean aspect ratio (F) | 1.125 | 1.326 | 1.124 | 1.081 | 1.108 | 1.108 | — | — |
| Area variation (F) | 0.135 | 0.341 | 0.136 | 0.190 | 0.142 | 0.142 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.63 | 1.54 | 0.72 | 0.42 | 0.35 | 0.35 | — | — |
| Mean distance, ‰ (H) | 0.216 | 0.408 | 0.221 | 0.168 | 0.191 | 0.191 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.81 | 99.59 | 99.79 | 100.01 | 100.01 | 100.01 | — | — |
| Edges on closed loops, % (L) | 6.1 | 2.0 | — | — | 95.5 | 95.5 | — | — |
| Edges on spiralling loops, % (L) | 70.1 | 74.4 | — | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 5 | 6 | — | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 15.4 | 23.6 | — | — | 4.5 | 4.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 276 | 298 | — | — | 23 | 23 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 3.6 | 30.5 | 3.0 | 0.0 | 0.3 | 0.3 | — | — |
| Time, seconds (T) | 1.81 | 0.40 | 0.34 | 1.94 | 3.39 | 3.16 | — | — |

## torus (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2976 | 2672 | 3078 | 2720 | 2952 | 2952 | — | — |
| Irregular vertices (S) | 30 | 0 | 125 | 0 | 0 | 0 | — | — |
| … of which above the minimum | 30 | 0 | 125 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 2976 | 1 | 3078 | 1 | 1 | 1 | — | — |
| Alignment, degrees (A) | 10.14 | 10.37 | 14.28 | 0.65 | 0.62 | 0.62 | — | — |
| Mean angle deviation, degrees (F) | 10.25 | 4.17 | 5.57 | 1.45 | 1.43 | 1.43 | — | — |
| Angle deviation, 95th percentile (F) | 26.29 | 11.07 | 18.30 | 2.89 | 2.35 | 2.35 | — | — |
| Mean aspect ratio (F) | 1.239 | 2.545 | 1.163 | 1.351 | 1.323 | 1.323 | — | — |
| Area variation (F) | 0.223 | 0.256 | 0.163 | 0.281 | 0.307 | 0.307 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 1.86 | 1.50 | 1.38 | 0.59 | 0.58 | 0.58 | — | — |
| Mean distance, ‰ (H) | 0.299 | 0.465 | 0.298 | 0.208 | 0.191 | 0.191 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.46 | 99.09 | 99.44 | 100.02 | 100.02 | 100.02 | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 100.0 | 100.0 | — | — |
| Edges on spiralling loops, % (L) | 43.5 | 100.0 | — | — | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 7 | 6 | — | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 56.5 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 411 | 0 | — | — | 0 | 0 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 0.8 | 10.9 | 2.6 | 9.3 | 1.6 | 1.6 | — | — |
| Time, seconds (T) | 1.56 | 0.53 | 0.22 | 2.01 | 2.44 | 1.57 | — | — |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (15 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 6 | 0 | 4 | 4 | 1 | 0 | 0 |
| Base complex patches (C) | 1 | 3 | 0 | 1 | 7 | 3 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 2 | 1 | 12 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 2 | 1 | 6 | 5 | 0 | 1 |
| Mean aspect ratio (F) | 1 | 0 | 10 | 0 | 3 | 0 | 0 | 1 |
| Valid meshes (V) | 36/75 | 4/15 | 48/75 | 10/15 | 14/15 | 14/15 | 0/2 | 2/2 |

### mechanical (4 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 3 | 0 | 4 | 4 | 4 | 0 | 0 |
| Base complex patches (C) | 1 | 0 | 0 | 2 | 4 | 4 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 3 | 1 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 0 |
| Mean aspect ratio (F) | 3 | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| Valid meshes (V) | 18/20 | 3/4 | 20/20 | 4/4 | 4/4 | 4/4 | 0/0 | 0/0 |
