// The 3D viewer of a model page: the meshes of the remeshers, with their
// quads drawn as quads. Nothing is fetched before the visitor asks for it.
//
// The page holds <div class="viewer" data-index="…/index.json" lang="…">, or
// <div class="viewer" data-models="ogre spot …" data-base="…/mesh/" lang="…">
// to pick a model; each index.json lists the meshes made by scripts/viewer_meshes.py, in the .ocqm
// format that script describes.

const WORDS = {
  en: {
    open: "View in 3D",
    model: "Model",
    remesher: "Remesher",
    budget: "Quads asked for",
    handmade: "Modelled by hand",
    count: (n, one, many) => `${n.toLocaleString("en")} ${n === 1 ? one : many}`,
    quads: ["quad", "quads"],
    faces: ["face", "faces"],
    vertices: ["vertex", "vertices"],
    poles: ["pole", "poles"],
    closed: ["closed loop", "closed loops"],
    spiral: ["spiral", "spirals"],
    run: (date) => `bench run of ${date}`,
    reset: "Reset the view",
    help: "Drag to turn, scroll or pinch to zoom, right-drag or two fingers to move.",
    loading: "Loading…",
    failed: "The mesh could not be loaded.",
    webgl: "This browser cannot show 3D: WebGL is off or missing.",
    canvas: "The remeshed model in 3D",
  },
  fr: {
    open: "Voir en 3D",
    model: "Modèle",
    remesher: "Remailleur",
    budget: "Quads demandés",
    handmade: "Modélisé à la main",
    count: (n, one, many) => `${n.toLocaleString("fr").replace(/ | /g, " ")} ${n <= 1 ? one : many}`,
    quads: ["quad", "quads"],
    faces: ["face", "faces"],
    vertices: ["sommet", "sommets"],
    poles: ["pôle", "pôles"],
    closed: ["boucle fermée", "boucles fermées"],
    spiral: ["spirale", "spirales"],
    run: (date) => `passage du banc du ${date}`,
    reset: "Revenir à la vue de départ",
    help: "Glisser pour tourner, molette ou pincement pour zoomer, clic droit ou deux doigts pour déplacer.",
    loading: "Chargement…",
    failed: "Le maillage n'a pas pu être chargé.",
    webgl: "Ce navigateur ne peut pas afficher la 3D : WebGL est désactivé ou absent.",
    canvas: "Le modèle remaillé en 3D",
  },
};

const NAMES = {
  openclare: "openclare",
  quadriflow: "QuadriFlow (Blender)",
  autoremesher: "AutoRemesher",
  "instant-meshes-jakob": "Instant Meshes",
  "quadwild-bimdf": "QuadWild-BiMDF",
};

function decode(buffer) {
  const view = new DataView(buffer);
  const magic = String.fromCharCode(...new Uint8Array(buffer, 0, 4));
  if (magic !== "OCQM" || view.getUint32(4, true) !== 2) throw new Error("not an .ocqm mesh");
  const short = (view.getUint32(8, true) & 1) === 1;
  const vertices = view.getUint32(12, true);
  const faces = view.getUint32(16, true);
  const corners = view.getUint32(20, true);
  const edges = view.getUint32(24, true);
  const low = [0, 1, 2].map((i) => view.getFloat32(28 + 4 * i, true));
  const high = [0, 1, 2].map((i) => view.getFloat32(40 + 4 * i, true));
  let offset = 52;
  const quantized = new Uint16Array(buffer.slice(offset, offset + 6 * vertices));
  offset += 6 * vertices;
  const sizes = new Uint8Array(buffer, offset, faces);
  offset += faces;
  offset += (4 - (offset % 4)) % 4;
  const indices = short
    ? new Uint16Array(buffer.slice(offset, offset + 2 * corners))
    : new Uint32Array(buffer.slice(offset, offset + 4 * corners));
  offset += (short ? 2 : 4) * corners;
  // The valence of each pole (0 elsewhere), and what the loop of each edge does.
  const valences = new Uint8Array(buffer, offset, vertices);
  const loops = new Uint8Array(buffer, offset + vertices, edges);
  const positions = new Float32Array(3 * vertices);
  for (let v = 0; v < vertices; v++) {
    for (let axis = 0; axis < 3; axis++) {
      const span = high[axis] - low[axis] || 1;
      positions[3 * v + axis] = low[axis] + (quantized[3 * v + axis] / 0xffff) * span;
    }
  }
  return { positions, sizes, indices, faces, valences, loops };
}

// The codes of the loop of an edge in an .ocqm (scripts/viewer_meshes.py).
const CLOSED = 1;
const SPIRAL = 2;

