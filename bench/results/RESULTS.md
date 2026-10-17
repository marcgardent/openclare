# Bench results

Requested budget: 3000 quads. Median over 5 seeds; Quad Remesher, AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

The last two columns compare at equal quad count: `quadremesher-as-made` is the mesh of `quadremesher` measured against the number of quads it has rather than the budget, and `openclare-matched` is `openclare` asked for that same number.

## blob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2781 | 1165 | 3090 | 2507 | 2959 | 3055 | 3370 | 2755 | 3370 | 3397 | — | — |
| Irregular vertices (S) | 38 | 26 | 143 | 26 | 24 | 24 | 26 | 24 | 26 | 24 | — | — |
| … of which above the minimum | 30 | — | 135 | 18 | 16 | 16 | 18 | 16 | 18 | 16 | — | — |
| Base complex patches (C) | 2697 | — | 3048 | 951 | 995 | 1433 | 1826 | 1350 | 1826 | 2407 | — | — |
| Alignment, degrees (A) | 17.48 | 6.27 | 15.74 | 6.23 | 5.13 | 5.63 | 3.93 | 4.55 | 3.94 | 5.25 | — | — |
| Mean angle deviation, degrees (F) | 8.71 | 9.45 | 5.87 | 7.89 | 6.31 | 7.14 | 5.76 | 5.45 | 5.76 | 6.24 | — | — |
| Angle deviation, 95th percentile (F) | 27.30 | 30.58 | 18.26 | 21.16 | 18.12 | 21.40 | 17.21 | 15.61 | 17.21 | 18.04 | — | — |
| Mean aspect ratio (F) | 1.187 | 1.934 | 1.174 | 1.427 | 1.457 | 1.483 | 1.618 | 1.276 | 1.618 | 1.423 | — | — |
| Area variation (F) | 0.205 | 0.575 | 0.175 | 0.351 | 0.428 | 0.426 | 0.442 | 0.313 | 0.442 | 0.421 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 8.52 | 6.35 | 10.14 | 5.25 | 3.06 | 2.65 | 4.21 | 7.28 | 4.21 | 2.59 | — | — |
| Mean distance, ‰ (H) | 0.486 | 1.885 | 0.459 | 0.272 | 0.197 | 0.184 | 0.407 | 0.513 | 0.407 | 0.167 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.56 | 96.90 | 99.63 | 100.03 | 100.02 | 100.02 | 99.40 | 99.52 | 99.40 | 100.02 | — | — |
| Edges on closed loops, % (L) | 0.0 | 17.2 | — | — | 33.7 | 28.9 | 22.9 | — | — | 14.6 | — | — |
| Edges on spiralling loops, % (L) | 14.7 | 31.9 | — | — | 23.7 | 17.0 | 34.1 | — | — | 67.5 | — | — |
| Spiralling loops (L) | 3 | 7 | — | — | 4 | 3 | 7 | — | — | 13 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.3 | 50.9 | — | — | 42.7 | 54.1 | 43.0 | — | — | 18.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 321 | 120 | — | — | 341 | 360 | 274 | — | — | 172 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 7.3 | 61.2 | 3.0 | 16.4 | 1.4 | 1.8 | 12.3 | 8.2 | 0.0 | 0.8 | — | — |
| Time, seconds (T) | 2.17 | 0.74 | 1.71 | 4.09 | 2.59 | 2.55 | 16.38 | 12.28 | 16.38 | 4.38 | — | — |

## rock (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3036 | 2412 | 2898 | 2687 | 3039 | 3010 | 3539 | 2531 | 3539 | 3506 | — | — |
| Irregular vertices (S) | 8 | 20 | 91 | 50 | 54 | 64 | 114 | 101 | 114 | 72 | — | — |
| … of which above the minimum | 0 | 12 | 83 | 42 | 46 | 56 | 106 | 93 | 106 | 64 | — | — |
| Base complex patches (C) | 2855 | 2412 | 2896 | 2687 | 3039 | 3010 | 3539 | 2531 | 3539 | 3506 | — | — |
| Alignment, degrees (A) | 22.96 | 18.78 | 23.21 | 15.81 | 16.22 | 15.75 | 14.17 | 14.68 | 14.63 | 15.93 | — | — |
| Mean angle deviation, degrees (F) | 5.84 | 7.48 | 5.18 | 8.82 | 5.35 | 5.10 | 10.07 | 10.61 | 10.07 | 5.29 | — | — |
| Angle deviation, 95th percentile (F) | 16.05 | 17.91 | 14.78 | 22.11 | 13.54 | 13.37 | 24.88 | 26.21 | 24.88 | 14.46 | — | — |
| Mean aspect ratio (F) | 1.164 | 1.409 | 1.146 | 1.253 | 1.211 | 1.203 | 1.470 | 1.393 | 1.470 | 1.233 | — | — |
| Area variation (F) | 0.202 | 0.331 | 0.151 | 0.229 | 0.270 | 0.282 | 0.308 | 0.269 | 0.308 | 0.288 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.45 | 7.64 | 5.28 | 4.49 | 4.50 | 4.53 | 5.13 | 5.79 | 5.13 | 4.79 | — | — |
| Mean distance, ‰ (H) | 0.788 | 1.157 | 0.820 | 0.740 | 0.694 | 0.696 | 0.731 | 0.889 | 0.731 | 0.635 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.78 | 99.49 | 99.70 | 100.01 | 100.00 | 100.00 | 99.73 | 99.62 | 99.73 | 100.00 | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 56.9 | 27.1 | — | — | 11.0 | 9.3 | 10.3 | — | — | 2.0 | — | — |
| Spiralling loops (L) | 5 | 4 | — | — | 3 | 3 | 6 | — | — | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 43.1 | 72.9 | — | — | 89.0 | 90.7 | 89.7 | — | — | 98.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 528 | 317 | — | — | 238 | 207 | 145 | — | — | 434 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 2.2 | 19.6 | 3.4 | 10.4 | 1.3 | 0.3 | 18.0 | 15.6 | 0.0 | 0.9 | — | — |
| Time, seconds (T) | 1.22 | 1.86 | 1.03 | 6.92 | 2.47 | 2.42 | 20.49 | 16.98 | 20.49 | 3.15 | — | — |

## creature (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2667 | 1991 | 3070 | 2597 | 3041 | 2952 | 2941 | 2719 | 2941 | 2945 | — | — |
| Irregular vertices (S) | 24 | 20 | 111 | 41 | 36 | 40 | 42 | 46 | 42 | 40 | — | — |
| … of which above the minimum | 16 | 12 | 103 | 33 | 28 | 32 | 34 | 38 | 34 | 32 | — | — |
| Base complex patches (C) | 2667 | 747 | 2856 | 913 | 1245 | 860 | 1465 | 1301 | 1465 | 899 | — | — |
| Alignment, degrees (A) | 12.89 | 9.58 | 12.09 | 6.96 | 7.49 | 5.62 | 5.18 | 5.32 | 5.12 | 5.62 | — | — |
| Mean angle deviation, degrees (F) | 6.85 | 4.75 | 4.65 | 8.21 | 5.77 | 5.02 | 6.12 | 6.53 | 6.12 | 5.14 | — | — |
| Angle deviation, 95th percentile (F) | 18.98 | 13.44 | 14.73 | 24.05 | 16.44 | 13.90 | 17.13 | 18.98 | 17.13 | 14.15 | — | — |
| Mean aspect ratio (F) | 1.189 | 1.839 | 1.144 | 1.305 | 1.269 | 1.320 | 1.332 | 1.334 | 1.332 | 1.308 | — | — |
| Area variation (F) | 0.237 | 0.549 | 0.146 | 0.286 | 0.333 | 0.476 | 0.348 | 0.313 | 0.348 | 0.484 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 8.01 | 2.39 | 3.47 | 2.16 | 2.09 | 1.99 | 2.51 | 2.37 | 2.51 | 1.84 | — | — |
| Mean distance, ‰ (H) | 0.320 | 0.537 | 0.279 | 0.231 | 0.181 | 0.206 | 0.297 | 0.348 | 0.297 | 0.207 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.41 | 98.97 | 99.47 | 100.00 | 100.02 | 100.03 | 99.39 | 99.28 | 99.39 | 100.02 | — | — |
| Edges on closed loops, % (L) | 0.0 | 31.2 | — | — | 31.0 | 38.1 | 25.1 | — | — | 36.2 | — | — |
| Edges on spiralling loops, % (L) | 30.9 | 33.6 | — | — | 14.9 | 5.9 | 18.9 | — | — | 1.6 | — | — |
| Spiralling loops (L) | 4 | 6 | — | — | 6 | 3 | 8 | — | — | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 69.1 | 35.2 | — | — | 54.0 | 56.0 | 56.0 | — | — | 62.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 377 | 166 | — | — | 226 | 321 | 331 | — | — | 222 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 11.1 | 33.6 | 2.3 | 13.4 | 1.4 | 1.6 | 2.0 | 9.4 | 0.0 | 0.1 | — | — |
| Time, seconds (T) | 1.35 | 1.49 | 0.93 | 6.08 | 2.71 | 2.49 | 11.24 | 9.11 | 11.24 | 4.92 | — | — |

## cage (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2891 | 4039 | 2944 | 2874 | 3005 | 3011 | 5809 | 2785 | 5809 | 5769 | — | — |
| Irregular vertices (S) | 66 | 97 | 200 | 41 | 48 | 48 | 44 | 43 | 44 | 52 | — | — |
| … of which above the minimum | 34 | — | 168 | 9 | 16 | 16 | 12 | 11 | 12 | 20 | — | — |
| Base complex patches (C) | 1943 | — | 2319 | 427 | 341 | 310 | 1092 | 1001 | 1092 | 186 | — | — |
| Alignment, degrees (A) | 8.10 | 8.92 | 7.97 | 3.38 | 3.25 | 2.34 | 2.04 | 2.57 | 2.08 | 2.09 | — | — |
| Mean angle deviation, degrees (F) | 5.86 | 9.02 | 6.39 | 4.00 | 2.92 | 3.33 | 3.91 | 4.30 | 3.91 | 2.49 | — | — |
| Angle deviation, 95th percentile (F) | 17.11 | 28.25 | 23.33 | 12.37 | 9.00 | 10.42 | 11.90 | 12.82 | 11.90 | 7.09 | — | — |
| Mean aspect ratio (F) | 1.183 | 2.030 | 1.252 | 1.202 | 1.180 | 1.238 | 1.993 | 1.318 | 1.993 | 1.186 | — | — |
| Area variation (F) | 0.147 | 0.496 | 0.203 | 0.193 | 0.195 | 0.330 | 0.314 | 0.238 | 0.314 | 0.359 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 11.45 | 13.32 | 14.46 | 11.74 | 11.02 | 10.53 | 5.74 | 9.66 | 5.74 | 3.10 | — | — |
| Mean distance, ‰ (H) | 0.734 | 0.800 | 0.860 | 0.814 | 0.774 | 0.492 | 0.693 | 1.276 | 0.693 | 0.257 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.99 | 99.63 | 98.70 | 99.89 | 99.94 | 99.98 | 99.00 | 97.85 | 99.00 | 100.00 | — | — |
| Edges on closed loops, % (L) | 18.2 | 4.2 | — | — | 65.4 | 61.8 | 55.0 | — | — | 82.3 | — | — |
| Edges on spiralling loops, % (L) | 38.5 | 29.9 | — | — | 12.1 | 17.9 | 28.2 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 20 | 24 | — | — | 4 | 5 | 10 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 41.8 | 65.8 | — | — | 22.5 | 20.3 | 16.8 | — | — | 17.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 86 | 110 | — | — | 59 | 69 | 92 | — | — | 62 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.6 | 34.6 | 1.9 | 4.2 | 0.2 | 0.4 | 93.6 | 7.2 | 0.0 | 0.7 | — | — |
| Time, seconds (T) | 1.26 | 2.38 | 0.34 | 8.42 | 2.37 | 3.45 | 4.21 | 3.92 | 4.21 | 4.14 | — | — |

