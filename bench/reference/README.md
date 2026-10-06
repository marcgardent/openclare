# Mires conservées

Sorties d'outils qu'on ne pourra pas relancer à volonté. Elles sont versionnées pour que le banc
de mesure continue à s'y comparer une fois la licence expirée.

## quadremesher/

Quad Remesher 1.4.1 (Exoside), licence d'essai, lancé le 6 octobre 2026 depuis Blender 5.2.2 sur
les modèles de `bench/corpus/`. Usage non commercial.

Un fichier par modèle, budget et réglage : `<modèle>__<quads demandés>__<réglage>.obj`, avec à côté
un `.seconds` qui garde le temps de calcul mesuré.

| Réglage | Signification |
|---|---|
| `default` | réglages d'origine : taille adaptative à 50 %, nombre de quads ajusté |
| `uniform` | taille constante et nombre de quads exact, pour comparer à armes égales |

Budgets : 1000, 3000 et 8000 quads. Détection des arêtes vives activée sur les modèles mécaniques
(`part`, `cube`) seulement ; symétrie désactivée.

Ces maillages ne valent que pour les sources exactes de `bench/corpus/`, elles aussi versionnées :
régénérer le corpus avec une autre version de Blender donnerait des sources légèrement différentes.

Pour en produire d'autres tant qu'une licence est disponible :

```bash
python3 bench/make_references.py --targets 1000,3000,8000
```

Les mires déjà présentes sont conservées ; `--refresh` les recalcule.
