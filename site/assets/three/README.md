# three.js, for the 3D viewer of the website

- `three.module.min.js` and `OrbitControls.js` from the npm package `three@0.169.0`
  (sha1 of the tarball `4a62114988ad9728d73526d1f1de6760c56b4adc`, as listed by the registry),
  MIT licence, in `LICENSE`.
- The one change: `OrbitControls.js` imports `./three.module.min.js` instead of `three`, so that
  the pages need no import map.
- Served by the site itself, with no CDN; loaded only when a visitor asks for the 3D view.