## suzanne (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 2/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3030 | 1255 | 3070 | 2757 | 3006 | 2973 | 4310 | 2628 | 4310 | 4247 | — | — |
| Irregular vertices (S) | 78 | 69 | 212 | 52 | 53 | 56 | 80 | 71 | 80 | 56 | — | — |
| … of which above the minimum | 54 | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2884 | — | 3002 | 1858 | 1476 | 1388 | 2513 | 1838 | 2513 | 1941 | — | — |
| Alignment, degrees (A) | 15.33 | 13.19 | 17.02 | 10.46 | 10.50 | 10.11 | 8.75 | 10.24 | 8.92 | 9.98 | — | — |
| Mean angle deviation, degrees (F) | 8.61 | 11.13 | 7.08 | 7.14 | 4.39 | 4.57 | 7.87 | 7.87 | 7.87 | 4.56 | — | — |
| Angle deviation, 95th percentile (F) | 26.08 | 41.63 | 25.30 | 18.16 | 11.87 | 12.52 | 21.85 | 20.46 | 21.85 | 11.42 | — | — |
| Mean aspect ratio (F) | 1.260 | 1.871 | 1.263 | 1.221 | 1.196 | 1.209 | 1.668 | 1.284 | 1.668 | 1.212 | — | — |
| Area variation (F) | 0.837 | 0.611 | 0.221 | 0.250 | 0.298 | 0.387 | 0.473 | 0.236 | 0.473 | 0.371 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 42.26 | 33.04 | 20.15 | 8.61 | 7.51 | 7.65 | 6.77 | 9.47 | 6.77 | 5.28 | — | — |
| Mean distance, ‰ (H) | 0.834 | 2.167 | 0.906 | 0.738 | 0.572 | 0.495 | 0.603 | 1.053 | 0.603 | 0.346 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 16.4 | 28.9 | 21.4 | 16.0 | 8.1 | 7.8 | 5.1 | 17.1 | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.9 | 8.6 | — | — | 18.6 | 21.5 | 15.9 | — | — | 19.7 | — | — |
| Edges on spiralling loops, % (L) | 15.3 | 14.6 | — | — | 6.5 | 4.5 | 11.8 | — | — | 9.6 | — | — |
| Spiralling loops (L) | 12 | 8 | — | — | 3 | 2 | 7 | — | — | 4 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 80.2 | 76.8 | — | — | 62.1 | 64.2 | 64.9 | — | — | 58.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 233 | 82 | — | — | 176 | 170 | 207 | — | — | 210 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 12.8 | 9.8 | 7.4 | — | — | 11.8 | — | — |
| Off budget, % (N) | 1.0 | 58.2 | 2.3 | 8.1 | 0.2 | 0.9 | 43.7 | 12.4 | 0.0 | 1.5 | — | — |
| Time, seconds (T) | 3.31 | 0.83 | 0.28 | 8.24 | 2.12 | 1.99 | 3.44 | 3.21 | 3.44 | 2.77 | — | — |

## ogre (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 0/1 | 4/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2935 | 1081 | 2874 | 2465 | 2994 | 3000 | 6134 | 2417 | 6134 | 6249 | — | — |
| Irregular vertices (S) | 106 | 84 | 216 | 77 | 78 | 159 | 314 | 228 | 314 | 210 | — | — |
| … of which above the minimum | 98 | — | 202 | — | 70 | 151 | 306 | 220 | 306 | 202 | — | — |
| Base complex patches (C) | 2857 | — | 2802 | — | 1720 | 2198 | 5170 | 2009 | 5170 | 4257 | — | — |
| Alignment, degrees (A) | 19.31 | 16.24 | 18.57 | 12.24 | 12.34 | 11.56 | 8.42 | 10.81 | 9.14 | 10.77 | — | — |
| Mean angle deviation, degrees (F) | 12.01 | 10.37 | 7.49 | 9.03 | 6.89 | 6.56 | 9.10 | 11.45 | 9.10 | 5.89 | — | — |
| Angle deviation, 95th percentile (F) | 32.96 | 32.56 | 24.38 | 23.88 | 18.22 | 17.90 | 25.01 | 30.48 | 25.01 | 16.49 | — | — |
| Mean aspect ratio (F) | 1.403 | 1.826 | 1.283 | 1.546 | 1.495 | 1.416 | 2.364 | 1.612 | 2.364 | 1.378 | — | — |
| Area variation (F) | 0.324 | 0.590 | 0.224 | 0.389 | 0.396 | 0.473 | 0.716 | 0.393 | 0.716 | 0.437 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 36.59 | 31.05 | 46.14 | 35.86 | 24.67 | 24.80 | 5.98 | 39.35 | 5.98 | 14.86 | — | — |
| Mean distance, ‰ (H) | 1.268 | 3.042 | 1.460 | 1.109 | 0.812 | 0.715 | 0.565 | 1.366 | 0.565 | 0.365 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 54.0 | 66.6 | 99.5 | 66.0 | 31.3 | 6.8 | 6.7 | 75.2 | 9.8 | 5.0 | — | — |
| Volume kept, % of the source (V) | 98.56 | — | 98.55 | 99.78 | 99.94 | 100.02 | 99.24 | 98.42 | 99.24 | 100.00 | — | — |
| Edges on closed loops, % (L) | 1.7 | — | — | — | 21.3 | 13.4 | 7.9 | — | — | 15.9 | — | — |
| Edges on spiralling loops, % (L) | 9.6 | — | — | — | 19.8 | 6.2 | 15.1 | — | — | 10.0 | — | — |
| Spiralling loops (L) | 14 | — | — | — | 10 | 18 | 20 | — | — | 17 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 88.6 | — | — | — | 58.9 | 80.4 | 77.1 | — | — | 74.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 250 | — | — | — | 155 | 137 | 199 | — | — | 199 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 4.3 | 64.0 | 4.2 | 17.8 | 0.2 | 0.0 | 104.5 | 19.4 | 0.0 | 1.9 | — | — |
| Time, seconds (T) | 3.31 | 1.22 | 2.70 | 8.13 | 4.19 | 5.96 | 19.04 | 17.16 | 19.04 | 11.18 | — | — |

## human_male (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 854 | 2814 | 2710 | 3013 | 2979 | 5322 | 2598 | 5322 | 5408 | 10590 | 9426 |
| Irregular vertices (S) | — | 88 | 194 | 68 | 98 | 158 | 248 | 110 | 248 | 191 | 476 | 141 |
| … of which above the minimum | — | — | 178 | — | 90 | 150 | — | — | — | 183 | — | 133 |
| Base complex patches (C) | — | — | 2014 | — | 1005 | 1159 | — | — | — | 1956 | — | 5382 |
| Alignment, degrees (A) | — | 9.53 | 10.54 | 7.00 | 7.96 | 7.82 | 6.04 | 6.18 | 6.80 | 7.24 | 9.88 | 8.45 |
| Mean angle deviation, degrees (F) | — | 10.46 | 7.19 | 6.01 | 5.54 | 6.04 | 6.62 | 6.87 | 6.62 | 4.50 | 9.24 | 6.33 |
| Angle deviation, 95th percentile (F) | — | 37.89 | 25.75 | 21.11 | 19.18 | 20.29 | 22.36 | 21.99 | 22.36 | 14.95 | 26.27 | 18.39 |
| Mean aspect ratio (F) | — | 2.541 | 1.301 | 1.355 | 1.583 | 1.400 | 1.879 | 1.436 | 1.879 | 1.312 | 1.616 | 1.333 |
| Area variation (F) | — | 0.757 | 0.248 | 0.337 | 0.425 | 0.648 | 0.568 | 0.327 | 0.568 | 0.523 | 1.417 | 0.294 |
| Hausdorff, ‰ of the diagonal (H) | — | 31.67 | 47.23 | 38.14 | 15.53 | 12.14 | 5.29 | 48.08 | 5.29 | 12.20 | 6.47 | 8.89 |
| Mean distance, ‰ (H) | — | 5.764 | 4.380 | 2.749 | 1.425 | 0.908 | 0.488 | 3.106 | 0.488 | 0.652 | 0.172 | 0.557 |
| Thin parts: mean distance, % of a quad edge (V) | — | 79.0 | 84.2 | 57.3 | 22.0 | 11.3 | 5.3 | 64.6 | 7.3 | 11.4 | 1.5 | 25.0 |
| Volume kept, % of the source (V) | — | — | 97.16 | 99.70 | 99.91 | 99.97 | 98.88 | — | 98.88 | 100.01 | 98.80 | 99.93 |
| Edges on closed loops, % (L) | — | — | 14.8 | 35.4 | 33.3 | 30.5 | 28.7 | — | 28.7 | 31.9 | 31.6 | 22.1 |
| Edges on spiralling loops, % (L) | — | — | 7.8 | 4.4 | 6.9 | 4.6 | 5.9 | — | 5.9 | 5.5 | 2.5 | 32.6 |
| Spiralling loops (L) | — | — | 8 | 8 | 13 | 8 | 7 | — | 7 | 16 | 3 | 44 |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 76.8 | 60.3 | 59.8 | 64.8 | 65.4 | — | 65.4 | 62.6 | 65.8 | 45.2 |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 86 | 242 | 314 | 121 | 177 | — | 177 | 270 | 274 | 467 |
| Edges on loops from a border to a border, % (L) | — | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 71.5 | 6.2 | 9.7 | 0.4 | 0.7 | 77.4 | 13.4 | 0.0 | 1.6 | 0.0 | 11.0 |
| Time, seconds (T) | — | 4.48 | 22.16 | 18.09 | 16.34 | 16.96 | 17.66 | 17.14 | 17.66 | 21.47 | — | 75.16 |

Failures: quadriflow: cancelled.

