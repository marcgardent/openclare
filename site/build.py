#!/usr/bin/env python3
"""Build the openclare website into site/_build, English and French.

Plain Python 3, standard library only; Pillow, where it is installed, turns
the PNG pictures into smaller WebP ones (without it they are copied as they
are). Every link in the output is relative, so the site works from any
folder: https://<user>.github.io/openclare/ as well as a local server.

Sources:
- site/content/<page>.md and <page>.fr.md: home, theory, results, research,
  get it (theory.md comes from the research note's branch; a placeholder
  stands in until it is there);
- docs/GLOSSARY.md and docs/GLOSSARY.fr.md;
- docs/models/<model>.md and docs/models/img/: one page per model;
- bench/results/SCORES.md and scores.json: the tables of the results page;
- docs/retopology/: the tools for retopology in Blender, their platforms and
  their tags, made into the retopology page;
- CREDITS.md: who made each model, and which licences forbid passing a model
  on. The pictures of those models are left out of the site.

Every page but the glossary links the first use of each term of the
glossary to its entry, the definition in the link's title.

    python3 site/build.py [--out DIR]
"""

import argparse
import hashlib
import html
import json
import os
import posixpath
import re
import shutil
import sys
import tomllib
import unicodedata
from pathlib import Path

SITE = Path(__file__).resolve().parent
ROOT = SITE.parent
sys.path.insert(0, str(SITE))
import md  # noqa: E402

CONTENT = SITE / "content"
REPOSITORY = "https://github.com/marcgardent/openclare"


def source_ref():
    """The branch the site is built from, so that links to the repository's
    files point at the same content as the pages: GitHub Actions names it,
    a local build asks git, and main is the fallback."""
    import os
    import subprocess

    name = os.environ.get("GITHUB_REF_NAME")
    if name:
        return name
    try:
        name = subprocess.run(
            ["git", "rev-parse", "--abbrev-ref", "HEAD"], capture_output=True, text=True, check=True
        ).stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return "main"
    return name if name and name != "HEAD" else "main"


REF = source_ref()

# The papers the site serves itself, and their names in it.
PAPERS = {
    "docs/research/closing-loops/paper.pdf": "files/closing-edge-loops.pdf",
    "docs/research/closing-loops/paper.fr.pdf": "files/closing-edge-loops.fr.pdf",
    "docs/research/turns-of-a-t-mesh/paper.pdf": "files/turns-of-a-t-mesh.pdf",
    "docs/research/turns-of-a-t-mesh/paper.fr.pdf": "files/turns-of-a-t-mesh.fr.pdf",
}
LANGUAGES = ("en", "fr")
IMAGE_SUFFIXES = {".png", ".jpg", ".jpeg", ".webp", ".svg", ".gif"}
# The widest a picture is kept, in pixels; wider ones are scaled down.
WIDEST = 1800

try:
    from PIL import Image, features

    WEBP = features.check("webp")
except ImportError:  # the site is built all the same, pictures as they are
    Image = None
    WEBP = False


# ---------------------------------------------------------------- words

WORDS = {
    "en": {
        "language_name": "English",
        "skip": "Skip to the content",
        "nav_label": "Main",
        "on_this_page": "On this page",
        "home": "Home",
        "theory": "Theory",
        "glossary": "Glossary",
        "results": "Results",
        "research": "Research",
        "get": "Get it",
        "market": "State of the market",
        "articles": "Articles",
        "ecosystem": "Blender ecosystem",
        "insight": "In depth",
        "retopology": "Retopology Tools 2026",
        "engines": "Retopology Engines 2026",
        "platforms": "Distribution platforms 2026",
        "github": "GitHub ↗",
        "models": "Model by model",
        "other_title": "Lire cette page en français",
        "language_label": "Language",
        "language_names": {"en": "English", "fr": "Français"},
        "repository_label": "Repository",
        "releases": "Releases",
        "issues": "Issues",
        "glossary_sections": "Sections of the glossary",
        "footer": (
            "<p><strong>openclare</strong>: automatic quad retopology. Code under the GNU GPL, version 3 or later; "
            "the pictures of each model under the licence of that model, listed in "
            '<a href="{credits}">CREDITS.md</a>.</p>'
            "<p>No cookies, no trackers. Scripts only for the 3D viewer, served from this site. "
            "Built from the repository by <code>site/build.py</code>"
            ' (<a href="{repository}">GitHub</a>).</p>'
        ),
        "sitemap": "Site map",
        "placeholder_theory": (
            "# Theory\n\nThis page, the theory of openclare in plain words, is being written. Meanwhile, the "
            "[glossary](../../docs/GLOSSARY.md) explains the words with pictures, and the "
            "[research note](research.md) gives the full story.\n"
        ),
    },
    "fr": {
        "language_name": "Français",
        "skip": "Aller au contenu",
        "nav_label": "Navigation principale",
        "on_this_page": "Sur cette page",
        "home": "Accueil",
        "theory": "Théorie",
        "glossary": "Glossaire",
        "results": "Résultats",
        "research": "Recherche",
        "get": "L'obtenir",
        "market": "État du marché",
        "articles": "Articles",
        "ecosystem": "Écosystème Blender",
        "insight": "En profondeur",
        "retopology": "Outils de retopologie 2026",
        "engines": "Moteurs de retopologie 2026",
        "platforms": "Plateformes de distribution 2026",
        "github": "GitHub ↗",
        "models": "Modèle par modèle",
        "other_title": "Read this page in English",
        "language_label": "Langue",
        "language_names": {"en": "English", "fr": "Français"},
        "repository_label": "Dépôt",
        "releases": "Versions",
        "issues": "Tickets",
        "glossary_sections": "Sections du glossaire",
        "footer": (
            "<p><strong>openclare</strong> : la retopologie automatique en quads. Code sous GNU GPL, version 3 ou "
            "ultérieure ; les images de chaque modèle sous la licence de ce modèle, listée dans "
            '<a href="{credits}">CREDITS.md</a>.</p>'
            "<p>Ni cookies, ni pisteurs. Des scripts seulement pour la visionneuse 3D, servis par ce site. "
            "Construit depuis le dépôt par <code>site/build.py</code>"
            ' (<a href="{repository}">GitHub</a>).</p>'
        ),
        "sitemap": "Plan du site",
        "placeholder_theory": (
            "# Théorie\n\nCette page, la théorie d'openclare en mots simples, est en cours d'écriture. En attendant, "
            "le [glossaire](../../docs/GLOSSARY.fr.md) explique les mots en images, et la "
            "[note de recherche](research.fr.md) raconte toute l'histoire.\n"
        ),
    },
}

# The logo leads home; the navigation lists the other pages.
# The menu: the home page is for the artist (the logo), the articles go
# deeper, the glossary gives the words. "Get it" is a button beside it.
NAVIGATION = ("articles", "glossary")
# A menu item with no page of its own leads to this one.
NAVIGATION_TARGETS = {}
# The articles, by category, as tiles on the articles page; each has a banner,
# site/assets/articles/<key>.svg (scripts/article_banners.py).
ARTICLES = (("ecosystem", ("retopology", "engines", "platforms")), ("insight", ("theory", "results", "research")))
CATEGORY_OF = {key: category for category, keys in ARTICLES for key in keys}
# The pages each item of the navigation stands for, beside its own.
# The site map of the footer: a column per group, the articles by category.
SITEMAP = (("openclare", ("index", "get", "articles", "glossary")),) + tuple(
    (category, ("market",) * (category == "ecosystem") + keys) for category, keys in ARTICLES
)

SECTIONS = {"articles": ("models/", "market", "retopology", "engines", "platforms", "theory", "results", "research")}
# The lists of the state of the market, in the order they are shown.
MARKET = ("retopology", "engines", "platforms")

# Display names of the remeshers of the bench, and what the rows that are
# not one of the remeshers compared are.
TOOLS = {
    "openclare": ("openclare", "openclare"),
    "quadriflow": ("QuadriFlow (Blender)", "QuadriFlow (Blender)"),
    "autoremesher": ("AutoRemesher", "AutoRemesher"),
    "openclare-uniform": ("openclare, quads of one size", "openclare, quads d'une seule taille"),
    "integer-grid": ("openclare, integer-grid chain", "openclare, chaîne integer-grid"),
    "instant-meshes": ("openclare, Instant Meshes chain", "openclare, chaîne Instant Meshes"),
    "handmade": ("Modelled by hand", "Modélisé à la main"),
    "openclare-as-handmade": ("openclare at the hand-made count", "openclare au nombre du fait main"),
}
COMPARED = ("openclare", "quadriflow", "autoremesher")
BUDGETS = (1000, 3000, 8000)
VARIANTS = ("openclare-uniform", "integer-grid", "instant-meshes", "handmade", "openclare-as-handmade")

GROUPS = {
    "organic": ("Organic models", "Modèles organiques"),
    "mechanical": ("Mechanical models", "Modèles mécaniques"),
    "calibration": ("Calibration models", "Modèles d'étalonnage"),
    "extra": ("Extra models", "Modèles extra"),
}
CRITERIA = {
    "Quads": ("Quads", "Quads"),
    "Overall": ("Overall", "Note globale"),
    "Validity": ("Validity", "Validité"),
    "Alignment": ("Alignment", "Alignement"),
    "Quad shape": ("Quad shape", "Forme des quads"),
    "Fidelity": ("Fidelity", "Fidélité"),
    "Regularity": ("Regularity", "Régularité"),
    "Structure": ("Structure", "Structure"),
    "Edge loops": ("Edge loops", "Boucles d'arêtes"),
    "Budget": ("Budget", "Budget"),
}

RESULT_WORDS = {
    "en": {
        "budget": "{n} quads asked for",
        "over": "Over {count} models: {names}.",
        "remesher": "Remesher",
        "compared": "The three remeshers compared",
        "variants": "Other settings and chains, for reference",
        "not_everywhere": "Not run on every model: {list}.",
        "of": "{name} ({done} of {total})",
        "legend_intro": "The rows set apart under each table:",
        "legend": (
            "<dl class=\"legend\">"
            "<dt>openclare, quads of one size</dt><dd>openclare from the command line, every quad the same size: like with like against the remeshers that have nothing else.</dd>"
            "<dt>openclare, integer-grid and Instant Meshes chains</dt><dd>earlier chains of openclare's own engine, kept as yardsticks. The Instant Meshes chain is the project's port of that method; the Instant Meshes program itself is not run.</dd>"
            "<dt>Modelled by hand</dt><dd>the base meshes an artist made for the two human bodies, the only models that have one; openclare is also run at their number of quads.</dd>"
            "</dl>"
        ),
        "model": "Model",
        "kind": "Kind",
        "models_intro": "Overall mark of each remesher on each model. A name links to the model's page, with pictures.",
        "withheld_mark": "no pictures (licence)",
        "withheld_intro": (
            "The licence of these models, BlenderKit Royalty Free, allows using them in a project but not passing "
            "them on. They are kept out of the repository, and this site shows no picture of them: only their "
            "marks, in the tables above."
        ),
        "best": "The best mark of each row is highlighted.",
        "asked_for": "Quads asked for",
        "models_in_group": "{n} models",
        "all_criteria": "Every criterion, with the other settings and chains",
        "loops_label": "{name}: {parts}",
    },
    "fr": {
        "budget": "{n} quads demandés",
        "over": "Sur {count} modèles : {names}.",
        "remesher": "Remailleur",
        "compared": "Les trois remailleurs comparés",
        "variants": "Autres réglages et chaînes, pour référence",
        "not_everywhere": "Pas lancé sur tous les modèles : {list}.",
        "of": "{name} ({done} sur {total})",
        "legend_intro": "Les lignes mises à part sous chaque tableau :",
        "legend": (
            "<dl class=\"legend\">"
            "<dt>openclare, quads d'une seule taille</dt><dd>openclare en ligne de commande, tous les quads de la même taille : à armes égales face aux remailleurs qui n'ont rien d'autre.</dd>"
            "<dt>openclare, chaînes integer-grid et Instant Meshes</dt><dd>des chaînes antérieures du moteur d'openclare lui-même, gardées comme étalons. La chaîne Instant Meshes est le portage de cette méthode par le projet ; le programme Instant Meshes lui-même n'est pas lancé.</dd>"
            "<dt>Modélisé à la main</dt><dd>les maillages de base qu'un artiste a faits pour les deux corps humains, les seuls modèles qui en ont un ; openclare est aussi lancé à leur nombre de quads.</dd>"
            "</dl>"
        ),
        "model": "Modèle",
        "kind": "Type",
        "models_intro": "La note globale de chaque remailleur sur chaque modèle. Un nom mène à la page du modèle, avec ses images.",
        "withheld_mark": "sans images (licence)",
        "withheld_intro": (
            "La licence de ces modèles, BlenderKit Royalty Free, permet de les utiliser dans un projet mais pas de "
            "les transmettre. Ils sont gardés hors du dépôt, et ce site n'en montre aucune image : seulement leurs "
            "notes, dans les tableaux ci-dessus."
        ),
        "best": "La meilleure note de chaque ligne est surlignée.",
        "asked_for": "Quads demandés",
        "models_in_group": "{n} modèles",
        "all_criteria": "Tous les critères, avec les autres réglages et chaînes",
        "loops_label": "{name} : {parts}",
    },
}

