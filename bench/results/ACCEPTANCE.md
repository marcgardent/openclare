# Acceptance

Chain under test: `openclare`.
The thresholds are those of ALGORITHM.md, section 0, against QuadriFlow at the same budget.
They were set for the organic models; the mechanical ones are held to the same for reference.

## 1000 quads, organic models

Models: blob, rock, creature, cage, suzanne, ogre, spot, bob, blub, pig, jerry, bridge. Medians over those each tool was run on.

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Models run | 12 | 12 | 12 | 12 | 12 | 12 | 12 |
| Models valid on every run (V) | 4 | 6 | 8 | 12 | 12 | 11 | 8 |
| Irregular vertices (S) | 62 | 88 | 40 | 42 | 48 | 72 | 61 |
| Base complex patches (C) | 891 | 886 | 598 | 582 | 544 | 1079 | 437 |
| Alignment, degrees (A) | 18.0 | 17.8 | 10.1 | 9.0 | 8.4 | 6.4 | 8.1 |
| Mean angle deviation, degrees (F) | 9.7 | 8.1 | 8.8 | 5.4 | 5.7 | 8.2 | 8.8 |
| Hausdorff, per mille of the diagonal (H) | 38.9 | 41.1 | 20.7 | 17.2 | 11.8 | 7.9 | 18.3 |
| Off budget, % (N) | 5.9 | 3.5 | 9.5 | 0.8 | 0.9 | 71.4 | 11.3 |
| Area variation | 0.27 | 0.25 | 0.30 | 0.35 | 0.46 | 0.55 | 0.33 |
| Mean distance, per mille of the diagonal | 1.55 | 1.95 | 1.23 | 0.99 | 0.93 | 0.90 | 1.65 |
| Time, seconds (T) | 1.3 | 1.1 | 4.9 | 2.0 | 2.3 | 13.5 | 12.9 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| blob | 1/1 / 4/5 | 4.4 / 18.0 | 24 / 36 | 790 / 941 | 5.8 / 9.3 | 6.3 / 14.5 | 1.0 % | — |
| rock | 1/1 / 5/5 | 16.9 / 23.1 | 26 / 8 | 981 / 941 | 5.0 / 6.2 | 7.7 / 9.4 | 1.9 % | SC |
| creature | 1/1 / 4/5 | 5.7 / 14.8 | 40 / 16 | 355 / 812 | 5.5 / 12.6 | 3.2 / 40.5 | 0.1 % | S |
| cage | 1/1 / 5/5 | 3.2 / 7.7 | 48 / 64 | 208 / 716 | 5.0 / 7.0 | 10.7 / 21.4 | 0.1 % | — |
| suzanne | 1/1 / 0/5 | 10.4 / 15.7 | 48 / 67 | 554 / 995 | 5.6 / 9.7 | 12.9 / 44.8 | 0.6 % | — |
| ogre | 1/1 / 3/5 | 11.2 / 20.9 | 102 / 73 | 534 / 891 | 9.3 / 14.2 | 29.8 / 95.0 | 3.7 % | — |
| spot | 1/1 / 5/5 | 7.7 / 18.2 | 68 / 62 | 721 / 965 | 7.0 / 7.6 | 15.1 / 20.4 | 0.9 % | — |
| bob | 1/1 / 5/5 | 3.6 / 20.0 | 20 / 28 | 185 / 958 | 4.5 / 7.9 | 4.3 / 12.6 | 2.8 % | — |
| blub | 1/1 / 3/5 | 9.2 / 16.3 | 50 / 44 | 523 / 785 | 5.7 / 10.0 | 17.3 / 38.9 | 1.1 % | — |
| pig | 1/1 / 0/5 | 9.3 / 17.3 | 92 / 92 | 603 / 848 | 7.3 / 15.5 | 18.6 / 130.1 | 0.9 % | — |
| jerry | 1/1 / 4/5 | 15.3 / 18.5 | 83 / 69 | 886 / 724 | 9.0 / 10.2 | 25.1 / 58.1 | 0.5 % | C |
| bridge | 1/1 / 0/5 | 6.7 / — | 36 / — | 451 / — | 5.6 / — | 9.1 / — | 0.8 % | — |