## human_female (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 689 | 2837 | 2766 | 2981 | 3040 | 5391 | 2635 | 5391 | 5309 | 10590 | 10602 |
| Irregular vertices (S) | — | 77 | 204 | 92 | 110 | 172 | 248 | 142 | 248 | 233 | 476 | 280 |
| … of which above the minimum | — | — | 196 | — | 102 | 164 | — | — | — | 225 | — | 272 |
| Base complex patches (C) | — | — | 2366 | — | 1399 | 1282 | — | — | — | 2291 | — | 3904 |
| Alignment, degrees (A) | — | 11.80 | 10.59 | 6.68 | 7.37 | 7.80 | 5.43 | 6.33 | 6.12 | 7.03 | 9.43 | 7.20 |
| Mean angle deviation, degrees (F) | — | 14.87 | 7.68 | 6.60 | 4.75 | 5.31 | 6.40 | 7.65 | 6.40 | 4.46 | 8.82 | 4.44 |
| Angle deviation, 95th percentile (F) | — | 40.05 | 25.19 | 21.54 | 16.79 | 18.78 | 21.59 | 25.61 | 21.59 | 16.24 | 25.47 | 14.29 |
| Mean aspect ratio (F) | — | 2.191 | 1.291 | 1.348 | 1.473 | 1.389 | 1.900 | 1.425 | 1.900 | 1.311 | 1.657 | 1.281 |
| Area variation (F) | — | 0.762 | 0.239 | 0.327 | 0.408 | 0.643 | 0.585 | 0.374 | 0.585 | 0.545 | 1.432 | 0.482 |
| Hausdorff, ‰ of the diagonal (H) | — | 40.87 | 56.57 | 39.14 | 17.10 | 13.94 | 7.43 | 43.64 | 7.43 | 7.59 | 4.44 | 4.92 |
| Mean distance, ‰ (H) | — | 6.121 | 4.562 | 2.685 | 1.416 | 1.074 | 0.431 | 2.663 | 0.431 | 0.441 | 0.156 | 0.181 |
| Thin parts: mean distance, % of a quad edge (V) | — | 87.6 | 97.9 | 57.7 | 27.1 | 17.6 | 5.3 | 56.1 | 7.5 | 8.8 | 1.4 | 6.1 |
| Volume kept, % of the source (V) | — | 85.52 | 96.97 | 99.64 | 99.90 | 100.01 | 98.76 | 97.05 | 98.76 | 100.03 | 98.78 | 100.02 |
| Edges on closed loops, % (L) | — | 11.9 | 8.2 | 24.7 | 26.7 | 28.9 | 29.4 | 32.3 | 29.4 | 28.4 | 31.6 | 31.6 |
| Edges on spiralling loops, % (L) | — | 20.2 | 9.7 | 17.5 | 7.5 | 4.4 | 1.4 | 3.6 | 1.4 | 4.6 | 2.5 | 3.5 |
| Spiralling loops (L) | — | 18 | 18 | 15 | 12 | 15 | 5 | 13 | 5 | 14 | 3 | 8 |
| Edges on loops that stop at an irregular vertex, % (L) | — | 67.9 | 80.5 | 57.8 | 65.8 | 66.7 | 69.2 | 64.1 | 69.2 | 67.0 | 65.8 | 64.9 |
| Longest loop that stops at an irregular vertex, edges (L) | — | 46 | 108 | 188 | 223 | 187 | 211 | 185 | 211 | 268 | 274 | 325 |
| Edges on loops from a border to a border, % (L) | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 77.0 | 5.4 | 7.8 | 0.6 | 1.3 | 79.7 | 12.2 | 0.0 | 1.5 | 0.0 | 0.1 |
| Time, seconds (T) | — | 4.77 | 20.88 | 17.37 | 17.65 | 17.88 | 21.63 | 19.65 | 21.63 | 28.24 | — | 33.99 |

Failures: quadriflow: cancelled.

## spot (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2964 | 1284 | 3044 | 2659 | 2981 | 2988 | 3478 | 2686 | 3478 | 3461 | — | — |
| Irregular vertices (S) | 68 | 68 | 153 | 70 | 70 | 70 | 82 | 87 | 82 | 70 | — | — |
| … of which above the minimum | 60 | 60 | 145 | 62 | 62 | 62 | 74 | 79 | 74 | 62 | — | — |
| Base complex patches (C) | 2902 | 1022 | 2978 | 1987 | 1253 | 1404 | 2428 | 2118 | 2428 | 1375 | — | — |
| Alignment, degrees (A) | 18.12 | 12.08 | 17.17 | 9.37 | 8.87 | 8.14 | 6.29 | 7.56 | 6.35 | 7.90 | — | — |
| Mean angle deviation, degrees (F) | 7.25 | 8.87 | 5.59 | 9.51 | 5.60 | 5.61 | 6.35 | 7.05 | 6.35 | 5.28 | — | — |
| Angle deviation, 95th percentile (F) | 20.42 | 24.77 | 18.71 | 24.15 | 15.72 | 16.00 | 17.74 | 19.43 | 17.74 | 15.09 | — | — |
| Mean aspect ratio (F) | 1.194 | 1.451 | 1.179 | 1.344 | 1.363 | 1.348 | 1.460 | 1.318 | 1.460 | 1.359 | — | — |
| Area variation (F) | 0.177 | 0.666 | 0.172 | 0.289 | 0.349 | 0.436 | 0.392 | 0.268 | 0.392 | 0.451 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 17.71 | 17.26 | 19.38 | 8.80 | 4.54 | 3.58 | 6.64 | 10.63 | 6.64 | 3.49 | — | — |
| Mean distance, ‰ (H) | 0.448 | 1.588 | 0.479 | 0.237 | 0.207 | 0.199 | 0.372 | 0.476 | 0.372 | 0.173 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 56.3 | 54.6 | 53.8 | 14.8 | 5.1 | 3.9 | 8.5 | 21.2 | — | — | — | — |
| Volume kept, % of the source (V) | 99.37 | 97.44 | 99.35 | 100.00 | 100.03 | 100.02 | 99.35 | 99.27 | 99.35 | 100.01 | — | — |
| Edges on closed loops, % (L) | 0.0 | 10.2 | — | — | 31.7 | 27.5 | 15.5 | — | — | 32.0 | — | — |
| Edges on spiralling loops, % (L) | 22.5 | 5.6 | — | — | 8.9 | 9.0 | 16.7 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 10 | 3 | — | — | 6 | 3 | 7 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 77.5 | 84.2 | — | — | 59.4 | 63.4 | 67.8 | — | — | 68.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 299 | 90 | — | — | 180 | 170 | 156 | — | — | 240 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 1.2 | 57.2 | 1.5 | 11.4 | 0.6 | 0.4 | 15.9 | 10.5 | 0.0 | 0.5 | — | — |
| Time, seconds (T) | 1.80 | 0.94 | 1.23 | 6.69 | 3.06 | 4.39 | 19.14 | 15.16 | 19.14 | 3.93 | — | — |

## bob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2928 | 1583 | 3040 | 2642 | 2967 | 2951 | 2920 | 2696 | 2920 | 2977 | — | — |
| Irregular vertices (S) | 32 | 18 | 130 | 20 | 20 | 20 | 28 | 30 | 28 | 20 | — | — |
| … of which above the minimum | 32 | 18 | 130 | 20 | 20 | 20 | 28 | 30 | 28 | 20 | — | — |
| Base complex patches (C) | 2903 | 533 | 3040 | 714 | 219 | 292 | 2390 | 2568 | 2390 | 230 | — | — |
| Alignment, degrees (A) | 19.37 | 6.64 | 20.80 | 4.37 | 3.97 | 3.69 | 4.85 | 5.15 | 4.84 | 3.97 | — | — |
| Mean angle deviation, degrees (F) | 6.41 | 7.07 | 5.33 | 4.82 | 3.39 | 3.25 | 5.52 | 5.94 | 5.52 | 3.68 | — | — |
| Angle deviation, 95th percentile (F) | 18.61 | 21.04 | 17.96 | 14.72 | 9.41 | 9.04 | 15.59 | 15.77 | 15.59 | 10.78 | — | — |
| Mean aspect ratio (F) | 1.175 | 2.090 | 1.158 | 1.409 | 1.362 | 1.367 | 1.368 | 1.360 | 1.368 | 1.347 | — | — |
| Area variation (F) | 0.159 | 0.474 | 0.161 | 0.339 | 0.401 | 0.423 | 0.360 | 0.324 | 0.360 | 0.450 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 6.82 | 4.03 | 7.32 | 2.49 | 2.24 | 1.49 | 1.96 | 2.27 | 1.96 | 1.90 | — | — |
| Mean distance, ‰ (H) | 0.309 | 0.960 | 0.325 | 0.152 | 0.136 | 0.134 | 0.322 | 0.355 | 0.322 | 0.137 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.48 | 98.10 | 99.44 | 100.01 | 100.02 | 100.02 | 99.37 | 99.31 | 99.37 | 100.02 | — | — |
| Edges on closed loops, % (L) | 0.6 | 45.2 | — | — | 75.3 | 70.3 | 9.3 | — | — | 74.2 | — | — |
| Edges on spiralling loops, % (L) | 35.4 | 11.1 | — | — | 0.0 | 0.0 | 45.9 | — | — | 2.0 | — | — |
| Spiralling loops (L) | 9 | 3 | — | — | 0 | 0 | 12 | — | — | 1 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 64.6 | 43.7 | — | — | 24.7 | 29.7 | 44.8 | — | — | 23.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 309 | 172 | — | — | 112 | 121 | 218 | — | — | 117 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 2.4 | 47.2 | 1.3 | 11.9 | 1.1 | 1.6 | 2.7 | 10.1 | 0.0 | 2.0 | — | — |
| Time, seconds (T) | 1.72 | 1.14 | 1.45 | 6.22 | 1.96 | 2.25 | 17.24 | 17.35 | 17.24 | 2.41 | — | — |

## blub (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2698 | 1343 | 3036 | 2680 | 3045 | 2943 | 3889 | 2703 | 3889 | 3869 | — | — |
| Irregular vertices (S) | 52 | 47 | 158 | 43 | 46 | 56 | 55 | 46 | 55 | 52 | — | — |
| … of which above the minimum | 44 | — | 150 | 35 | 38 | 48 | 47 | 38 | 47 | 44 | — | — |
| Base complex patches (C) | 2584 | — | 2922 | 902 | 629 | 1127 | 2003 | 2057 | 2003 | 675 | — | — |
| Alignment, degrees (A) | 15.95 | 10.62 | 15.45 | 7.64 | 7.71 | 7.54 | 5.26 | 7.01 | 5.48 | 7.29 | — | — |
| Mean angle deviation, degrees (F) | 10.72 | 9.14 | 5.55 | 8.08 | 4.68 | 5.00 | 6.30 | 7.24 | 6.30 | 4.80 | — | — |
| Angle deviation, 95th percentile (F) | 28.70 | 28.42 | 19.72 | 22.68 | 13.66 | 14.88 | 16.74 | 18.88 | 16.74 | 14.98 | — | — |
| Mean aspect ratio (F) | 1.271 | 2.046 | 1.192 | 1.327 | 1.273 | 1.300 | 1.828 | 1.351 | 1.828 | 1.337 | — | — |
| Area variation (F) | 0.261 | 0.522 | 0.178 | 0.266 | 0.283 | 0.398 | 0.445 | 0.269 | 0.445 | 0.373 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 40.70 | 17.71 | 15.68 | 12.42 | 8.83 | 8.72 | 5.19 | 11.32 | 5.19 | 6.76 | — | — |
| Mean distance, ‰ (H) | 0.588 | 1.082 | 0.390 | 0.254 | 0.182 | 0.171 | 0.230 | 0.392 | 0.230 | 0.123 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 46.5 | 17.7 | 24.8 | 19.4 | 9.0 | 9.4 | 7.7 | 20.0 | 10.8 | 5.0 | — | — |
| Volume kept, % of the source (V) | 98.93 | — | 99.17 | 99.88 | 99.96 | 99.98 | 99.55 | 99.13 | 99.55 | 100.00 | — | — |
| Edges on closed loops, % (L) | 0.0 | — | — | — | 51.0 | 34.4 | 25.2 | — | — | 54.1 | — | — |
| Edges on spiralling loops, % (L) | 32.5 | — | — | — | 6.4 | 11.5 | 20.5 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 13 | — | — | — | 1 | 4 | 6 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 64.2 | — | — | — | 42.6 | 54.2 | 54.3 | — | — | 45.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 225 | — | — | — | 194 | 160 | 201 | — | — | 258 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 10.1 | 55.2 | 1.2 | 10.7 | 1.5 | 1.9 | 29.6 | 9.9 | 0.0 | 0.5 | — | — |
| Time, seconds (T) | 1.68 | 1.23 | 1.25 | 6.03 | 2.27 | 2.55 | 14.34 | 12.32 | 14.34 | 3.65 | — | — |