# The fixed sentences of the model pages (scripts/pages.py), in French. The
# regular expressions come after the plain phrases.
MODEL_PHRASES_FR = [
    ("At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside "
     "itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on "
     "each, those of borders and of faces that are not quads left out; \"—\" where the mesh is not a manifold.",
     "À 3000 quads demandés. Chaque boucle est suivie tout droit à travers chaque sommet régulier : elle se ferme, revient à "
     "côté d'elle-même un quad plus loin (une spirale), s'arrête sur un sommet irrégulier, ou va d'un bord à un bord. Part des "
     "arêtes sur chacune, celles des bords et des faces qui ne sont pas des quads laissées de côté ; « — » là où le maillage "
     "n'est pas une variété."),
    ("## At 3000 quads asked for", "## À 3000 quads demandés"),
    ("## Edge loops", "## Boucles d'arêtes"),
    ("## Marks", "## Notes"),
    ("| | closed | spiral | stops | border to border | spirals | longest that stops |",
     "| | fermées | en spirale | s'arrêtent | de bord à bord | spirales | plus longue qui s'arrête |"),
    ("| Modelled by hand |", "| Modélisé à la main |"),
    ("![Modelled by hand]", "![Modélisé à la main]"),
    ("| Remesher | Quads | Overall | Validity | Alignment | Quad shape | Fidelity | Regularity | Structure | Edge loops | Budget | Speed |",
     "| Remailleur | Quads | Note globale | Validité | Alignement | Forme des quads | Fidélité | Régularité | Structure | Boucles d'arêtes | Budget | Vitesse |"),
    ("Lines of the scene, as openclare reports them:", "Lignes de la scène, telles qu'openclare les rapporte :"),
    ("edges laid along the guide curves and the borders between materials", "arêtes posées le long des courbes guides et des frontières entre matériaux"),
    ("along the sharp curves", "le long des courbes vives"),
    ("faces left across a border", "faces laissées à cheval sur une frontière"),
    ("lines let go on", "lignes abandonnées sur"),
    (" pieces.", " pièces."),
]
MODEL_PATTERNS_FR = [
    (r"(?m)^### (\d+) quads$", r"### \1 quads"),
    (r"\| (\d[\d ]*) triangles \|", r"| \1 triangles |"),
    (r"(\d) quads\b", r"\1 quads"),
]
# The retopology page: the facets that filter its table, and the words of
# the page. The tags and their meanings are in docs/retopology/taxonomy.toml.
RETOPOLOGY = ROOT / "docs" / "retopology"
# The first switches are shown; the others fold under "More filters".
FILTERS = ("mode", "output", "os", "blender", "price")
MORE_FILTERS = ("scope", "guide", "engine", "use", "extra", "licence", "source", "install", "requires", "density",
                "edge-flow", "input", "bake")
TAGS_FR = {
    "mode": "Mode", "scope": "Portée", "output": "Sortie", "price": "Prix", "engine": "Moteur",
    "automatic": "automatique", "semi-automatic": "semi-automatique", "semi-manual": "semi-manuel",
    "manual": "manuel", "helper": "aide", "global": "globale", "local": "locale", "quads": "quads",
    "quad-dominant": "quads dominants", "triangles": "triangles", "n-gons": "n-gones", "template": "modèle",
    "free": "gratuit", "pay-what-you-want": "prix libre", "freemium": "freemium", "paid": "payant",
    "own": "le sien", "proprietary": "propriétaire",
    "parameters": "paramètres", "density": "densité", "marked-edges": "arêtes marquées", "strokes": "traits",
    "points": "points", "boundary": "contour", "selection": "sélection",
    "sculpt": "sculptures", "scan": "scans", "ai-generated": "maillages générés par IA", "cad": "CAO",
    "hard-surface": "surfaces dures", "organic": "organique", "head": "têtes", "cylinder": "membres et cylindres",
    "relief": "reliefs", "parts": "assemblage de pièces", "open-boundaries": "bords ouverts", "game-asset": "assets de jeu",
    "uv": "UV", "lod": "LOD", "repair": "réparation", "symmetry": "symétrie",
    "keeps-attributes": "garde UV, matériaux et poids", "cloud": "calcul sur serveur",
    "color": "couleur", "normal": "normales", "height": "hauteur", "ao": "occlusion ambiante",
    "material": "canaux de matériau",
    "bake": "Bake", "os": "Systèmes", "blender": "Blender", "install": "Installation", "requires": "Demande",
    "density": "Densité", "edge-flow": "Flux d'arêtes", "input": "Entrée", "guide": "Guide", "use": "Usage",
    "extra": "Aussi", "5-x": "5.x", "4-x": "4.2 à 4.5", "legacy": "avant 4.2", "unstated": "non indiquée",
    "built-in": "intégré", "get-extensions": "Get Extensions", "zip": "ZIP", "external": "programme à part",
    "external-program": "programme à part", "account": "compte", "internet": "connexion", "api-key": "clé d'API",
    "gpu": "GPU", "uniform": "uniforme", "adaptive": "adaptative", "feature-lines": "arêtes vives",
    "curvature": "courbure", "pole-control": "contrôle des pôles", "loops": "boucles imposées",
    "manifold": "manifold", "single-object": "un objet", "any": "tout maillage", "windows": "Windows",
    "linux": "Linux", "macos": "macOS", "ml": "apprentissage automatique",
    "licence": "Licence", "source": "Code source", "open-source": "open source", "not-stated": "non indiquée",
    "public": "public", "with-download": "dans le téléchargement", "closed": "fermé",
}
RETOPOLOGY_WORDS = {
    "en": {
        "all": "all", "old": "Hide stale and abandoned tools", "filters": "Filter by tags", "more_filters": "More filters", "reset": "Clear",
        "help": "Check any tags, in any facet: a tool is shown when it carries every checked tag.", "os": "OS",
        "tool": "Tool", "where": "Where", "blender": "Blender", "status": "Status",
        "platform": "Platform", "kind": "Kind", "sells": "Sells add-ons", "share": "What the seller keeps",
        "tools_here": "Tools", "yes": "yes", "no": "no", "method": "Method", "purpose": "For",
        "author": "Author", "licence": "Licence", "price": "Price", "stores": "Stores", "papers": "Papers",
        "notes": "Notes", "sources": "Sources read", "tags": "Tags", "checked": "checked",
        "licence_rule": "Licence rule", "install": "Install", "updates": "Updates", "source": "Source code",
        "count": "{shown} tools listed, {old} of them stale or abandoned.",
        "visit": "Visit", "tools_on": "Tools distributed here", "operator": "Operator", "paid_label": "Sells add-ons",
        "share_label": "What the seller keeps", "blender_label": "Blender", "market_tools": "{n} tools: automatic, semi-automatic, semi-manual, manual and helpers, with their tags.",
        "market_platforms": "{n} platforms: who runs them, what the seller keeps, how a user installs and updates.",
        "get_it": "Get it", "facts": "Facts", "same_engine": "Other tools on the same solver",
        "market_engines": "{n} engines: the solvers inside the tools, and the tools built on each.",
        "engine": "Engine", "authors": "Authors", "year": "Year", "own_program": "Its own program",
        "used_by": "Used by, in Blender", "elsewhere": "Used elsewhere", "engine_count": "{n} tools",
        "own_solver": "a solver of its own", "engine_kinds": {"paper": "paper", "library": "library", "program": "program"},
        "guide": "Guided by", "use": "Made for", "extra": "Also", "bake": "Bakes", "scope": "Acts", "release": "Latest release",
    },
    "fr": {
        "all": "tous", "old": "Cacher les outils anciens et abandonnés", "filters": "Filtrer par tags", "more_filters": "Plus de filtres", "reset": "Effacer",
        "help": "Cochez autant de tags que voulu, dans toutes les facettes : un outil est montré s'il porte tous les tags cochés.", "os": "OS",
        "tool": "Outil", "where": "Où", "blender": "Blender", "status": "État",
        "platform": "Plateforme", "kind": "Type", "sells": "Vend des greffons", "share": "Ce que garde le vendeur",
        "tools_here": "Outils", "yes": "oui", "no": "non", "method": "Méthode", "purpose": "Pour",
        "author": "Auteur", "licence": "Licence", "price": "Prix", "stores": "Boutiques", "papers": "Articles",
        "notes": "Notes", "sources": "Sources lues", "tags": "Tags", "checked": "vérifié le",
        "licence_rule": "Règle de licence", "install": "Installation", "updates": "Mises à jour", "source": "Code source",
        "count": "{shown} outils listés, dont {old} anciens ou abandonnés.",
        "visit": "Visiter", "tools_on": "Outils distribués ici", "operator": "Exploitant", "paid_label": "Vend des greffons",
        "share_label": "Ce que garde le vendeur", "blender_label": "Blender", "market_tools": "{n} outils : automatiques, semi-automatiques, semi-manuels, manuels et aides, avec leurs tags.",
        "market_platforms": "{n} plateformes : qui les tient, ce que garde le vendeur, comment on installe et met à jour.",
        "get_it": "L'obtenir", "facts": "Fiche", "same_engine": "Autres outils sur le même solveur",
        "market_engines": "{n} moteurs : les solveurs dans les outils, et les outils bâtis sur chacun.",
        "engine": "Moteur", "authors": "Auteurs", "year": "Année", "own_program": "Son propre programme",
        "used_by": "Utilisé par, dans Blender", "elsewhere": "Utilisé ailleurs", "engine_count": "{n} outils",
        "own_solver": "un solveur à lui", "engine_kinds": {"paper": "article", "library": "bibliothèque", "program": "programme"},
        "guide": "Guidé par", "use": "Fait pour", "extra": "Aussi", "bake": "Cuit", "scope": "Agit", "release": "Dernière version",
        "prose_en": "Les descriptions de cette fiche sont en anglais.",
    },
}
STATUS_FR = {"active": "actif", "stale": "ancien", "abandoned": "abandonné", "unknown": "inconnu"}

KINDS_FR = {"organic": "organique", "mechanical": "mécanique", "calibration": "d'étalonnage", "extra": "extra"}


# ---------------------------------------------------------------- glossary terms

# Every page but the glossary links the first time it uses a term of the
# glossary to that term's entry. The terms are the bold names of the entries,
# split at their commas ("Spiral, helix"), with their plurals.
#
# Words of the glossary too common to link wherever they stand: either
# everyday words the pages also use in their everyday sense, or words so
# general that a link on them would say nothing.
GENERIC_TERMS = {
    "en": {
        "mesh", "meshes", "quad", "quads", "edge", "edges", "vertex", "vertices", "face", "faces", "map", "maps",
        "field", "fields", "loop", "loops", "way", "ways", "around", "piece", "pieces", "border", "borders",
        "index", "indices", "cap", "caps", "lean", "wall", "walls", "node", "nodes", "trace", "traces",
        "station", "stations", "candidate", "candidates", "projection", "projections", "balance", "balances",
        "confidence", "aspect", "fold", "folds", "seam", "seams", "deviation", "deviations",
    },
    "fr": {
        "maillage", "maillages", "quad", "quads", "arête", "arêtes", "sommet", "sommets", "face", "faces",
        "champ", "champs", "boucle", "boucles", "variante", "variantes", "pièce", "pièces", "bord", "bords",
        "indice", "indices", "calotte", "calottes", "inclinaison", "mur", "murs", "nœud", "nœuds", "trace",
        "traces", "station", "stations", "candidat", "candidats", "projection", "projections", "équilibre",
        "équilibres", "confiance", "pli", "plis", "couture", "coutures", "écart", "écarts", "portée", "portées",
        "pas", "genre", "genres", "à grille entière",
    },
}
# Phrases that hold a term in another sense: they are read before the term
# inside them, and never linked.
NOT_TERMS = {
    "en": ["quarter turn", "quarter turns", "half turn", "in turn", "turn into", "turns into", "turn out", "turns out",
           "turned out", "can turn", "to turn", "will turn", "it turns", "flow turns", "cross turns", "turns by"],
    "fr": ["quart de tour", "quarts de tour", "demi-tour", "à son tour", "à leur tour", "tour à tour", "le tour",
           "un tour de", "un tour autour", "fait le tour", "faire le tour", "fait une fois le tour",
           "tour complet"],
}
# Forms the plain rule gets wrong or misses, by the English name of the entry.
TERM_FORMS = {
    "en": {},
    "fr": {
        "seamless-map": ["paramétrisation à grille entière", "paramétrisations à grille entière"],
        "t-mesh": ["T-meshes"],
    },
}
PLURALS = {
    "en": {"helix": "helices", "vertex": "vertices", "separatrix": "separatrices", "index": "indices"},
    "fr": {"anneau": "anneaux", "patch": "patchs", "t-mesh": "t-mesh", "nœud": "nœuds"},
}
FRENCH_LINKS = {"de", "du", "des", "en", "à", "au", "aux", "pour", "qui", "sans"}


def term_slug(text):
    """The anchor of a glossary entry: its first English name, plain ASCII,
    the same in both languages."""
    text = re.sub(r"[–—‑]", "-", text)
    text = unicodedata.normalize("NFKD", text).encode("ascii", "ignore").decode().lower()
    return re.sub(r"[^a-z0-9]+", "-", text).strip("-")


def plain_text(markdown):
    """Markdown of a sentence as plain text, for a title attribute."""
    text = re.sub(r"!\[[^\]]*\]\([^)]*\)", "", markdown)
    text = re.sub(r"\[([^\]]*)\]\([^)]*\)", r"\1", text)
    text = re.sub(r"[*`]", "", text)
    return " ".join(text.split())


