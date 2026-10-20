# Obtenir openclare

<p class="subtitle">Le code source est sur GitHub, à <a href="https://github.com/marcgardent/openclare">marcgardent/openclare</a>. Sa documentation est en anglais.</p>

## L'extension Blender

C'est l'édition Community de l'extension : elle remaille un objet en un nombre de quads, avec les préréglages de la forme.

Une seule archive contient tout : l'extension et son moteur, compilé. Il n'y a rien d'autre à installer, ni paquet Python, ni programme à côté, et rien n'est téléchargé pendant qu'elle tourne.

1. Prenez l'archive de votre système dans les [versions publiées](https://github.com/marcgardent/openclare/releases) : `openclare-<version>-linux-x64.zip` ou `openclare-<version>-windows-x64.zip`. Ou construisez-la vous-même (plus bas).
2. Dans Blender 4.2 ou plus récent : *Edit › Preferences › Get Extensions*, la flèche en haut à droite, *Install from Disk*, et choisissez l'archive.

Un panneau nommé *openclare* apparaît dans l'éditeur Properties, sur l'onglet du maillage (Object Data). Sélectionnez un maillage, donnez un nombre de quads, choisissez un préréglage de la forme (organique, animation, surface dure), appuyez sur *Remesh to Quads*. Le résultat est un nouvel objet à côté de l'original, qui reste tel quel. Le moteur tourne en arrière-plan : Blender reste libre, et le panneau montre une barre de progression avec un bouton pour l'arrêter.

La version Linux tourne partout où Blender tourne (glibc 2.28 ou plus récente). La version Windows passe les tests du moteur mais n'a pas encore tourné dans un Blender pour Windows ; il n'y a pas encore de version pour macOS.

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