## pig (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2675 | 1808 | 3016 | 2680 | 3023 | 3049 | 4698 | 2552 | 4698 | 4613 | — | — |
| Irregular vertices (S) | 122 | 99 | 209 | 82 | 78 | 104 | 106 | 96 | 106 | 104 | — | — |
| … of which above the minimum | 114 | — | 192 | — | 70 | 96 | 98 | 88 | 98 | 96 | — | — |
| Base complex patches (C) | 2612 | — | 2912 | — | 1597 | 1737 | 3004 | 1840 | 3004 | 2289 | — | — |
| Alignment, degrees (A) | 17.41 | 11.38 | 16.24 | 10.81 | 10.79 | 8.90 | 7.22 | 9.36 | 7.34 | 8.85 | — | — |
| Mean angle deviation, degrees (F) | 14.32 | 8.90 | 6.83 | 8.29 | 6.64 | 5.72 | 7.22 | 7.93 | 7.22 | 5.46 | — | — |
| Angle deviation, 95th percentile (F) | 41.52 | 29.13 | 23.86 | 23.45 | 16.90 | 17.20 | 21.68 | 23.91 | 21.68 | 15.71 | — | — |
| Mean aspect ratio (F) | 1.383 | 1.804 | 1.244 | 1.266 | 1.250 | 1.289 | 1.876 | 1.322 | 1.876 | 1.301 | — | — |
| Area variation (F) | 0.398 | 0.713 | 0.210 | 0.262 | 0.274 | 0.471 | 0.636 | 0.287 | 0.636 | 0.471 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 44.59 | 52.50 | 48.32 | 25.35 | 17.17 | 10.77 | 10.22 | 12.72 | 10.22 | 7.16 | — | — |
| Mean distance, ‰ (H) | 1.056 | 2.185 | 1.012 | 0.532 | 0.393 | 0.290 | 0.375 | 0.724 | 0.375 | 0.200 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 67.8 | 29.1 | 26.2 | 25.0 | 16.0 | 3.8 | 3.7 | 16.4 | 6.3 | 7.4 | — | — |
| Volume kept, % of the source (V) | 98.78 | 97.54 | 98.84 | 99.69 | 99.87 | 100.00 | 99.54 | 98.93 | 99.54 | 99.99 | — | — |
| Edges on closed loops, % (L) | 0.4 | 10.6 | — | — | 27.4 | 23.4 | 18.0 | — | — | 29.7 | — | — |
| Edges on spiralling loops, % (L) | 9.5 | 5.7 | — | — | 8.8 | 5.6 | 24.2 | — | — | 8.7 | — | — |
| Spiralling loops (L) | 12 | 5 | — | — | 4 | 6 | 20 | — | — | 13 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 90.5 | 83.7 | — | — | 63.8 | 71.0 | 57.8 | — | — | 61.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 218 | 113 | — | — | 178 | 194 | 191 | — | — | 233 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 10.8 | 39.7 | 1.1 | 10.7 | 0.8 | 1.6 | 56.6 | 14.9 | 0.0 | 1.8 | — | — |
| Time, seconds (T) | 1.65 | 1.53 | 1.15 | 9.77 | 3.02 | 3.77 | 14.59 | 16.02 | 14.59 | 5.03 | — | — |

## jerry (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 1/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3122 | 1102 | 2959 | 2595 | 2976 | 3012 | 6872 | 2573 | 6872 | 6751 | — | — |
| Irregular vertices (S) | 104 | 84 | 265 | 97 | 84 | 160 | 206 | 181 | 206 | 264 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2211 | — | 2959 | 1552 | 1774 | 2536 | — | 2088 | — | 5542 | — | — |
| Alignment, degrees (A) | 17.93 | 17.98 | 18.35 | 14.87 | 14.22 | 13.88 | 11.02 | 13.58 | 11.29 | 12.48 | — | — |
| Mean angle deviation, degrees (F) | 11.25 | 15.26 | 8.58 | 9.60 | 6.40 | 6.97 | 9.67 | 10.03 | 9.67 | 6.32 | — | — |
| Angle deviation, 95th percentile (F) | 29.73 | 53.91 | 29.29 | 24.87 | 16.65 | 21.41 | 26.54 | 28.28 | 26.54 | 18.07 | — | — |
| Mean aspect ratio (F) | 1.301 | 2.330 | 1.357 | 1.324 | 1.309 | 1.349 | 2.063 | 1.483 | 2.063 | 1.292 | — | — |
| Area variation (F) | 0.278 | 2.551 | 0.258 | 0.272 | 0.319 | 0.557 | 0.724 | 0.324 | 0.724 | 0.541 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 38.82 | 200.69 | 49.21 | 48.31 | 28.08 | 21.29 | 6.74 | 15.07 | 6.74 | 13.79 | — | — |
| Mean distance, ‰ (H) | 1.467 | 5.546 | 2.158 | 1.786 | 1.126 | 0.842 | 0.529 | 1.488 | 0.529 | 0.337 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 23.4 | 76.1 | 40.3 | 57.3 | 32.2 | 15.1 | 4.8 | 19.2 | 6.8 | 10.2 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 4.9 | 2.1 | 3.3 | — | — | 3.2 | — | — |
| Edges on spiralling loops, % (L) | 0.7 | 13.4 | — | — | 1.7 | 1.3 | 6.4 | — | — | 1.3 | — | — |
| Spiralling loops (L) | 1 | 6 | — | — | 1 | 3 | 10 | — | — | 3 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 82.0 | 86.6 | — | — | 75.0 | 90.5 | 85.0 | — | — | 88.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 155 | 64 | — | — | 139 | 92 | 282 | — | — | 190 | — | — |
| Edges on loops from a border to a border, % (L) | 17.0 | 0.0 | — | — | 18.4 | 6.0 | 5.3 | — | — | 7.1 | — | — |
| Off budget, % (N) | 4.1 | 63.3 | 1.4 | 13.5 | 0.8 | 0.4 | 129.1 | 14.2 | 0.0 | 1.8 | — | — |
| Time, seconds (T) | 1.74 | 1.38 | 0.59 | 7.59 | 2.14 | 2.94 | 6.74 | 6.79 | 6.74 | 8.00 | — | — |

## rose (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2085 | 636 | 2226 | 1923 | 3314 | 3580 | 35874 | 2689 | 35874 | 36157 | — | — |
| Irregular vertices (S) | 539 | 243 | 499 | 252 | 853 | 1025 | 1772 | 763 | 1772 | 2389 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | 1773 | — | — |
| Base complex patches (C) | 1872 | — | 1954 | — | — | — | — | — | — | 24636 | — | — |
| Alignment, degrees (A) | 18.30 | 19.39 | 19.07 | 16.12 | 14.62 | 14.94 | 8.98 | 14.12 | 11.16 | 9.26 | — | — |
| Mean angle deviation, degrees (F) | 16.45 | 31.06 | 15.58 | 13.29 | 19.53 | 19.85 | 9.06 | 13.63 | 9.06 | 7.18 | — | — |
| Angle deviation, 95th percentile (F) | 48.25 | 82.38 | 48.45 | 38.66 | 64.20 | 62.50 | 26.38 | 39.32 | 26.38 | 22.18 | — | — |
| Mean aspect ratio (F) | 1.647 | 5.541 | 1.752 | 1.809 | 4.255 | 6.200 | 1.790 | 1.653 | 1.790 | 1.536 | — | — |
| Area variation (F) | 0.587 | 1.193 | 0.465 | 0.441 | 1.318 | 1.661 | 1.388 | 0.599 | 1.388 | 0.524 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 75.61 | 108.28 | 79.25 | 72.60 | 33.22 | 31.50 | 74.46 | 78.76 | 74.46 | 7.75 | — | — |
| Mean distance, ‰ (H) | 3.765 | 21.969 | 4.183 | 3.665 | 2.763 | 3.766 | 0.842 | 3.206 | 0.842 | 0.160 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 39.1 | 157.4 | 46.1 | 40.1 | 26.5 | 35.2 | 11.9 | 35.3 | 116.7 | 9.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 79.13 | — | — | — | — | 99.64 | — | — |
| Edges on closed loops, % (L) | 3.2 | — | — | — | 13.2 | — | — | — | — | 16.4 | — | — |
| Edges on spiralling loops, % (L) | 17.6 | — | — | — | 26.7 | — | — | — | — | 9.7 | — | — |
| Spiralling loops (L) | 82 | — | — | — | 400 | — | — | — | — | 233 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 79.7 | — | — | — | 60.2 | — | — | — | — | 73.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 38 | — | — | — | 60 | — | — | — | — | 254 | — | — |
| Edges on loops from a border to a border, % (L) | 0.5 | — | — | — | 0.0 | — | — | — | — | 0.0 | — | — |
| Off budget, % (N) | 30.5 | 78.8 | 25.8 | 35.9 | 10.5 | 19.3 | 1095.8 | 10.4 | 0.0 | 0.8 | — | — |
| Time, seconds (T) | 2.21 | 1.16 | 2.12 | 16.37 | 3.75 | 9.48 | 9.53 | 10.86 | 9.53 | 38.63 | — | — |

