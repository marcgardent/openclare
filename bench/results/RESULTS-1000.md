# Bench results

Requested budget: 1000 quads. Median over 5 seeds; AutoRemesher and integer-grid have no seed and run once. The first row is how many quads each tool made; below it, lower is better for every metric. "—" means it could not be measured.

## blob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 941 | 400 | 1036 | 835 | 1002 | 1010 | — | — |
| Irregular vertices (S) | 36 | 24 | 76 | 26 | 24 | 24 | — | — |
| … of which above the minimum | 28 | 16 | 68 | 18 | 16 | 16 | — | — |
| Base complex patches (C) | 941 | 314 | 1036 | 671 | 290 | 392 | — | — |
| Alignment, degrees (A) | 18.04 | 8.83 | 17.88 | 7.68 | 5.37 | 5.10 | — | — |
| Mean angle deviation, degrees (F) | 9.26 | 12.46 | 7.67 | 9.62 | 6.87 | 7.07 | — | — |
| Angle deviation, 95th percentile (F) | 25.27 | 33.98 | 23.42 | 25.34 | 17.86 | 17.94 | — | — |
| Mean aspect ratio (F) | 1.221 | 1.652 | 1.235 | 1.472 | 1.434 | 1.421 | — | — |
| Area variation (F) | 0.218 | 0.577 | 0.218 | 0.381 | 0.476 | 0.465 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 14.52 | 16.95 | 15.11 | 13.55 | 10.47 | 8.04 | — | — |
| Mean distance, ‰ (H) | 1.325 | 5.131 | 1.340 | 0.729 | 0.540 | 0.575 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.58 | 92.19 | 98.90 | 100.08 | 100.06 | 100.06 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 10.8 | 0.0 | 9.8 | 40.6 | 31.0 | — | — |
| Edges on spiralling loops, % (L) | 25.3 | 35.2 | 7.7 | 38.0 | 6.2 | 7.0 | — | — |
| Spiralling loops (L) | 4 | 7 | 2 | 7 | 2 | 1 | — | — |
| Edges on helices of any pitch, % (L) | 14.0 | 11.5 | 0.0 | 20.8 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 74.7 | 54.0 | 92.3 | 52.2 | 53.1 | 62.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 108 | 44 | 70 | 114 | 100 | 100 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 5.9 | 60.0 | 3.6 | 16.5 | 0.2 | 1.0 | — | — |
| Time, seconds (T) | 1.54 | 0.53 | 1.17 | 4.68 | 2.53 | 3.91 | — | — |

## rock (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 947 | 778 | 966 | 867 | 1011 | 982 | — | — |
| Irregular vertices (S) | 8 | 10 | 40 | 28 | 28 | 28 | — | — |
| … of which above the minimum | 0 | 2 | 32 | 20 | 20 | 20 | — | — |
| Base complex patches (C) | 941 | 506 | 912 | 867 | 1011 | 879 | — | — |
| Alignment, degrees (A) | 23.14 | 19.26 | 23.62 | 16.17 | 16.67 | 16.73 | — | — |
| Mean angle deviation, degrees (F) | 6.16 | 7.46 | 6.09 | 10.61 | 6.33 | 6.19 | — | — |
| Angle deviation, 95th percentile (F) | 16.14 | 18.13 | 18.18 | 25.20 | 16.00 | 15.65 | — | — |
| Mean aspect ratio (F) | 1.162 | 1.494 | 1.188 | 1.303 | 1.326 | 1.283 | — | — |
| Area variation (F) | 0.179 | 0.473 | 0.178 | 0.261 | 0.335 | 0.346 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 9.38 | 14.09 | 10.32 | 8.44 | 7.00 | 7.27 | — | — |
| Mean distance, ‰ (H) | 1.512 | 2.480 | 1.641 | 1.389 | 1.206 | 1.227 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.30 | 98.22 | 99.04 | 99.99 | 100.02 | 100.02 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 3.7 | 2.7 | 0.0 | 0.0 | 5.3 | — | — |
| Edges on spiralling loops, % (L) | 60.3 | 69.2 | 20.7 | 28.4 | 0.0 | 4.4 | — | — |
| Spiralling loops (L) | 3 | 6 | 4 | 5 | 0 | 1 | — | — |
| Edges on helices of any pitch, % (L) | 56.8 | 56.7 | 0.0 | 0.0 | 8.4 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 1 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 35.9 | 27.1 | 76.6 | 71.6 | 100.0 | 90.3 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 133 | 82 | 88 | 106 | 170 | 121 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 5.3 | 22.2 | 3.4 | 13.3 | 1.1 | 1.8 | — | — |
| Time, seconds (T) | 0.94 | 0.56 | 0.70 | 2.84 | 2.05 | 3.34 | — | — |

## creature (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 812 | 604 | 994 | 936 | 1009 | 1011 | — | — |
| Irregular vertices (S) | 16 | 12 | 51 | 28 | 26 | 38 | — | — |
| … of which above the minimum | 8 | 4 | 43 | 20 | 18 | 30 | — | — |
| Base complex patches (C) | 812 | 308 | 822 | 698 | 313 | 381 | — | — |
| Alignment, degrees (A) | 14.79 | 11.08 | 11.93 | 8.28 | 7.95 | 6.13 | — | — |
| Mean angle deviation, degrees (F) | 12.55 | 6.78 | 6.23 | 7.74 | 4.98 | 5.47 | — | — |
| Angle deviation, 95th percentile (F) | 30.83 | 16.26 | 18.64 | 23.52 | 13.66 | 17.12 | — | — |
| Mean aspect ratio (F) | 1.391 | 1.697 | 1.185 | 1.250 | 1.237 | 1.306 | — | — |
| Area variation (F) | 0.298 | 0.418 | 0.183 | 0.299 | 0.387 | 0.478 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 40.53 | 8.78 | 18.99 | 7.09 | 4.41 | 4.19 | — | — |
| Mean distance, ‰ (H) | 1.486 | 1.732 | 0.917 | 0.503 | 0.434 | 0.454 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 97.34 | 96.54 | 98.23 | 99.98 | 100.08 | 100.07 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 24.5 | 8.2 | 12.7 | 36.0 | 31.2 | — | — |
| Edges on spiralling loops, % (L) | 43.6 | 54.3 | 11.4 | 40.2 | 4.3 | 0.0 | — | — |
| Spiralling loops (L) | 6 | 4 | 5 | 5 | 1 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 50.6 | 35.6 | 4.8 | 26.2 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 4 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 56.4 | 21.2 | 80.3 | 47.1 | 59.7 | 68.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 141 | 52 | 76 | 66 | 91 | 80 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 18.8 | 39.6 | 1.4 | 6.4 | 0.9 | 1.1 | — | — |
| Time, seconds (T) | 1.53 | 0.80 | 1.09 | 3.22 | 2.42 | 1.80 | — | — |

## cage (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 1043 | 1720 | 974 | 1008 | 981 | 1014 | — | — |
| Irregular vertices (S) | 64 | 52 | 136 | 44 | 48 | 48 | — | — |
| … of which above the minimum | 32 | — | 104 | 12 | 16 | 16 | — | — |
| Base complex patches (C) | 716 | — | 779 | 382 | 173 | 165 | — | — |
| Alignment, degrees (A) | 7.66 | 8.80 | 10.19 | 7.47 | 4.68 | 2.95 | — | — |
| Mean angle deviation, degrees (F) | 6.96 | 10.60 | 9.36 | 7.93 | 3.89 | 4.52 | — | — |
| Angle deviation, 95th percentile (F) | 21.11 | 30.15 | 30.34 | 23.18 | 14.63 | 13.96 | — | — |
| Mean aspect ratio (F) | 1.250 | 2.067 | 1.402 | 1.308 | 1.224 | 1.239 | — | — |
| Area variation (F) | 0.155 | 0.381 | 0.279 | 0.237 | 0.191 | 0.348 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 21.45 | 52.83 | 28.51 | 20.14 | 11.17 | 11.97 | — | — |
| Mean distance, ‰ (H) | 1.546 | 2.978 | 2.710 | 2.254 | 1.491 | 1.490 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.12 | 96.35 | 95.62 | 99.19 | 99.47 | 99.35 | — | — |
| Sharp edges kept, % of their length (E) | 94.7 | 32.2 | 73.1 | 77.3 | 100.0 | 100.0 | — | — |
| Edges on closed loops, % (L) | 17.2 | 16.0 | 10.1 | 34.0 | 58.3 | 59.8 | — | — |
| Edges on spiralling loops, % (L) | 23.4 | 36.5 | 4.1 | 12.9 | 5.4 | 6.1 | — | — |
| Spiralling loops (L) | 10 | 20 | 3 | 4 | 2 | 2 | — | — |
| Edges on helices of any pitch, % (L) | 13.0 | 14.8 | 0.0 | 10.8 | 5.4 | 6.1 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 58.8 | 47.5 | 85.2 | 53.1 | 36.3 | 34.1 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 47 | 74 | 50 | 108 | 22 | 19 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 4.3 | 72.0 | 2.6 | 0.8 | 1.9 | 1.4 | — | — |
| Time, seconds (T) | 0.33 | 0.97 | 0.32 | 7.16 | 2.13 | 3.92 | — | — |