def first_sentence(text):
    """The first sentence of a definition, and the second too when the first
    says too little alone ("One rectangle of the T-mesh.")."""
    sentences = re.split(r"(?<=[.!?])\s+(?=[A-ZÀ-ÖØ-Ý«]|openclare\b)", text)
    short = len(sentences[0]) < 40
    return " ".join(sentences[:2] if short else sentences[:1])


def word_plural(word, language):
    """The plural of one word, or the word itself where it already reads as
    one."""
    lower = word.lower()
    if lower in PLURALS[language]:
        return PLURALS[language][lower]
    if not re.fullmatch(r"[^\W\d_]+(?:[-–][^\W\d_]+)*", word):
        return word  # LLL, 3/5 and the like
    if language == "en":
        if lower.endswith(("ss", "sh", "ch", "x")):
            return word + "es"
        if lower.endswith("s"):
            return word
        if re.search(r"[^aeiou]y$", lower):
            return word[:-1] + "ies"
        return word + "s"
    if lower.endswith(("s", "x", "z")):
        return word
    if lower.endswith(("au", "eu")):
        return word + "x"
    return word + "s"


def word_singular(word, language):
    lower = word.lower()
    for singular, plural in PLURALS[language].items():
        if lower == plural and singular != plural:
            return singular
    if lower.endswith("ss") or not lower.endswith("s") or len(lower) < 4:
        return word
    return word[:-1]


def phrase_forms(phrase, language):
    """A name of the glossary, its plural and, for a plural name of several
    words, its singular. In English only the head noun changes ("ring of the
    field", "rings of the field"); in French the noun and its adjectives,
    up to the first preposition."""
    words = phrase.split(" ")
    if language == "en":
        if "for" in words:
            return [phrase]
        head = words.index("of") - 1 if "of" in words else len(words) - 1
        changed = [head]
    else:
        changed = []
        for position, word in enumerate(words):
            if word.lower() in FRENCH_LINKS or re.match(r"(?i)d['’]", word):
                break
            changed.append(position)
    forms = [phrase]
    for change in (word_plural, word_singular):
        if change is word_singular and len(words) == 1:
            continue
        other = list(words)
        for position in changed:
            other[position] = change(other[position], language)
        forms.append(" ".join(other))
    return list(dict.fromkeys(forms))


def glossary_entries(text, language):
    """The entries of a glossary: (anchor, name, names a page may use,
    first sentence of the definition as plain text), in order."""
    entries = []
    for paragraph in re.split(r"\n\s*\n", text):
        paragraph = " ".join(paragraph.split())
        match = re.match(r"\*\*(.+?)\*\*(?: \(\*(.+?)\*\))?\. (.+)", paragraph)
        if not match:
            continue
        name, english, definition = match.groups()
        if language == "en":
            english = name
        elif not english:
            raise SystemExit(f"glossary: the French entry {name!r} gives no English name in (*...*)")
        anchor = term_slug(english.split(", ")[0])
        forms = []
        for variant in name.split(", "):
            forms += phrase_forms(variant, language)
        forms += TERM_FORMS[language].get(anchor, [])
        generic = GENERIC_TERMS[language]
        forms = [form for form in dict.fromkeys(forms) if form.lower() not in generic]
        entries.append((anchor, name, forms, first_sentence(plain_text(definition))))
    return entries


def term_pattern(form):
    """A regular expression for a name as a page may write it: any space,
    either dash, either apostrophe."""
    parts = []
    for character in html.escape(form, quote=False):
        if character == " ":
            parts.append(r"(?:\s|&nbsp;| )+")
        elif character in "-–‑":
            parts.append("[-–‑]")
        elif character in "'’":
            parts.append("(?:'|’|&#x27;|&#39;)")
        else:
            parts.append(re.escape(character))
    return "".join(parts)


def term_key(text):
    text = html.unescape(text).replace(" ", " ").replace("’", "'")
    text = re.sub(r"[–‑]", "-", text)
    return " ".join(text.lower().split())


class Terms:
    """The glossary of one language, ready to link the pages."""

    # Text inside these is never linked: headings and the page's own links,
    # code, the headers of tables, the folded titles, the table of contents.
    SKIPPED = {"a", "code", "pre", "kbd", "h1", "h2", "h3", "h4", "h5", "h6", "th", "summary", "nav", "aside",
               "script", "style", "title"}

    def __init__(self, entries, language):
        self.language = language
        self.entries = entries
        self.anchor = {}  # name as matched -> anchor, or None for a phrase never linked
        self.definition = {}
        for anchor, name, forms, definition in entries:
            self.definition[anchor] = definition
            for form in forms:
                self.anchor.setdefault(term_key(form), anchor)
        for phrase in NOT_TERMS[language]:
            self.anchor[term_key(phrase)] = None
        names = sorted(self.anchor, key=len, reverse=True)
        self.pattern = re.compile(
            r"(?<![\w\-–‑/])(?:" + "|".join(term_pattern(name) for name in names) + r")(?![\w\-–‑/])", re.IGNORECASE
        )

    def link(self, body, glossary):
        """`body` with the first use of each term linked to `glossary`#its
        anchor; the anchors linked, in order."""
        linked = []
        parts = re.split(r"(<!--.*?-->|<[^>]+>)", body, flags=re.S)
        depth = 0
        for position, part in enumerate(parts):
            if part.startswith("<"):
                tag = re.match(r"<(/?)([a-zA-Z][a-zA-Z0-9]*)", part)
                if tag and tag.group(2).lower() in self.SKIPPED and not part.endswith("/>"):
                    depth += -1 if tag.group(1) else 1
                continue
            if depth > 0 or not part.strip():
                continue

            def replace(match):
                anchor = self.anchor.get(term_key(match.group(0)))
                if anchor is None or anchor in linked:
                    return match.group(0)
                linked.append(anchor)
                title = html.escape(self.definition[anchor])
                return f'<a class="term" href="{glossary}#{anchor}" title="{title}">{match.group(0)}</a>'

            parts[position] = self.pattern.sub(replace, part)
        return "".join(parts), linked


# ---------------------------------------------------------------- data

def read(path):
    return Path(path).read_text(encoding="utf-8")


def credits():
    """From CREDITS.md: a short credit (Markdown) for every model, and the
    models whose licence does not allow passing them on."""
    text = read(ROOT / "CREDITS.md")
    credit, withheld = {}, {}
    section = None
    for line in text.splitlines():
        if line.startswith("## "):
            section = line[3:].strip()
            continue
        if not line.startswith("| `"):
            continue
        cells = md.Converter.cells(line)
        names = re.findall(r"`([a-z_0-9]+)`", cells[0])
        if section == "Models by the author of the project":
            entry = {"who": f"{cells[1]}, {cells[2]}", "licence": cells[4], "source": None}
        elif section == "Models by others":
            entry = {"who": cells[1], "licence": cells[3], "source": cells[2]}
        elif section == "Shapes made by the project":
            entry = {"who": None, "made_by": cells[1], "licence": "GPL-3.0-or-later", "source": None}
        else:
            continue
        for name in names:
            credit[name] = entry
            # CREDITS.md, "Pictures": pictures of every model are allowed, the
            # BlenderKit ones included; only passing those models on is not.
            # A model whose pictures must not be shown would go here.
    return credit, withheld


def number(value, language):
    text = f"{value:,}"
    return text.replace(",", " ") if language == "fr" else text


def scores():
    """The tables of bench/results/SCORES.md: averages per budget and
    group, the group of every model, and the per-model marks of
    scores.json."""
    averages, kinds, over, notes = {}, {}, {}, {}
    budget = group = mode = None
    header = []
    for line in read(ROOT / "bench/results/SCORES.md").splitlines():
        heading = re.match(r"## (\d+) quads$", line)
        if heading:
            budget, mode = int(heading.group(1)), None
            continue
        heading = re.match(r"### (\w+) models: average$", line)
        if heading:
            group, mode = heading.group(1).lower(), "average"
            continue
        if line.startswith("### "):
            mode = None
            continue
        heading = re.match(r"#### (\w+) \((\w+)\)$", line)
        if heading:
            kinds.setdefault(heading.group(1), heading.group(2))
            continue
        if mode == "average" and line.startswith("Over "):
            over[(budget, group)] = [name.strip(" .") for name in line[5:].split(",")]
        if mode == "average" and line.startswith("Not run on every model:"):
            notes[(budget, group)] = re.findall(r"([\w-]+) \((\d+) of (\d+)\)", line)
        if not line.startswith("|") or line.startswith("|---"):
            continue
        cells = [cell.replace("**", "") for cell in md.Converter.cells(line)]
        if cells[0] in ("Remesher", "Model"):
            header = cells
            continue
        if mode == "average":
            averages.setdefault((budget, group), {})[cells[0]] = dict(zip(header[1:], cells[1:]))
    per_model = json.loads(read(ROOT / "bench/results/scores.json"))
    return {"averages": averages, "kinds": kinds, "over": over, "notes": notes, "models": per_model}


# ---------------------------------------------------------------- output