## bridge (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 1893 | 2903 | 2750 | 3020 | 2954 | 4533 | 2726 | 4533 | 4486 | — | — |
| Irregular vertices (S) | — | 63 | 151 | 53 | 50 | 46 | 81 | 64 | 81 | 52 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1879 | 520 | 854 | 705 | — | — | — | 1209 | — | — |
| Alignment, degrees (A) | — | 9.17 | 12.58 | 8.69 | 7.94 | 7.48 | 6.49 | 7.93 | 7.33 | 7.93 | — | — |
| Mean angle deviation, degrees (F) | — | 7.60 | 6.12 | 7.12 | 3.95 | 4.08 | 6.65 | 7.20 | 6.65 | 4.21 | — | — |
| Angle deviation, 95th percentile (F) | — | 22.56 | 20.51 | 20.62 | 12.01 | 12.05 | 18.96 | 19.72 | 18.96 | 11.73 | — | — |
| Mean aspect ratio (F) | — | 1.778 | 1.215 | 1.305 | 1.261 | 1.251 | 1.753 | 1.413 | 1.753 | 1.269 | — | — |
| Area variation (F) | — | 0.765 | 0.174 | 0.232 | 0.285 | 0.441 | 0.421 | 0.275 | 0.421 | 0.411 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 14.76 | 28.19 | 7.13 | 11.19 | 6.74 | 6.05 | 7.00 | 6.05 | 7.13 | — | — |
| Mean distance, ‰ (H) | — | 1.042 | 0.638 | 0.500 | 0.446 | 0.444 | 0.414 | 0.670 | 0.414 | 0.344 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 19.4 | — | — | 20.5 | 25.7 | 15.1 | — | — | 24.0 | — | — |
| Edges on spiralling loops, % (L) | — | 8.2 | — | — | 5.8 | 4.6 | 7.8 | — | — | 1.3 | — | — |
| Spiralling loops (L) | — | 4 | — | — | 2 | 2 | 7 | — | — | 2 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 62.1 | — | — | 51.0 | 46.9 | 60.0 | — | — | 52.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 70 | — | — | 107 | 132 | 145 | — | — | 121 | — | — |
| Edges on loops from a border to a border, % (L) | — | 10.3 | — | — | 22.7 | 22.8 | 17.1 | — | — | 21.9 | — | — |
| Off budget, % (N) | — | 36.9 | 3.2 | 8.3 | 0.7 | 1.5 | 51.1 | 9.1 | 0.0 | 1.0 | — | — |
| Time, seconds (T) | — | 1.21 | 2.98 | 10.79 | 3.49 | 3.66 | 11.41 | 11.88 | 11.41 | 6.21 | — | — |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2890 | 1855 | 2938 | 2976 | 3028 | 2978 | 2815 | 2815 | 2815 | 2780 | — | — |
| Irregular vertices (S) | 28 | 117 | 204 | 24 | 24 | 24 | 24 | 24 | 24 | 24 | — | — |
| … of which above the minimum | 20 | — | 196 | 16 | 16 | 16 | 16 | 16 | 16 | 16 | — | — |
| Base complex patches (C) | 619 | — | 1782 | 78 | 78 | 78 | 501 | 501 | 501 | 92 | — | — |
| Alignment, degrees (A) | 12.97 | 15.75 | 13.96 | 11.65 | 12.19 | 10.48 | 10.90 | 10.90 | 10.90 | 10.37 | — | — |
| Mean angle deviation, degrees (F) | 2.66 | 12.33 | 6.10 | 2.61 | 1.34 | 1.59 | 3.02 | 3.02 | 3.02 | 1.56 | — | — |
| Angle deviation, 95th percentile (F) | 10.11 | 37.42 | 25.03 | 10.79 | 4.98 | 6.04 | 12.04 | 12.04 | 12.04 | 6.33 | — | — |
| Mean aspect ratio (F) | 1.116 | 2.476 | 1.233 | 1.163 | 1.158 | 1.164 | 1.326 | 1.326 | 1.326 | 1.138 | — | — |
| Area variation (F) | 0.100 | 0.672 | 0.199 | 0.126 | 0.125 | 0.365 | 0.196 | 0.196 | 0.196 | 0.400 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 5.54 | 17.76 | 9.41 | 0.80 | 11.60 | 0.48 | 0.89 | 0.89 | 0.89 | 0.51 | — | — |
| Mean distance, ‰ (H) | 0.114 | 1.229 | 0.067 | 0.054 | 0.068 | 0.056 | 0.068 | 0.068 | 0.068 | 0.061 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.15 | — | 100.14 | 100.00 | 100.00 | 100.00 | 100.12 | 100.12 | 100.12 | 100.00 | — | — |
| Edges on closed loops, % (L) | 51.3 | — | — | — | 83.8 | 84.0 | 55.7 | — | — | 82.0 | — | — |
| Edges on spiralling loops, % (L) | 26.0 | — | — | — | 0.0 | 0.0 | 24.0 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 7 | — | — | — | 0 | 0 | 7 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 21.7 | — | — | — | 16.2 | 16.0 | 20.3 | — | — | 18.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 90 | — | — | — | 57 | 48 | 91 | — | — | 45 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.7 | 38.2 | 2.1 | 0.8 | 0.9 | 0.7 | 6.2 | 6.2 | 0.0 | 1.2 | — | — |
| Time, seconds (T) | 2.72 | 1.14 | 0.71 | 4.01 | 1.91 | 2.22 | 2.83 | 2.88 | 2.83 | 1.92 | — | — |

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
| Angle deviation, 95th percentile (F) | 0.43 | 11.71 | 4.89 | 2.01 | 0.01 | 0.85 | 0.00 | 0.00 | 0.00 | 0.93 | — | — |
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
| Time, seconds (T) | 1.21 | 0.63 | 0.20 | 5.18 | 1.43 | 1.53 | 2.97 | 3.00 | 2.97 | 2.06 | — | — |

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
| Time, seconds (T) | 1.19 | 0.82 | 0.36 | 23.07 | 1.23 | 1.33 | 2.73 | 2.90 | 2.73 | 1.40 | — | — |

## cylinder (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2900 | 1306 | 3100 | 2533 | 2988 | 3035 | 2458 | 2458 | 2458 | 2458 | — | — |
| Irregular vertices (S) | 8 | 8 | 91 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | 0 | 83 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 1003 | 68 | 3052 | 643 | 726 | 541 | 408 | 408 | 408 | 186 | — | — |
| Alignment, degrees (A) | 4.18 | 4.72 | 5.46 | 1.56 | 1.50 | 1.87 | 1.52 | 1.52 | 1.64 | 1.42 | — | — |
| Mean angle deviation, degrees (F) | 4.32 | 8.13 | 4.18 | 4.16 | 3.63 | 3.17 | 3.22 | 3.22 | 3.22 | 2.56 | — | — |
| Angle deviation, 95th percentile (F) | 17.78 | 26.13 | 15.60 | 17.67 | 14.23 | 8.20 | 11.87 | 11.87 | 11.87 | 8.98 | — | — |
| Mean aspect ratio (F) | 1.087 | 2.019 | 1.132 | 1.355 | 1.384 | 1.563 | 1.511 | 1.511 | 1.511 | 1.725 | — | — |
| Area variation (F) | 0.103 | 0.640 | 0.140 | 0.374 | 0.370 | 0.246 | 0.403 | 0.403 | 0.403 | 0.278 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 2.07 | 9.51 | 3.74 | 1.00 | 0.93 | 0.90 | 1.43 | 1.43 | 1.43 | 1.13 | — | — |
| Mean distance, ‰ (H) | 0.144 | 0.790 | 0.121 | 0.145 | 0.123 | 0.127 | 0.369 | 0.369 | 0.369 | 0.198 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.91 | 99.21 | 99.90 | 100.00 | 100.00 | 100.00 | 99.56 | 99.56 | 99.56 | 100.00 | — | — |
| Edges on closed loops, % (L) | 33.0 | 74.5 | — | — | 37.9 | 41.1 | 41.7 | — | — | 46.9 | — | — |
| Edges on spiralling loops, % (L) | 55.4 | 0.0 | — | — | 26.5 | 54.4 | 39.4 | — | — | 23.2 | — | — |
| Spiralling loops (L) | 6 | 0 | — | — | 1 | 6 | 2 | — | — | 2 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 11.6 | 25.5 | — | — | 35.7 | 4.5 | 18.9 | — | — | 29.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 207 | 178 | — | — | 739 | 112 | 345 | — | — | 280 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.3 | 56.5 | 3.3 | 15.6 | 0.4 | 1.2 | 18.1 | 18.1 | 0.0 | 0.0 | — | — |
| Time, seconds (T) | 1.05 | 0.65 | 0.17 | 2.92 | 1.40 | 1.63 | 2.68 | 2.49 | 2.68 | 1.24 | — | — |

## venturi (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 4046 | 2489 | 2412 | 2789 | 3134 | 14193 | 9910 | 14193 | 14351 | — | — |
| Irregular vertices (S) | — | 1527 | 432 | 169 | 201 | 412 | 613 | 613 | 613 | 838 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1464 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 11.84 | 11.84 | 11.55 | 11.48 | 14.34 | 7.08 | 6.76 | 8.23 | 10.59 | — | — |
| Mean angle deviation, degrees (F) | — | 48.87 | 13.84 | 8.20 | 8.73 | 11.81 | 9.06 | 9.35 | 9.06 | 6.67 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.70 | 52.08 | 25.25 | 29.09 | 42.69 | 41.05 | 48.55 | 41.05 | 25.35 | — | — |
| Mean aspect ratio (F) | — | 12.801 | 1.928 | 1.344 | 1.411 | 1.719 | 2.237 | 2.425 | 2.237 | 1.395 | — | — |
| Area variation (F) | — | 2.373 | 0.475 | 0.291 | 0.335 | 0.488 | 1.573 | 1.464 | 1.573 | 0.334 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 71.10 | 40.69 | 38.37 | 40.59 | 40.59 | 14.05 | 18.60 | 14.05 | 23.03 | — | — |
| Mean distance, ‰ (H) | — | 5.233 | 4.298 | 5.725 | 4.643 | 4.199 | 0.337 | 0.555 | 0.337 | 1.145 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 24.5 | 28.2 | 36.2 | 28.5 | 25.8 | 1.7 | 2.6 | 3.0 | 24.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 9.1 | 4.1 | 5.9 | — | — | 19.7 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 39.8 | 26.3 | 3.4 | — | — | 23.3 | — | — |
| Spiralling loops (L) | — | — | — | — | 64 | 48 | 18 | — | — | 31 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 51.1 | 69.6 | 29.5 | — | — | 54.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 120 | 120 | 112 | — | — | 367 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | 61.3 | — | — | 2.5 | — | — |
| Off budget, % (N) | — | 34.9 | 17.0 | 19.6 | 7.0 | 4.5 | 373.1 | 230.3 | 0.0 | 1.1 | — | — |
| Time, seconds (T) | — | 1.67 | 1.05 | 22.47 | 11.17 | 16.85 | 5.47 | 4.22 | 5.47 | 113.89 | — | — |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 3/5 | 0/1 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 1283 | 2836 | 2515 | 2968 | 2962 | 6269 | 2409 | 6269 | 6327 | — | — |
| Irregular vertices (S) | — | 87 | 223 | 85 | 90 | 190 | 275 | 210 | 275 | 310 | — | — |
| … of which above the minimum | — | — | 219 | — | 82 | 182 | 267 | — | 267 | 302 | — | — |
| Base complex patches (C) | — | — | 2718 | — | 2052 | 2424 | 4923 | — | 4923 | 4523 | — | — |
| Alignment, degrees (A) | — | 16.86 | 18.44 | 13.66 | 13.48 | 12.43 | 9.30 | 11.90 | 9.51 | 10.67 | — | — |
| Mean angle deviation, degrees (F) | — | 10.34 | 7.84 | 9.13 | 6.63 | 6.77 | 9.35 | 10.70 | 9.35 | 6.60 | — | — |
| Angle deviation, 95th percentile (F) | — | 29.60 | 25.71 | 24.63 | 19.05 | 19.94 | 24.49 | 28.47 | 24.49 | 19.30 | — | — |
| Mean aspect ratio (F) | — | 1.786 | 1.295 | 1.502 | 1.553 | 1.408 | 2.322 | 1.600 | 2.322 | 1.384 | — | — |
| Area variation (F) | — | 0.654 | 0.236 | 0.339 | 0.394 | 0.771 | 0.763 | 0.383 | 0.763 | 0.908 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 30.78 | 61.14 | 38.53 | 25.80 | 24.38 | 6.85 | 42.33 | 6.85 | 11.09 | — | — |
| Mean distance, ‰ (H) | — | 2.902 | 1.839 | 1.177 | 0.786 | 0.763 | 0.574 | 1.440 | 0.574 | 0.412 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 57.6 | 145.0 | 75.2 | 26.1 | 5.5 | 4.0 | 74.8 | 6.9 | 1.6 | — | — |
| Volume kept, % of the source (V) | — | 95.87 | 98.43 | 99.70 | 99.90 | 100.01 | 99.17 | — | 99.17 | 100.01 | — | — |
| Edges on closed loops, % (L) | — | 10.2 | — | — | 15.4 | 9.1 | 10.7 | — | — | 14.7 | — | — |
| Edges on spiralling loops, % (L) | — | 16.6 | — | — | 7.8 | 17.3 | 13.9 | — | — | 7.7 | — | — |
| Spiralling loops (L) | — | 12 | — | — | 7 | 18 | 18 | — | — | 21 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 73.3 | — | — | 76.8 | 73.6 | 75.3 | — | — | 77.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 51 | — | — | 179 | 139 | 182 | — | — | 122 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 57.2 | 5.5 | 16.2 | 1.1 | 1.3 | 109.0 | 19.7 | 0.0 | 0.9 | — | — |
| Time, seconds (T) | — | 13.40 | 89.83 | 57.56 | 52.50 | 112.43 | 36.00 | 36.17 | 36.00 | 66.43 | — | — |