## suzanne (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 1005 | 357 | 958 | 979 | 990 | 994 | — | — |
| Irregular vertices (S) | 67 | 48 | 114 | 48 | 44 | 48 | — | — |
| … of which above the minimum | — | 24 | 98 | — | — | — | — | — |
| Base complex patches (C) | 995 | 357 | 930 | 837 | 488 | 541 | — | — |
| Alignment, degrees (A) | 15.73 | 18.27 | 17.84 | 12.42 | 12.28 | 11.59 | — | — |
| Mean angle deviation, degrees (F) | 9.65 | 15.83 | 10.18 | 8.29 | 5.66 | 6.03 | — | — |
| Angle deviation, 95th percentile (F) | 29.23 | 49.29 | 34.26 | 23.64 | 19.05 | 21.00 | — | — |
| Mean aspect ratio (F) | 1.325 | 2.035 | 1.423 | 1.326 | 1.330 | 1.502 | — | — |
| Area variation (F) | 0.506 | 0.760 | 0.395 | 0.370 | 0.262 | 0.369 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 44.79 | 137.08 | 50.97 | 37.10 | 17.55 | 17.57 | — | — |
| Mean distance, ‰ (H) | 2.245 | 13.357 | 3.552 | 2.084 | 1.699 | 1.542 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 10.6 | 112.8 | 23.9 | 8.9 | 7.7 | 4.8 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | 28.0 | 3.8 | 31.4 | 37.6 | 58.7 | 64.7 | — | — |
| Edges on closed loops, % (L) | 2.3 | 0.0 | 2.2 | 2.8 | 19.4 | 17.5 | — | — |
| Edges on spiralling loops, % (L) | 7.9 | 5.7 | 4.6 | 21.4 | 0.0 | 3.0 | — | — |
| Spiralling loops (L) | 5 | 2 | 2 | 5 | 0 | 2 | — | — |
| Edges on helices of any pitch, % (L) | 3.6 | 0.0 | 0.0 | 13.8 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.6 | 94.3 | 94.3 | 66.9 | 62.4 | 71.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 97 | 43 | 56 | 62 | 54 | 107 | — | — |
| Edges on loops from a border to a border, % (L) | 0.6 | 0.0 | 0.8 | 8.9 | 18.1 | 7.8 | — | — |
| Off budget, % (N) | 1.0 | 64.3 | 4.2 | 2.1 | 1.0 | 0.6 | — | — |
| Time, seconds (T) | 0.48 | 0.90 | 0.22 | 4.70 | 2.62 | 4.11 | — | — |

## ogre (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 4/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 891 | 311 | 928 | 742 | 997 | 984 | — | — |
| Irregular vertices (S) | 73 | 36 | 113 | 46 | 46 | 107 | — | — |
| … of which above the minimum | 64 | — | 100 | 38 | 38 | 99 | — | — |
| Base complex patches (C) | 891 | — | 856 | 402 | 419 | 662 | — | — |
| Alignment, degrees (A) | 20.86 | 17.10 | 18.87 | 12.05 | 11.16 | 11.30 | — | — |
| Mean angle deviation, degrees (F) | 14.18 | 13.19 | 9.53 | 11.58 | 8.38 | 9.31 | — | — |
| Angle deviation, 95th percentile (F) | 39.23 | 35.34 | 29.81 | 31.26 | 22.32 | 26.12 | — | — |
| Mean aspect ratio (F) | 1.552 | 1.883 | 1.408 | 1.648 | 1.685 | 1.567 | — | — |
| Area variation (F) | 0.348 | 0.671 | 0.283 | 0.398 | 0.465 | 0.770 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 95.01 | 126.19 | 85.78 | 41.60 | 32.10 | 28.75 | — | — |
| Mean distance, ‰ (H) | 4.418 | 13.988 | 3.770 | 2.489 | 1.741 | 1.828 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 115.0 | 180.3 | 110.1 | 41.4 | 21.0 | 6.6 | — | — |
| Volume kept, % of the source (V) | 94.57 | 87.06 | 95.96 | 98.90 | 99.57 | 99.66 | — | — |
| Sharp edges kept, % of their length (E) | 24.1 | 0.4 | 17.2 | 18.4 | 42.7 | 37.4 | — | — |
| Edges on closed loops, % (L) | 0.3 | 7.8 | 4.3 | 22.9 | 29.0 | 16.4 | — | — |
| Edges on spiralling loops, % (L) | 10.9 | 25.0 | 4.5 | 1.7 | 0.0 | 9.8 | — | — |
| Spiralling loops (L) | 6 | 4 | 3 | 1 | 0 | 12 | — | — |
| Edges on helices of any pitch, % (L) | 8.9 | 9.1 | 0.0 | 1.7 | 0.0 | 7.1 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 1 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.1 | 67.2 | 89.1 | 75.4 | 71.0 | 73.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 102 | 17 | 64 | 57 | 152 | 58 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 10.9 | 68.9 | 7.2 | 25.8 | 0.3 | 1.6 | — | — |
| Time, seconds (T) | 2.69 | 1.17 | 2.05 | 7.56 | 7.09 | 57.79 | — | — |

## human_male (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 1/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 176 | 868 | 885 | 885 | 1014 | 10590 | 10484 |
| Irregular vertices (S) | — | 38 | 96 | 41 | 41 | 116 | 476 | 254 |
| … of which above the minimum | — | — | 76 | 33 | 33 | 108 | — | 246 |
| Base complex patches (C) | — | — | 578 | 241 | 241 | 492 | — | 3162 |
| Alignment, degrees (A) | — | 11.90 | 11.10 | 6.36 | 6.36 | 8.18 | 9.88 | 8.34 |
| Mean angle deviation, degrees (F) | — | 16.80 | 9.77 | 6.88 | 6.88 | 8.74 | 9.24 | 4.29 |
| Angle deviation, 95th percentile (F) | — | 55.64 | 35.19 | 21.64 | 21.64 | 28.66 | 26.27 | 13.79 |
| Mean aspect ratio (F) | — | 3.300 | 1.497 | 1.447 | 1.447 | 1.529 | 1.616 | 1.345 |
| Area variation (F) | — | 0.883 | 0.327 | 0.400 | 0.400 | 1.442 | 1.417 | 0.461 |
| Hausdorff, ‰ of the diagonal (H) | — | 160.81 | 101.17 | 45.97 | 45.97 | 17.65 | 6.47 | 4.74 |
| Mean distance, ‰ (H) | — | 58.173 | 13.524 | 5.330 | 5.330 | 2.080 | 0.172 | 0.190 |
| Thin parts: mean distance, % of a quad edge (V) | — | 331.3 | 119.6 | 51.4 | 51.4 | 8.7 | 1.5 | 5.8 |
| Volume kept, % of the source (V) | — | 51.85 | 90.43 | 99.16 | 99.16 | 98.76 | 98.80 | 100.02 |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | 71.3 | 94.3 |
| Edges on closed loops, % (L) | — | 21.5 | 15.7 | 36.4 | 36.4 | 25.7 | 31.6 | 36.5 |
| Edges on spiralling loops, % (L) | — | 35.4 | 7.9 | 15.2 | 15.2 | 11.7 | 2.5 | 1.5 |
| Spiralling loops (L) | — | 18 | 13 | 2 | 2 | 16 | 3 | 11 |
| Edges on helices of any pitch, % (L) | — | 0.0 | 0.0 | 5.7 | 5.7 | 6.2 | 0.0 | 0.0 |
| Helices of pitch above 4, which the spiral count misses (L) | — | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Edges on loops that stop at an irregular vertex, % (L) | — | 43.1 | 78.4 | 48.4 | 48.4 | 62.5 | 65.8 | 62.1 |
| Longest loop that stops at an irregular vertex, edges (L) | — | 16 | 40 | 54 | 54 | 41 | 274 | 401 |
| Edges on loops from a border to a border, % (L) | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 82.4 | 13.2 | 11.5 | 11.5 | 1.4 | 0.0 | 1.0 |
| Time, seconds (T) | — | 4.84 | 17.33 | 15.30 | 52.10 | 66.75 | — | 172.27 |

Failures: quadriflow: cancelled.

## human_female (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 1/1 | 1/1 | 1/1 | 0/1 | 1/1 |
| Quads made | — | 172 | 877 | 771 | 1016 | 1000 | 10590 | 10551 |
| Irregular vertices (S) | — | 38 | 105 | 52 | 54 | 128 | 476 | 274 |
| … of which above the minimum | — | — | 72 | 44 | 46 | 120 | — | 266 |
| Base complex patches (C) | — | — | 639 | 317 | 394 | 480 | — | 3845 |
| Alignment, degrees (A) | — | 12.34 | 11.11 | 6.99 | 7.43 | 7.24 | 9.43 | 7.33 |
| Mean angle deviation, degrees (F) | — | 30.39 | 9.85 | 8.23 | 6.12 | 8.57 | 8.82 | 4.37 |
| Angle deviation, 95th percentile (F) | — | 82.98 | 32.30 | 25.00 | 20.28 | 27.67 | 25.47 | 13.74 |
| Mean aspect ratio (F) | — | 3.326 | 1.445 | 1.477 | 1.560 | 1.538 | 1.657 | 1.277 |
| Area variation (F) | — | 1.040 | 0.326 | 0.352 | 0.455 | 1.514 | 1.432 | 0.484 |
| Hausdorff, ‰ of the diagonal (H) | — | 100.86 | 70.01 | 56.82 | 28.38 | 22.67 | 4.44 | 4.09 |
| Mean distance, ‰ (H) | — | 33.458 | 11.587 | 5.165 | 2.921 | 2.283 | 0.156 | 0.182 |
| Thin parts: mean distance, % of a quad edge (V) | — | 191.1 | 92.0 | 48.6 | 27.0 | 12.5 | 1.4 | 5.6 |
| Volume kept, % of the source (V) | — | 35.72 | 90.12 | 98.55 | 99.67 | 98.41 | 98.78 | 100.02 |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | 67.0 | 88.9 |
| Edges on closed loops, % (L) | — | 16.2 | 14.7 | 29.4 | 30.6 | 26.0 | 31.6 | 31.8 |
| Edges on spiralling loops, % (L) | — | 46.8 | 7.6 | 12.8 | 3.0 | 12.9 | 2.5 | 1.4 |
| Spiralling loops (L) | — | 18 | 10 | 9 | 1 | 26 | 3 | 6 |
| Edges on helices of any pitch, % (L) | — | 9.2 | 0.0 | 2.2 | 3.0 | 2.1 | 0.0 | 0.6 |
| Helices of pitch above 4, which the spiral count misses (L) | — | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Edges on loops that stop at an irregular vertex, % (L) | — | 37.0 | 79.2 | 57.7 | 66.4 | 61.1 | 65.8 | 66.9 |
| Longest loop that stops at an irregular vertex, edges (L) | — | 11 | 42 | 54 | 154 | 34 | 274 | 485 |
| Edges on loops from a border to a border, % (L) | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| Off budget, % (N) | — | 82.8 | 12.3 | 22.9 | 1.6 | 0.0 | 0.0 | 0.4 |
| Time, seconds (T) | — | 3.67 | 21.04 | 18.64 | 27.11 | 60.69 | — | 204.37 |

