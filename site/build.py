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
- CREDITS.md: who made each model, and which licences forbid passing a model
  on. The pictures of those models are left out of the site.

    python3 site/build.py [--out DIR]
"""

import argparse
import html
import json
import os
import posixpath
import re
import shutil
import sys
from pathlib import Path

SITE = Path(__file__).resolve().parent
ROOT = SITE.parent
sys.path.insert(0, str(SITE))
import md  # noqa: E402

CONTENT = SITE / "content"
REPOSITORY = "https://github.com/marcgardent/openclare"
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
        "research": "Research note",
        "get": "Get it",
        "models": "Model by model",
        "other_title": "Lire cette page en français",
        "footer": (
            "<p><strong>openclare</strong>: automatic quad retopology. Code under the GNU GPL, version 3 or later; "
            "the pictures of each model under the licence of that model, listed in "
            '<a href="{credits}">CREDITS.md</a>.</p>'
            "<p>No cookies, no trackers, no scripts. Built from the repository by <code>site/build.py</code>"
            ' (<a href="{repository}">GitHub</a>, private for now).</p>'
        ),
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
        "research": "Note de recherche",
        "get": "L'obtenir",
        "models": "Modèle par modèle",
        "other_title": "Read this page in English",
        "footer": (
            "<p><strong>openclare</strong> : la retopologie automatique en quads. Code sous GNU GPL, version 3 ou "
            "ultérieure ; les images de chaque modèle sous la licence de ce modèle, listée dans "
            '<a href="{credits}">CREDITS.md</a>.</p>'
            "<p>Ni cookies, ni pisteurs, ni scripts. Construit depuis le dépôt par <code>site/build.py</code>"
            ' (<a href="{repository}">GitHub</a>, privé pour l\'instant).</p>'
        ),
        "placeholder_theory": (
            "# Théorie\n\nCette page, la théorie d'openclare en mots simples, est en cours d'écriture. En attendant, "
            "le [glossaire](../../docs/GLOSSARY.fr.md) explique les mots en images, et la "
            "[note de recherche](research.fr.md) raconte toute l'histoire.\n"
        ),
    },
}

NAVIGATION = ("index", "theory", "glossary", "results", "research", "get")

# Display names of the remeshers of the bench, and what the rows that are
# not one of the four remeshers compared are.
TOOLS = {
    "openclare": ("openclare", "openclare"),
    "quadremesher": ("Quad Remesher", "Quad Remesher"),
    "quadriflow": ("QuadriFlow (Blender)", "QuadriFlow (Blender)"),
    "autoremesher": ("AutoRemesher", "AutoRemesher"),
    "openclare-uniform": ("openclare, quads of one size", "openclare, quads d'une seule taille"),
    "integer-grid": ("openclare, integer-grid chain", "openclare, chaîne integer-grid"),
    "instant-meshes": ("openclare, Instant Meshes chain", "openclare, chaîne Instant Meshes"),
    "quadremesher-uniform": ("Quad Remesher, uniform setting", "Quad Remesher, réglage uniforme"),
    "handmade": ("Modelled by hand", "Modélisé à la main"),
    "openclare-as-handmade": ("openclare at the hand-made count", "openclare au nombre du fait main"),
}
COMPARED = ("openclare", "quadremesher", "quadriflow", "autoremesher")
VARIANTS = ("openclare-uniform", "integer-grid", "instant-meshes", "quadremesher-uniform", "handmade", "openclare-as-handmade")

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
        "compared": "The four remeshers compared",
        "variants": "Other settings and chains, for reference",
        "not_everywhere": "Not run on every model: {list}.",
        "of": "{name} ({done} of {total})",
        "legend_intro": "The rows set apart under each table:",
        "legend": (
            "<dl class=\"legend\">"
            "<dt>openclare, quads of one size</dt><dd>openclare from the command line, every quad the same size: like with like against the remeshers that have nothing else.</dd>"
            "<dt>openclare, integer-grid and Instant Meshes chains</dt><dd>earlier chains of openclare's own engine, kept as yardsticks. The Instant Meshes chain is the project's port of that method; the Instant Meshes program itself is not run.</dd>"
            "<dt>Quad Remesher, uniform setting</dt><dd>Quad Remesher with its adaptive size turned off.</dd>"
            "<dt>Modelled by hand</dt><dd>the base meshes an artist made for the two human bodies, the only models that have one; openclare is also run at their number of quads.</dd>"
            "</dl>"
        ),
        "equal_intro": (
            "Quad Remesher's mesh as it is, marked against the number of quads it has, next to openclare asked for "
            "that same number. Budget does not count here. Average over the models of each group."
        ),
        "group": "Group",
        "quads_pair": "Quads (Quad Remesher / openclare)",
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
    },
    "fr": {
        "budget": "{n} quads demandés",
        "over": "Sur {count} modèles : {names}.",
        "remesher": "Remailleur",
        "compared": "Les quatre remailleurs comparés",
        "variants": "Autres réglages et chaînes, pour référence",
        "not_everywhere": "Pas lancé sur tous les modèles : {list}.",
        "of": "{name} ({done} sur {total})",
        "legend_intro": "Les lignes mises à part sous chaque tableau :",
        "legend": (
            "<dl class=\"legend\">"
            "<dt>openclare, quads d'une seule taille</dt><dd>openclare en ligne de commande, tous les quads de la même taille : à armes égales face aux remailleurs qui n'ont rien d'autre.</dd>"
            "<dt>openclare, chaînes integer-grid et Instant Meshes</dt><dd>des chaînes antérieures du moteur d'openclare lui-même, gardées comme étalons. La chaîne Instant Meshes est le portage de cette méthode par le projet ; le programme Instant Meshes lui-même n'est pas lancé.</dd>"
            "<dt>Quad Remesher, réglage uniforme</dt><dd>Quad Remesher avec sa taille adaptative désactivée.</dd>"
            "<dt>Modélisé à la main</dt><dd>les maillages de base qu'un artiste a faits pour les deux corps humains, les seuls modèles qui en ont un ; openclare est aussi lancé à leur nombre de quads.</dd>"
            "</dl>"
        ),
        "equal_intro": (
            "Le maillage de Quad Remesher tel quel, noté au nombre de quads qu'il a, à côté d'openclare à qui l'on "
            "demande ce même nombre. Le budget ne compte pas ici. Moyenne sur les modèles de chaque groupe."
        ),
        "group": "Groupe",
        "quads_pair": "Quads (Quad Remesher / openclare)",
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
    },
}

# The fixed sentences of the model pages (scripts/pages.py), in French. The
# regular expressions come after the plain phrases.
MODEL_PHRASES_FR = [
    ("At equal quad count: Quad Remesher's mesh as it is, and openclare asked for as many quads.",
     "À nombre de quads égal : le maillage de Quad Remesher tel quel, et openclare à qui l'on demande autant de quads."),
    ("Quad Remesher makes more quads than the 3000 it is asked for. Next to its mesh, openclare asked for as many quads as that mesh has.",
     "Quad Remesher fait plus que les 3000 quads qu'on lui demande. À côté de son maillage, openclare à qui l'on a demandé autant de quads que ce maillage en a."),
    ("At 3000 quads asked for. Each loop is followed straight on through every regular vertex: it closes, comes back beside "
     "itself a quad further (a spiral), stops at an irregular vertex, or runs from a border to a border. Share of the edges on "
     "each, those of borders and of faces that are not quads left out; \"—\" where the mesh is not a manifold.",
     "À 3000 quads demandés. Chaque boucle est suivie tout droit à travers chaque sommet régulier : elle se ferme, revient à "
     "côté d'elle-même un quad plus loin (une spirale), s'arrête sur un sommet irrégulier, ou va d'un bord à un bord. Part des "
     "arêtes sur chacune, celles des bords et des faces qui ne sont pas des quads laissées de côté ; « — » là où le maillage "
     "n'est pas une variété."),
    ("## At 3000 quads asked for", "## À 3000 quads demandés"),
    ("## At equal quad count", "## À nombre de quads égal"),
    ("## Edge loops", "## Boucles d'arêtes"),
    ("## Marks", "## Notes"),
    ("| | closed | spiral | stops | border to border | spirals | longest that stops |",
     "| | fermées | en spirale | s'arrêtent | de bord à bord | spirales | plus longue qui s'arrête |"),
    ("| Quad Remesher | openclare, as many quads |", "| Quad Remesher | openclare, autant de quads |"),
    ("![openclare, as many quads]", "![openclare, autant de quads]"),
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
KINDS_FR = {"organic": "organique", "mechanical": "mécanique", "calibration": "d'étalonnage", "extra": "extra"}


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
            if "BlenderKit Royalty Free" in entry["licence"]:
                withheld[name] = entry
    return credit, withheld


def number(value, language):
    text = f"{value:,}"
    return text.replace(",", " ") if language == "fr" else text


def scores():
    """The tables of bench/results/SCORES.md: averages and equal-count
    averages per budget and group, the group of every model, and the
    per-model marks of scores.json."""
    averages, equal, kinds, over, notes = {}, {}, {}, {}, {}
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
        heading = re.match(r"### (\w+) models at equal quad count$", line)
        if heading:
            group, mode = heading.group(1).lower(), "equal"
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
        elif mode == "equal" and cells[0] == "average":
            equal[(budget, group)] = dict(zip(header[1:], cells[1:]))
    per_model = json.loads(read(ROOT / "bench/results/scores.json"))
    return {"averages": averages, "equal": equal, "kinds": kinds, "over": over, "notes": notes, "models": per_model}


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
        self.warnings = []

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
        elif repository_path == "docs/research/closing-helices/paper.pdf":
            target = "files/closing-edge-loops.pdf"
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
            if path == "docs/research/closing-helices/paper.pdf":
                return relative(self.asset(path))
            page_of = self.page_of_source(path, language)
            if page_of:
                return relative(self.page_path(*page_of)) + fragment
            kind = "tree" if (ROOT / path).is_dir() else "blob"
            return f"{REPOSITORY}/{kind}/main/{path}{fragment}"

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

    def write(self, key, language, title, body, headings, kind, description="", breadcrumb="", toc=True):
        page = self.page_path(key, language)
        root = posixpath.relpath(".", posixpath.dirname(page) or ".")
        root = "" if root == "." else root + "/"
        here = posixpath.dirname(page) or "."
        words = WORDS[language]
        other = "fr" if language == "en" else "en"
        nav = []
        for item in NAVIGATION:
            target = posixpath.relpath(self.page_path(item, language), here)
            current = ' aria-current="page"' if item == key or (item == "results" and key.startswith("models/")) else ""
            nav.append(f'<li><a href="{target}"{current}>{words["home" if item == "index" else item]}</a></li>')
        sections = [h for h in headings if h[0] == 2]
        toc_html = ""
        if toc and len(sections) >= 3:
            items = "".join(f'<li><a href="#{identifier}">{re.sub(r"<[^>]+>", "", text)}</a></li>' for _, text, identifier in sections)
            # Beside the text on a wide screen; folded under the title on a
            # narrow one.
            toc_html = f'<aside class="toc toc-side"><p>{words["on_this_page"]}</p><ol>{items}</ol></aside>'
            inline = f'<details class="toc toc-inline"><summary>{words["on_this_page"]}</summary><ol>{items}</ol></details>'
            body = re.sub(r"(</h1>\n(?:<p class=\"subtitle\">.*?</p>\n)?)", lambda m: m.group(1) + inline + "\n", body, count=1)
        values = {
            "lang": language,
            "title": html.escape(title if key == "index" else f"{title} · openclare"),
            "description": html.escape(description or title),
            "root": root,
            "home": posixpath.relpath(self.page_path("index", language), here),
            "other_lang": other,
            "other_href": posixpath.relpath(self.page_path(key, other), here),
            "other_name": other.upper(),
            "other_title": words["other_title"],
            "skip": words["skip"],
            "nav_label": words["nav_label"],
            "nav": "<ul>" + "".join(nav) + "</ul>",
            "kind": kind,
            "layout_class": " with-toc" if toc_html else "",
            "toc": toc_html,
            "breadcrumb": breadcrumb,
            "body": body,
            "footer": words["footer"].format(
                credits=f"{REPOSITORY}/blob/main/CREDITS.md", repository=REPOSITORY
            ),
        }
        if language == "fr":
            values["body"] = french_spaces(body)
        template = read(SITE / "template.html")
        output = re.sub(r"\{(\w+)\}", lambda m: values[m.group(1)], template)
        destination = self.out / page
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text(output, encoding="utf-8")
        self.pages.append((key, language, page))

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
        """{{mark:3000:organic:openclare}}, {{equal:3000:organic:openclare}},
        {{count:organic}}, {{count:all}} in a source."""

        def value(match):
            parts = match.group(1).split(":")
            if parts[0] == "mark":
                return self.data["averages"][(int(parts[1]), parts[2])][parts[3]]["Overall"]
            if parts[0] == "equal":
                pair = self.data["equal"][(int(parts[1]), parts[2])]["Overall"].split(" / ")
                return pair[0] if parts[3] == "quadremesher" else pair[1]
            if parts[0] == "count":
                if parts[1] == "all":
                    return str(len(self.data["models"]["3000"]))
                return str(len(self.data["over"][(3000, parts[1])]))
            return match.group(0)

        return re.sub(r"\{\{((?:mark|equal|count):[\w:-]+)\}\}", value, text)

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
            self.write(
                key, language, self.title_of(text), body, headings, kind, self.description_of(text), toc=key not in ("index", "results")
            )

    def glossary(self):
        for language in LANGUAGES:
            source = "docs/GLOSSARY.md" if language == "en" else "docs/GLOSSARY.fr.md"
            text = self.drop_language_link(read(ROOT / source))
            body, headings = self.render(text, source, self.page_path("glossary", language), language)
            self.write("glossary", language, self.title_of(text), body, headings, "glossary", self.description_of(text))

    # ---- results

    def marks_table(self, rows, columns, language, names=None, extra_class=""):
        """A table of marks: `rows` is a list of (label, {column: value},
        css class)."""
        index = 0 if language == "en" else 1
        head = "".join(f"<th>{html.escape(CRITERIA.get(c, (c, c))[index])}</th>" for c in columns)
        body = []
        for label, cells, css in rows:
            if css == "divider":
                body.append(f'<tr class="divider"><td colspan="{len(columns) + 1}">{label}</td></tr>')
                continue
            tds = []
            for column in columns:
                value = cells.get(column, "—")
                classes = []
                if column == "Overall":
                    classes.append("overall")
                if value in ("—", "", None):
                    classes.append("none")
                    value = "—"
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

    def averages_html(self, language):
        words = RESULT_WORDS[language]
        index = 0 if language == "en" else 1
        columns = ["Quads", "Overall", "Validity", "Alignment", "Quad shape", "Fidelity", "Regularity", "Structure", "Edge loops", "Budget"]
        parts = []
        for budget in (3000, 1000, 8000):
            inner = []
            for group in GROUPS:
                table = self.data["averages"].get((budget, group))
                if not table:
                    continue
                rows = []
                compared = sorted((t for t in COMPARED if t in table), key=lambda t: -int(table[t]["Overall"]))
                for tool in compared:
                    rows.append((TOOLS[tool][index], table[tool], "ours" if tool == "openclare" else ""))
                variants = [t for t in VARIANTS if t in table]
                if variants:
                    rows.append((words["variants"], {}, "divider"))
                    for tool in sorted(variants, key=lambda t: -int(table[t]["Overall"])):
                        rows.append((TOOLS[tool][index], table[tool], "variant"))
                names = self.data["over"].get((budget, group), [])
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
            opened = " open" if budget == 3000 else ""
            parts.append(f'<details class="budget"{opened}><summary>{title}</summary><div class="inner">{"".join(inner)}</div></details>')
        parts.append(f'<p class="note">{words["legend_intro"]}</p>' + words["legend"])
        return "\n".join(parts)

    def equal_html(self, language):
        words = RESULT_WORDS[language]
        index = 0 if language == "en" else 1
        head = (
            f'<th>{words["group"]}</th><th>{words["quads_pair"]}</th>'
            f'<th>Quad Remesher</th><th>openclare</th>'
        )
        parts = [f'<p>{words["equal_intro"]}</p>']
        for budget in (1000, 3000, 8000):
            rows = []
            for group in GROUPS:
                average = self.data["equal"].get((budget, group))
                if not average:
                    continue
                quads = " / ".join(number(int(q), language) if q.isdigit() else q for q in average["Quads"].split(" / "))
                theirs, ours = average["Overall"].split(" / ")
                best_theirs = ' class="overall best"' if int(theirs) > int(ours) else ' class="overall"'
                best_ours = ' class="overall best"' if int(ours) > int(theirs) else ' class="overall"'
                rows.append(
                    f"<tr><td>{GROUPS[group][index]}</td><td>{quads}</td>"
                    f"<td{best_theirs}>{theirs}</td><td{best_ours}>{ours}</td></tr>"
                )
            title = words["budget"].format(n=number(budget, language))
            parts.append(f"<h4>{title}</h4>")
            parts.append(
                f'<div class="table-wrap"><table class="marks"><thead><tr>{head}</tr></thead>'
                f'<tbody>{"".join(rows)}</tbody></table></div>'
            )
        return "\n".join(parts)

    def models_html(self, language, page):
        words = RESULT_WORDS[language]
        index = 0 if language == "en" else 1
        here = posixpath.dirname(page) or "."
        parts = [f'<p>{words["models_intro"]} {words["best"]}</p>']
        for budget in (3000, 1000, 8000):
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
                        css = "overall best" if values[tool] == best else ""
                        css += " ours" if tool == "openclare" else ""
                        cells.append(f'<td class="{css.strip()}">{values[tool]}</td>')
                    else:
                        cells.append('<td class="none">—</td>')
                rows.append(f"<tr><td>{label}</td><td>{kind_label}</td>{''.join(cells)}</tr>")
            head = f'<th>{words["model"]}</th><th>{words["kind"]}</th>' + "".join(f"<th>{TOOLS[t][index]}</th>" for t in COMPARED)
            table = (
                f'<div class="table-wrap"><table class="marks models"><thead><tr>{head}</tr></thead>'
                f'<tbody>{"".join(rows)}</tbody></table></div>'
            )
            title = words["budget"].format(n=number(budget, language))
            opened = " open" if budget == 3000 else ""
            parts.append(f'<details class="budget"{opened}><summary>{title}</summary><div class="inner">{table}</div></details>')
        return "\n".join(parts)

    def withheld_html(self, language, page):
        words = RESULT_WORDS[language]
        resolve = self.resolver("CREDITS.md", page, language)
        converter = md.Converter(resolve)
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
                ("equal", lambda: self.equal_html(language)),
                ("models", lambda: self.models_html(language, page)),
                ("withheld", lambda: self.withheld_html(language, page)),
            ):
                body = body.replace(f"<p>{{{{{token}}}}}</p>", make())
            self.write(
                "results", language, self.title_of(text), body, headings, "results", self.description_of(text), toc=False
            )

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
        body, headings = self.render(text, source, page, language)
        body = body.replace(marker, self.credit_line(name, language, page))
        # Each picture opens at full size.
        body = re.sub(r'(<img src="([^"]+)"[^>]*>)', r'<a class="picture" href="\2">\1</a>', body)
        here = posixpath.dirname(page)
        results_page = posixpath.relpath(self.page_path("results", language), here)
        words = WORDS[language]
        breadcrumb = (
            f'<nav class="breadcrumb" aria-label="breadcrumb"><a href="{results_page}">{words["results"]}</a> › '
            f'<a href="{results_page}#{"model-by-model" if language == "en" else "modèle-par-modèle"}">{words["models"]}</a> › {name}</nav>'
        )
        title = name
        description = self.description_of(text.replace("@@credit@@", ""))
        self.write(f"models/{name}", language, title, body, headings, "model", description, breadcrumb, toc=False)

    # ---- the whole site

    def build(self):
        if self.out.exists():
            shutil.rmtree(self.out)
        self.out.mkdir(parents=True)
        shutil.copytree(SITE / "assets", self.out / "assets")
        (self.out / ".nojekyll").write_text("")
        self.simple("index", "home")
        self.simple("theory", "theory", {language: WORDS[language]["placeholder_theory"] for language in LANGUAGES})
        self.glossary()
        self.results()
        self.simple("research", "research")
        self.simple("get", "get")
        for name in self.models:
            if name in self.withheld:
                continue
            for language in LANGUAGES:
                self.model_page(name, language)
        return self


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