Failures: quadriflow: cancelled.

## watermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 2675 | 709 | 1784 | 2260 | 2249 | 2241 | 22110 | 2850 | 22110 | — | — | — |
| Irregular vertices (S) | 302 | 129 | 307 | 226 | 209 | 189 | 1250 | 484 | 1250 | — | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 2588 | — | 1676 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 14.64 | 19.69 | 16.44 | 15.73 | 15.66 | 15.66 | 10.86 | 15.13 | 12.09 | — | — | — |
| Mean angle deviation, degrees (F) | 11.57 | 21.19 | 17.11 | 11.91 | 11.32 | 11.31 | 8.01 | 11.64 | 8.01 | — | — | — |
| Angle deviation, 95th percentile (F) | 39.87 | 71.32 | 57.84 | 47.77 | 46.08 | 45.92 | 24.75 | 41.56 | 24.75 | — | — | — |
| Mean aspect ratio (F) | 1.472 | 2.842 | 1.945 | 1.601 | 1.645 | 1.645 | 1.720 | 1.617 | 1.720 | — | — | — |
| Area variation (F) | 0.335 | 0.833 | 0.490 | 0.562 | 0.756 | 0.753 | 0.834 | 0.372 | 0.834 | — | — | — |
| Hausdorff, ‰ of the diagonal (H) | 86.07 | 187.44 | 74.17 | 77.11 | 75.48 | 75.48 | 33.67 | 57.63 | 33.67 | — | — | — |
| Mean distance, ‰ (H) | 5.115 | 23.533 | 7.799 | 6.793 | 6.539 | 6.542 | 1.298 | 4.420 | 1.298 | — | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 31.4 | 104.7 | 43.1 | 39.8 | 37.2 | 37.2 | 12.7 | 26.0 | 35.7 | — | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | 99.75 | — | 99.75 | — | — | — |
| Edges on closed loops, % (L) | 2.2 | — | — | — | — | — | 5.2 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 6.9 | — | — | — | — | — | 9.1 | — | — | — | — | — |
| Spiralling loops (L) | 27 | — | — | — | — | — | 75 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.4 | — | — | — | — | — | 85.7 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 76 | — | — | — | — | — | 228 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 2.6 | — | — | — | — | — | 0.0 | — | — | — | — | — |
| Off budget, % (N) | 10.8 | 76.4 | 40.5 | 24.7 | 25.0 | 25.3 | 637.0 | 5.0 | 0.0 | — | — | — |
| Time, seconds (T) | 2.98 | 1.89 | 0.55 | 23.13 | 104.45 | 555.23 | 413.72 | 382.00 | 413.72 | — | — | — |

Failures: openclare-matched: timed out after 1800 s.

## towermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 2724 | 1088 | 1904 | 2395 | 3036 | 2304 | 16107 | 2897 | 16107 | 12530 | — | — |
| Irregular vertices (S) | 378 | 194 | 370 | 147 | 327 | 158 | 813 | 375 | 813 | 385 | — | — |
| … of which above the minimum | — | — | — | — | 23 | — | — | — | — | — | — | — |
| Base complex patches (C) | 2661 | — | 1899 | — | 2822 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | 13.28 | 16.22 | 16.31 | 12.77 | 14.09 | 12.61 | 10.58 | 11.73 | 11.77 | 12.08 | — | — |
| Mean angle deviation, degrees (F) | 11.93 | 19.35 | 19.44 | 9.80 | 14.03 | 10.16 | 8.97 | 9.30 | 8.97 | 7.35 | — | — |
| Angle deviation, 95th percentile (F) | 38.83 | 64.57 | 61.40 | 35.38 | 46.11 | 35.17 | 27.41 | 33.36 | 27.41 | 24.67 | — | — |
| Mean aspect ratio (F) | 1.501 | 2.846 | 2.098 | 1.446 | 4.475 | 1.480 | 2.149 | 1.547 | 2.149 | 1.308 | — | — |
| Area variation (F) | 0.329 | 0.711 | 0.515 | 0.322 | 0.735 | 0.330 | 0.715 | 0.327 | 0.715 | 0.285 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 56.00 | 98.83 | 77.09 | 63.94 | 62.97 | 55.25 | 27.59 | 44.98 | 27.59 | 34.83 | — | — |
| Mean distance, ‰ (H) | 3.285 | 12.802 | 6.169 | 4.799 | 3.373 | 4.940 | 1.555 | 3.480 | 1.555 | 1.230 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 16.0 | 56.9 | 34.1 | 27.6 | 20.4 | 28.1 | 11.3 | 17.6 | 25.1 | 28.9 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 93.39 | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.6 | — | — | — | 3.6 | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 10.8 | — | — | — | 8.9 | — | — | — | — | — | — | — |
| Spiralling loops (L) | 20 | — | — | — | 44 | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 88.2 | — | — | — | 87.6 | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 61 | — | — | — | 86 | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 0.4 | — | — | — | 0.0 | — | — | — | — | — | — | — |
| Off budget, % (N) | 9.2 | 63.7 | 36.5 | 20.2 | 1.2 | 23.2 | 436.9 | 3.4 | 0.0 | 22.2 | — | — |
| Time, seconds (T) | 3.14 | 1.83 | 0.47 | 12.81 | 31.21 | 23.78 | 106.09 | 101.48 | 106.09 | 128.01 | — | — |

## stonemill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 2/5 | 0/1 | 1/5 | 0/1 | 1/1 | 1/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 2635 | 1809 | 2664 | 2538 | 3040 | 3012 | 14140 | 2859 | 14140 | 14146 | — | — |
| Irregular vertices (S) | 242 | 99 | 287 | 105 | 126 | 229 | 779 | 368 | 779 | 389 | — | — |
| … of which above the minimum | 206 | — | 247 | — | 62 | 165 | — | — | — | 325 | — | — |
| Base complex patches (C) | 2605 | — | 2586 | — | 2124 | 2946 | — | — | — | 11873 | — | — |
| Alignment, degrees (A) | 13.78 | 14.83 | 14.29 | 13.89 | 13.26 | 12.98 | 11.44 | 13.23 | 12.79 | 12.16 | — | — |
| Mean angle deviation, degrees (F) | 9.37 | 11.26 | 9.56 | 8.82 | 6.08 | 7.14 | 7.97 | 8.50 | 7.97 | 5.10 | — | — |
| Angle deviation, 95th percentile (F) | 28.53 | 33.83 | 32.90 | 24.82 | 18.95 | 23.75 | 24.83 | 29.72 | 24.83 | 15.65 | — | — |
| Mean aspect ratio (F) | 1.324 | 2.004 | 1.440 | 1.264 | 1.289 | 1.418 | 1.797 | 1.392 | 1.797 | 1.229 | — | — |
| Area variation (F) | 0.239 | 0.710 | 0.288 | 0.245 | 0.266 | 0.473 | 0.725 | 0.278 | 0.725 | 0.314 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 42.66 | 106.61 | 66.01 | 59.45 | 20.35 | 23.22 | 21.42 | 39.68 | 21.42 | 11.51 | — | — |
| Mean distance, ‰ (H) | 2.229 | 4.106 | 2.742 | 2.632 | 1.895 | 1.875 | 1.214 | 2.515 | 1.214 | 0.706 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 40.9 | 155.1 | 65.7 | 77.5 | 19.6 | 22.7 | 19.0 | 45.6 | 62.7 | 20.2 | — | — |
| Volume kept, % of the source (V) | 98.96 | — | 98.49 | — | 99.96 | 99.96 | 98.92 | — | 98.92 | 100.02 | — | — |
| Edges on closed loops, % (L) | 2.5 | — | — | — | 16.2 | 1.3 | 1.8 | — | — | 8.9 | — | — |
| Edges on spiralling loops, % (L) | 5.2 | — | — | — | 5.7 | 10.7 | 11.3 | — | — | 3.7 | — | — |
| Spiralling loops (L) | 12 | — | — | — | 17 | 15 | 37 | — | — | 11 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 92.4 | — | — | — | 78.0 | 88.0 | 86.8 | — | — | 87.5 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 92 | — | — | — | 137 | 131 | 189 | — | — | 340 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 12.2 | 39.7 | 11.2 | 15.4 | 1.3 | 0.4 | 371.3 | 4.7 | 0.0 | 0.0 | — | — |
| Time, seconds (T) | 2.73 | 0.95 | 0.53 | 9.68 | 5.03 | 6.55 | 37.80 | 33.37 | 37.80 | 18.48 | — | — |