Failures: quadriflow: cancelled.

## spot (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 1024 | 404 | 1012 | 938 | 1012 | 1013 | — | — |
| Irregular vertices (S) | 62 | 39 | 100 | 60 | 64 | 68 | — | — |
| … of which above the minimum | 54 | — | 92 | 52 | 56 | 60 | — | — |
| Base complex patches (C) | 965 | — | 924 | 712 | 624 | 605 | — | — |
| Alignment, degrees (A) | 18.23 | 16.98 | 17.83 | 10.31 | 9.87 | 8.82 | — | — |
| Mean angle deviation, degrees (F) | 7.55 | 9.29 | 7.94 | 9.01 | 5.95 | 6.66 | — | — |
| Angle deviation, 95th percentile (F) | 22.42 | 25.28 | 26.61 | 25.24 | 18.70 | 20.39 | — | — |
| Mean aspect ratio (F) | 1.252 | 1.483 | 1.291 | 1.332 | 1.335 | 1.362 | — | — |
| Area variation (F) | 0.209 | 0.498 | 0.231 | 0.254 | 0.316 | 0.403 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 20.44 | 66.11 | 58.33 | 29.28 | 14.35 | 10.56 | — | — |
| Mean distance, ‰ (H) | 1.330 | 5.964 | 1.816 | 0.748 | 0.659 | 0.590 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 29.6 | 41.7 | 40.3 | 34.0 | 15.7 | 11.6 | — | — |
| Volume kept, % of the source (V) | 98.16 | 91.73 | 97.83 | 99.90 | 99.99 | 100.05 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 1.8 | 2.1 | 3.4 | 12.2 | 19.4 | 20.1 | — | — |
| Edges on spiralling loops, % (L) | 13.9 | 4.8 | 7.5 | 16.6 | 0.3 | 0.0 | — | — |
| Spiralling loops (L) | 5 | 2 | 3 | 8 | 4 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 5.2 | 0.0 | 0.0 | 12.8 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.6 | 93.0 | 89.1 | 71.2 | 80.3 | 79.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 79 | 48 | 52 | 97 | 76 | 79 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 2.8 | 59.6 | 2.0 | 6.2 | 1.2 | 1.3 | — | — |
| Time, seconds (T) | 1.22 | 1.05 | 0.93 | 5.72 | 7.01 | 5.05 | — | — |

## bob (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 972 | 576 | 1046 | 849 | 1019 | 997 | — | — |
| Irregular vertices (S) | 28 | 12 | 79 | 16 | 18 | 20 | — | — |
| … of which above the minimum | 28 | 12 | 79 | 16 | 18 | 20 | — | — |
| Base complex patches (C) | 958 | 125 | 1046 | 223 | 145 | 217 | — | — |
| Alignment, degrees (A) | 20.00 | 7.87 | 20.04 | 4.84 | 4.13 | 3.81 | — | — |
| Mean angle deviation, degrees (F) | 7.93 | 9.54 | 7.22 | 5.98 | 4.29 | 4.42 | — | — |
| Angle deviation, 95th percentile (F) | 22.21 | 23.72 | 24.08 | 16.01 | 11.71 | 12.78 | — | — |
| Mean aspect ratio (F) | 1.201 | 2.466 | 1.238 | 1.440 | 1.381 | 1.424 | — | — |
| Area variation (F) | 0.159 | 0.448 | 0.198 | 0.356 | 0.463 | 0.433 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 12.64 | 11.15 | 21.59 | 18.55 | 5.64 | 3.37 | — | — |
| Mean distance, ‰ (H) | 0.966 | 2.793 | 0.955 | 0.469 | 0.382 | 0.379 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.35 | 94.76 | 98.37 | 99.98 | 100.05 | 100.04 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 49.1 | 0.0 | 42.5 | 65.4 | 55.2 | — | — |
| Edges on spiralling loops, % (L) | 27.7 | 17.7 | 3.5 | 27.2 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 5 | 2 | 2 | 3 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 8.1 | 0.0 | 0.0 | 20.8 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 1 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 71.2 | 33.2 | 96.5 | 30.3 | 34.6 | 44.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 145 | 76 | 70 | 62 | 71 | 68 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 2.8 | 42.4 | 4.6 | 15.1 | 1.9 | 0.3 | — | — |
| Time, seconds (T) | 1.21 | 0.97 | 1.10 | 4.42 | 2.31 | 1.76 | — | — |

## blub (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 3/5 | 0/1 | 5/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 847 | 604 | 982 | 940 | 1014 | 1011 | — | — |
| Irregular vertices (S) | 44 | 58 | 75 | 44 | 42 | 58 | — | — |
| … of which above the minimum | 32 | — | 67 | — | 34 | 50 | — | — |
| Base complex patches (C) | 785 | — | 922 | — | 414 | 455 | — | — |
| Alignment, degrees (A) | 16.32 | 14.60 | 17.14 | 9.29 | 8.35 | 9.69 | — | — |
| Mean angle deviation, degrees (F) | 10.04 | 10.41 | 7.23 | 6.97 | 4.94 | 5.96 | — | — |
| Angle deviation, 95th percentile (F) | 28.85 | 32.33 | 26.82 | 19.43 | 14.03 | 18.92 | — | — |
| Mean aspect ratio (F) | 1.336 | 1.771 | 1.271 | 1.258 | 1.256 | 1.358 | — | — |
| Area variation (F) | 0.267 | 0.634 | 0.206 | 0.245 | 0.344 | 0.443 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 38.91 | 70.95 | 38.35 | 41.55 | 9.86 | 13.61 | — | — |
| Mean distance, ‰ (H) | 1.731 | 4.041 | 1.721 | 1.018 | 0.490 | 0.525 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 30.4 | 31.8 | 34.3 | 22.5 | 5.3 | 4.5 | — | — |
| Volume kept, % of the source (V) | 96.59 | — | 97.12 | 99.20 | 99.89 | 99.93 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 4.2 | 6.4 | 2.3 | 14.9 | 32.0 | 30.6 | — | — |
| Edges on spiralling loops, % (L) | 19.3 | 7.7 | 10.8 | 25.2 | 0.0 | 2.4 | — | — |
| Spiralling loops (L) | 7 | 3 | 3 | 5 | 0 | 2 | — | — |
| Edges on helices of any pitch, % (L) | 9.3 | 4.4 | 0.0 | 20.3 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 75.3 | 85.9 | 86.4 | 59.9 | 68.0 | 67.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 87 | 42 | 84 | 80 | 85 | 78 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 15.3 | 39.6 | 1.8 | 6.0 | 1.4 | 1.1 | — | — |
| Time, seconds (T) | 1.28 | 1.78 | 1.25 | 4.32 | 3.50 | 13.53 | — | — |

## pig (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 861 | 548 | 936 | 884 | 997 | 981 | — | — |
| Irregular vertices (S) | 92 | 69 | 113 | 52 | 54 | 80 | — | — |
| … of which above the minimum | 76 | — | 85 | 44 | 46 | 72 | — | — |
| Base complex patches (C) | 848 | — | 912 | 492 | 545 | 587 | — | — |
| Alignment, degrees (A) | 17.33 | 14.59 | 16.11 | 13.21 | 12.36 | 10.90 | — | — |
| Mean angle deviation, degrees (F) | 15.46 | 12.13 | 9.58 | 8.83 | 6.36 | 7.85 | — | — |
| Angle deviation, 95th percentile (F) | 46.32 | 38.98 | 33.62 | 26.20 | 21.12 | 24.18 | — | — |
| Mean aspect ratio (F) | 1.533 | 1.947 | 1.397 | 1.360 | 1.353 | 1.402 | — | — |
| Area variation (F) | 0.437 | 0.836 | 0.287 | 0.271 | 0.316 | 0.785 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 130.05 | 63.50 | 93.63 | 45.02 | 25.54 | 16.83 | — | — |
| Mean distance, ‰ (H) | 4.074 | 8.335 | 4.966 | 2.017 | 1.143 | 1.076 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 84.1 | 44.9 | 94.6 | 26.4 | 13.5 | 4.2 | — | — |
| Volume kept, % of the source (V) | 96.89 | 90.89 | 96.62 | 98.39 | 99.30 | 99.79 | — | — |
| Sharp edges kept, % of their length (E) | 18.3 | 7.2 | 8.4 | 36.1 | 64.2 | 61.0 | — | — |
| Edges on closed loops, % (L) | 0.9 | 13.1 | 0.9 | 24.3 | 24.3 | 20.7 | — | — |
| Edges on spiralling loops, % (L) | 14.4 | 4.5 | 7.0 | 21.8 | 7.5 | 5.5 | — | — |
| Spiralling loops (L) | 9 | 4 | 12 | 14 | 11 | 6 | — | — |
| Edges on helices of any pitch, % (L) | 9.4 | 0.0 | 0.0 | 0.0 | 0.0 | 3.6 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 83.9 | 82.4 | 91.6 | 53.9 | 68.3 | 73.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 94 | 24 | 52 | 58 | 76 | 80 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 13.9 | 45.2 | 6.4 | 11.6 | 0.3 | 1.9 | — | — |
| Time, seconds (T) | 1.28 | 1.11 | 0.86 | 7.15 | 8.30 | 14.34 | — | — |

