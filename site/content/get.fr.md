# Obtenir openclare

<p class="subtitle">Le code source est sur GitHub, à <a href="https://github.com/marcgardent/openclare">marcgardent/openclare</a>. Sa documentation est en anglais.</p>

## L'extension Blender

C'est l'édition Community de l'extension.

Une seule archive contient tout : l'extension et son moteur, compilé. Il n'y a rien d'autre à installer, ni paquet Python, ni programme à côté, et rien n'est téléchargé pendant qu'elle tourne.

1. Prenez l'archive de votre système dans les [versions publiées](https://github.com/marcgardent/openclare/releases) : `openclare-<version>-<plateforme>.zip`, la plateforme étant `linux-x64`, `windows-x64`, `macos-arm64` ou `macos-x64`. Ou construisez-la vous-même (plus bas).
2. Dans Blender 4.2 ou plus récent : *Edit › Preferences › Get Extensions*, la flèche en haut à droite, *Install from Disk*, et choisissez l'archive.

Un panneau nommé *openclare* apparaît dans l'éditeur Properties, sur l'onglet du maillage (Object Data). Sélectionnez un maillage, réglez ce qui suit, appuyez sur *Remesh to Quads*. Le résultat est un nouvel objet à côté de l'original, qui reste tel quel. Le moteur tourne en arrière-plan : Blender reste libre, et le panneau montre une barre de progression avec un bouton pour l'arrêter.

### Ce qu'elle fait

- **Combien de quads, et où.** Un nombre de quads, ou les tailles du plus grand et du plus petit. *Show Sizes* colore l'objet de la taille qu'auront ses quads.
- **La densité.** *Adaptive Size* rend les quads plus petits là où la forme se courbe ; les parties fines (un doigt, une oreille) gardent les quads qui en font le tour. Une densité peinte en Weight Paint dit où ils sont plus petits, et *Density from Shape* écrit la densité de la forme elle-même en un groupe à repeindre.
- **Les lignes d'arêtes.** Les arêtes vives, les arêtes marquées sharp et les frontières entre matériaux sont gardées comme lignes d'arêtes.
- **Des courbes tracées sur le modèle.** *Draw Curve* et *Cut Volume*, dans la barre d'outils, tracent une courbe sur le modèle ou la boucle autour d'un membre. Une courbe *Guide* donne une ligne d'arêtes ; une courbe *Sharp* donne une ligne d'arêtes vives.
- **Le résultat.** La symétrie selon X, Y ou Z. Les arêtes vives marquées sharp et crease, pour qu'un *Subdivision Surface* les garde. L'ombrage par angle, lisse ou plat. Des préréglages enregistrés des réglages. *Remesh Again* refait le résultat à sa place, et le panneau rend compte du calcul.

La version Linux tourne partout où Blender tourne (glibc 2.28 ou plus récente). Les versions Windows et macOS passent les tests de l'interface C sur leur système mais n'ont pas encore tourné dans un Blender sur ces systèmes ; celle pour macOS est signée ad hoc, non notariée. Chaque version publiée contient aussi le moteur en bibliothèque C avec son en-tête, `openclare-capi-<version>-<plateforme>.tar.gz`, pour d'autres programmes.

Ce que propose le panneau, réglage par réglage, est dans le [README](../../README.md).

### Construire l'extension

Avec la [chaîne d'outils Rust](https://www.rust-lang.org/tools/install) installée, depuis la racine du dépôt :

```sh
make addon      # dist/openclare-<version>-linux-x64.zip, à installer depuis Blender
make install    # ou directement dans les extensions du Blender du PATH
```

`make addon-windows` construit l'archive Windows depuis Linux (il faut la cible Rust `x86_64-pc-windows-gnu` et MinGW-w64). `make help` liste tous les raccourcis.

## La ligne de commande

```sh
cargo build --release
./target/release/openclare remesh input.obj output.obj --quads 5000
./target/release/openclare remesh part.obj quads.obj --preset hard-surface --quads 3000
./target/release/openclare primitive torus torus.obj     # une forme pour l'essayer
./target/release/openclare measure input.obj output.obj --target 5000
```

`openclare --help` liste toutes les commandes et options. Les préréglages et réglages enregistrés, les tailles au lieu d'un nombre de quads, les courbes guides, la bibliothèque Rust et l'interface C pour d'autres hôtes sont décrits dans [USAGE.md](../../USAGE.md).

## Plus loin (en anglais)

<div class="cards">
<a class="card" href="../../README.md"><strong>README</strong><span>L'extension, réglage par réglage.</span></a>
<a class="card" href="../../USAGE.md"><strong>USAGE.md</strong><span>La ligne de commande, la bibliothèque, l'interface C, et regarder la chaîne travailler.</span></a>
<a class="card" href="../../CONTRIBUTING.md"><strong>CONTRIBUTING.md</strong><span>Construire, tester, le banc, l'organisation du code.</span></a>
<a class="card" href="../../ALGORITHM.md"><strong>ALGORITHM.md</strong><span>Comment ça marche, étape par étape.</span></a>
<a class="card" href="../../CREDITS.md"><strong>CREDITS.md</strong><span>Auteurs, sources et licences des modèles 3D utilisés.</span></a>
</div>

## Licence

Le code est sous GNU GPL, version 3 ou ultérieure. Les modèles 3D de l'auteur du projet (l'ogre, la voiture) sont sous CC BY-SA 4.0 ; ceux des autres gardent chacun leur licence, listée dans [CREDITS.md](../../CREDITS.md).
