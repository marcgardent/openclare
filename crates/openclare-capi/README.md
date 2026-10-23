# openclare C library

The engine of [openclare](https://github.com/marcgardent/openclare), automatic quad retopology,
as a shared library with a C interface: a triangle mesh goes in as flat arrays, a quad mesh comes
out. The Blender add-on calls the same library.

```
lib/libopenclare_capi.so         Linux x86-64, glibc 2.28 or later
lib/libopenclare_capi.dylib      macOS 11 or later (arm64 or x86-64, one per archive)
lib/openclare_capi.dll           Windows x86-64
lib/openclare_capi.dll.lib       its import library, for MSVC
include/openclare.h              every function and type, with who frees what
LICENSE                          GPL-3.0-or-later
```

The version of the library is in the name of the archive, and `openclare_version()` returns it.

## Linking

```sh
# Linux
cc -I include app.c -L lib -lopenclare_capi -Wl,-rpath,'$ORIGIN/lib' -o app
# macOS (the library is found through the rpath: @rpath/libopenclare_capi.dylib)
cc -I include app.c -L lib -lopenclare_capi -Wl,-rpath,@executable_path/lib -o app
```

```bat
rem Windows, MSVC: openclare_capi.dll goes beside app.exe, or on the PATH
cl /I include app.c lib\openclare_capi.dll.lib
```

From another language, load the library and declare the functions as `openclare.h` does
(Python's ctypes, for one: that is what the Blender add-on does).

## Calling it

```c
OpenclareResult *result = openclare_remesh(
    positions, vertex_count, triangles, triangle_count,
    NULL, 0,                    /* triangles hold vertex indices */
    NULL,                       /* no painted density */
    NULL,                       /* no groups of triangles */
    NULL, NULL, NULL, 0,        /* no guide curve */
    "organic",                  /* preset: organic, animation, hard-surface */
    "quads = 3000\n",           /* settings on top of it */
    NULL, NULL);                /* nobody follows the run */
if (openclare_result_ok(result)) {
    float *out_positions = malloc(3 * openclare_result_vertex_count(result) * sizeof(float));
    uint32_t *sizes = malloc(openclare_result_face_count(result) * sizeof(uint32_t));
    uint32_t *corners = malloc(openclare_result_corner_count(result) * sizeof(uint32_t));
    openclare_result_copy_positions(result, out_positions);
    openclare_result_copy_faces(result, sizes, corners);
    /* ... */
} else {
    puts(openclare_result_error(result));
}
openclare_result_free(result);
```

Who frees what:

- What you pass in stays yours: the library reads it during the call and keeps nothing.
- `openclare_remesh`, `openclare_bake_height`, `openclare_analyse` and `openclare_hard_edges`
  never return NULL. Each handle they return is freed once, by `openclare_result_free`,
  `openclare_baked_free`, `openclare_analysis_free` or `openclare_hard_edges_free`, whether it
  holds what was asked for or an error.
- The `copy` functions write into arrays you allocate, of the size the `count` functions give.
- Texts returned by the library are its own: never free them. Errors and reports live until
  their handle is freed; the version for ever.

A failure inside the library comes back as the error of the handle; nothing unwinds into the
caller.

## On macOS

The library is signed ad hoc, not notarized. A copy downloaded with a browser is put in
quarantine by Gatekeeper, which then refuses to load it; take it out of quarantine once:

```sh
xattr -d -r com.apple.quarantine openclare-capi-*
```

## Licence

GPL-3.0-or-later: a program that links to the library is a work based on it, to be distributed
under a compatible licence.