## jerry (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 4/5 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 900 | 305 | 900 | 902 | 1011 | 988 | — | — |
| Irregular vertices (S) | 69 | 33 | 102 | 56 | 48 | 110 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 724 | — | 893 | 746 | 587 | 893 | — | — |
| Alignment, degrees (A) | 18.49 | 20.52 | 19.85 | 15.79 | 14.69 | 15.22 | — | — |
| Mean angle deviation, degrees (F) | 10.23 | 18.53 | 10.70 | 9.84 | 8.02 | 12.10 | — | — |
| Angle deviation, 95th percentile (F) | 28.90 | 57.15 | 34.84 | 29.21 | 24.31 | 37.60 | — | — |
| Mean aspect ratio (F) | 1.359 | 1.887 | 1.469 | 1.442 | 1.542 | 1.846 | — | — |
| Area variation (F) | 0.306 | 1.686 | 0.306 | 0.295 | 0.379 | 0.885 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 58.10 | 201.66 | 80.03 | 55.99 | 23.63 | 28.67 | — | — |
| Mean distance, ‰ (H) | 4.435 | 12.497 | 5.493 | 4.282 | 2.732 | 2.974 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 28.0 | 71.5 | 40.3 | 29.6 | 16.3 | 7.9 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | 20.2 | 0.6 | 8.5 | 27.3 | 47.6 | 70.7 | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 7.5 | 3.1 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 19.9 | 0.0 | 6.2 | 0.0 | 0.8 | — | — |
| Spiralling loops (L) | 0 | 3 | 0 | 2 | 0 | 1 | — | — |
| Edges on helices of any pitch, % (L) | 0.0 | 15.1 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 85.7 | 80.1 | 98.0 | 82.3 | 75.5 | 94.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 76 | 24 | 46 | 56 | 89 | 40 | — | — |
| Edges on loops from a border to a border, % (L) | 14.3 | 0.0 | 2.0 | 11.6 | 17.0 | 1.8 | — | — |
| Off budget, % (N) | 10.0 | 69.5 | 10.0 | 9.8 | 1.1 | 1.2 | — | — |
| Time, seconds (T) | 0.64 | 0.65 | 0.55 | 5.41 | 2.53 | 25.27 | — | — |

## rose (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 494 | 157 | 543 | 271 | 1467 | 1493 | — | — |
| Irregular vertices (S) | 164 | 122 | 197 | 134 | 730 | 747 | — | — |
| … of which above the minimum | — | — | 20 | — | — | — | — | — |
| Base complex patches (C) | 453 | — | 498 | — | — | — | — | — |
| Alignment, degrees (A) | 18.75 | 18.25 | 19.24 | 18.28 | 12.56 | 14.10 | — | — |
| Mean angle deviation, degrees (F) | 18.64 | 49.67 | 20.54 | 21.65 | 32.21 | 36.94 | — | — |
| Angle deviation, 95th percentile (F) | 52.24 | 87.99 | 57.09 | 63.45 | 79.14 | 80.31 | — | — |
| Mean aspect ratio (F) | 2.033 | 8.023 | 2.098 | 2.201 | 11.726 | 11.807 | — | — |
| Area variation (F) | 0.594 | 1.952 | 0.702 | 1.073 | 2.809 | 2.660 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 76.78 | 195.94 | 96.23 | 82.46 | 68.70 | 69.93 | — | — |
| Mean distance, ‰ (H) | 13.504 | 46.413 | 13.671 | 13.498 | 13.900 | 12.790 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 64.2 | 184.7 | 65.4 | 64.9 | 51.0 | 46.2 | — | — |
| Volume kept, % of the source (V) | — | — | 32.32 | — | 7.11 | — | — | — |
| Sharp edges kept, % of their length (E) | 6.6 | 0.7 | 4.1 | 3.9 | 27.5 | 30.1 | — | — |
| Edges on closed loops, % (L) | 3.9 | — | 6.8 | — | 0.9 | — | — | — |
| Edges on spiralling loops, % (L) | 19.8 | — | 10.7 | — | 38.8 | — | — | — |
| Spiralling loops (L) | 46 | — | 47 | — | 608 | — | — | — |
| Edges on helices of any pitch, % (L) | 6.4 | — | 0.0 | — | 11.3 | — | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | — | 0 | — | 1 | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 75.7 | — | 76.8 | — | 60.3 | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 16 | — | 14 | — | 42 | — | — | — |
| Edges on loops from a border to a border, % (L) | 1.5 | — | 4.9 | — | 0.0 | — | — | — |
| Off budget, % (N) | 50.6 | 84.3 | 45.7 | 72.9 | 46.7 | 49.3 | — | — |
| Time, seconds (T) | 1.97 | 0.69 | 1.58 | 22.99 | 219.71 | 88.84 | — | — |

## bridge (organic)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 2/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 748 | 817 | 990 | 1019 | 990 | — | — |
| Irregular vertices (S) | — | 54 | 74 | 33 | 34 | 34 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 483 | 235 | 246 | 259 | — | — |
| Alignment, degrees (A) | — | 11.10 | 14.05 | 8.32 | 7.19 | 6.64 | — | — |
| Mean angle deviation, degrees (F) | — | 16.94 | 8.62 | 6.39 | 3.63 | 4.94 | — | — |
| Angle deviation, 95th percentile (F) | — | 65.68 | 28.91 | 18.98 | 12.22 | 15.14 | — | — |
| Mean aspect ratio (F) | — | 2.304 | 1.359 | 1.284 | 1.253 | 1.281 | — | — |
| Area variation (F) | — | 1.107 | 0.259 | 0.274 | 0.297 | 0.458 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 98.43 | 94.26 | 13.21 | 17.46 | 17.46 | — | — |
| Mean distance, ‰ (H) | — | 7.488 | 3.875 | 0.975 | 0.934 | 0.888 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 15.2 | 46.8 | 5.8 | 4.5 | 2.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | — | 19.1 | 3.9 | 19.5 | 17.4 | 24.8 | — | — |
| Edges on spiralling loops, % (L) | — | 9.4 | 1.9 | 2.2 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | — | 4 | 2 | 1 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | — | 5.9 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 71.5 | 67.0 | 48.3 | 51.2 | 54.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 41 | 36 | 59 | 46 | 49 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | 25.2 | 30.1 | 31.5 | 21.2 | — | — |
| Off budget, % (N) | — | 25.2 | 18.3 | 1.0 | 1.9 | 1.0 | — | — |
| Time, seconds (T) | — | 0.92 | 1.57 | 3.81 | 3.74 | 3.85 | — | — |

Failures: quadriflow: cancelled.

## part (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 943 | 348 | 1036 | 1002 | 988 | 992 | — | — |
| Irregular vertices (S) | 30 | 69 | 115 | 24 | 24 | 24 | — | — |
| … of which above the minimum | 22 | — | 107 | 16 | 16 | 16 | — | — |
| Base complex patches (C) | 227 | — | 630 | 124 | 66 | 58 | — | — |
| Alignment, degrees (A) | 13.41 | 19.45 | 14.60 | 12.60 | 13.91 | 10.55 | — | — |
| Mean angle deviation, degrees (F) | 3.78 | 19.61 | 8.58 | 3.16 | 2.31 | 2.02 | — | — |
| Angle deviation, 95th percentile (F) | 15.04 | 60.84 | 35.19 | 10.45 | 10.46 | 10.09 | — | — |
| Mean aspect ratio (F) | 1.191 | 2.662 | 1.407 | 1.184 | 1.259 | 1.174 | — | — |
| Area variation (F) | 0.176 | 0.647 | 0.263 | 0.139 | 0.211 | 0.413 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 8.20 | 93.30 | 20.39 | 20.64 | 15.37 | 1.96 | — | — |
| Mean distance, ‰ (H) | 0.335 | 8.243 | 0.238 | 1.184 | 0.360 | 0.199 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.43 | — | 100.43 | 99.86 | 99.97 | 100.01 | — | — |
| Sharp edges kept, % of their length (E) | 100.0 | 2.1 | 97.8 | 81.3 | 98.4 | 100.0 | — | — |
| Edges on closed loops, % (L) | 47.9 | — | 22.2 | 63.2 | 73.6 | 76.0 | — | — |
| Edges on spiralling loops, % (L) | 13.6 | — | 1.2 | 9.4 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 4 | — | 1 | 3 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 9.0 | — | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | — | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 36.6 | — | 77.2 | 27.4 | 26.4 | 24.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 36 | — | 64 | 52 | 32 | 25 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 5.7 | 65.2 | 3.6 | 0.2 | 1.2 | 0.8 | — | — |
| Time, seconds (T) | 0.57 | 1.51 | 0.29 | 12.49 | 1.82 | 2.13 | — | — |