class Site:
    def __init__(self, out):
        self.out = Path(out)
        self.assets = {}  # repository path -> output path
        self.credit, self.withheld = credits()
        self.data = scores()
        self.models = sorted(
            (path.stem for path in (ROOT / "docs/models").glob("*.md") if path.stem != "README"),
            key=self.model_order,
        )
        self.pages = []  # (key, language, output path)
        # A digest of the style and the viewer, so that a browser does not keep an old one.
        self.version = hashlib.sha256(
            b"".join((SITE / "assets" / name).read_bytes() for name in ("style.css", "viewer.js"))
        ).hexdigest()[:10]
        self.warnings = []
        self.terms = {}
        for language in LANGUAGES:
            entries = glossary_entries(read(ROOT / self.glossary_source(language)), language)
            self.terms[language] = Terms(entries, language)
        english = [entry[0] for entry in self.terms["en"].entries]
        french = [entry[0] for entry in self.terms["fr"].entries]
        if len(set(english)) != len(english):
            raise SystemExit("glossary: two English entries share an anchor")
        if english != french:
            self.warnings.append(
                "the English and French glossaries do not have the same entries: "
                + ", ".join(sorted(set(english) ^ set(french)) or ["(same entries, other order)"])
            )
        self.linked = {}  # output path -> anchors of the glossary linked on it
        self.meta = {}  # (key, language) -> title and description, for the articles page

    def model_order(self, name):
        order = list(GROUPS)
        kind = self.data["kinds"].get(name, "extra")
        listed = list(self.data["kinds"])
        return (order.index(kind) if kind in order else len(order), listed.index(name) if name in listed else 99, name)

    # ---- where things go

    @staticmethod
    def page_path(key, language):
        name = "index.html" if key == "index" else f"{key}.html"
        return name if language == "en" else f"fr/{name}"

    def page_of_source(self, repository_path, language):
        """The page a Markdown file of the repository becomes, or None."""
        path = repository_path
        match = re.fullmatch(r"site/content/(\w+?)(\.fr)?\.md", path)
        if match:
            return match.group(1), "fr" if match.group(2) else "en"
        match = re.fullmatch(r"docs/GLOSSARY(\.fr)?\.md", path)
        if match:
            return "glossary", "fr" if match.group(1) else "en"
        match = re.fullmatch(r"docs/models/(\w+)\.md", path)
        if match:
            name = match.group(1)
            if name == "README" or name in self.withheld:
                return "results", language
            return f"models/{name}", language
        return None

    def asset(self, repository_path):
        """Copy (or convert) a picture or file of the repository into the
        site, once; its path in the site."""
        if repository_path in self.assets:
            return self.assets[repository_path]
        source = ROOT / repository_path
        suffix = source.suffix.lower()
        if repository_path.startswith("docs/models/img/"):
            target = "img/models/" + source.name
        elif repository_path.startswith("docs/glossary/"):
            target = "img/glossary/" + source.name
        elif repository_path.startswith("docs/images/"):
            target = "img/" + repository_path[len("docs/images/"):]
        elif repository_path.startswith("docs/research/") and suffix in IMAGE_SUFFIXES:
            target = "img/research/" + source.name
        elif repository_path in PAPERS:
            target = PAPERS[repository_path]
        else:
            target = "files/" + repository_path.replace("/", "-")
        convert = WEBP and suffix in (".png", ".jpg", ".jpeg")
        if convert:
            target = str(Path(target).with_suffix(".webp"))
        destination = self.out / target
        destination.parent.mkdir(parents=True, exist_ok=True)
        if convert:
            with Image.open(source) as picture:
                if picture.width > WIDEST:
                    height = round(picture.height * WIDEST / picture.width)
                    picture = picture.resize((WIDEST, height), Image.LANCZOS)
                if picture.mode not in ("RGB", "RGBA"):
                    picture = picture.convert("RGBA")
                picture.save(destination, "WEBP", quality=90, method=6)
        else:
            shutil.copyfile(source, destination)
        self.assets[repository_path] = target
        return target

    def is_withheld(self, repository_path):
        name = Path(repository_path).name
        return any(name.startswith(model + "__") for model in self.withheld)

    def resolver(self, source, page, language):
        """The function md.render calls for each link and picture of `source`
        (a path of the repository), written into the page `page`."""
        here = posixpath.dirname(page)

        def relative(target):
            return posixpath.relpath(target, here or ".")

        def resolve(target, image):
            if re.match(r"^[a-z]+:", target) or target.startswith("#"):
                return target
            target, _, fragment = target.partition("#")
            fragment = "#" + fragment if fragment else ""
            path = posixpath.normpath(posixpath.join(posixpath.dirname(source), target))
            if path.startswith("../") or not (ROOT / path).exists():
                self.warnings.append(f"{source}: link to {target} that is not in the repository")
                return target + fragment
            suffix = posixpath.splitext(path)[1].lower()
            if image or suffix in IMAGE_SUFFIXES:
                if self.is_withheld(path):
                    return None if image else relative(self.page_path("results", language)) + "#withheld"
                return relative(self.asset(path)) + fragment
            if path in PAPERS:
                return relative(self.asset(path))
            page_of = self.page_of_source(path, language)
            if page_of:
                return relative(self.page_path(*page_of)) + fragment
            kind = "tree" if (ROOT / path).is_dir() else "blob"
            return f"{REPOSITORY}/{kind}/{REF}/{path}{fragment}"

        return resolve

    # ---- pages

    def render(self, text, source, page, language):
        resolve = self.resolver(source, page, language)

        def attributes(match):
            name, value = match.group(1), match.group(2)
            resolved = resolve(value, name != "href")
            if resolved is None:
                resolved = ""
            return f'{name}="{html.escape(resolved)}"'

        # The raw HTML of a source has its links and pictures rewritten as
        # well (Markdown itself never writes these attributes).
        text = re.sub(r'\b(src|href|srcset)="([^"]+)"', attributes, text)
        return md.render(text, resolve)

    def write(self, key, language, title, body, headings, kind, description="", breadcrumb="", toc=True, extra="", scripts=""):
        """Write one page. Its body loses its title (and subtitle), which go
        into the band at the top of the page with `breadcrumb` or a kicker,
        and `extra` (a switch, the sections of the glossary). The home page
        is cut into bands instead, one per section."""
        page = self.page_path(key, language)
        root = posixpath.relpath(".", posixpath.dirname(page) or ".")
        root = "" if root == "." else root + "/"
        here = posixpath.dirname(page) or "."
        words = WORDS[language]
        other = "fr" if language == "en" else "en"
        nav = []
        for item in NAVIGATION:
            target = posixpath.relpath(self.page_path(NAVIGATION_TARGETS.get(item, item), language), here)
            current = ' aria-current="page"' if item == key or key.split("/")[0] in SECTIONS.get(item, ()) or any(key.startswith(p) for p in SECTIONS.get(item, ()) if p.endswith("/")) else ""
            nav.append(f'<li><a href="{target}"{current}>{words[item]}</a></li>')
        nav.append(f'<li><a href="{REPOSITORY}" rel="noopener">{words["github"]}</a></li>')
        get = posixpath.relpath(self.page_path("get", language), here)
        current = ' aria-current="page"' if key == "get" else ""
        nav.append(f'<li class="nav-get"><a class="button primary" href="{get}"{current}>{words["get"]}</a></li>')
        other_href = posixpath.relpath(self.page_path(key, other), here)
        cells = []
        for code in LANGUAGES:
            name = words["language_names"][code]
            if code == language:
                cells.append(f'<span class="current" lang="{code}"><span aria-hidden="true">{code.upper()}</span><span class="visually-hidden">{name}</span></span>')
            else:
                cells.append(
                    f'<a href="{other_href}" hreflang="{code}" lang="{code}" title="{words["other_title"]}">'
                    f'<span aria-hidden="true">{code.upper()}</span><span class="visually-hidden">{name}</span></a>'
                )
        language_switch = f'<nav class="language" aria-label="{words["language_label"]}">{"".join(cells)}</nav>'
        body = number_headings(body)
        body = shell_prompts(body)
        if kind != "home":
            body = night_sections(body)
        sections = [h for h in headings if h[0] == 2]
        toc_html = ""
        if toc and len(sections) >= 3:
            items = "".join(f'<li><a href="#{identifier}">{heading_label(text)}</a></li>' for _, text, identifier in sections)
            # Beside the text on a wide screen; folded above it on a narrow one.
            toc_html = f'<aside class="toc toc-side"><p>{words["on_this_page"]}</p><ol>{items}</ol></aside>'
            body = f'<details class="toc toc-inline"><summary>{words["on_this_page"]}</summary><ol>{items}</ol></details>\n' + body
        if kind != "glossary":
            glossary = posixpath.relpath(self.page_path("glossary", language), here)
            body, self.linked[page] = self.terms[language].link(body, glossary)
        if language == "fr":
            body = french_spaces(body)
        body, viewer = self.viewers(body, here, language)
        if viewer and not scripts:
            scripts = VIEWER_SCRIPT
        if key in CATEGORY_OF and not breadcrumb:
            articles = posixpath.relpath(self.page_path("articles", language), here)
            breadcrumb = (f'<nav class="breadcrumb" aria-label="breadcrumb"><a href="{articles}">{words["articles"]}</a> › '
                          f'{words[CATEGORY_OF[key]]}</nav>')
        self.meta[(key, language)] = (title, description)
        if kind == "home":
            intro, body = home_bands(self.mark_strips(body, here))
        else:
            kicker = breadcrumb or f'<p class="kicker">{words.get(kind if kind in words else key, "")}</p>'
            intro, body = page_intro(body, kicker, extra, self.banner(key))
        values = {
            "lang": language,
            "title": html.escape(title if key == "index" else f"{title} · openclare"),
            "description": html.escape(description or title),
            "root": root,
            "home": posixpath.relpath(self.page_path("index", language), here),
            "other_lang": other,
            "other_href": other_href,
            "language": language_switch,
            "skip": words["skip"],
            "nav_label": words["nav_label"],
            "nav": "<ul>" + "".join(nav) + "</ul>",
            "kind": kind,
            "intro": intro,
            "layout_class": " full" if kind == "home" else " with-toc" if toc_html else "",
            "toc": toc_html,
            "body": body,
            "footer": words["footer"].format(
                credits=f"{REPOSITORY}/blob/{REF}/CREDITS.md", repository=REPOSITORY
            ),
            "sitemap_label": words["sitemap"],
            "sitemap": self.sitemap(language, here),
            "version": self.version,
            "scripts": scripts.format(root=root, version=self.version),
        }
        template = read(SITE / "template.html")
        output = re.sub(r"\{(\w+)\}", lambda m: values[m.group(1)], template)
        destination = self.out / page
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text(output, encoding="utf-8")
        self.pages.append((key, language, page))

    def sitemap(self, language, here):
        """The site map of the footer, with the links of the repository last."""
        words = WORDS[language]
        columns = []
        for group, keys in SITEMAP:
            title = group if group == "openclare" else words[group]
            links = "".join(
                f'<li><a href="{posixpath.relpath(self.page_path(k, language), here)}">{words["home" if k == "index" else k]}</a></li>' for k in keys
            )
            columns.append(f"<div><p>{title}</p><ul>{links}</ul></div>")
        links = "".join(
            f'<li><a href="{REPOSITORY}{path}">{label}</a></li>'
            for path, label in (("", "GitHub"), ("/releases", words["releases"]), ("/issues", words["issues"]))
        )
        columns.append(f'<div><p>{words["repository_label"]}</p><ul>{links}</ul></div>')
        return "".join(columns)

    @staticmethod
    def banner(key):
        """The SVG banner of an article, inline so that it takes the theme's colours."""
        path = SITE / "assets/articles" / f"{key}.svg"
        return read(path).strip() if key in CATEGORY_OF and path.exists() else ""

    def articles(self):
        """The articles page: a tile per article, with its banner, by category.
        Written last, from the titles and descriptions of the pages."""
        for language in LANGUAGES:
            text, body, headings = self.content_page("articles", language, "articles")
            here = posixpath.dirname(self.page_path("articles", language)) or "."
            words = WORDS[language]
            parts = []
            for category, keys in ARTICLES:
                cards = []
                for key in keys:
                    title, description = self.meta[(key, language)]
                    href = posixpath.relpath(self.page_path(key, language), here)
                    cards.append(
                        f'<a class="article-card" href="{href}">{self.banner(key)}'
                        f'<span class="article-text"><span class="kicker">{words[category]}</span>'
                        f'<strong>{html.escape(title)}</strong><span>{html.escape(description)}</span></span></a>'
                    )
                parts.append(f'<h2 id="{category}">{words[category]}</h2>\n<div class="article-cards">{"".join(cards)}</div>')
            body = body.replace("<p>{{articles}}</p>", "\n".join(parts))
            headings = headings + [(2, words[c], c) for c, _ in ARTICLES]
            self.write("articles", language, self.title_of(text), body, headings, "articles", self.description_of(text), toc=False)

    def viewers(self, body, here, language):
        """A <div class="viewer" data-models="…"> of a page's source: the 3D
        viewer with a choice of models ("all": every model with meshes), given
        the folder of the meshes and the language. Returns the body and
        whether it holds a viewer."""
        found = 'class="viewer"' in body

        def viewer(match):
            names = match.group(1).split()
            meshed = [p.parent.name for p in (ROOT / "docs/models/mesh").glob("*/index.json")]
            names = sorted(meshed, key=self.model_order) if names == ["all"] else [n for n in names if n in meshed]
            if not names:
                return ""
            base = posixpath.relpath("mesh", here) + "/"
            # The ogre first where it is there: the mascot, and every remesher's mesh.
            first = "ogre" if "ogre" in names else names[0]
            return (f'<div class="viewer" data-models="{" ".join(names)}" data-first="{first}" data-base="{base}" '
                    f'lang="{language}"></div>')

        return re.sub(r'<div class="viewer" data-models="([^"]*)"></div>', viewer, body), found

    def mark_strips(self, body, here):
        """A figure whose picture is a long strip (at least twice as wide as
        high) gets the class "strip": it keeps the whole width of a band."""

        def figure(match):
            if Image is None:
                return match.group(0)
            path = self.out / posixpath.normpath(posixpath.join(here, html.unescape(match.group(1))))
            try:
                with Image.open(path) as picture:
                    wide = picture.width >= 2 * picture.height
            except OSError:
                return match.group(0)
            return match.group(0).replace("<figure>", '<figure class="strip">', 1) if wide else match.group(0)

        return re.sub(r'<figure><img src="([^"]+)"', figure, body)

    @staticmethod
    def title_of(text):
        match = re.search(r"(?m)^# (.+)$", text)
        return re.sub(r"[*_`]", "", match.group(1)).strip() if match else "openclare"

    @staticmethod
    def drop_language_link(text):
        """The sources point to their other language at the top; the site has
        its own switch."""
        return re.sub(r"(?m)^\*\[(?:Version française|English version)\]\([^)]*\)\*\n\n?", "", text)

    @staticmethod
    def description_of(text):
        for paragraph in re.split(r"\n\s*\n", text):
            paragraph = paragraph.strip()
            if not paragraph or paragraph.startswith(("#", "<", "!", "|", "*[", "-", "`")):
                if paragraph.startswith('<p class="subtitle">') or paragraph.startswith('<p class="lede">'):
                    return re.sub(r"<[^>]+>", "", paragraph)
                if paragraph.startswith('<div class="hero">'):
                    match = re.search(r'<p class="lede">(.*?)</p>', paragraph)
                    if match:
                        return match.group(1)
                continue
            plain = re.sub(r"!\[[^\]]*\]\([^)]*\)", "", paragraph)
            plain = re.sub(r"\[([^\]]*)\]\([^)]*\)", r"\1", plain)
            plain = re.sub(r"[*_`]", "", plain)
            plain = " ".join(plain.split())
            return plain[:220].rsplit(" ", 1)[0] + "…" if len(plain) > 220 else plain
        return ""

    def tokens(self, text, language):
        """{{mark:3000:organic:openclare}}, {{count:organic}}, {{count:all}},
        {{theorems}} and {{shot:human_male:openclare}} (the caption of a
        picture of a model page) in a source."""

        def value(match):
            parts = match.group(1).split(":")
            if parts[0] == "mark":
                return self.data["averages"][(int(parts[1]), parts[2])][parts[3]]["Overall"]
            if parts[0] == "count":
                if parts[1] == "all":
                    return str(len(self.data["models"]["3000"]))
                return str(len(self.data["over"][(3000, parts[1])]))
            if parts[0] == "theorems":
                return str(self.theorems())
            if parts[0] == "shot":
                return self.shot_caption(parts[1], parts[2], language)
            return match.group(0)

        return re.sub(r"\{\{((?:mark|count|shot):[\w:-]+|theorems)\}\}", value, text)

    def theorems(self):
        """The `theorem` declarations of the Lean proofs, outside comments,
        as the proofs' README counts them; a warning when the README says
        another number."""
        folder = ROOT / "docs/research/turns-of-a-t-mesh/lean"
        total = 0
        for path in sorted(folder.rglob("*.lean")):
            if ".lake" in path.parts:
                continue
            text = re.sub(r"--.*", "", strip_block_comments(read(path)))
            declaration = r"(?m)^\s*(?:@\[[^\]]*\]\s*)?(?:(?:private|protected|noncomputable|nonrec)\s+)*theorem\b"
            total += len(re.findall(declaration, text))
        stated = re.search(r"(\d+) `theorem` declarations", read(folder / "README.md"))
        if stated and int(stated.group(1)) != total:
            self.warnings.append(f"the Lean README says {stated.group(1)} theorems, the sources have {total}")
        return total

    def shot_caption(self, name, column, language):
        """The caption under one picture of a model page ("2 979 quads"),
        its number written for `language`."""
        lines = read(ROOT / f"docs/models/{name}.md").splitlines()
        for position, line in enumerate(lines):
            if line.startswith("| Source |"):
                header = md.Converter.cells(line)
                caption = md.Converter.cells(lines[position + 3])[header.index(column)]
                return re.sub(r"\d[\d ]*\d|\d", lambda m: number(int(m.group(0).replace(" ", "")), language), caption)
        raise SystemExit(f"docs/models/{name}.md: no pictures")

    def content_page(self, key, language, kind, fallback=None):
        name = key if language == "en" else f"{key}.fr"
        path = CONTENT / f"{name}.md"
        if path.exists():
            text = read(path)
        else:
            text = fallback
            self.warnings.append(f"site/content/{name}.md is missing: a placeholder stands in")
        text = self.tokens(self.drop_language_link(text), language)
        source = f"site/content/{name}.md"
        page = self.page_path(key, language)
        body, headings = self.render(text, source, page, language)
        return text, body, headings

    def simple(self, key, kind, fallback=None):
        for language in LANGUAGES:
            text, body, headings = self.content_page(key, language, kind, fallback and fallback[language])
            if kind == "theory":
                body = theory_marks(body)
            self.write(key, language, self.title_of(text), body, headings, kind, self.description_of(text), toc=kind == "theory")

    @staticmethod
    def glossary_source(language):
        return "docs/GLOSSARY.md" if language == "en" else "docs/GLOSSARY.fr.md"

    def glossary(self):
        for language in LANGUAGES:
            source = self.glossary_source(language)
            text = self.drop_language_link(read(ROOT / source))
            body, headings = self.render(text, source, self.page_path("glossary", language), language)
            body = self.entry_anchors(body, language, {identifier for _, _, identifier in headings})
            chips = "".join(
                f'<a href="#{identifier}">{heading_label(text)}</a>' for level, text, identifier in headings if level == 2
            )
            extra = f'<nav class="chips" aria-label="{WORDS[language]["glossary_sections"]}">{chips}</nav>'
            self.write(
                "glossary", language, self.title_of(text), body, headings, "glossary", self.description_of(text), toc=False, extra=extra
            )

    def entry_anchors(self, body, language, taken):
        """Every entry of the glossary page gets the anchor the other pages
        link to: the same in English and in French."""
        entries = iter(self.terms[language].entries)
        missing = []

        def anchor(match):
            name = html.unescape(re.sub(r"<[^>]+>", "", match.group(1)))
            for entry in entries:
                if entry[1] == name:
                    if entry[0] in taken:
                        raise SystemExit(f"glossary: the anchor {entry[0]} of {name!r} is also a heading's")
                    return f'<p id="{entry[0]}" class="entry"><strong>{match.group(1)}</strong>'
                missing.append(entry[1])
            return match.group(0)

        body = re.sub(r"<p><strong>(.+?)</strong>", anchor, body)
        missing += [entry[1] for entry in entries]
        if missing:
            raise SystemExit(f"glossary: no paragraph found for the entries {', '.join(missing)}")
        return body

    # ---- results

    def marks_table(self, rows, columns, language, names=None, extra_class=""):
        """A table of marks: `rows` is a list of (label, {column: value},
        css class)."""
        index = 0 if language == "en" else 1
        head = "".join(
            ('<th class="overall-head">' if c == "Overall" else "<th>") + html.escape(CRITERIA.get(c, (c, c))[index]) + "</th>"
            for c in columns
        )
        body = []
        for label, cells, css in rows:
            if css == "divider":
                body.append(f'<tr class="divider"><td colspan="{len(columns) + 1}">{label}</td></tr>')
                continue
            tds = []
            for column in columns:
                value = cells.get(column, "—")
                classes = []
                if value in ("—", "", None):
                    classes.append("none")
                    value = "—"
                elif column == "Overall":
                    classes.append("overall")
                    value = score_cell(value, css == "ours")
                if column == "Quads" and value != "—":
                    value = number(int(value), language)
                attribute = f' class="{" ".join(classes)}"' if classes else ""
                tds.append(f"<td{attribute}>{value}</td>")
            body.append(f'<tr class="{css}"><td>{label}</td>{"".join(tds)}</tr>')
        first = RESULT_WORDS[language]["remesher"] if names is None else names
        return (
            f'<div class="table-wrap"><table class="marks{extra_class}"><thead><tr><th>{first}</th>{head}</tr></thead>'
            f'<tbody>{"".join(body)}</tbody></table></div>'
        )

    @staticmethod
    def chart_legend(language):
        index = 0 if language == "en" else 1
        names = {"openclare": "openclare", "quadriflow": "QuadriFlow", "autoremesher": "AutoRemesher"}
        items = "".join(f'<span><i class="swatch tool-{tool}"></i>{names[tool]}</span>' for tool in COMPARED)
        return f'<p class="chart-legend" aria-hidden="true">{items}</p>'

    def averages_html(self, language):
        """For each budget, a panel: the overall mark of the three
        remeshers on each group of models, as bars, then every criterion
        and the other settings in tables, folded."""
        words = RESULT_WORDS[language]
        index = 0 if language == "en" else 1
        columns = ["Quads", "Overall", "Validity", "Alignment", "Quad shape", "Fidelity", "Regularity", "Structure", "Edge loops", "Budget"]
        short = {"openclare": "openclare", "quadriflow": "QuadriFlow", "autoremesher": "AutoRemesher"}
        parts = [self.chart_legend(language)]
        for budget in BUDGETS:
            inner, groups = [], []
            for group in GROUPS:
                table = self.data["averages"].get((budget, group))
                if not table:
                    continue
                rows = []
                compared = sorted((t for t in COMPARED if t in table), key=lambda t: -int(table[t]["Overall"]))
                bars = []
                for tool in compared:
                    rows.append((TOOLS[tool][index], table[tool], "ours" if tool == "openclare" else ""))
                    mark = int(table[tool]["Overall"])
                    ours = " ours" if tool == "openclare" else ""
                    bars.append(
                        f'<div class="bar-row{ours}"><span class="tool">{short[tool]}</span>'
                        f'<span class="track" aria-hidden="true"><span class="fill tool-{tool}" style="width: {mark}%"></span></span>'
                        f"<b>{mark}</b></div>"
                    )
                variants = [t for t in VARIANTS if t in table]
                if variants:
                    rows.append((words["variants"], {}, "divider"))
                    for tool in sorted(variants, key=lambda t: -int(table[t]["Overall"])):
                        rows.append((TOOLS[tool][index], table[tool], "variant"))
                names = self.data["over"].get((budget, group), [])
                count = words["models_in_group"].format(n=len(names))
                groups.append(
                    f'<div class="group"><p class="group-name"><b>{GROUPS[group][index]}</b><span>{count}</span></p>{"".join(bars)}</div>'
                )
                shown = ", ".join(names)
                inner.append(f"<h4>{GROUPS[group][index]}</h4>")
                inner.append(f'<p class="note">{words["over"].format(count=len(names), names=shown)}</p>')
                inner.append(self.marks_table(rows, columns, language))
                missing = self.data["notes"].get((budget, group))
                if missing:
                    listed = ", ".join(
                        words["of"].format(name=TOOLS.get(n, (n, n))[index], done=d, total=t) for n, d, t in missing
                    )
                    inner.append(f'<p class="note">{words["not_everywhere"].format(list=listed)}</p>')
            title = words["budget"].format(n=number(budget, language))
            parts.append(
                f'<div class="budget-panel" data-budget="{budget}"><h3>{title}</h3>'
                f'<div class="groups">{"".join(groups)}</div>'
                f'<details class="more"><summary>{words["all_criteria"]}</summary><div class="inner">{"".join(inner)}</div></details></div>'
            )
        parts.append(f'<p class="note">{words["legend_intro"]}</p>' + words["legend"])
        return "\n".join(parts)

    def models_html(self, language, page):
        words = RESULT_WORDS[language]
        index = 0 if language == "en" else 1
        here = posixpath.dirname(page) or "."
        parts = [f'<p>{words["models_intro"]} {words["best"]}</p>']
        for budget in BUDGETS:
            marks = self.data["models"].get(str(budget), {})
            rows = []
            for name in self.models:
                tools = marks.get(name, {})
                values = {t: round(tools[t]["overall"]) for t in COMPARED if t in tools and tools[t].get("overall") is not None}
                best = max(values.values()) if values else None
                kind = self.data["kinds"].get(name, "")
                kind_label = kind if language == "en" else KINDS_FR.get(kind, kind)
                if name in self.withheld:
                    label = f'{name} <span class="note">({words["withheld_mark"]})</span>'
                else:
                    target = posixpath.relpath(self.page_path(f"models/{name}", language), here)
                    label = f'<a href="{target}">{name}</a>'
                cells = []
                for tool in COMPARED:
                    if tool in values:
                        css = "best" if values[tool] == best else ""
                        content = values[tool]
                        if tool == "openclare":
                            css += " overall ours"
                            content = score_cell(values[tool], True)
                        cells.append(f'<td class="{css.strip()}">{content}</td>')
                    else:
                        cells.append('<td class="none">—</td>')
                rows.append(f"<tr><td>{label}</td><td>{kind_label}</td>{''.join(cells)}</tr>")
            head = f'<th>{words["model"]}</th><th>{words["kind"]}</th>' + "".join(f"<th>{TOOLS[t][index]}</th>" for t in COMPARED)
            table = (
                f'<div class="table-wrap"><table class="marks models"><thead><tr>{head}</tr></thead>'
                f'<tbody>{"".join(rows)}</tbody></table></div>'
            )
            title = words["budget"].format(n=number(budget, language))
            parts.append(f'<div class="budget-panel" data-budget="{budget}"><h3>{title}</h3>{table}</div>')
        return "\n".join(parts)

    def withheld_html(self, language, page):
        words = RESULT_WORDS[language]
        resolve = self.resolver("CREDITS.md", page, language)
        converter = md.Converter(resolve)
        if not self.withheld:
            return ""
        items = []
        for name in sorted(self.withheld):
            entry = self.withheld[name]
            items.append(f"<li><strong>{name}</strong>: {converter.inline(entry['who'])}</li>")
        return f'<p id="withheld">{words["withheld_intro"]}</p><ul>{"".join(items)}</ul>'

    def results(self):
        for language in LANGUAGES:
            text, body, headings = self.content_page("results", language, "results")
            page = self.page_path("results", language)
            for token, make in (
                ("averages", lambda: self.averages_html(language)),
                ("models", lambda: self.models_html(language, page)),
                ("withheld", lambda: self.withheld_html(language, page)),
            ):
                body = body.replace(f"<p>{{{{{token}}}}}</p>", make())
            self.write(
                "results", language, self.title_of(text), body, headings, "results", self.description_of(text), toc=False,
                extra=budget_switch(language),
            )

    # ---- retopology

    def retopology_data(self):
        tools = [tomllib.loads(read(path)) for path in sorted((RETOPOLOGY / "tools").glob("*.toml"))]
        platforms = {p["id"]: p for p in tomllib.loads(read(RETOPOLOGY / "platforms.toml"))["platform"]}
        taxonomy = tomllib.loads(read(RETOPOLOGY / "taxonomy.toml"))["facet"]
        engines = {e["id"]: e for e in tomllib.loads(read(RETOPOLOGY / "engines.toml"))["engine"]}
        return sorted(tools, key=lambda e: e["name"].lower()), platforms, taxonomy, engines

    @staticmethod
    def tag_values(entry, facet):
        return [tag.split(":", 1)[1] for tag in entry["tags"] if tag.startswith(facet + ":")]

    @staticmethod
    def tag_label(value, language):
        english = {"5-x": "5.x", "4-x": "4.2–4.5", "legacy": "before 4.2"}
        return TAGS_FR.get(value, value) if language == "fr" else english.get(value, value)

    def retopology_filters(self, taxonomy, language):
        """One box per tag, as many checked as wanted: a tool is shown when it
        carries every checked tag. CSS on :has() hides the others; without
        :has() the boxes stay hidden and every tool is shown. A reset button
        clears the form without a script."""
        words = RETOPOLOGY_WORDS[language]
        groups, rules, more, checked = [], [], [], []
        for facet in FILTERS + MORE_FILTERS:
            values = [v for v in taxonomy[facet] if v not in ("required", "title")]
            title = TAGS_FR.get(facet, facet) if language == "fr" else taxonomy[facet]["title"].replace(" (computed)", "")
            cells = []
            for value in values:
                cells.append(f'<input type="checkbox" id="f-{facet}-{value}">'
                             f'<label for="f-{facet}-{value}">{self.tag_label(value, language)}</label>')
                rules.append(f'body:has(#f-{facet}-{value}:checked) .tool:not([data-{facet}~="{value}"])')
                checked.append(f'body:has(#f-{facet}-{value}:checked) label.tag[for="f-{facet}-{value}"]')
            group = (f'<fieldset class="switch tag-switch"><legend>{html.escape(title)}</legend>'
                     f'<div class="switch-cells">{"".join(cells)}</div></fieldset>')
            (groups if facet in FILTERS else more).append(group)
        rules.append("body:has(#f-old:checked) .tool[data-old]")
        style = ("<style>@supports selector(:has(a)) {" + ",\n".join(rules) + " { display: none; }\n"
                 + ",\n".join(checked) + " { background: var(--ink); color: var(--bg); border-color: var(--ink); }\n"
                 + ",\n".join(r + " .facet" for r in checked) + " { color: var(--bg); } }</style>")
        old = (f'<p class="tag-old"><input type="checkbox" id="f-old"> <label for="f-old">{words["old"]}</label> '
               f'<button type="reset" class="button">{words["reset"]}</button></p>')
        more_html = (f'<details class="more-filters"><summary>{words["more_filters"]}</summary>'
                     f'<div class="tag-filters">{"".join(more)}</div></details>')
        return (f'{style}<form class="tag-form" aria-label="{words["filters"]}"><p class="note tag-help">{words["help"]}</p>'
                f'<div class="tag-filters">{"".join(groups)}{old}</div>{more_html}</form>')

    def tag_chips(self, entry, language, toggles=False):
        """The tags of a tool, facet by facet in the order of the taxonomy.
        On the tiles each tag is the label of its filter box: a click checks
        or clears it."""
        chips = []
        for facet in self.taxonomy:
            for value in self.tag_values(entry, facet):
                facet_name = TAGS_FR.get(facet, facet) if language == "fr" else facet
                inner = f'<span class="facet">{html.escape(facet_name)}</span> {html.escape(self.tag_label(value, language))}'
                if toggles:
                    chips.append(f'<li><label class="tag" for="f-{facet}-{value}">{inner}</label></li>')
                else:
                    chips.append(f'<li><span class="tag">{inner}</span></li>')
        return f'<ul class="tags">{"".join(chips)}</ul>'

    def retopology_tools(self, tools, platforms, language):
        """The tiles of the retopology page, one per tool: its name leads to
        the tool's own page, its tags toggle the filters."""
        words = RETOPOLOGY_WORDS[language]
        here = posixpath.dirname(self.page_path("retopology", language)) or "."
        tiles = []
        old = 0
        for entry in tools:
            slug, dist = entry["slug"], entry["distribution"]
            data = " ".join(f'data-{facet}="{" ".join(self.tag_values(entry, facet))}"' for facet in FILTERS + MORE_FILTERS)
            if entry["status"] in ("stale", "abandoned"):
                data += " data-old"
                old += 1
            stores = [dist] + dist.get("also", [])
            where = ", ".join(dict.fromkeys(re.sub(r" \(.*\)$", "", platforms[s["channel"]]["name"]) for s in stores))
            short = html.escape(re.split(r" - | \| |: ", entry["name"])[0])
            target = posixpath.relpath(self.page_path(f"retopology/{slug}", language), here)
            status = self.status_label(entry, language)
            css = " tile-old" if entry["status"] in ("stale", "abandoned") else ""
            tiles.append(
                f'<article class="tool tile{css}" {data}><div class="tile-facts">'
                f'<h3><a href="{target}">{short}</a></h3>'
                f'<p class="tile-by">{html.escape(entry["author"])} · {status}</p>'
                f'<p class="tile-what">{html.escape(entry.get("purpose") or entry["method"])}</p>'
                f'<p class="tile-price">{html.escape(dist["price"])} · {html.escape(where)}</p></div>'
                f'{self.tag_chips(entry, language, toggles=True)}</article>'
            )
        count = words["count"].format(shown=len(tools), old=old)
        return f'<p>{count}</p><div class="tiles">{"".join(tiles)}</div>'

    def os_cell(self, entry):
        """W, L, M for the systems a tool states; a dash when none is stated."""
        systems = self.tag_values(entry, "os")
        return " ".join(letter for name, letter in (("windows", "W"), ("linux", "L"), ("macos", "M")) if name in systems) or "—"

    @staticmethod
    def blender_span(blender):
        span = "–".join(dict.fromkeys(v for v in (blender.get("min"), blender.get("max")) if v)) or "—"
        return span + "+" if blender.get("min") and not blender.get("max") else span

    @staticmethod
    def status_label(entry, language):
        return STATUS_FR.get(entry["status"], entry["status"]) if language == "fr" else entry["status"]

    @staticmethod
    def facet_title(facet, language):
        english = {"mode": "Mode", "scope": "Scope", "output": "Output", "engine": "Engine", "price": "Price",
                   "licence": "Licence", "source": "Source code"}
        return TAGS_FR.get(facet, facet) if language == "fr" else english.get(facet, facet)

    def tool_page(self, entry, tools, platforms, language):
        """The page of one tool: what it is, where to get it, its tags and
        the pages its facts come from."""
        words = RETOPOLOGY_WORDS[language]
        slug, dist, blender = entry["slug"], entry["distribution"], entry.get("blender", {})
        key = f"retopology/{slug}"
        here = posixpath.dirname(self.page_path(key, language))
        main = posixpath.relpath(self.page_path("retopology", language), here)
        link = lambda url, text=None: f'<a href="{html.escape(url)}" rel="noopener">{html.escape(text or url)}</a>'
        tags = lambda facet: ", ".join(self.tag_label(v, language) for v in self.tag_values(entry, facet))
        stores = [dist] + dist.get("also", [])

        facts = [(words["author"], html.escape(entry["author"]))]
        facts.append((words["method"], html.escape(entry["method"])))
        engine_links = [
            f'<a href="{posixpath.relpath(self.page_path("engines/" + v, language), here)}">{html.escape(self.engines[v]["name"])}</a>'
            if v in self.engines else html.escape(self.tag_label(v, language))
            for v in self.tag_values(entry, "engine")]
        if engine_links:
            facts.append((words["engine"], ", ".join(engine_links)))
        facts.append((words["licence"], f'{tags("licence")} ({html.escape(dist["licence"])})'))
        facts.append((words["source"], link(dist["source"]) if dist.get("source") else tags("source")))
        facts.append((words["blender"], self.blender_span(blender)))
        if blender.get("release"):
            facts.append((words["release"], html.escape(blender["release"])))
        facts.append((words["status"], self.status_label(entry, language)))
        store_items = "".join(
            f'<li><a href="{posixpath.relpath(self.page_path("platforms/" + s["channel"], language), here)}">'
            f'{html.escape(platforms[s["channel"]]["name"])}</a>' + (f': {html.escape(s["price"])}' if s.get("price") else "")
            + f' · {link(s["url"], words["get_it"] + " ↗")}</li>'
            for s in stores)
        facts.append((words["stores"], f"<ul>{store_items}</ul>"))
        if entry.get("papers"):
            facts.append((words["papers"], "<ul>" + "".join(
                f'<li>{html.escape(p["authors"])}, “{html.escape(p["title"])}”, {html.escape(p["venue"])}, {p["year"]}'
                + (", " + link(f'https://doi.org/{p["doi"]}', f'doi:{p["doi"]}') if p.get("doi") else "") + "</li>"
                for p in entry["papers"]) + "</ul>"))
        if entry.get("notes"):
            facts.append((words["notes"], html.escape(entry["notes"])))
        rows = "".join(f"<dt>{label}</dt><dd>{value}</dd>" for label, value in facts)

        engines = [e for e in self.tag_values(entry, "engine") if e not in ("own", "proprietary")]
        same = [other for other in tools if other is not entry and set(engines) & set(self.tag_values(other, "engine"))]
        sources = "".join(f'<li>{link(s["url"])}: {html.escape(s["what"])}</li>' for s in entry["sources"])
        name = html.escape(entry["name"])
        subtitle = html.escape(entry.get("purpose") or entry["method"])
        body = [f'<h1 id="{slug}">{name}</h1>', f'<p class="subtitle">{subtitle}</p>',
                f'<p><a class="button primary" href="{html.escape(dist["url"])}" rel="noopener">{words["get_it"]} ↗</a> '
                f'{html.escape(dist["price"])}</p>']
        body.append(self.tag_chips(entry, language))
        if language == "fr":
            body.append(f'<p class="note">{words["prose_en"]}</p>')
        body += [f'<h2 id="facts">{words["facts"]}</h2>', f'<dl class="facts">{rows}</dl>']
        if same:
            items = "".join(
                f'<li><a href="{posixpath.relpath(self.page_path("retopology/" + o["slug"], language), here)}">'
                f'{html.escape(o["name"])}</a></li>' for o in same)
            body += [f'<h2 id="same-engine">{words["same_engine"]}</h2>', f"<ul>{items}</ul>"]
        body += [f'<h2 id="sources">{words["sources"]}</h2>', f"<ul>{sources}</ul>",
                 f'<p class="note">{words["checked"]} {entry["checked"]}</p>']
        breadcrumb = self.market_crumbs(key, language, "retopology", re.split(r" - | [|] |: ", entry["name"])[0])
        body = "\n".join(body)
        self.write(key, language, entry["name"], body, [], "retopology", subtitle, breadcrumb, toc=False)

    def built_on(self, engine_id, tools):
        """The tools that run an engine, the engine's own program first."""
        origin = self.engines[engine_id].get("origin")
        return ([e for e in tools if e["slug"] == origin]
                + [e for e in tools if e["slug"] != origin and engine_id in self.tag_values(e, "engine")])

    def engines_list(self, engines, tools, language):
        """The engines page: one tile per engine, leading to its page, with
        the tools built on it."""
        words = RETOPOLOGY_WORDS[language]
        here = posixpath.dirname(self.page_path("engines", language)) or "."
        tiles = []
        for key, engine in engines.items():
            target = posixpath.relpath(self.page_path(f"engines/{key}", language), here)
            on = self.built_on(key, tools)
            chips = [(words["kind"].lower(), words["engine_kinds"][engine["kind"]])]
            if engine.get("licence"):
                chips.append((words["licence"].lower(), engine["licence"]))
            chips.append((words["tools_here"].lower(), str(len(on))))
            tags = "".join(f'<li><span class="tag"><span class="facet">{html.escape(f)}</span> {html.escape(v)}</span></li>' for f, v in chips)
            names = ", ".join(
                f'<a href="{posixpath.relpath(self.page_path("retopology/" + e["slug"], language), here)}">'
                f'{html.escape(re.split(r" - | [|] |: ", e["name"])[0])}</a>' for e in on)
            tiles.append(
                f'<article class="tile"><div class="tile-facts"><h3><a href="{target}">{html.escape(engine["name"])}</a></h3>'
                f'<p class="tile-by">{html.escape(engine["authors"])}</p>'
                f'<p class="tile-what">{html.escape(engine["method"])}</p>'
                f'<p class="tile-price">{names}</p></div><ul class="tags">{tags}</ul></article>'
            )
        return f'<div class="tiles">{"".join(tiles)}</div>'

    def own_solvers(self, tools, language):
        """The tools that run a solver of their own, none of the engines."""
        words = RETOPOLOGY_WORDS[language]
        here = posixpath.dirname(self.page_path("engines", language)) or "."
        own = [e for e in tools if set(self.tag_values(e, "engine")) & {"own", "proprietary", "simulation", "particles"}
               and e["slug"] not in {engine.get("origin") for engine in self.engines.values()}]
        items = "".join(
            f'<li><a href="{posixpath.relpath(self.page_path("retopology/" + e["slug"], language), here)}">'
            f'{html.escape(re.split(r" - | [|] |: ", e["name"])[0])}</a>: {html.escape(e["method"])}</li>' for e in own)
        return f"<ul>{items}</ul>"

    def engine_page(self, engine, tools, language):
        """The page of one engine: what it is, where its code is, and the
        tools that run it."""
        words = RETOPOLOGY_WORDS[language]
        key = f"engines/{engine['id']}"
        here = posixpath.dirname(self.page_path(key, language))
        link = lambda url, text=None: f'<a href="{html.escape(url)}" rel="noopener">{html.escape(text or url)}</a>'
        tool = lambda e: f'<a href="{posixpath.relpath(self.page_path("retopology/" + e["slug"], language), here)}">{html.escape(e["name"])}</a>'
        facts = [(words["authors"], html.escape(engine["authors"])), (words["kind"], words["engine_kinds"][engine["kind"]]),
                 (words["method"], html.escape(engine["method"]))]
        if engine.get("year"):
            facts.append((words["year"], str(engine["year"])))
        if engine.get("licence"):
            facts.append((words["licence"], html.escape(engine["licence"])))
        if engine.get("source"):
            facts.append((words["source"], link(engine["source"])))
        by_slug = {e["slug"]: e for e in tools}
        if engine.get("origin") in by_slug:
            facts.append((words["own_program"], tool(by_slug[engine["origin"]])))
        if engine.get("papers"):
            facts.append((words["papers"], "<ul>" + "".join(
                f'<li>{html.escape(p["authors"])}, “{html.escape(p["title"])}”, {html.escape(p["venue"])}, {p["year"]}'
                + (", " + link(f'https://doi.org/{p["doi"]}', f'doi:{p["doi"]}') if p.get("doi") else "") + "</li>"
                for p in engine["papers"]) + "</ul>"))
        if engine.get("notes"):
            facts.append((words["notes"], html.escape(engine["notes"])))
        rows = "".join(f"<dt>{label}</dt><dd>{value}</dd>" for label, value in facts)
        on = self.built_on(engine["id"], tools)
        body = [f'<h1 id="{html.escape(engine["id"])}">{html.escape(engine["name"])}</h1>',
                f'<p class="subtitle">{html.escape(engine["method"])}</p>']
        if language == "fr":
            body.append(f'<p class="note">{words["prose_en"]}</p>')
        body += [f'<h2 id="facts">{words["facts"]}</h2>', f'<dl class="facts">{rows}</dl>']
        if on:
            body += [f'<h2 id="used-by">{words["used_by"]} ({len(on)})</h2>',
                     "<ul>" + "".join(f"<li>{tool(e)}</li>" for e in on) + "</ul>"]
        if engine.get("elsewhere"):
            body += [f'<h2 id="elsewhere">{words["elsewhere"]}</h2>', "<ul>" + "".join(
                f'<li>{link(e["url"], e["name"])}: {html.escape(e["what"])}</li>' for e in engine["elsewhere"]) + "</ul>"]
        sources = "".join(f'<li>{link(s["url"])}: {html.escape(s["what"])}</li>' for s in engine["sources"])
        body += [f'<h2 id="sources">{words["sources"]}</h2>', f"<ul>{sources}</ul>",
                 f'<p class="note">{words["checked"]} {engine["checked"]}</p>']
        breadcrumb = self.market_crumbs(key, language, "engines", engine["name"])
        self.write(key, language, engine["name"], "\n".join(body), [], "engines", engine["method"], breadcrumb, toc=False)

    def market_crumbs(self, key, language, parent=None, name=None):
        """The breadcrumb of a page of the Blender ecosystem: the articles,
        the list the page belongs to (`parent`), then its name."""
        here = posixpath.dirname(self.page_path(key, language)) or "."
        words = WORDS[language]
        steps = ["articles"] + ([parent] if parent else [])
        links = " › ".join(f'<a href="{posixpath.relpath(self.page_path(k, language), here)}">{html.escape(words[k])}</a>' for k in steps)
        return f'<nav class="breadcrumb" aria-label="breadcrumb">{links} › {html.escape(name or words[key])}</nav>'

    def market_nav(self, key, language):
        """The lists of the state of the market, as chips under a title."""
        here = posixpath.dirname(self.page_path(key, language)) or "."
        words = WORDS[language]
        items = []
        for target in MARKET:
            current = ' aria-current="page"' if target == key else ""
            items.append(f'<a href="{posixpath.relpath(self.page_path(target, language), here)}"{current}>{words[target]}</a>')
        return f'<nav class="chips market-nav" aria-label="{words["market"]}">{"".join(items)}</nav>'

    def tools_on(self, platforms, tools):
        """For each platform, the tools it distributes, with their price there."""
        found = {key: [] for key in platforms}
        for entry in tools:
            dist = entry["distribution"]
            for store in [dist] + dist.get("also", []):
                found[store["channel"]].append((entry, store))
        return found

    def platforms_list(self, platforms, tools, language):
        """The platforms page: one band per platform, leading to its page."""
        words = RETOPOLOGY_WORDS[language]
        here = posixpath.dirname(self.page_path("platforms", language)) or "."
        on = self.tools_on(platforms, tools)
        bands = []
        for key, p in platforms.items():
            target = posixpath.relpath(self.page_path(f"platforms/{key}", language), here)
            chips = [(words["kind"].lower(), p["kind"]), (words["sells"].lower(), words["yes"] if p["paid"] else words["no"]),
                     (words["tools_here"].lower(), str(len({e["slug"] for e, _ in on[key]})))]
            tags = "".join(f'<li><span class="tag"><span class="facet">{html.escape(f)}</span> {html.escape(v)}</span></li>' for f, v in chips)
            share = f'<p class="tile-what">{html.escape(p["seller_share"])}</p>' if p.get("seller_share") else ""
            bands.append(
                f'<article class="tile"><div class="tile-facts"><h3><a href="{target}">{html.escape(p["name"])}</a></h3>'
                f'<p class="tile-by">{html.escape(p["operator"])}</p>{share}</div><ul class="tags">{tags}</ul></article>'
            )
        return f'<div class="tiles">{"".join(bands)}</div>'

    def platform_page(self, key, p, tools, platforms, language):
        """The page of one platform: its terms, and the tools it distributes."""
        words = RETOPOLOGY_WORDS[language]
        page_key = f"platforms/{key}"
        here = posixpath.dirname(self.page_path(page_key, language))
        link = lambda url, text=None: f'<a href="{html.escape(url)}" rel="noopener">{html.escape(text or url)}</a>'
        facts = [(words["operator"], html.escape(p["operator"])), (words["kind"], html.escape(p["kind"])),
                 (words["paid_label"], words["yes"] if p["paid"] else words["no"])]
        for field, label in (("seller_share", words["share_label"]), ("licence_rule", words["licence_rule"]),
                             ("install", words["install"]), ("updates", words["updates"]), ("blender", words["blender_label"]),
                             ("notes", words["notes"])):
            if p.get(field):
                facts.append((label, html.escape(p[field])))
        rows = "".join(f"<dt>{label}</dt><dd>{value}</dd>" for label, value in facts)
        listed = sorted(self.tools_on(platforms, tools)[key], key=lambda pair: pair[0]["name"].lower())
        items = "".join(
            f'<li><a href="{posixpath.relpath(self.page_path("retopology/" + e["slug"], language), here)}">{html.escape(e["name"])}</a>'
            + (f': {html.escape(store["price"])}' if store.get("price") else "") + "</li>"
            for e, store in listed)
        sources = "".join(f'<li>{link(s["url"])}: {html.escape(s["what"])}</li>' for s in p["sources"])
        subtitle = f'{html.escape(p["kind"])} · {words["tools_here"].lower()} {len({e["slug"] for e, _ in listed})}'
        body = [f'<h1 id="{html.escape(key)}">{html.escape(p["name"])}</h1>', f'<p class="subtitle">{subtitle}</p>']
        if p.get("url"):
            body.append(f'<p><a class="button primary" href="{html.escape(p["url"])}" rel="noopener">{words["visit"]} ↗</a></p>')
        if language == "fr":
            body.append(f'<p class="note">{words["prose_en"]}</p>')
        body += [f'<h2 id="facts">{words["facts"]}</h2>', f'<dl class="facts">{rows}</dl>']
        if items:
            body += [f'<h2 id="tools">{words["tools_on"]} ({len(listed)})</h2>', f"<ul>{items}</ul>"]
        body += [f'<h2 id="sources">{words["sources"]}</h2>', f"<ul>{sources}</ul>"]
        breadcrumb = self.market_crumbs(page_key, language, "platforms", p["name"])
        self.write(page_key, language, p["name"], "\n".join(body), [], "platforms", p.get("seller_share", p["name"]), breadcrumb, toc=False)

    def market(self, tools, platforms, engines):
        for language in LANGUAGES:
            words, here = RETOPOLOGY_WORDS[language], posixpath.dirname(self.page_path("market", language)) or "."
            text, body, headings = self.content_page("market", language, "market")
            bands = []
            for target, line in (("retopology", words["market_tools"].format(n=len(tools))),
                                 ("engines", words["market_engines"].format(n=len(engines))),
                                 ("platforms", words["market_platforms"].format(n=len(platforms)))):
                href = posixpath.relpath(self.page_path(target, language), here)
                bands.append(f'<article class="tile"><div class="tile-facts"><h3><a href="{href}">{WORDS[language][target]}</a></h3>'
                             f'</div><p class="tile-what">{line}</p></article>')
            body = body.replace("<p>{{market}}</p>", f'<div class="tiles">{"".join(bands)}</div>')
            self.write("market", language, self.title_of(text), body, headings, "market", self.description_of(text), toc=False,
                       breadcrumb=self.market_crumbs("market", language))

    def retopology(self):
        tools, platforms, taxonomy, engines = self.retopology_data()
        self.taxonomy, self.engines = taxonomy, engines
        self.market(tools, platforms, engines)
        for language in LANGUAGES:
            text, body, headings = self.content_page("retopology", language, "retopology")
            body = body.replace("<p>{{tools}}</p>", self.retopology_tools(tools, platforms, language))
            self.write(
                "retopology", language, self.title_of(text), body, headings, "retopology", self.description_of(text),
                toc=False, breadcrumb="",
                extra=self.market_nav("retopology", language) + self.retopology_filters(taxonomy, language),
            )
            text, body, headings = self.content_page("platforms", language, "platforms")
            body = body.replace("<p>{{platforms}}</p>", self.platforms_list(platforms, tools, language))
            self.write(
                "platforms", language, self.title_of(text), body, headings, "platforms", self.description_of(text),
                toc=False, breadcrumb="", extra=self.market_nav("platforms", language),
            )
            text, body, headings = self.content_page("engines", language, "engines")
            body = body.replace("<p>{{engines}}</p>", self.engines_list(engines, tools, language))
            body = body.replace("<p>{{own}}</p>", self.own_solvers(tools, language))
            self.write(
                "engines", language, self.title_of(text), body, headings, "engines", self.description_of(text),
                toc=False, breadcrumb="", extra=self.market_nav("engines", language),
            )
            for engine in engines.values():
                self.engine_page(engine, tools, language)
            for entry in tools:
                self.tool_page(entry, tools, platforms, language)
            for key, p in platforms.items():
                self.platform_page(key, p, tools, platforms, language)

    # ---- model pages

    def credit_line(self, name, language, page):
        entry = self.credit.get(name)
        if not entry:
            return ""
        converter = md.Converter(self.resolver("CREDITS.md", page, language))
        french = language == "fr"
        parts = []
        if entry.get("made_by"):
            parts.append(("Made by the project: " if not french else "Fait par le projet : ") + converter.inline(entry["made_by"]))
        else:
            parts.append(("Model: " if not french else "Modèle : ") + converter.inline(entry["who"]))
        if entry.get("source"):
            parts.append(("Source: " if not french else "Source : ") + converter.inline(entry["source"]))
        parts.append(("Licence: " if not french else "Licence : ") + converter.inline(entry["licence"]))
        return '<p class="credit">' + "<br>".join(parts) + "</p>"

    def model_page(self, name, language):
        source = f"docs/models/{name}.md"
        text = read(ROOT / source)
        page = self.page_path(f"models/{name}", language)
        marker = "<p>@@credit@@</p>"
        text = text.replace("Author, source and licence: [CREDITS.md](../../CREDITS.md).", "\n@@credit@@\n")
        anchor = "how-the-marks-are-made" if language == "en" else "comment-les-notes-sont-faites"
        results = "../../site/content/results.md" if language == "en" else "../../site/content/results.fr.md"
        text = text.replace(
            "From 0 to 100; how they are made is in [SCORES.md](../../bench/results/SCORES.md).",
            f"From 0 to 100; how they are made is on the [results page]({results}#{anchor}).",
        )
        if language == "fr":
            text = text.replace(
                f"From 0 to 100; how they are made is on the [results page]({results}#{anchor}).",
                f"De 0 à 100 ; la façon dont elles sont faites est sur la [page des résultats]({results}#{anchor}).",
            )
            match = re.search(r"(?m)^An? (organic|mechanical|calibration|extra) model((?:, with [a-z ]+)*); ([\d ]+) triangles in the source\.$", text)
            if match:
                extras = match.group(2).replace(", with hard edges", ", avec des arêtes vives").replace(", with open borders", ", avec des bords ouverts")
                kind = KINDS_FR[match.group(1)]
                sentence = f"Un modèle {kind}{extras} ; {match.group(3)} triangles dans la source."
                text = text.replace(match.group(0), sentence)
            for english, french in MODEL_PHRASES_FR:
                text = text.replace(english, french)
            for pattern, replacement in MODEL_PATTERNS_FR:
                text = re.sub(pattern, replacement, text)
        legend = RESULT_WORDS[language]["legend"]
        text = re.sub(r"(?m)^(## (?:Marks|Notes))$", lambda m: m.group(1) + "\n\n" + legend.replace("\n", ""), text, count=1)
        text = self.loop_bars(self.shots(text), language)
        body, headings = self.render(text, source, page, language)
        body = budget_panels(marks_tables(body), language)
        body = body.replace(marker, self.credit_line(name, language, page))
        # Each picture opens at full size.
        body = re.sub(r'(<img src="([^"]+)"[^>]*>)', r'<a class="picture" href="\2">\1</a>', body)
        here = posixpath.dirname(page)
        results_page = posixpath.relpath(self.page_path("results", language), here)
        words = WORDS[language]
        breadcrumb = (
            f'<nav class="breadcrumb" aria-label="breadcrumb"><a href="{posixpath.relpath(self.page_path("articles", language), here)}">{words["articles"]}</a> › '
            f'<a href="{results_page}">{words["results"]}</a> › '
            f'<a href="{results_page}#{"model-by-model" if language == "en" else "modèle-par-modèle"}">{words["models"]}</a> › {name}</nav>'
        )
        title = name
        description = self.description_of(text.replace("@@credit@@", ""))
        scripts = ""
        if (ROOT / "docs/models/mesh" / name / "index.json").exists():
            # The 3D viewer, under the row of pictures: its script is the only
            # one of the site, and fetches nothing before it is asked to.
            index = posixpath.relpath(f"mesh/{name}/index.json", here)
            viewer = f'<div class="viewer" data-index="{index}" lang="{language}"></div>'
            start = body.find('<div class="shots">')
            end = body.find("</div>", start) + len("</div>") if start >= 0 else -1
            body = body[:end] + viewer + body[end:] if end > 0 else viewer + body
            scripts = VIEWER_SCRIPT
        self.write(f"models/{name}", language, title, body, headings, "model", description, breadcrumb, toc=False, scripts=scripts)

    def shots(self, text):
        """The table of pictures of a model page as a row of figures, the
        remesher's name and the size of its mesh under each."""
        lines = text.split("\n")
        for position, line in enumerate(lines):
            if not line.startswith("| Source |") or position + 3 >= len(lines):
                continue
            names = md.Converter.cells(line)
            images = md.Converter.cells(lines[position + 2])
            captions = md.Converter.cells(lines[position + 3])
            figures = []
            for name, image, caption in zip(names, images, captions):
                match = re.match(r"!\[([^\]]*)\]\(([^)]+)\)", image)
                picture = ""
                if match:
                    alt, target = match.groups()
                    if self.is_withheld(target):
                        picture = f'<span class="withheld">{html.escape(alt)}</span>'
                    else:
                        picture = f'<img src="{html.escape(target)}" alt="{html.escape(alt)}" loading="lazy" decoding="async">'
                ours = " ours" if name == "openclare" else ""
                figures.append(
                    f'<figure class="shot{ours}">{picture}<figcaption><b>{html.escape(name)}</b>'
                    f"<span>{html.escape(caption)}</span></figcaption></figure>"
                )
            lines[position: position + 4] = ['<div class="shots">' + "".join(figures) + "</div>"]
            break
        return "\n".join(lines)

    @staticmethod
    def loop_bars(text, language):
        """Above the table of edge loops of a model page, one bar per mesh:
        its edges on closed loops, spirals, loops that stop, and border to
        border loops."""
        parts = ("closed", "spiral", "stops", "border")
        lines = text.split("\n")
        for position, line in enumerate(lines):
            if not line.startswith("| | ") or position + 1 >= len(lines) or not lines[position + 1].startswith("|---"):
                continue
            header = md.Converter.cells(line)
            bars = []
            row = position + 2
            while row < len(lines) and lines[row].startswith("|"):
                cells = md.Converter.cells(lines[row])
                row += 1
                try:
                    values = [float(cell.replace("%", "").strip()) for cell in cells[1:5]]
                except ValueError:
                    continue
                name = cells[0]
                spans = "".join(
                    f'<span class="part-{part}" style="flex-grow: {value:g}"></span>' for part, value in zip(parts, values) if value > 0
                )
                shares = ", ".join(f"{label} {cell}" for label, cell in zip(header[1:5], cells[1:5]))
                label = html.escape(RESULT_WORDS[language]["loops_label"].format(name=name, parts=shares))
                ours = " ours" if name == "openclare" else ""
                bars.append(
                    f'<div class="loop-bar{ours}"><p><b>{html.escape(name)}</b><span>{cells[5]} {header[5]}</span></p>'
                    f'<div class="stack" role="img" aria-label="{label}">{spans}</div></div>'
                )
            if bars:
                legend = "".join(f'<span><i class="part-{part}"></i>{label}</span>' for part, label in zip(parts, header[1:5]))
                block = '<div class="loop-bars">' + "".join(bars) + f'<p class="loop-legend" aria-hidden="true">{legend}</p></div>'
                lines[position:position] = [block, ""]
            break
        return "\n".join(lines)

    # ---- the whole site

    def build(self):
        if self.out.exists():
            shutil.rmtree(self.out)
        self.out.mkdir(parents=True)
        shutil.copytree(SITE / "assets", self.out / "assets")
        if (ROOT / "docs/models/mesh").exists():
            # The meshes of the 3D viewer (scripts/viewer_meshes.py).
            shutil.copytree(ROOT / "docs/models/mesh", self.out / "mesh")
        (self.out / ".nojekyll").write_text("")
        self.simple("index", "home")
        self.simple("theory", "theory", {language: WORDS[language]["placeholder_theory"] for language in LANGUAGES})
        self.glossary()
        self.results()
        self.simple("research", "research")
        self.simple("get", "get")
        self.retopology()
        self.articles()
        for name in self.models:
            if name in self.withheld:
                continue
            for language in LANGUAGES:
                self.model_page(name, language)
        return self


