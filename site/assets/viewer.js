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
    faces: (n) => `${n.toLocaleString("en")} faces`,
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
    faces: (n) => `${n.toLocaleString("fr").replace(/ | /g, " ")} faces`,
    run: (date) => `passage du banc du ${date}`,
    reset: "Revenir à la vue de départ",
    help: "Glisser pour tourner, molette ou pincement pour zoomer, clic droit ou deux doigts pour déplacer.",
    loading: "Chargement…",
    failed: "Le maillage n'a pas pu être chargé.",
    webgl: "Ce navigateur ne peut pas afficher la 3D : WebGL est désactivé ou absent.",
    canvas: "Le modèle remaillé en 3D",
  },
};

const NAMES = { openclare: "openclare", quadriflow: "QuadriFlow (Blender)", autoremesher: "AutoRemesher" };

function decode(buffer) {
  const view = new DataView(buffer);
  const magic = String.fromCharCode(...new Uint8Array(buffer, 0, 4));
  if (magic !== "OCQM" || view.getUint32(4, true) !== 1) throw new Error("not an .ocqm mesh");
  const short = (view.getUint32(8, true) & 1) === 1;
  const vertices = view.getUint32(12, true);
  const faces = view.getUint32(16, true);
  const corners = view.getUint32(20, true);
  const low = [0, 1, 2].map((i) => view.getFloat32(24 + 4 * i, true));
  const high = [0, 1, 2].map((i) => view.getFloat32(36 + 4 * i, true));
  let offset = 48;
  const quantized = new Uint16Array(buffer.slice(offset, offset + 6 * vertices));
  offset += 6 * vertices;
  const sizes = new Uint8Array(buffer, offset, faces);
  offset += faces;
  offset += (4 - (offset % 4)) % 4;
  const indices = short
    ? new Uint16Array(buffer.slice(offset, offset + 2 * corners))
    : new Uint32Array(buffer.slice(offset, offset + 4 * corners));
  const positions = new Float32Array(3 * vertices);
  for (let v = 0; v < vertices; v++) {
    for (let axis = 0; axis < 3; axis++) {
      const span = high[axis] - low[axis] || 1;
      positions[3 * v + axis] = low[axis] + (quantized[3 * v + axis] / 0xffff) * span;
    }
  }
  return { positions, sizes, indices, faces };
}

// Triangles for the shading (a fan in each face), and the edges of the faces
// for the lines: the quads are drawn, not their triangles.
function build(THREE, mesh, up) {
  const triangles = [];
  const edges = [];
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
        seen.add(key);
        edges.push(a, b);
      }
    }
  }
  const surface = new THREE.BufferGeometry();
  surface.setAttribute("position", new THREE.BufferAttribute(mesh.positions, 3));
  surface.setIndex(triangles);
  // A y-up model stands as Blender stands it for the pictures: z up.
  if (up === "y") surface.rotateX(Math.PI / 2);
  const lines = new THREE.BufferGeometry();
  lines.setAttribute("position", surface.getAttribute("position"));
  lines.setIndex(edges);
  return { surface, lines };
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

  const info = document.createElement("p");
  info.className = "viewer-info";
  const help = document.createElement("p");
  help.className = "viewer-help";
  help.textContent = words.help;
  status.remove();
  root.append(bar, stage, info, help);

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
      scene.add(shown);
      geometry.surface.computeBoundingSphere();
      if (!home) {
        // The same view for every remesher of a model, so that they compare.
        home = geometry.surface.boundingSphere.clone();
        frame(home);
      }
      info.textContent = `${words.faces(entry.faces)} · ${words.run(entry.run)}`;
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