## cube (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 1014 | 573 | 900 | 999 | 1014 | 1014 | — | — |
| Irregular vertices (S) | 8 | 13 | 16 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | — | 8 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 6 | — | 174 | 999 | 6 | 6 | — | — |
| Alignment, degrees (A) | 0.17 | 3.25 | 1.29 | 1.55 | 0.11 | 0.11 | — | — |
| Mean angle deviation, degrees (F) | 0.08 | 4.61 | 2.07 | 2.64 | 0.00 | 0.00 | — | — |
| Angle deviation, 95th percentile (F) | 0.23 | 14.18 | 7.03 | 6.80 | 0.01 | 0.01 | — | — |
| Mean aspect ratio (F) | 1.003 | 1.707 | 1.093 | 1.086 | 1.000 | 1.000 | — | — |
| Area variation (F) | 0.002 | 0.500 | 0.103 | 0.074 | 0.000 | 0.000 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 2.31 | 23.35 | 0.00 | 25.44 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.015 | 1.194 | 0.000 | 0.399 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 100.00 | 99.43 | 100.00 | 99.93 | 100.00 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | 100.0 | 56.6 | 100.0 | 83.2 | 100.0 | 100.0 | — | — |
| Edges on closed loops, % (L) | 92.3 | 44.9 | 51.9 | 0.0 | 92.3 | 92.3 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 0.0 | 12.9 | 92.9 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 0 | 0 | 2 | 3 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 0.0 | 0.0 | 7.7 | 92.9 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 7.7 | 55.1 | 37.7 | 7.1 | 7.7 | 7.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 13 | 40 | 50 | 26 | 13 | 13 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 1.4 | 42.7 | 10.0 | 0.1 | 1.4 | 1.4 | — | — |
| Time, seconds (T) | 0.27 | 0.44 | 0.24 | 2.78 | 1.45 | 2.31 | — | — |

## lblock (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 930 | 588 | 932 | 992 | 994 | 994 | — | — |
| Irregular vertices (S) | 12 | 14 | 16 | 12 | 12 | 12 | — | — |
| … of which above the minimum | 4 | 6 | 8 | 4 | 4 | 4 | — | — |
| Base complex patches (C) | 85 | 132 | 150 | 26 | 14 | 14 | — | — |
| Alignment, degrees (A) | 1.36 | 4.35 | 1.42 | 1.80 | 0.56 | 0.56 | — | — |
| Mean angle deviation, degrees (F) | 1.26 | 4.89 | 2.16 | 2.01 | 0.10 | 0.10 | — | — |
| Angle deviation, 95th percentile (F) | 4.73 | 14.86 | 5.47 | 6.27 | 0.33 | 0.33 | — | — |
| Mean aspect ratio (F) | 1.064 | 1.571 | 1.073 | 1.120 | 1.072 | 1.072 | — | — |
| Area variation (F) | 0.059 | 0.415 | 0.078 | 0.080 | 0.070 | 0.070 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 9.75 | 29.11 | 0.00 | 13.72 | 0.00 | 0.00 | — | — |
| Mean distance, ‰ (H) | 0.123 | 1.880 | 0.000 | 0.591 | 0.000 | 0.000 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.99 | 99.10 | 100.00 | 99.86 | 100.00 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | 99.2 | 11.0 | 100.0 | 74.5 | 100.0 | 100.0 | — | — |
| Edges on closed loops, % (L) | 68.3 | 52.4 | 57.9 | 83.3 | 88.1 | 88.1 | — | — |
| Edges on spiralling loops, % (L) | 13.6 | 11.2 | 17.1 | 0.0 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 1 | 2 | 2 | 0 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 13.6 | 11.2 | 17.1 | 0.0 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 16.6 | 36.4 | 22.9 | 16.7 | 11.9 | 11.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 34 | 50 | 52 | 40 | 26 | 26 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 7.0 | 41.2 | 6.8 | 0.8 | 0.6 | 0.6 | — | — |
| Time, seconds (T) | 0.20 | 0.51 | 0.08 | 2.54 | 1.40 | 2.31 | — | — |

## cylinder (mechanical)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 995 | 612 | 1006 | 960 | 1000 | 978 | — | — |
| Irregular vertices (S) | 8 | 8 | 37 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | 0 | 29 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 376 | 164 | 546 | 6 | 6 | 6 | — | — |
| Alignment, degrees (A) | 3.95 | 5.40 | 5.10 | 1.26 | 1.14 | 1.23 | — | — |
| Mean angle deviation, degrees (F) | 4.50 | 8.90 | 5.58 | 2.88 | 1.83 | 1.97 | — | — |
| Angle deviation, 95th percentile (F) | 17.64 | 26.84 | 18.43 | 11.34 | 7.85 | 8.01 | — | — |
| Mean aspect ratio (F) | 1.124 | 1.965 | 1.167 | 1.146 | 1.187 | 1.337 | — | — |
| Area variation (F) | 0.112 | 0.576 | 0.167 | 0.209 | 0.206 | 0.138 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 3.89 | 14.27 | 14.78 | 1.23 | 1.29 | 1.36 | — | — |
| Mean distance, ‰ (H) | 0.373 | 2.062 | 0.364 | 0.262 | 0.264 | 0.304 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.73 | 98.02 | 99.68 | 99.99 | 100.00 | 100.00 | — | — |
| Sharp edges kept, % of their length (E) | 100.0 | 31.6 | 99.0 | 100.0 | 100.0 | 100.0 | — | — |
| Edges on closed loops, % (L) | 30.7 | 36.6 | 22.9 | 91.9 | 92.0 | 91.8 | — | — |
| Edges on spiralling loops, % (L) | 45.4 | 50.2 | 13.1 | 0.0 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 3 | 4 | 3 | 0 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 24.8 | 50.2 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 24.2 | 13.2 | 62.3 | 8.1 | 8.0 | 8.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 134 | 30 | 56 | 19 | 20 | 21 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 1.8 | 38.8 | 1.0 | 4.0 | 0.0 | 2.2 | — | — |
| Time, seconds (T) | 0.28 | 0.53 | 0.07 | 1.93 | 1.43 | 2.50 | — | — |

## venturi (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 2390 | 639 | 769 | 997 | 953 | — | — |
| Irregular vertices (S) | — | 1182 | 184 | 153 | 166 | 175 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 460 | — | 492 | — | — | — |
| Alignment, degrees (A) | — | 8.64 | 12.88 | 13.98 | 10.48 | 11.79 | — | — |
| Mean angle deviation, degrees (F) | — | 53.69 | 17.66 | 14.05 | 16.47 | 13.03 | — | — |
| Angle deviation, 95th percentile (F) | — | 89.78 | 59.41 | 41.94 | 53.40 | 41.96 | — | — |
| Mean aspect ratio (F) | — | 7.059 | 2.099 | 1.787 | 5.105 | 2.514 | — | — |
| Area variation (F) | — | 5.029 | 0.480 | 0.453 | 0.671 | 0.443 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 174.38 | 68.00 | 40.93 | 38.09 | 46.94 | — | — |
| Mean distance, ‰ (H) | — | 17.134 | 12.935 | 7.521 | 4.149 | 5.571 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 31.9 | 39.2 | 22.0 | 11.5 | 15.6 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 4.6 | 10.3 | 9.8 | 19.8 | 28.3 | — | — |
| Edges on closed loops, % (L) | — | — | 0.0 | 12.8 | 28.7 | 8.3 | — | — |
| Edges on spiralling loops, % (L) | — | — | 14.6 | 1.7 | 4.3 | 37.6 | — | — |
| Spiralling loops (L) | — | — | 84 | 11 | 73 | 73 | — | — |
| Edges on helices of any pitch, % (L) | — | — | 0.0 | 0.0 | 0.6 | 26.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 55.5 | 84.9 | 64.8 | 53.6 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 16 | 67 | 66 | 46 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | 29.5 | 0.7 | 2.2 | 0.5 | — | — |
| Off budget, % (N) | — | 139.0 | 36.1 | 23.1 | 0.3 | 4.7 | — | — |
| Time, seconds (T) | — | 1.11 | 0.97 | 12.91 | 10.04 | 85.30 | — | — |

Failures: quadriflow: cancelled.

## ogre_hp (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 4/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 334 | 940 | 754 | 1001 | 1012 | — | — |
| Irregular vertices (S) | — | 38 | 121 | 47 | 46 | 120 | — | — |
| … of which above the minimum | — | — | 112 | 39 | 38 | 112 | — | — |
| Base complex patches (C) | — | — | 910 | 576 | 393 | 724 | — | — |
| Alignment, degrees (A) | — | 18.93 | 19.57 | 14.94 | 14.07 | 17.17 | — | — |
| Mean angle deviation, degrees (F) | — | 14.98 | 10.46 | 11.33 | 8.06 | 9.39 | — | — |
| Angle deviation, 95th percentile (F) | — | 43.87 | 32.04 | 30.29 | 23.44 | 28.28 | — | — |
| Mean aspect ratio (F) | — | 1.799 | 1.433 | 1.590 | 1.848 | 1.529 | — | — |
| Area variation (F) | — | 0.758 | 0.296 | 0.410 | 0.413 | 0.946 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 124.75 | 81.49 | 37.41 | 28.36 | 32.57 | — | — |
| Mean distance, ‰ (H) | — | 11.455 | 3.775 | 2.364 | 1.757 | 1.872 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 89.7 | 99.1 | 37.5 | 21.1 | 6.8 | — | — |
| Volume kept, % of the source (V) | — | 86.48 | 95.96 | 99.14 | 99.72 | 99.69 | — | — |
| Sharp edges kept, % of their length (E) | — | 0.0 | 39.0 | 13.4 | 37.0 | 45.5 | — | — |
| Edges on closed loops, % (L) | — | 5.8 | 1.1 | 11.8 | 30.4 | 14.6 | — | — |
| Edges on spiralling loops, % (L) | — | 18.0 | 7.2 | 31.0 | 0.0 | 11.4 | — | — |
| Spiralling loops (L) | — | 9 | 7 | 6 | 0 | 21 | — | — |
| Edges on helices of any pitch, % (L) | — | 8.5 | 0.0 | 25.1 | 0.0 | 2.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 76.2 | 91.2 | 57.2 | 69.6 | 74.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 28 | 42 | 57 | 77 | 57 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | — | 66.6 | 6.0 | 24.6 | 0.1 | 1.2 | — | — |
| Time, seconds (T) | — | 12.47 | 80.70 | 67.69 | 76.00 | 96.99 | — | — |