# ---------------------------------------------------------------- page parts

# Written in a source right under a heading, it puts that section on a night
# band.
NIGHT = "<!-- section: night -->"


def number_headings(body):
    """"## 1. The problem" shows its number apart, in vermillion."""
    return re.sub(r'(<h2 id="[^"]*">)(\d+)\. ', r'\1<span class="num">\2</span>', body)


def heading_label(text):
    """A heading as a link of a table of contents."""
    return re.sub(r"^(\d+)\. ", r'<span class="num">\1</span>', re.sub(r"<[^>]+>", "", text))


def shell_prompts(body):
    """A prompt before each command of a shell block (not copied with it),
    its comments dimmed."""

    def lines(match):
        out = []
        for line in match.group(2).split("\n"):
            if line.lstrip().startswith("#"):
                out.append(f'<span class="comment">{line}</span>')
                continue
            comment = ""
            trailing = re.search(r"(\s+)(#.*)$", line)
            if trailing:
                line, comment = line[: trailing.start()], f'{trailing.group(1)}<span class="comment">{trailing.group(2)}</span>'
            prompt = '<span class="prompt" aria-hidden="true">$ </span>' if line.strip() and not line.startswith(" ") else ""
            out.append(prompt + line + comment)
        return match.group(1) + "\n".join(out) + "</code></pre>"

    return re.sub(r'(<pre><code class="language-sh">)(.*?)</code></pre>', lines, body, flags=re.S)