Thresholds met, out of 12 models: V 12, A 12, S 10, C 10, F 12, H 12, N 12.

## 1000 quads, mechanical models

Models: part, cube, lblock, cylinder. Medians over those each tool was run on.

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Models run | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| Models valid on every run (V) | 4 | 3 | 4 | 4 | 4 | 4 | 4 |
| Irregular vertices (S) | 10 | 28 | 10 | 10 | 10 | 10 | 10 |
| Base complex patches (C) | 156 | 375 | 283 | 10 | 41 | 76 | 76 |
| Alignment, degrees (A) | 2.7 | 3.1 | 1.9 | 0.9 | 1.2 | 1.1 | 1.1 |
| Mean angle deviation, degrees (F) | 2.5 | 3.6 | 3.5 | 1.1 | 1.3 | 2.2 | 2.2 |
| Hausdorff, per mille of the diagonal (H) | 6.0 | 7.7 | 15.5 | 0.7 | 0.7 | 0.8 | 1.1 |
| Off budget, % (N) | 3.8 | 6.7 | 2.5 | 1.2 | 1.6 | 1.4 | 4.5 |
| Area variation | 0.09 | 0.14 | 0.11 | 0.12 | 0.15 | 0.17 | 0.16 |
| Mean distance, per mille of the diagonal | 0.23 | 0.12 | 0.50 | 0.13 | 0.08 | 0.07 | 0.09 |
| Time, seconds (T) | 0.3 | 0.1 | 3.4 | 1.1 | 1.3 | 2.9 | 2.8 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| part | 1/1 / 5/5 | 10.9 / 13.4 | 24 / 30 | 68 / 227 | 2.2 / 3.8 | 1.4 / 8.2 | 1.8 % | — |
| cube | 1/1 / 5/5 | 0.3 / 0.2 | 8 / 8 | 6 / 6 | 0.3 / 0.1 | 0.0 / 2.3 | 1.4 % | AF |
| lblock | 1/1 / 5/5 | 0.6 / 1.4 | 12 / 12 | 14 / 85 | 0.4 / 1.3 | 0.0 / 9.7 | 0.6 % | A |
| cylinder | 1/1 / 5/5 | 1.8 / 3.9 | 8 / 8 | 186 / 376 | 3.7 / 4.5 | 1.7 / 3.9 | 3.2 % | — |

Thresholds met, out of 4 models: V 4, A 2, S 4, C 4, F 3, H 4, N 4.

## 3000 quads, organic models