// A pole's mark, its valence told by its shape as well as said in the
// legend: a disc for 3, a ring for 5, a square for the others (2, 6 and more).
const SHAPE = (valence) => (valence === 3 ? "disc" : valence === 5 ? "ring" : "square");
function mark(THREE, shape) {
  const canvas = document.createElement("canvas");
  canvas.width = canvas.height = 32;
  const context = canvas.getContext("2d");
  context.fillStyle = context.strokeStyle = "#ffffff";
  context.lineWidth = 7;
  context.beginPath();
  if (shape === "square") context.rect(5, 5, 22, 22);
  else context.arc(16, 16, shape === "ring" ? 11.5 : 13, 0, 2 * Math.PI);
  if (shape === "ring") context.stroke();
  else context.fill();
  return new THREE.CanvasTexture(canvas);
}

// Triangles for the shading (a fan in each face), the edges of the faces for
// the lines (the quads are drawn, not their triangles), those of the loops
// that close or spiral, and the poles by shape. The edges come in the order of
// the format: as first met going round the faces.
function build(THREE, mesh, up) {
  const triangles = [];
  const edges = [];
  const closed = [];
  const spiral = [];
  const seen = new Set();
  const count = mesh.positions.length / 3;
  let corner = 0;
  for (let f = 0; f < mesh.faces; f++) {
    const size = mesh.sizes[f];
    const face = mesh.indices.subarray(corner, corner + size);
    corner += size;
    for (let i = 1; i + 1 < size; i++) triangles.push(face[0], face[i], face[i + 1]);
    for (let i = 0; i < size; i++) {
      const a = face[i];
      const b = face[(i + 1) % size];
      const key = a < b ? a * count + b : b * count + a;
      if (!seen.has(key)) {
        const loop = mesh.loops[seen.size];
        seen.add(key);
        edges.push(a, b);
        if (loop === CLOSED) closed.push(a, b);
        if (loop === SPIRAL) spiral.push(a, b);
      }
    }
  }
  const surface = new THREE.BufferGeometry();
  surface.setAttribute("position", new THREE.BufferAttribute(mesh.positions, 3));
  surface.setIndex(triangles);
  // A y-up model stands as Blender stands it for the pictures: z up.
  if (up === "y") surface.rotateX(Math.PI / 2);
  const position = surface.getAttribute("position");
  const segments = (pairs) => new THREE.BufferGeometry().setAttribute("position", position).setIndex(pairs);
  const poles = {};
  mesh.valences.forEach((valence, v) => {
    if (valence) (poles[SHAPE(valence)] ||= []).push(v);
  });
  const points = Object.fromEntries(Object.entries(poles).map(([shape, list]) => [shape, segments(list)]));
  return { surface, lines: segments(edges), closed: segments(closed), spiral: segments(spiral), points };
}