## windmill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 1003 | 1886 | — | 207 | 294 | 17287 | 2787 | 17287 | 751 | — | — |
| Irregular vertices (S) | — | 145 | 312 | — | 28 | 81 | 905 | 382 | 905 | 207 | — | — |
| … of which above the minimum | — | — | — | — | 4 | 57 | — | — | — | 183 | — | — |
| Base complex patches (C) | — | — | 1778 | — | 199 | 294 | — | — | — | 743 | — | — |
| Alignment, degrees (A) | — | 14.39 | 15.52 | — | 19.02 | 19.42 | 9.49 | 11.63 | 11.73 | 17.20 | — | — |
| Mean angle deviation, degrees (F) | — | 20.17 | 15.16 | — | 28.90 | 25.05 | 8.30 | 10.62 | 8.30 | 19.59 | — | — |
| Angle deviation, 95th percentile (F) | — | 69.91 | 53.30 | — | 68.68 | 63.63 | 25.71 | 37.24 | 25.71 | 51.62 | — | — |
| Mean aspect ratio (F) | — | 2.697 | 1.760 | — | 57.264 | 3.154 | 1.847 | 1.540 | 1.847 | 3.874 | — | — |
| Area variation (F) | — | 0.807 | 0.437 | — | 1.435 | 1.170 | 0.787 | 0.335 | 0.787 | 1.119 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 193.85 | 69.08 | — | 532.11 | 530.76 | 16.02 | 36.49 | 16.02 | 529.65 | — | — |
| Mean distance, ‰ (H) | — | 18.937 | 5.814 | — | 248.302 | 240.577 | 0.917 | 2.648 | 0.917 | 217.715 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 122.8 | 42.9 | — | 1176.0 | 1165.9 | 9.2 | 17.8 | 19.1 | 2496.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 1.39 | 1.29 | 99.26 | — | 99.26 | 1.30 | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 1.9 | 0.0 | 7.5 | — | — | 0.5 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 6.5 | 4.8 | 10.7 | — | — | 2.5 | — | — |
| Spiralling loops (L) | — | — | — | — | 5 | 4 | 46 | — | — | 3 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 91.5 | 95.2 | 81.8 | — | — | 97.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 38 | 17 | 180 | — | — | 19 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 66.6 | 37.1 | — | 93.1 | 90.2 | 476.2 | 7.1 | 0.0 | 95.7 | — | — |
| Time, seconds (T) | — | 1.35 | 0.52 | — | 1.04 | 1.74 | 171.59 | 233.11 | 171.59 | 17.49 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 2 non-manifold or inconsistently oriented edges.

## terrain (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 | 0/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2717 | 919 | 1753 | 2502 | 3029 | 3032 | 4677 | 2597 | 4677 | 4676 | — | — |
| Irregular vertices (S) | 112 | 45 | 115 | 55 | 58 | 97 | 231 | 167 | 231 | 139 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 826 | — | 1209 | 997 | 1498 | 2229 | — | 1662 | — | 3902 | — | — |
| Alignment, degrees (A) | 20.16 | 20.27 | 21.17 | 18.74 | 18.13 | 18.43 | 18.30 | 18.61 | 18.51 | 18.08 | — | — |
| Mean angle deviation, degrees (F) | 6.49 | 6.70 | 8.24 | 9.36 | 6.49 | 6.48 | 9.57 | 8.13 | 9.57 | 6.90 | — | — |
| Angle deviation, 95th percentile (F) | 23.46 | 24.56 | 27.33 | 27.14 | 18.44 | 19.56 | 26.86 | 23.75 | 26.86 | 18.93 | — | — |
| Mean aspect ratio (F) | 1.209 | 1.423 | 1.291 | 1.338 | 1.277 | 1.337 | 1.806 | 1.429 | 1.806 | 1.360 | — | — |
| Area variation (F) | 0.187 | 0.413 | 0.215 | 0.276 | 0.308 | 0.433 | 0.552 | 0.247 | 0.552 | 0.469 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 23.91 | 34.95 | 143.82 | 24.92 | 19.27 | 15.98 | 12.55 | 25.16 | 12.55 | 12.34 | — | — |
| Mean distance, ‰ (H) | 1.851 | 3.051 | 2.384 | 1.715 | 1.374 | 1.100 | 0.967 | 1.780 | 0.967 | 0.889 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 32.8 | 56.8 | 36.9 | 35.4 | 26.5 | 18.1 | 14.4 | 32.3 | 18.9 | 18.7 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 2.7 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | — | — | 0.0 | 1.8 | 0.0 | — | — | 2.2 | — | — |
| Spiralling loops (L) | 0 | 0 | — | — | 0 | 1 | 0 | — | — | 2 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 53.0 | 67.6 | — | — | 69.2 | 83.3 | 84.8 | — | — | 88.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 46 | 27 | — | — | 98 | 110 | 105 | — | — | 111 | — | — |
| Edges on loops from a border to a border, % (L) | 47.0 | 32.4 | — | — | 30.8 | 14.9 | 15.2 | — | — | 6.1 | — | — |
| Off budget, % (N) | 9.4 | 69.4 | 41.6 | 16.6 | 1.0 | 1.1 | 55.9 | 13.4 | 0.0 | 0.0 | — | — |
| Time, seconds (T) | 19.92 | 1.90 | 8.92 | 14.15 | 10.09 | 8.99 | 21.17 | 19.56 | 21.17 | 13.18 | — | — |

## mahaveer (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 1325 | 1699 | 2080 | 2838 | 3322 | 46202 | 4324 | 46202 | 45905 | — | — |
| Irregular vertices (S) | — | 422 | 175 | 64 | 476 | 488 | 3076 | 1079 | 3076 | 1444 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1227 | — | 804 | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 9.34 | 9.97 | 7.13 | 9.25 | 7.20 | 13.54 | 11.62 | 13.50 | 5.77 | — | — |
| Mean angle deviation, degrees (F) | — | 32.26 | 9.66 | 7.58 | 18.19 | 22.37 | 12.36 | 12.56 | 12.36 | 5.60 | — | — |
| Angle deviation, 95th percentile (F) | — | 85.28 | 34.02 | 26.60 | 82.06 | 82.64 | 39.42 | 48.70 | 39.42 | 21.52 | — | — |
| Mean aspect ratio (F) | — | 5.235 | 1.583 | 1.656 | 93.141 | 126.926 | 1.824 | 1.786 | 1.824 | 10.057 | — | — |
| Area variation (F) | — | 1.443 | 0.344 | 0.570 | 2.670 | 2.913 | 2.664 | 0.975 | 2.664 | 0.929 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 44.66 | 52.26 | 26.46 | 50.03 | 50.03 | 13.60 | 23.83 | 13.60 | 13.38 | — | — |
| Mean distance, ‰ (H) | — | 5.890 | 7.890 | 3.785 | 4.005 | 3.962 | 0.929 | 2.320 | 0.929 | 0.652 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 46.8 | 75.1 | 35.8 | 34.1 | 33.8 | 11.6 | 21.9 | 49.3 | 38.3 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 21.5 | — | — | — | — | 23.1 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 56.3 | — | — | — | — | 22.2 | — | — |
| Spiralling loops (L) | — | — | — | — | 401 | — | — | — | — | 386 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 18.8 | — | — | — | — | 43.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 10 | — | — | — | — | 787 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 3.5 | — | — | — | — | 10.8 | — | — |
| Off budget, % (N) | — | 55.8 | 43.4 | 30.7 | 5.4 | 10.7 | 1440.1 | 44.1 | 0.0 | 0.6 | — | — |
| Time, seconds (T) | — | 1.33 | 7.16 | 18.77 | 9.45 | 12.02 | 14.10 | 14.84 | 14.10 | 37.14 | — | — |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 870 | 1225 | — | 8941 | 9054 | 214018 | 11340 | 214018 | 201871 | — | — |
| Irregular vertices (S) | — | 858 | 616 | — | 6050 | 6012 | 10315 | 4939 | 10315 | 11954 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1136 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 18.06 | 17.78 | — | 12.74 | 12.04 | 10.18 | 13.63 | 12.25 | 8.49 | — | — |
| Mean angle deviation, degrees (F) | — | 52.53 | 23.80 | — | 21.75 | 21.23 | 9.86 | 17.03 | 9.86 | 5.17 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.72 | 60.14 | — | 68.55 | 66.73 | 29.30 | 51.38 | 29.30 | 19.76 | — | — |
| Mean aspect ratio (F) | — | 14.155 | 2.545 | — | 5.235 | 4.238 | 1.580 | 1.889 | 1.580 | 1.374 | — | — |
| Area variation (F) | — | 7.748 | 0.789 | — | 3.361 | 3.360 | 2.386 | 2.005 | 2.386 | 0.564 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 9.99 | 7.35 | — | 7.04 | 5.99 | 1.80 | 3.03 | 1.80 | 3.04 | — | — |
| Mean distance, ‰ (H) | — | 2.873 | 1.806 | — | 0.448 | 0.448 | 0.123 | 0.419 | 0.123 | 0.037 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 87.1 | 57.0 | — | 12.5 | 12.8 | 7.6 | 14.6 | 59.1 | 18.4 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | — | — | — | 36.5 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | — | — | — | 11.9 | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | — | — | — | 1225 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | — | — | — | 45.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | — | — | — | 300 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | — | — | — | 6.5 | — | — |
| Off budget, % (N) | — | 71.0 | 59.2 | — | 198.0 | 201.8 | 7033.9 | 278.0 | 0.0 | 5.7 | — | — |
| Time, seconds (T) | — | 0.99 | 4.00 | — | 14.68 | 21.31 | 12.21 | 10.92 | 12.21 | 172.65 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 8 non-manifold or inconsistently oriented edges.