Models: blob, rock, creature, cage, suzanne, ogre, spot, bob, blub, pig, jerry, bridge. Medians over those each tool was run on.

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Models run | 12 | 12 | 12 | 12 | 12 | 12 | 12 |
| Models valid on every run (V) | 5 | 8 | 9 | 12 | 12 | 8 | 10 |
| Irregular vertices (S) | 66 | 156 | 50 | 50 | 57 | 80 | 68 |
| Base complex patches (C) | 2697 | 2917 | 951 | 1886 | 1924 | 2409 | 2009 |
| Alignment, degrees (A) | 17.5 | 16.6 | 8.8 | 8.4 | 7.7 | 6.4 | 7.7 |
| Mean angle deviation, degrees (F) | 8.6 | 6.0 | 8.0 | 5.5 | 5.1 | 6.5 | 7.2 |
| Hausdorff, per mille of the diagonal (H) | 17.7 | 17.5 | 7.8 | 8.0 | 7.1 | 5.9 | 9.6 |
| Off budget, % (N) | 4.1 | 2.1 | 11.1 | 1.4 | 1.0 | 36.6 | 10.3 |
| Area variation | 0.24 | 0.18 | 0.27 | 0.31 | 0.43 | 0.43 | 0.28 |
| Mean distance, per mille of the diagonal | 0.73 | 0.73 | 0.54 | 0.42 | 0.37 | 0.41 | 0.70 |
| Time, seconds (T) | 2.0 | 1.2 | 8.6 | 3.0 | 3.1 | 14.5 | 12.3 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| blob | 1/1 / 3/5 | 4.9 / 17.5 | 24 / 38 | 2045 / 2697 | 5.7 / 8.7 | 2.4 / 8.5 | 0.2 % | — |
| rock | 1/1 / 5/5 | 15.8 / 23.0 | 66 / 8 | 2984 / 2855 | 5.1 / 5.8 | 4.6 / 5.4 | 0.5 % | SC |
| creature | 1/1 / 5/5 | 5.8 / 12.9 | 40 / 24 | 2839 / 2667 | 5.1 / 6.8 | 2.0 / 8.0 | 1.5 % | SC |
| cage | 1/1 / 5/5 | 2.3 / 8.1 | 48 / 66 | 304 / 1943 | 3.2 / 5.9 | 10.6 / 11.5 | 1.1 % | — |
| suzanne | 1/1 / 0/5 | 9.9 / 15.3 | 56 / 78 | 2000 / 2884 | 4.3 / 8.6 | 7.8 / 42.3 | 0.5 % | — |
| ogre | 1/1 / 1/5 | 11.6 / 19.3 | 155 / 106 | 2483 / 2857 | 6.3 / 12.0 | 28.4 / 36.6 | 0.6 % | — |
| spot | 1/1 / 5/5 | 7.9 / 18.1 | 70 / 68 | 1663 / 2902 | 5.6 / 7.2 | 5.6 / 17.7 | 0.7 % | — |
| bob | 1/1 / 5/5 | 3.6 / 19.4 | 20 / 32 | 237 / 2903 | 3.2 / 6.4 | 1.4 / 6.8 | 1.7 % | — |
| blub | 1/1 / 3/5 | 7.5 / 16.0 | 58 / 52 | 1333 / 2584 | 4.9 / 10.7 | 7.5 / 40.7 | 0.9 % | — |
| pig | 1/1 / 0/5 | 9.1 / 17.4 | 102 / 122 | 1849 / 2612 | 5.3 / 14.3 | 10.0 / 44.6 | 1.0 % | — |
| jerry | 1/1 / 4/5 | 13.6 / 17.9 | 162 / 104 | 2491 / 2211 | 6.7 / 11.3 | 21.7 / 38.8 | 1.8 % | SC |
| bridge | 1/1 / 0/5 | 7.4 / — | 46 / — | 764 / — | 4.0 / — | 6.7 / — | 1.4 % | — |

Thresholds met, out of 12 models: V 12, A 12, S 9, C 9, F 12, H 12, N 12.

## 3000 quads, mechanical models