async function start(root) {
  const words = WORDS[root.lang] || WORDS.en;
  // One model (data-index, on its page) or several to pick from (data-models
  // and data-base, the folder of the meshes, on the home and results pages).
  const models = root.dataset.models ? root.dataset.models.split(" ") : [null];
  const indexOf = (model) =>
    model ? new URL(`${model}/index.json`, new URL(root.dataset.base, document.baseURI)) : new URL(root.dataset.index, document.baseURI);
  root.replaceChildren();
  const status = document.createElement("p");
  status.className = "viewer-status";
  status.textContent = words.loading;
  root.append(status);

  const [THREE, { OrbitControls }] = await Promise.all([
    import("./three/three.module.min.js"),
    import("./three/OrbitControls.js"),
  ]);

  const bar = document.createElement("div");
  bar.className = "viewer-bar";
  // The choices: the model where there are several, the remesher, and the
  // number of quads asked of it (none for a mesh modelled by hand, the same
  // at every budget).
  const chooser = (text) => {
    const label = document.createElement("label");
    label.textContent = text + " ";
    const select = document.createElement("select");
    label.append(select);
    return [label, select];
  };
  const fill = (select, values, name) => {
    const kept = select.value;
    select.replaceChildren(...values.map((value) => Object.assign(document.createElement("option"), { value: String(value), textContent: name(value) })));
    if (values.map(String).includes(kept)) select.value = kept;
  };
  const number = (n) => n.toLocaleString(root.lang).replace(/ | /g, " ");
  const [modelLabel, modelSelect] = chooser(words.model);
  fill(modelSelect, models, (m) => m);
  const first = models.includes(root.dataset.first) ? root.dataset.first : models[0];
  modelSelect.value = String(first);
  const [remesherLabel, remesherSelect] = chooser(words.remesher);
  const [budgetLabel, budgetSelect] = chooser(words.budget);
  const reset = document.createElement("button");
  reset.type = "button";
  reset.className = "button";
  reset.textContent = words.reset;
  if (models[0]) bar.append(modelLabel);
  bar.append(remesherLabel, budgetLabel, reset);

  const stage = document.createElement("div");
  stage.className = "viewer-stage";
  let renderer;
  try {
    renderer = new THREE.WebGLRenderer({ antialias: true });
  } catch {
    status.textContent = words.webgl;
    return;
  }
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  renderer.domElement.setAttribute("role", "img");
  renderer.domElement.setAttribute("aria-label", words.canvas);
  stage.append(renderer.domElement);

  // The layers drawn over the mesh, each box its legend and its count.
  const layers = document.createElement("div");
  layers.className = "viewer-layers";
  const layer = (name) => {
    const label = document.createElement("label");
    const box = Object.assign(document.createElement("input"), { type: "checkbox", checked: true });
    const text = document.createElement("span");
    label.append(box, text);
    label.dataset.layer = name;
    layers.append(label);
    box.addEventListener("change", () => {
      if (shown) for (const object of shown.children) if (object.userData.layer === name) object.visible = box.checked;
      draw();
    });
    return { box, text };
  };
  const toggles = { poles: layer("poles"), closed: layer("closed"), spiral: layer("spiral") };
  const info = document.createElement("p");
  info.className = "viewer-info";
  const help = document.createElement("p");
  help.className = "viewer-help";
  help.textContent = words.help;
  status.remove();
  root.append(bar, stage, layers, info, help);

  const style = getComputedStyle(root);
  const scene = new THREE.Scene();
  scene.background = new THREE.Color(style.getPropertyValue("--viewer-bg").trim() || "#ffffff");
  const camera = new THREE.PerspectiveCamera(35, 1, 0.01, 100);
  camera.up.set(0, 0, 1);
  scene.add(camera);
  scene.add(new THREE.HemisphereLight(0xffffff, 0x8796a8, 1.6));
  const light = new THREE.DirectionalLight(0xffffff, 1.8);
  light.position.set(-1, 2, 1);
  camera.add(light);
  const controls = new OrbitControls(camera, renderer.domElement);
  controls.enableDamping = false;

  const surfaceMaterial = new THREE.MeshStandardMaterial({
    color: style.getPropertyValue("--viewer-mesh").trim() || "#bbcbdc",
    flatShading: true,
    roughness: 0.85,
    metalness: 0,
    polygonOffset: true,
    polygonOffsetFactor: 1,
    polygonOffsetUnits: 1,
  });
  const lineMaterial = new THREE.LineBasicMaterial({ color: style.getPropertyValue("--viewer-edge").trim() || "#101828" });
  // Teal what closes, vermillion what breaks the regularity: the colours of
  // the site. They are drawn after the edges they lie on, and cover them.
  const teal = style.getPropertyValue("--viewer-closed").trim() || "#0A8F8A";
  const hot = style.getPropertyValue("--viewer-hot").trim() || "#D4572A";
  const loopMaterials = {
    closed: new THREE.LineBasicMaterial({ color: teal }),
    spiral: new THREE.LineBasicMaterial({ color: hot }),
  };
  const poleMaterials = Object.fromEntries(
    ["disc", "ring", "square"].map((shape) => [
      shape,
      new THREE.PointsMaterial({ color: hot, map: mark(THREE, shape), size: 11, sizeAttenuation: false, alphaTest: 0.5 }),
    ]),
  );
  let index = null;
  let indexUrl = null;
  let view = null;
  let towards = null;
  let shown = null;
  let home = null;

  const draw = () => renderer.render(scene, camera);
  controls.addEventListener("change", draw);

  const resize = () => {
    const width = stage.clientWidth;
    const height = stage.clientHeight;
    renderer.setSize(width, height, false);
    camera.aspect = width / height;
    camera.updateProjectionMatrix();
    draw();
  };
  new ResizeObserver(resize).observe(stage);

  const frame = (sphere) => {
    const distance = sphere.radius / Math.sin(THREE.MathUtils.degToRad(camera.fov / 2)) * 1.05;
    camera.near = distance / 100;
    camera.far = distance * 10;
    camera.position.copy(sphere.center).addScaledVector(towards, distance);
    camera.updateProjectionMatrix();
    controls.target.copy(sphere.center);
    controls.update();
    draw();
  };

  const cache = new Map();
  const show = async () => {
    const remesher = remesherSelect.value;
    const own = index.meshes.filter((m) => m.remesher === remesher);
    // Only the budgets this remesher has; a mesh by hand has none.
    for (const option of budgetSelect.options) option.disabled = !own.some((m) => m.budget === Number(option.value));
    budgetSelect.disabled = own.every((m) => m.budget === null);
    let entry = own.find((m) => m.budget === Number(budgetSelect.value));
    if (!entry) {
      entry = own[0];
      if (entry.budget) budgetSelect.value = String(entry.budget);
    }
    info.textContent = words.loading;
    try {
      const url = new URL(entry.file, indexUrl).href;
      let geometry = cache.get(url);
      if (!geometry) {
        const response = await fetch(url);
        if (!response.ok) throw new Error(response.statusText);
        geometry = build(THREE, decode(await response.arrayBuffer()), view.up);
        cache.set(url, geometry);
      }
      if (shown) scene.remove(shown);
      shown = new THREE.Group();
      shown.add(new THREE.Mesh(geometry.surface, surfaceMaterial), new THREE.LineSegments(geometry.lines, lineMaterial));
      const over = (object, name) => {
        object.userData.layer = name;
        object.renderOrder = 1;
        object.visible = toggles[name].box.checked;
        shown.add(object);
      };
      for (const name of ["closed", "spiral"]) over(new THREE.LineSegments(geometry[name], loopMaterials[name]), name);
      for (const [shape, points] of Object.entries(geometry.points)) over(new THREE.Points(points, poleMaterials[shape]), "poles");
      scene.add(shown);
      geometry.surface.computeBoundingSphere();
      if (!home) {
        // The same view for every remesher of a model, so that they compare.
        home = geometry.surface.boundingSphere.clone();
        frame(home);
      }
      const count = (n, [one, many]) => words.count(n, one, many);
      const size = entry.quads === entry.faces ? count(entry.quads, words.quads) : `${count(entry.faces, words.faces)} (${count(entry.quads, words.quads)})`;
      info.textContent = `${size} · ${count(entry.vertices, words.vertices)} · ${words.run(entry.run)}`;
      const valences = Object.entries(entry.poles);
      const poles = valences.reduce((sum, [, n]) => sum + n, 0);
      toggles.poles.text.replaceChildren(
        count(poles, words.poles),
        ...valences.map(([valence, n]) =>
          Object.assign(document.createElement("span"), { className: `viewer-pole ${SHAPE(Number(valence))}`, textContent: `${number(n)} ×${valence}` }),
        ),
      );
      // A mesh that is not a manifold has no loops measured: no boxes for them.
      for (const name of ["closed", "spiral"]) {
        toggles[name].box.parentElement.hidden = !entry.loops;
        if (entry.loops) toggles[name].text.textContent = count(entry.loops[name], words[name]);
      }
      draw();
    } catch (error) {
      info.textContent = words.failed;
    }
  };

  const load = async (model) => {
    info.textContent = words.loading;
    indexUrl = indexOf(model);
    const response = await fetch(indexUrl);
    if (!response.ok) throw new Error(response.statusText);
    index = await response.json();
    // Where the camera of the pictures stands (scripts/render.py), z up.
    view = index.view || { up: "z", azimuth: -60, elevation: 20 };
    const azimuth = THREE.MathUtils.degToRad(view.azimuth);
    const elevation = THREE.MathUtils.degToRad(view.elevation);
    towards = new THREE.Vector3(Math.cos(elevation) * Math.cos(azimuth), Math.cos(elevation) * Math.sin(azimuth), Math.sin(elevation));
    fill(remesherSelect, [...new Set(index.meshes.map((m) => m.remesher))], (r) => (r === "handmade" ? words.handmade : NAMES[r] || r));
    const budgets = [...new Set(index.meshes.filter((m) => m.budget).map((m) => m.budget))].sort((a, b) => a - b);
    const first = !budgetSelect.options.length;
    fill(budgetSelect, budgets, number);
    if (first && budgets.includes(3000)) budgetSelect.value = "3000";
    home = null;
    await show();
  };

  modelSelect.addEventListener("change", () => load(modelSelect.value).catch(() => (info.textContent = words.failed)));
  remesherSelect.addEventListener("change", show);
  budgetSelect.addEventListener("change", show);
  reset.addEventListener("click", () => home && frame(home));
  resize();
  await load(first);
}

for (const root of document.querySelectorAll(".viewer[data-index], .viewer[data-models]")) {
  const words = WORDS[root.lang] || WORDS.en;
  const button = document.createElement("button");
  button.type = "button";
  button.className = "button primary";
  button.textContent = words.open;
  button.addEventListener("click", () =>
    start(root).catch(() => {
      root.replaceChildren(Object.assign(document.createElement("p"), { className: "viewer-status", textContent: words.failed }));
    }),
  );
  root.append(button);
}