def night_sections(body):
    """The sections marked with NIGHT, each in a night block."""
    if NIGHT not in body:
        return body
    parts = re.split(r'(?=<h2 id=")', body)
    for position, part in enumerate(parts):
        if NIGHT in part:
            parts[position] = f'<section class="night night-block">\n{part.replace(NIGHT, "").strip()}\n</section>\n'
    return "".join(parts)


# The script of the 3D viewer, on the pages that hold one.
VIEWER_SCRIPT = '<script type="module" src="{root}assets/viewer.js?v={version}"></script>'


def page_intro(body, kicker, extra="", banner=""):
    """The band at the top of a page: the kicker (or breadcrumb), the title
    and subtitle taken out of the body, `extra`, and the banner of an article."""
    head = ""
    match = re.search(r'<h1 id="[^"]*">(.*?)</h1>\n?(?:<p class="subtitle">.*?</p>\n?)?', body, re.S)
    if match:
        head = match.group(0).strip()
        body = body[: match.start()] + body[match.end():]
        # A kicker that only repeats the title says nothing.
        if re.sub(r"<[^>]+>", "", kicker).strip().lower() == re.sub(r"<[^>]+>", "", match.group(1)).strip().lower():
            kicker = ""
    intro = f'<section class="intro"><div class="wrap">\n{kicker}\n{head}\n{extra}\n{banner}\n</div></section>'
    return intro, body.strip("\n")