Models: part, cube, lblock, cylinder. Medians over those each tool was run on.

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Models run | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| Models valid on every run (V) | 2 | 4 | 4 | 4 | 4 | 4 | 4 |
| Irregular vertices (S) | 10 | 58 | 12 | 10 | 10 | 10 | 10 |
| Base complex patches (C) | 352 | 1123 | 407 | 46 | 150 | 211 | 211 |
| Alignment, degrees (A) | 2.6 | 3.7 | 1.2 | 1.0 | 1.2 | 1.1 | 1.1 |
| Mean angle deviation, degrees (F) | 1.7 | 2.9 | 1.7 | 0.7 | 1.4 | 1.7 | 1.7 |
| Hausdorff, per mille of the diagonal (H) | 5.2 | 1.9 | 0.9 | 0.5 | 0.2 | 0.4 | 0.4 |
| Off budget, % (N) | 3.3 | 2.7 | 1.9 | 1.1 | 0.8 | 4.5 | 4.5 |
| Area variation | 0.07 | 0.12 | 0.09 | 0.07 | 0.17 | 0.11 | 0.11 |
| Mean distance, per mille of the diagonal | 0.07 | 0.03 | 0.06 | 0.03 | 0.03 | 0.03 | 0.03 |
| Time, seconds (T) | 1.6 | 0.3 | 6.2 | 1.9 | 2.0 | 2.8 | 2.9 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| part | 1/1 / 4/5 | 10.5 / 13.0 | 24 / 28 | 285 / 619 | 2.2 / 2.7 | 0.5 / 5.5 | 0.7 % | — |
| cube | 1/1 / 5/5 | 0.4 / 0.3 | 8 / 8 | 6 / 6 | 0.4 / 0.2 | 0.0 / 4.8 | 0.3 % | AF |
| lblock | 1/1 / 4/5 | 0.9 / 1.1 | 12 / 12 | 14 / 86 | 0.5 / 0.8 | 0.0 / 10.3 | 0.9 % | A |
| cylinder | 1/1 / 5/5 | 1.6 / 4.2 | 8 / 8 | 463 / 1003 | 2.7 / 4.3 | 0.8 / 2.1 | 1.3 % | — |

Thresholds met, out of 4 models: V 4, A 2, S 4, C 4, F 3, H 4, N 4.

## 8000 quads, organic models

Models: blob, rock, creature, cage, suzanne, ogre, spot, bob, blub, pig, jerry, bridge. Medians over those each tool was run on.

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Models run | 12 | 12 | 12 | 12 | 12 | 12 | 12 |
| Models valid on every run (V) | 3 | 9 | 7 | 12 | 12 | 11 | 9 |
| Irregular vertices (S) | 70 | 278 | 66 | 67 | 80 | 72 | 70 |
| Base complex patches (C) | 7342 | 7812 | 3344 | 4436 | 4895 | 4605 | 3789 |
| Alignment, degrees (A) | 15.8 | 15.8 | 8.9 | 8.7 | 7.9 | 7.5 | 8.6 |
| Mean angle deviation, degrees (F) | 6.4 | 4.5 | 8.7 | 6.1 | 5.7 | 6.4 | 6.8 |
| Hausdorff, per mille of the diagonal (H) | 6.9 | 6.5 | 4.6 | 3.3 | 2.7 | 4.3 | 5.9 |
| Off budget, % (N) | 3.0 | 1.2 | 10.5 | 0.8 | 1.0 | 12.7 | 11.9 |
| Area variation | 0.21 | 0.14 | 0.28 | 0.32 | 0.41 | 0.37 | 0.28 |
| Mean distance, per mille of the diagonal | 0.27 | 0.26 | 0.20 | 0.18 | 0.15 | 0.24 | 0.31 |
| Time, seconds (T) | 4.5 | 1.4 | 33.2 | 7.5 | 8.5 | 15.5 | 13.0 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| blob | 1/1 / 3/5 | 5.5 / 14.8 | 24 / 40 | 6819 / 7412 | 5.8 / 7.8 | 1.8 / 5.2 | 0.3 % | — |
| rock | 1/1 / 5/5 | 16.3 / 22.4 | 134 / 10 | 7940 / 7342 | 5.4 / 6.2 | 2.6 / 7.0 | 0.8 % | SC |
| creature | 1/1 / 4/5 | 5.6 / 13.7 | 42 / 22 | 3225 / 8170 | 5.6 / 6.0 | 0.9 / 3.2 | 2.2 % | S |
| cage | 1/1 / 5/5 | 1.9 / 8.2 | 58 / 72 | 275 / 7283 | 2.3 / 5.8 | 2.9 / 6.6 | 0.7 % | — |
| suzanne | 1/1 / 5/5 | 9.8 / 14.3 | 82 / 55 | 4783 / 6194 | 6.0 / 6.1 | 3.0 / 6.9 | 1.3 % | — |
| ogre | 1/1 / 0/5 | 10.8 / 17.8 | 240 / 162 | 6091 / 7352 | 6.3 / 11.6 | 8.9 / 27.3 | 0.6 % | — |
| spot | 1/1 / 4/5 | 7.5 / 16.8 | 78 / 76 | 5007 / 7772 | 6.5 / 6.2 | 2.1 / 5.3 | 1.2 % | F |
| bob | 1/1 / 3/5 | 3.8 / 17.3 | 20 / 36 | 2898 / 8263 | 3.8 / 6.4 | 1.0 / 3.2 | 0.2 % | — |
| blub | 1/1 / 0/5 | 6.4 / 15.2 | 56 / 70 | 4424 / 6275 | 6.1 / 10.3 | 2.4 / 21.9 | 0.1 % | — |
| pig | 1/1 / 0/5 | 8.2 / 15.8 | 98 / 136 | 5403 / 6841 | 5.7 / 11.4 | 6.1 / 56.2 | 1.8 % | — |
| jerry | 1/1 / 4/5 | 12.4 / 17.6 | 273 / 112 | 7075 / 4991 | 5.9 / 7.8 | 10.0 / 31.3 | 1.3 % | SC |
| bridge | 1/1 / 0/5 | 9.0 / — | 81 / — | 2725 / — | 4.6 / — | 6.7 / — | 1.4 % | — |