## f16 (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 2/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 4122 | 1500 | 1333 | 4505 | 4395 | 36038 | 6315 | 36038 | 33809 | — | — |
| Irregular vertices (S) | — | 2217 | 403 | 242 | 1830 | 1758 | 3062 | 1343 | 3062 | 1939 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1329 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 14.85 | 12.61 | 8.71 | 6.11 | 5.89 | 5.01 | 7.09 | 4.91 | 3.97 | — | — |
| Mean angle deviation, degrees (F) | — | 41.32 | 19.76 | 15.35 | 18.12 | 16.99 | 10.99 | 14.23 | 10.99 | 6.97 | — | — |
| Angle deviation, 95th percentile (F) | — | 87.07 | 64.46 | 49.46 | 68.74 | 66.31 | 41.48 | 46.16 | 41.48 | 29.25 | — | — |
| Mean aspect ratio (F) | — | 7.957 | 2.173 | 1.844 | 104.856 | 116.397 | 2.408 | 3.370 | 2.408 | 15.122 | — | — |
| Area variation (F) | — | 2.547 | 0.539 | 0.650 | 2.769 | 2.422 | 1.937 | 1.320 | 1.937 | 0.946 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 87.83 | 95.94 | 63.38 | 33.11 | 39.00 | 8.24 | 47.42 | 8.24 | 80.32 | — | — |
| Mean distance, ‰ (H) | — | 9.067 | 20.293 | 13.479 | 1.231 | 1.499 | 0.363 | 2.336 | 0.363 | 0.864 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 62.6 | 143.8 | 98.2 | 7.7 | 9.7 | 3.4 | 18.2 | 11.6 | 8.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Spiralling loops (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | — | — | — | — | — | — | — | — |
| Off budget, % (N) | — | 37.4 | 50.0 | 55.6 | 50.2 | 46.5 | 1101.3 | 110.5 | 0.0 | 6.2 | — | — |
| Time, seconds (T) | — | 2.15 | 5.15 | 88.05 | 11.24 | 11.72 | 9.36 | 9.51 | 9.36 | 119.70 | — | — |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2222 | 2216 | 1109 | 3276 | 3445 | 12621 | 5458 | 12621 | 12629 | — | — |
| Irregular vertices (S) | — | 338 | 265 | 70 | 204 | 202 | 443 | 322 | 443 | 291 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 1675 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 7.28 | 8.44 | 8.56 | 5.33 | 5.21 | 4.33 | 3.81 | 3.56 | 3.41 | — | — |
| Mean angle deviation, degrees (F) | — | 22.14 | 11.96 | 12.59 | 18.06 | 20.25 | 7.26 | 8.23 | 7.26 | 10.31 | — | — |
| Angle deviation, 95th percentile (F) | — | 84.37 | 45.38 | 46.79 | 73.19 | 74.42 | 26.15 | 33.02 | 26.15 | 52.30 | — | — |
| Mean aspect ratio (F) | — | 4.205 | 1.771 | 2.901 | 16.052 | 49.040 | 8.875 | 18.163 | 8.875 | 23.469 | — | — |
| Area variation (F) | — | 2.439 | 0.430 | 0.902 | 1.348 | 1.421 | 1.511 | 0.973 | 1.511 | 0.819 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 50.21 | 35.63 | 45.56 | 26.86 | 26.86 | 13.66 | 28.78 | 13.66 | 19.79 | — | — |
| Mean distance, ‰ (H) | — | 5.701 | 4.253 | 5.359 | 2.485 | 2.309 | 0.560 | 1.134 | 0.560 | 0.638 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 25.0 | 19.2 | 22.4 | 11.5 | 11.6 | 2.4 | 5.0 | 4.6 | 12.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 52.5 | 50.1 | — | — | — | 52.2 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 8.0 | 6.2 | — | — | — | 2.0 | — | — |
| Spiralling loops (L) | — | — | — | — | 85 | 82 | — | — | — | 53 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 27.9 | 32.7 | — | — | — | 23.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 50 | 72 | — | — | — | 203 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 11.5 | 11.1 | — | — | — | 22.0 | — | — |
| Off budget, % (N) | — | 25.9 | 26.1 | 63.0 | 9.2 | 14.8 | 320.7 | 81.9 | 0.0 | 0.1 | — | — |
| Time, seconds (T) | — | 0.92 | 1.12 | 70.22 | 9.82 | 3.11 | 5.03 | 4.34 | 5.03 | 6.89 | — | — |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 625 | 2774 | 2420 | 2390 | 2394 | 14984 | 2308 | 14984 | 13116 | — | — |
| Irregular vertices (S) | — | 125 | 430 | 185 | 221 | 217 | 775 | 331 | 775 | 610 | — | — |
| … of which above the minimum | — | — | 387 | — | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 2517 | — | — | — | — | — | — | — | — | — |
| Alignment, degrees (A) | — | 14.57 | 14.83 | 10.13 | 11.04 | 10.94 | 9.19 | 9.89 | 10.20 | 10.34 | — | — |
| Mean angle deviation, degrees (F) | — | 16.08 | 11.44 | 9.70 | 10.21 | 10.22 | 9.40 | 11.17 | 9.40 | 9.57 | — | — |
| Angle deviation, 95th percentile (F) | — | 44.86 | 37.21 | 27.88 | 29.69 | 30.09 | 27.92 | 33.61 | 27.92 | 27.62 | — | — |
| Mean aspect ratio (F) | — | 2.516 | 1.550 | 1.474 | 1.628 | 1.678 | 1.735 | 1.574 | 1.735 | 1.413 | — | — |
| Area variation (F) | — | 0.933 | 0.323 | 0.335 | 0.358 | 0.357 | 0.938 | 0.374 | 0.938 | 0.324 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 89.72 | 44.20 | 33.89 | 43.77 | 43.77 | 21.30 | 37.66 | 21.30 | 11.09 | — | — |
| Mean distance, ‰ (H) | — | 9.165 | 3.465 | 2.989 | 2.662 | 2.679 | 0.702 | 3.072 | 0.702 | 0.387 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 125.4 | 88.0 | 69.3 | 60.7 | 61.2 | 17.5 | 65.3 | 44.1 | 38.7 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | — | — | — | 20.3 | 20.4 | 15.8 | — | — | 14.0 | — | — |
| Edges on spiralling loops, % (L) | — | — | — | — | 11.1 | 10.7 | 7.2 | — | — | 20.2 | — | — |
| Spiralling loops (L) | — | — | — | — | 44 | 40 | 70 | — | — | 95 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | — | — | 68.4 | 68.6 | 76.9 | — | — | 65.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | — | — | 86 | 86 | 170 | — | — | 305 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | — | — | 0.2 | 0.2 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | — | 79.2 | 7.5 | 19.3 | 20.3 | 20.2 | 399.5 | 23.1 | 0.0 | 12.5 | — | — |
| Time, seconds (T) | — | 0.74 | 0.34 | 22.46 | 23.60 | 23.65 | 26.98 | 30.23 | 26.98 | 402.71 | — | — |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 3107 | 2086 | 3090 | 3000 | 2990 | 2990 | 2740 | 2740 | 2740 | 2728 | — | — |
| Irregular vertices (S) | 8 | 10 | 97 | 32 | 8 | 8 | 10 | 10 | 10 | 8 | — | — |
| … of which above the minimum | 0 | 2 | 89 | 24 | 0 | 0 | 2 | 2 | 2 | 0 | — | — |
| Base complex patches (C) | 2730 | 2004 | 3090 | 30 | 6 | 6 | 1794 | 1794 | 1794 | 6 | — | — |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.49 | 7.65 | 4.19 | 8.38 | 1.23 | 1.23 | 6.09 | 6.09 | 6.09 | 1.23 | — | — |
| Angle deviation, 95th percentile (F) | 12.67 | 19.88 | 14.49 | 17.72 | 3.57 | 3.58 | 15.49 | 15.49 | 15.49 | 3.71 | — | — |
| Mean aspect ratio (F) | 1.125 | 1.326 | 1.124 | 1.081 | 1.108 | 1.108 | 1.185 | 1.185 | 1.185 | 1.108 | — | — |
| Area variation (F) | 0.135 | 0.341 | 0.136 | 0.190 | 0.142 | 0.142 | 0.312 | 0.312 | 0.312 | 0.144 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 0.63 | 1.54 | 0.72 | 0.42 | 0.35 | 0.35 | 0.82 | 0.82 | 0.82 | 0.34 | — | — |
| Mean distance, ‰ (H) | 0.216 | 0.408 | 0.221 | 0.168 | 0.191 | 0.191 | 0.258 | 0.258 | 0.258 | 0.173 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.81 | 99.59 | 99.79 | 100.01 | 100.01 | 100.01 | 99.74 | 99.74 | 99.74 | 100.01 | — | — |
| Edges on closed loops, % (L) | 6.1 | 2.0 | — | — | 95.5 | 95.5 | 17.3 | — | — | 95.3 | — | — |
| Edges on spiralling loops, % (L) | 70.1 | 74.4 | — | — | 0.0 | 0.0 | 69.2 | — | — | 0.0 | — | — |
| Spiralling loops (L) | 5 | 6 | — | — | 0 | 0 | 6 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 15.4 | 23.6 | — | — | 4.5 | 4.5 | 13.5 | — | — | 4.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 276 | 298 | — | — | 23 | 23 | 158 | — | — | 22 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | — | — | 0.0 | 0.0 | 0.0 | — | — | 0.0 | — | — |
| Off budget, % (N) | 3.6 | 30.5 | 3.0 | 0.0 | 0.3 | 0.3 | 8.7 | 8.7 | 0.0 | 0.4 | — | — |
| Time, seconds (T) | 1.81 | 0.40 | 0.34 | 1.94 | 2.93 | 3.23 | 3.56 | 3.39 | 3.56 | 1.89 | — | — |

## torus (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 1/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 2976 | 2672 | 3078 | 2720 | 2952 | 2952 | 2824 | 2824 | 2824 | 2800 | — | — |
| Irregular vertices (S) | 30 | 0 | 125 | 0 | 0 | 0 | 10 | 10 | 10 | 0 | — | — |
| … of which above the minimum | 30 | 0 | 125 | 0 | 0 | 0 | 10 | 10 | 10 | 0 | — | — |
| Base complex patches (C) | 2976 | 1 | 3078 | 1 | 1 | 1 | 2264 | 2264 | 2264 | 1 | — | — |
| Alignment, degrees (A) | 10.14 | 10.37 | 14.28 | 0.65 | 0.62 | 0.62 | 6.90 | 6.90 | 6.90 | 0.62 | — | — |
| Mean angle deviation, degrees (F) | 10.25 | 4.17 | 5.57 | 1.45 | 1.43 | 1.43 | 5.23 | 5.23 | 5.23 | 1.48 | — | — |
| Angle deviation, 95th percentile (F) | 26.29 | 11.07 | 18.30 | 2.89 | 2.35 | 2.35 | 14.40 | 14.40 | 14.40 | 2.40 | — | — |
| Mean aspect ratio (F) | 1.239 | 2.545 | 1.163 | 1.351 | 1.323 | 1.323 | 1.213 | 1.213 | 1.213 | 1.321 | — | — |
| Area variation (F) | 0.223 | 0.256 | 0.163 | 0.281 | 0.307 | 0.307 | 0.233 | 0.233 | 0.233 | 0.308 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 1.86 | 1.50 | 1.38 | 0.59 | 0.58 | 0.58 | 1.71 | 1.71 | 1.71 | 0.62 | — | — |
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
| Time, seconds (T) | 1.56 | 0.53 | 0.22 | 2.01 | 2.63 | 2.46 | 4.71 | 2.89 | 4.71 | 2.01 | — | — |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (15 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 2 | 6 | 0 | 5 | 3 | 2 | 0 | 1 | 0 | 1 | 0 | 0 |
| Base complex patches (C) | 1 | 3 | 0 | 3 | 5 | 2 | 0 | 0 | 0 | 1 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 0 | 1 | 13 | 0 | 1 | 0 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 1 | 0 | 3 | 2 | 0 | 1 | 0 | 7 | 0 | 1 |
| Mean aspect ratio (F) | 0 | 0 | 10 | 0 | 2 | 0 | 0 | 0 | 0 | 2 | 0 | 1 |
| Valid meshes (V) | 36/75 | 4/15 | 48/75 | 10/15 | 14/15 | 14/15 | 8/15 | 10/15 | 8/15 | 15/15 | 0/2 | 2/2 |

### mechanical (4 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform | quadremesher-as-made | openclare-matched | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 3 | 0 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 0 | 0 |
| Base complex patches (C) | 1 | 1 | 0 | 2 | 3 | 3 | 2 | 2 | 2 | 2 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 1 | 1 | 2 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 0 | 2 | 0 | 1 | 1 | 1 | 1 | 0 | 0 |
| Mean aspect ratio (F) | 3 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Valid meshes (V) | 18/20 | 3/4 | 20/20 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 4/4 | 0/0 | 0/0 |