def home_bands(body):
    """The home page: its hero at the top, then one band per section,
    numbered; a section marked with NIGHT on a night band. The first
    paragraph of a section stands beside its title."""
    parts = re.split(r'(?=<h2 id=")', body)
    intro = f'<section class="intro"><div class="wrap">\n{parts[0].strip()}\n</div></section>'
    bands = []
    for number, part in enumerate(parts[1:], 1):
        match = re.match(r'<h2 id="([^"]+)">(.*?)</h2>\n?', part, re.S)
        identifier, title = match.groups()
        rest = part[match.end():]
        night = NIGHT in rest
        rest = rest.replace(NIGHT, "").lstrip("\n")
        lead = ""
        first = re.match(r"<p>.*?</p>\n?", rest, re.S)
        if first:
            lead, rest = first.group(0).strip(), rest[first.end():]
        bands.append(
            f'<section class="band{" night" if night else ""}" aria-labelledby="{identifier}"><div class="wrap">\n'
            f'<div class="band-head"><h2 id="{identifier}"><span class="num">{number:02d}</span>{title}</h2>{lead}</div>\n'
            f"{rest.strip()}\n</div></section>"
        )
    return intro, "\n".join(bands)


def budget_switch(language):
    """The three cells that choose the budget whose panels are shown."""
    cells = []
    for budget in BUDGETS:
        checked = " checked" if budget == 3000 else ""
        cells.append(
            f'<input type="radio" name="budget" id="budget-{budget}" value="{budget}"{checked}>'
            f'<label for="budget-{budget}">{number(budget, language)}</label>'
        )
    legend = RESULT_WORDS[language]["asked_for"]
    return f'<fieldset class="switch"><legend>{legend}</legend><div class="switch-cells">{"".join(cells)}</div></fieldset>'