Failures: quadriflow: cancelled.

## watermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 763 | 225 | 472 | 793 | 756 | 756 | — | — |
| Irregular vertices (S) | 130 | 79 | 109 | 113 | 111 | 108 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 751 | — | 444 | — | — | — | — | — |
| Alignment, degrees (A) | 16.26 | 19.32 | 17.29 | 16.77 | 16.56 | 16.57 | — | — |
| Mean angle deviation, degrees (F) | 15.96 | 26.39 | 20.73 | 15.42 | 15.48 | 15.47 | — | — |
| Angle deviation, 95th percentile (F) | 53.61 | 82.13 | 60.52 | 54.80 | 56.88 | 57.25 | — | — |
| Mean aspect ratio (F) | 1.692 | 4.170 | 2.168 | 1.826 | 1.829 | 1.815 | — | — |
| Area variation (F) | 0.466 | 1.055 | 0.540 | 0.457 | 0.502 | 0.502 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 130.87 | 298.17 | 134.69 | 83.66 | 112.24 | 112.24 | — | — |
| Mean distance, ‰ (H) | 17.928 | 82.620 | 18.494 | 11.464 | 11.438 | 11.354 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 47.7 | 197.5 | 46.3 | 29.8 | 29.3 | 29.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | 21.9 | 1.4 | 10.5 | 20.9 | 22.0 | 21.9 | — | — |
| Edges on closed loops, % (L) | 1.7 | — | 2.9 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 6.5 | — | 4.1 | — | — | — | — | — |
| Spiralling loops (L) | 14 | — | 18 | — | — | — | — | — |
| Edges on helices of any pitch, % (L) | 0.5 | — | 0.0 | — | — | — | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | — | 0 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 86.8 | — | 82.0 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 35 | — | 20 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 5.2 | — | 10.8 | — | — | — | — | — |
| Off budget, % (N) | 23.7 | 77.5 | 52.8 | 20.7 | 24.4 | 24.4 | — | — |
| Time, seconds (T) | 0.72 | 1.53 | 0.32 | 15.31 | 20.89 | 34.91 | — | — |

## towermill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 866 | 405 | 482 | 751 | 751 | 751 | — | — |
| Irregular vertices (S) | 165 | 121 | 98 | 90 | 90 | 90 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 846 | — | 454 | — | — | — | — | — |
| Alignment, degrees (A) | 13.39 | 18.17 | 16.12 | 12.95 | 12.95 | 12.95 | — | — |
| Mean angle deviation, degrees (F) | 14.47 | 27.63 | 22.89 | 14.09 | 14.09 | 14.09 | — | — |
| Angle deviation, 95th percentile (F) | 46.45 | 75.88 | 63.87 | 50.43 | 50.43 | 50.43 | — | — |
| Mean aspect ratio (F) | 1.621 | 16.899 | 2.110 | 1.781 | 1.781 | 1.781 | — | — |
| Area variation (F) | 0.403 | 0.903 | 0.539 | 0.413 | 0.413 | 0.413 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 102.61 | 138.78 | 99.89 | 78.85 | 78.85 | 78.85 | — | — |
| Mean distance, ‰ (H) | 9.005 | 32.456 | 14.260 | 8.291 | 8.291 | 8.291 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 22.9 | 75.3 | 34.7 | 19.8 | 19.8 | 19.8 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | 27.5 | 3.9 | 12.0 | 20.3 | 20.3 | 20.3 | — | — |
| Edges on closed loops, % (L) | 0.0 | — | 1.0 | — | — | — | — | — |
| Edges on spiralling loops, % (L) | 8.1 | — | 2.0 | — | — | — | — | — |
| Spiralling loops (L) | 7 | — | 6 | — | — | — | — | — |
| Edges on helices of any pitch, % (L) | 2.7 | — | 0.0 | — | — | — | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | — | 0 | — | — | — | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 89.2 | — | 78.5 | — | — | — | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 23 | — | 14 | — | — | — | — | — |
| Edges on loops from a border to a border, % (L) | 1.5 | — | 18.6 | — | — | — | — | — |
| Off budget, % (N) | 13.4 | 59.5 | 51.8 | 24.9 | 24.9 | 24.9 | — | — |
| Time, seconds (T) | 0.67 | 1.32 | 0.28 | 8.57 | 12.25 | 13.35 | — | — |

## stonemill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 938 | 703 | 738 | 858 | 992 | 1003 | — | — |
| Irregular vertices (S) | 132 | 73 | 114 | 70 | 92 | 172 | — | — |
| … of which above the minimum | — | — | 86 | — | 28 | 108 | — | — |
| Base complex patches (C) | 926 | — | 736 | — | 910 | 925 | — | — |
| Alignment, degrees (A) | 15.11 | 17.16 | 16.29 | 14.45 | 14.01 | 15.41 | — | — |
| Mean angle deviation, degrees (F) | 12.56 | 11.97 | 11.19 | 10.11 | 9.37 | 15.81 | — | — |
| Angle deviation, 95th percentile (F) | 35.62 | 36.94 | 34.13 | 27.86 | 31.98 | 44.84 | — | — |
| Mean aspect ratio (F) | 1.391 | 1.994 | 1.491 | 1.393 | 1.778 | 2.278 | — | — |
| Area variation (F) | 0.275 | 0.656 | 0.330 | 0.276 | 0.425 | 1.293 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 98.74 | 186.96 | 89.64 | 55.28 | 29.72 | 39.64 | — | — |
| Mean distance, ‰ (H) | 4.506 | 9.114 | 7.238 | 4.782 | 3.529 | 3.326 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 38.2 | 76.4 | 57.4 | 35.0 | 19.9 | 12.2 | — | — |
| Volume kept, % of the source (V) | — | — | 95.72 | — | 99.49 | 99.81 | — | — |
| Sharp edges kept, % of their length (E) | 33.0 | 5.4 | 10.3 | 20.1 | 30.4 | 41.3 | — | — |
| Edges on closed loops, % (L) | 0.3 | — | 0.6 | — | 4.4 | 4.0 | — | — |
| Edges on spiralling loops, % (L) | 4.3 | — | 5.5 | — | 11.0 | 1.3 | — | — |
| Spiralling loops (L) | 14 | — | 13 | — | 12 | 2 | — | — |
| Edges on helices of any pitch, % (L) | 0.0 | — | 0.0 | — | 7.0 | 0.8 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | — | 0 | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 95.4 | — | 93.5 | — | 84.5 | 94.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 50 | — | 40 | — | 57 | 54 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | — | 0.0 | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 6.2 | 29.7 | 26.2 | 14.2 | 0.8 | 0.3 | — | — |
| Time, seconds (T) | 0.65 | 1.01 | 0.42 | 20.82 | 16.75 | 47.88 | — | — |

## windmill (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 370 | 595 | — | 90 | 131 | — | — |
| Irregular vertices (S) | — | 75 | 137 | — | 32 | 32 | — | — |
| … of which above the minimum | — | — | — | — | 8 | 8 | — | — |
| Base complex patches (C) | — | — | 570 | — | 90 | 131 | — | — |
| Alignment, degrees (A) | — | 14.45 | 15.41 | — | 19.29 | 22.35 | — | — |
| Mean angle deviation, degrees (F) | — | 28.12 | 18.16 | — | 37.15 | 31.80 | — | — |
| Angle deviation, 95th percentile (F) | — | 83.45 | 55.93 | — | 74.94 | 71.80 | — | — |
| Mean aspect ratio (F) | — | 3.970 | 1.917 | — | 5.320 | 7.569 | — | — |
| Area variation (F) | — | 0.923 | 0.547 | — | 1.587 | 1.539 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 203.64 | 86.56 | — | 535.29 | 536.37 | — | — |
| Mean distance, ‰ (H) | — | 36.911 | 12.469 | — | 260.567 | 256.787 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 110.1 | 39.0 | — | 716.1 | 713.1 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | 0.37 | 0.33 | — | — |
| Sharp edges kept, % of their length (E) | — | 1.8 | 11.6 | — | 1.4 | 1.7 | — | — |
| Edges on closed loops, % (L) | — | — | 0.4 | — | 0.0 | 0.0 | — | — |
| Edges on spiralling loops, % (L) | — | — | 4.6 | — | 6.7 | 4.6 | — | — |
| Spiralling loops (L) | — | — | 24 | — | 12 | 12 | — | — |
| Edges on helices of any pitch, % (L) | — | — | 0.0 | — | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 84.3 | — | 93.3 | 95.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 24 | — | 8 | 20 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | 10.8 | — | 0.0 | 0.0 | — | — |
| Off budget, % (N) | — | 63.0 | 40.5 | — | 91.0 | 86.9 | — | — |
| Time, seconds (T) | — | 0.79 | 0.50 | — | 2.89 | 9.03 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 2 non-manifold or inconsistently oriented edges.