Thresholds met, out of 12 models: V 12, A 12, S 9, C 10, F 11, H 12, N 12.

## 8000 quads, mechanical models

Models: part, cube, lblock, cylinder. Medians over those each tool was run on.

| | quadriflow | instant-meshes | integer-grid | openclare-uniform | openclare | quadremesher | quadremesher-uniform |
|---|---|---|---|---|---|---|---|
| Models run | 2 | 4 | 4 | 4 | 4 | 4 | 4 |
| Models valid on every run (V) | 1 | 3 | 4 | 4 | 4 | 4 | 4 |
| Irregular vertices (S) | 23 | 94 | 10 | 10 | 10 | 10 | 10 |
| Base complex patches (C) | 588 | 2692 | 737 | 98 | 53 | 372 | 372 |
| Alignment, degrees (A) | 6.9 | 2.3 | 1.0 | 1.0 | 1.1 | 1.1 | 1.1 |
| Mean angle deviation, degrees (F) | 1.5 | 2.0 | 1.3 | 0.6 | 0.7 | 1.8 | 1.8 |
| Hausdorff, per mille of the diagonal (H) | 3.0 | 1.0 | 0.4 | 0.2 | 0.1 | 0.3 | 0.3 |
| Off budget, % (N) | 0.6 | 1.5 | 1.5 | 0.8 | 1.0 | 4.5 | 4.5 |
| Area variation | 0.05 | 0.08 | 0.07 | 0.07 | 0.15 | 0.13 | 0.13 |
| Mean distance, per mille of the diagonal | 0.02 | 0.01 | 0.03 | 0.01 | 0.01 | 0.02 | 0.02 |
| Time, seconds (T) | 6.1 | 0.7 | 9.4 | 5.2 | 5.4 | 3.3 | 3.0 |

Model by model, against QuadriFlow (ours / QuadriFlow's). A letter in the last column is a threshold missed.

| Model | V | A | S | C | F | H | N | Missed |
|---|---|---|---|---|---|---|---|---|
| part | 1/1 / 3/5 | 10.3 / 13.3 | 24 / 38 | 92 / 1171 | 1.0 / 2.6 | 0.3 / 5.1 | 1.2 % | — |
| cube | 1/1 / 5/5 | 0.3 / 0.5 | 8 / 8 | 6 / 6 | 0.2 / 0.4 | 0.0 / 0.8 | 0.8 % | A |

Thresholds met, out of 2 models: V 2, A 1, S 2, C 2, F 2, H 2, N 2.