def score_cell(value, ours=False):
    """A mark with its bar: teal, vermillion under 50, slate for the rows
    that are not openclare's."""
    try:
        mark = int(re.sub(r"<[^>]+>", "", str(value)))
    except ValueError:
        return value
    tone = " low" if mark < 50 else "" if ours else " plain"
    return (
        f'<span class="score"><b>{mark}</b><span class="track" aria-hidden="true">'
        f'<span class="fill{tone}" style="width: {max(0, min(mark, 100))}%"></span></span></span>'
    )


def marks_tables(body):
    """The tables of marks of a model page: openclare's row set apart, the
    overall mark with its bar."""

    def table(match):
        text = match.group(0)
        head = re.search(r"<thead><tr>(.*?)</tr></thead>", text, re.S)
        if not head:
            return text
        names = [re.sub(r"<[^>]+>", "", cell) for cell in re.findall(r"<th[^>]*>(.*?)</th>", head.group(1), re.S)]
        overall = next((i for i, name in enumerate(names) if name in ("Overall", "Note globale")), None)
        if overall is None:
            return text
        text = text.replace("<table>", '<table class="marks">', 1)

        def row(match):
            cells = re.findall(r"<td[^>]*>(.*?)</td>", match.group(1), re.S)
            ours = cells and cells[0] == "<strong>openclare</strong>"
            out = []
            for position, cell in enumerate(cells):
                if position == overall:
                    out.append(f'<td class="overall">{score_cell(cell, ours)}</td>')
                elif cell in ("—", ""):
                    out.append('<td class="none">—</td>')
                else:
                    out.append(f"<td>{cell}</td>")
            css = ' class="ours"' if ours else ""
            return f"<tr{css}>{''.join(out)}</tr>"

        body_part = re.search(r"<tbody>(.*?)</tbody>", text, re.S)
        rows = re.sub(r"<tr>(.*?)</tr>", row, body_part.group(1), flags=re.S)
        text = text[: body_part.start(1)] + rows + text[body_part.end(1):]
        return re.sub(
            r"(<thead><tr>(?:<th[^>]*>.*?</th>){%d})<th>" % overall, r'\1<th class="overall-head">', text, count=1, flags=re.S
        )

    return re.sub(r'<div class="table-wrap"><table>.*?</table></div>', table, body, flags=re.S)


def budget_panels(body, language):
    """The "### 3000 quads" subsections of the marks of a model page, each a
    panel of the budget switch, the switch above the first."""
    pattern = re.compile(r'<h3 id="[^"]*">(\d+) quads</h3>')
    matches = list(pattern.finditer(body))
    if not matches:
        return body
    out = body[: matches[0].start()] + budget_switch(language) + "\n"
    for index, match in enumerate(matches):
        end = matches[index + 1].start() if index + 1 < len(matches) else len(body)
        chunk = body[match.start(): end]
        stop = re.search(r"<h2 ", chunk)
        tail = ""
        if stop:
            chunk, tail = chunk[: stop.start()], chunk[stop.start():]
        out += f'<div class="budget-panel" data-budget="{match.group(1)}">\n{chunk.strip()}\n</div>\n{tail}'
    return out


BADGES = {
    "proved": "proved", "measured": "measured", "conjectured": "conjectured", "refuted": "refuted",
    "démontré": "proved", "prouvé": "proved", "mesuré": "measured", "conjecturé": "conjectured", "réfuté": "refuted",
}


def theory_marks(body):
    """The theory page: each statement's kind (proved, measured...) as a
    badge, the steps of the engine numbered."""
    body = re.sub(
        r"<strong>(proved|measured|conjectured|refuted|démontré|prouvé|mesuré|conjecturé|réfuté)(e?s?)</strong>",
        lambda m: f'<strong class="badge badge-{BADGES[m.group(1)]}">{m.group(1)}{m.group(2)}</strong>',
        body,
    )
    return re.sub(
        r"<p><strong>(\d+)\. (.+?)</strong>(.*?)</p>",
        r'<p class="step"><span class="step-number">\1</span><span><strong>\2</strong>\3</span></p>',
        body,
        flags=re.S,
    )


def strip_block_comments(text):
    """Lean source without its /- ... -/ comments, which nest."""
    out, depth, position = [], 0, 0
    while position < len(text):
        if text.startswith("/-", position):
            depth, position = depth + 1, position + 2
        elif depth and text.startswith("-/", position):
            depth, position = depth - 1, position + 2
        else:
            if not depth:
                out.append(text[position])
            position += 1
    return "".join(out)


def french_spaces(body):
    """No line break before the French high punctuation: the space before
    ; : ! ? and inside « » becomes a non-breaking one, outside code."""
    parts = re.split(r"(<[^>]+>)", body)
    inside = 0
    for position, part in enumerate(parts):
        if part.startswith("<"):
            tag = re.match(r"</?(\w+)", part)
            if tag and tag.group(1) in ("code", "pre"):
                inside += -1 if part.startswith("</") else 1
            continue
        if inside == 0:
            part = re.sub(r" ([;:!?»])", "\u00a0\\1", part)
            parts[position] = re.sub(r"« ", "«\u00a0", part)
    return "".join(parts)


def shown(path):
    """A path as short as it can be written from here."""
    relative = os.path.relpath(path)
    return str(path) if relative.startswith("..") else relative


def size_of(folder):
    return sum(path.stat().st_size for path in Path(folder).rglob("*") if path.is_file())


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--out", default=str(SITE / "_build"), help="where to write the site (default: site/_build)")
    arguments = parser.parse_args()
    site = Site(arguments.out).build()
    for warning in sorted(set(site.warnings)):
        print(f"warning: {warning}")
    print(f"pictures left out (licence): {', '.join(sorted(site.withheld)) or 'none'}")
    if not WEBP:
        print("note: Pillow with WebP is not available, pictures copied as they are")
    print(f"written: {len(site.pages)} pages, {len(site.assets)} files, {size_of(site.out) / 1e6:.1f} MB in {shown(site.out)}")


if __name__ == "__main__":
    main()