## terrain (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 0/1 | 5/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | 872 | 308 | 573 | 880 | 880 | 880 | — | — |
| Irregular vertices (S) | 44 | 30 | 41 | 86 | 86 | 86 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | 307 | — | 396 | — | — | — | — | — |
| Alignment, degrees (A) | 19.22 | 22.42 | 19.02 | 19.14 | 19.14 | 19.14 | — | — |
| Mean angle deviation, degrees (F) | 6.74 | 26.16 | 8.56 | 12.27 | 12.27 | 12.27 | — | — |
| Angle deviation, 95th percentile (F) | 23.83 | 73.60 | 28.21 | 34.05 | 34.05 | 34.05 | — | — |
| Mean aspect ratio (F) | 1.223 | 1.911 | 1.333 | 1.494 | 1.494 | 1.494 | — | — |
| Area variation (F) | 0.195 | 0.775 | 0.222 | 0.361 | 0.361 | 0.361 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 30.58 | 234.03 | 146.80 | 28.70 | 28.70 | 28.70 | — | — |
| Mean distance, ‰ (H) | 3.158 | 9.103 | 4.387 | 2.704 | 2.704 | 2.704 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | 29.7 | 54.8 | 45.1 | 27.0 | 27.0 | 27.0 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | 7.6 | 1.4 | 2.6 | 8.8 | 8.8 | 8.8 | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Edges on spiralling loops, % (L) | 0.0 | 7.8 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 0 | 1 | 0 | 0 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 57.4 | 92.2 | 68.4 | 88.9 | 88.9 | 88.9 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 25 | 27 | 21 | 52 | 52 | 52 | — | — |
| Edges on loops from a border to a border, % (L) | 42.6 | 0.0 | 29.5 | 11.1 | 11.1 | 11.1 | — | — |
| Off budget, % (N) | 12.8 | 69.2 | 42.7 | 12.0 | 12.0 | 12.0 | — | — |
| Time, seconds (T) | 15.63 | 1.83 | 6.55 | 72.51 | 99.27 | 101.44 | — | — |

## mahaveer (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 636 | 530 | 670 | 1993 | 1837 | — | — |
| Irregular vertices (S) | — | 343 | 107 | 48 | 474 | 478 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 429 | — | 522 | 575 | — | — |
| Alignment, degrees (A) | — | 8.88 | 12.03 | 9.04 | 9.84 | 16.30 | — | — |
| Mean angle deviation, degrees (F) | — | 38.85 | 16.03 | 12.72 | 33.80 | 31.76 | — | — |
| Angle deviation, 95th percentile (F) | — | 87.58 | 55.73 | 41.00 | 85.34 | 80.95 | — | — |
| Mean aspect ratio (F) | — | 7.492 | 1.885 | 2.932 | 52.561 | 104.764 | — | — |
| Area variation (F) | — | 1.913 | 0.471 | 0.511 | 6.184 | 7.912 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 88.93 | 87.54 | 46.97 | 325.69 | 325.82 | — | — |
| Mean distance, ‰ (H) | — | 14.421 | 11.410 | 5.497 | 39.809 | 39.865 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 65.0 | 55.5 | 28.3 | 153.6 | 153.7 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 16.7 | 11.8 | 16.6 | 29.7 | 28.9 | — | — |
| Edges on closed loops, % (L) | — | — | 10.4 | 22.5 | 9.4 | 9.5 | — | — |
| Edges on spiralling loops, % (L) | — | — | 14.8 | 6.9 | 81.4 | 73.3 | — | — |
| Spiralling loops (L) | — | — | 29 | 12 | 676 | 631 | — | — |
| Edges on helices of any pitch, % (L) | — | — | 2.0 | 0.0 | 68.0 | 54.8 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 73.8 | 46.5 | 8.6 | 16.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 34 | 33 | 6 | 53 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | 1.1 | 24.1 | 0.6 | 0.5 | — | — |
| Off budget, % (N) | — | 36.4 | 47.0 | 33.0 | 99.3 | 83.7 | — | — |
| Time, seconds (T) | — | 1.11 | 5.03 | 15.77 | 398.50 | 451.46 | — | — |

Failures: quadriflow: cancelled.

## marine (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 3/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 1110 | 206 | — | 8008 | 7261 | — | — |
| Irregular vertices (S) | — | 905 | 133 | — | 6265 | 6095 | — | — |
| … of which above the minimum | — | — | -55 | — | — | — | — | — |
| Base complex patches (C) | — | — | 206 | — | — | 5879 | — | — |
| Alignment, degrees (A) | — | — | 18.05 | — | — | — | — | — |
| Mean angle deviation, degrees (F) | — | 40.06 | 29.54 | — | 20.46 | 19.73 | — | — |
| Angle deviation, 95th percentile (F) | — | 88.92 | 71.88 | — | 69.88 | 68.54 | — | — |
| Mean aspect ratio (F) | — | 4.992 | 3.111 | — | 6.788 | 65.257 | — | — |
| Area variation (F) | — | 5.647 | 0.784 | — | 2.593 | 2.646 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 11.21 | 11.18 | — | 6.82 | 6.93 | — | — |
| Mean distance, ‰ (H) | — | 3.155 | 3.916 | — | 0.380 | 0.391 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 56.2 | 69.4 | — | 6.8 | 6.9 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 10.5 | 6.7 | — | 83.9 | 83.6 | — | — |
| Edges on closed loops, % (L) | — | — | 0.2 | — | — | 9.3 | — | — |
| Edges on spiralling loops, % (L) | — | — | 31.6 | — | — | 56.2 | — | — |
| Spiralling loops (L) | — | — | 60 | — | — | 6842 | — | — |
| Edges on helices of any pitch, % (L) | — | — | 0.0 | — | — | 1.4 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 67.3 | — | — | 33.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 6 | — | — | 14 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | 0.0 | — | — | 0.8 | — | — |
| Off budget, % (N) | — | 11.0 | 79.4 | — | 700.8 | 626.1 | — | — |
| Time, seconds (T) | — | 1.64 | 3.49 | — | 672.83 | 436.93 | — | — |

Failures: quadriflow: cancelled; integer-grid: error: invalid mesh: work mesh has 10 non-manifold or inconsistently oriented edges.

## f16 (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 3/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 3146 | 465 | 352 | 2471 | 2588 | — | — |
| Irregular vertices (S) | — | 1983 | 163 | 84 | 1916 | 1896 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 442 | — | — | 2173 | — | — |
| Alignment, degrees (A) | — | 17.00 | 15.03 | 12.50 | 5.53 | 6.47 | — | — |
| Mean angle deviation, degrees (F) | — | 45.13 | 23.16 | 20.37 | 21.02 | 22.37 | — | — |
| Angle deviation, 95th percentile (F) | — | 87.51 | 66.13 | 57.82 | 79.12 | 79.21 | — | — |
| Mean aspect ratio (F) | — | 8.830 | 3.120 | 2.368 | 98.722 | 110.802 | — | — |
| Area variation (F) | — | 5.401 | 0.584 | 0.618 | 5.162 | 6.009 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 128.15 | 144.73 | 148.59 | 70.90 | 68.70 | — | — |
| Mean distance, ‰ (H) | — | 13.795 | 32.571 | 24.496 | 3.401 | 2.787 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 52.9 | 129.3 | 98.8 | 11.7 | 9.8 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 8.0 | 20.3 | 15.9 | 55.0 | 53.4 | — | — |
| Edges on closed loops, % (L) | — | — | 6.7 | — | — | 7.8 | — | — |
| Edges on spiralling loops, % (L) | — | — | 11.5 | — | — | 49.7 | — | — |
| Spiralling loops (L) | — | — | 54 | — | — | 1935 | — | — |
| Edges on helices of any pitch, % (L) | — | — | 0.0 | — | — | 3.3 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | — | — | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 77.0 | — | — | 41.2 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 16 | — | — | 25 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | 2.8 | — | — | 1.3 | — | — |
| Off budget, % (N) | — | 214.6 | 53.5 | 64.8 | 147.1 | 158.8 | — | — |
| Time, seconds (T) | — | 1.23 | 4.01 | 51.31 | 47.65 | 71.57 | — | — |

Failures: quadriflow: cancelled.

## camera (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | — | 988 | 508 | 834 | 1093 | 1125 | — | — |
| Irregular vertices (S) | — | 272 | 126 | 40 | 107 | 122 | — | — |
| … of which above the minimum | — | — | — | — | — | — | — | — |
| Base complex patches (C) | — | — | 428 | — | 234 | 332 | — | — |
| Alignment, degrees (A) | — | 9.41 | 9.38 | 6.55 | 5.14 | 5.43 | — | — |
| Mean angle deviation, degrees (F) | — | 25.43 | 18.32 | 9.55 | 14.73 | 15.33 | — | — |
| Angle deviation, 95th percentile (F) | — | 86.02 | 55.68 | 33.77 | 63.02 | 63.93 | — | — |
| Mean aspect ratio (F) | — | 3.381 | 2.053 | 1.834 | 60.947 | 10.914 | — | — |
| Area variation (F) | — | 1.855 | 0.710 | 0.337 | 1.975 | 1.784 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 72.47 | 81.73 | 34.69 | 50.39 | 40.98 | — | — |
| Mean distance, ‰ (H) | — | 8.475 | 10.964 | 5.156 | 4.497 | 4.450 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 17.9 | 20.7 | 11.8 | 10.4 | 10.3 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 23.4 | 23.6 | 30.4 | 45.5 | 50.2 | — | — |
| Edges on closed loops, % (L) | — | — | 3.9 | 48.3 | 70.1 | 59.5 | — | — |
| Edges on spiralling loops, % (L) | — | — | 7.4 | 7.2 | 4.1 | 12.7 | — | — |
| Spiralling loops (L) | — | — | 34 | 5 | 57 | 66 | — | — |
| Edges on helices of any pitch, % (L) | — | — | 0.0 | 6.7 | 0.0 | 7.6 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | — | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | — | 79.8 | 26.1 | 19.7 | 22.8 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | — | 24 | 36 | 10 | 23 | — | — |
| Edges on loops from a border to a border, % (L) | — | — | 7.6 | 18.5 | 6.1 | 5.0 | — | — |
| Off budget, % (N) | — | 1.2 | 49.2 | 16.6 | 9.3 | 12.5 | — | — |
| Time, seconds (T) | — | 0.55 | 0.61 | 31.38 | 16.14 | 18.09 | — | — |

Failures: quadriflow: cancelled.

## wolf (extra)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 0/5 | 0/1 | 0/5 | 0/1 | 0/1 | 0/1 | 0/0 | 0/0 |
| Quads made | — | 180 | 866 | 775 | 795 | 783 | — | — |
| Irregular vertices (S) | — | 56 | 181 | 85 | 136 | 136 | — | — |
| … of which above the minimum | — | — | 172 | — | — | — | — | — |
| Base complex patches (C) | — | — | 810 | — | — | — | — | — |
| Alignment, degrees (A) | — | 14.42 | 14.10 | 14.28 | 15.12 | 15.13 | — | — |
| Mean angle deviation, degrees (F) | — | 24.14 | 14.78 | 18.02 | 19.83 | 19.63 | — | — |
| Angle deviation, 95th percentile (F) | — | 87.70 | 45.64 | 48.21 | 53.22 | 52.89 | — | — |
| Mean aspect ratio (F) | — | 3.965 | 1.759 | 2.027 | 2.190 | 2.222 | — | — |
| Area variation (F) | — | 0.945 | 0.411 | 0.562 | 0.572 | 0.564 | — | — |
| Hausdorff, ‰ of the diagonal (H) | — | 123.45 | 52.06 | 40.00 | 42.61 | 42.61 | — | — |
| Mean distance, ‰ (H) | — | 18.850 | 12.293 | 5.917 | 4.992 | 5.232 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | 123.8 | 121.8 | 54.9 | 45.0 | 47.5 | — | — |
| Volume kept, % of the source (V) | — | — | — | — | — | — | — | — |
| Sharp edges kept, % of their length (E) | — | 2.8 | 14.5 | 25.4 | 25.0 | 24.7 | — | — |
| Edges on closed loops, % (L) | — | 7.5 | 5.2 | — | 13.6 | 14.1 | — | — |
| Edges on spiralling loops, % (L) | — | 16.4 | 1.7 | — | 18.0 | 20.3 | — | — |
| Spiralling loops (L) | — | 14 | 8 | — | 42 | 70 | — | — |
| Edges on helices of any pitch, % (L) | — | 0.0 | 0.0 | — | 12.8 | 13.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | — | 0 | 0 | — | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | — | 76.1 | 93.1 | — | 68.2 | 65.4 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | — | 11 | 40 | — | 53 | 53 | — | — |
| Edges on loops from a border to a border, % (L) | — | 0.0 | 0.0 | — | 0.2 | 0.2 | — | — |
| Off budget, % (N) | — | 82.0 | 13.4 | 22.5 | 20.5 | 21.7 | — | — |
| Time, seconds (T) | — | 1.33 | 0.17 | 15.50 | 19.95 | 17.46 | — | — |

Failures: quadriflow: cancelled.

## sphere (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 950 | 768 | 1072 | 1025 | 1012 | 1014 | — | — |
| Irregular vertices (S) | 8 | 8 | 52 | 8 | 8 | 8 | — | — |
| … of which above the minimum | 0 | 0 | 44 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 937 | 768 | 1072 | 71 | 6 | 6 | — | — |
| Alignment, degrees (A) | — | — | — | — | — | — | — | — |
| Mean angle deviation, degrees (F) | 4.56 | 5.13 | 5.69 | 3.08 | 1.70 | 1.79 | — | — |
| Angle deviation, 95th percentile (F) | 12.31 | 13.45 | 19.78 | 8.88 | 5.53 | 5.86 | — | — |
| Mean aspect ratio (F) | 1.148 | 1.336 | 1.173 | 1.138 | 1.114 | 1.113 | — | — |
| Area variation (F) | 0.136 | 0.229 | 0.189 | 0.123 | 0.165 | 0.160 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 1.47 | 3.06 | 1.70 | 0.79 | 0.77 | 0.82 | — | — |
| Mean distance, ‰ (H) | 0.670 | 1.508 | 0.634 | 0.326 | 0.328 | 0.328 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 99.38 | 98.48 | 99.38 | 100.02 | 100.02 | 100.02 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 0.0 | 0.0 | 59.4 | 92.3 | 92.3 | — | — |
| Edges on spiralling loops, % (L) | 69.5 | 76.7 | 9.2 | 32.3 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 4 | 4 | 3 | 1 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 56.5 | 84.9 | 0.0 | 32.3 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 1 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 25.1 | 23.3 | 90.8 | 8.3 | 7.7 | 7.7 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 145 | 126 | 78 | 26 | 14 | 13 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 5.0 | 23.2 | 7.2 | 2.5 | 1.2 | 1.4 | — | — |
| Time, seconds (T) | 0.39 | 0.54 | 0.24 | 2.33 | 1.66 | 18.15 | — | — |

## torus (calibration)

| | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| **Valid meshes (V)** | 5/5 | 1/1 | 5/5 | 1/1 | 1/1 | 1/1 | 0/0 | 0/0 |
| Quads made | 1028 | 728 | 1038 | 920 | 1008 | 1008 | — | — |
| Irregular vertices (S) | 20 | 0 | 80 | 0 | 0 | 0 | — | — |
| … of which above the minimum | 20 | 0 | 80 | 0 | 0 | 0 | — | — |
| Base complex patches (C) | 1016 | 1 | 878 | 1 | 1 | 1 | — | — |
| Alignment, degrees (A) | 13.64 | 1.94 | 6.81 | 0.63 | 0.16 | 0.16 | — | — |
| Mean angle deviation, degrees (F) | 7.73 | 4.57 | 8.58 | 2.51 | 2.40 | 2.40 | — | — |
| Angle deviation, 95th percentile (F) | 18.19 | 9.20 | 25.35 | 3.93 | 3.73 | 3.73 | — | — |
| Mean aspect ratio (F) | 1.178 | 3.376 | 1.260 | 1.362 | 1.295 | 1.295 | — | — |
| Area variation (F) | 0.180 | 0.121 | 0.230 | 0.294 | 0.351 | 0.351 | — | — |
| Hausdorff, ‰ of the diagonal (H) | 3.12 | 5.42 | 4.07 | 1.51 | 1.35 | 1.35 | — | — |
| Mean distance, ‰ (H) | 0.775 | 1.726 | 0.865 | 0.447 | 0.424 | 0.424 | — | — |
| Thin parts: mean distance, % of a quad edge (V) | — | — | — | — | — | — | — | — |
| Volume kept, % of the source (V) | 98.57 | 96.27 | 98.25 | 100.06 | 100.04 | 100.04 | — | — |
| Sharp edges kept, % of their length (E) | — | — | — | — | — | — | — | — |
| Edges on closed loops, % (L) | 0.0 | 100.0 | 7.7 | 100.0 | 100.0 | 100.0 | — | — |
| Edges on spiralling loops, % (L) | 52.8 | 0.0 | 9.3 | 0.0 | 0.0 | 0.0 | — | — |
| Spiralling loops (L) | 11 | 0 | 5 | 0 | 0 | 0 | — | — |
| Edges on helices of any pitch, % (L) | 36.1 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Helices of pitch above 4, which the spiral count misses (L) | 0 | 0 | 0 | 0 | 0 | 0 | — | — |
| Edges on loops that stop at an irregular vertex, % (L) | 44.7 | 0.0 | 83.8 | 0.0 | 0.0 | 0.0 | — | — |
| Longest loop that stops at an irregular vertex, edges (L) | 109 | 0 | 56 | 0 | 0 | 0 | — | — |
| Edges on loops from a border to a border, % (L) | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | — | — |
| Off budget, % (N) | 2.8 | 27.2 | 3.8 | 8.0 | 0.8 | 0.8 | — | — |
| Time, seconds (T) | 0.84 | 0.73 | 0.30 | 0.93 | 1.34 | 1.34 | — | — |

## Summary

Number of models on which each tool has the best median, calibration shapes excluded. Ties count for each.

### organic (15 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 1 | 9 | 0 | 3 | 3 | 1 | 0 | 0 |
| Base complex patches (C) | 1 | 4 | 0 | 5 | 4 | 2 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 3 | 5 | 8 | 0 | 0 |
| Mean angle deviation, degrees (F) | 1 | 0 | 1 | 0 | 11 | 0 | 0 | 2 |
| Mean aspect ratio (F) | 7 | 0 | 2 | 0 | 4 | 0 | 0 | 2 |
| Valid meshes (V) | 38/75 | 4/15 | 46/75 | 11/15 | 14/15 | 14/15 | 0/2 | 2/2 |

### mechanical (4 models)

| Metric | quadriflow | autoremesher | instant-meshes | integer-grid | openclare-uniform | openclare | handmade | openclare-as-handmade |
|---|---|---|---|---|---|---|---|---|
| Irregular vertices (S) | 3 | 1 | 0 | 4 | 4 | 4 | 0 | 0 |
| Base complex patches (C) | 1 | 0 | 0 | 1 | 3 | 4 | 0 | 0 |
| Alignment, degrees (A) | 0 | 0 | 0 | 0 | 2 | 2 | 0 | 0 |
| Mean angle deviation, degrees (F) | 0 | 0 | 0 | 0 | 3 | 1 | 0 | 0 |
| Mean aspect ratio (F) | 2 | 0 | 0 | 0 | 0 | 2 | 0 | 0 |
| Valid meshes (V) | 20/20 | 2/4 | 20/20 | 4/4 | 4/4 | 4/4 | 0/0 | 0/0 |
