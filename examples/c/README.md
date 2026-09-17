# Wickra Radar — C / C++ examples

The Wickra Radar C ABI is a single shared/static library plus a generated header
([`bindings/c/include/wickra_radar.h`](../../bindings/c/include/wickra_radar.h)). Any C-capable
language links against the same artifact; these examples show the plain-C path
and, through [`wickra_radar.hpp`](../../bindings/c/include/wickra_radar.hpp), the C++ one.

## Build the library

From the workspace root:

```sh
cargo build -p wickra-radar-c --release
```

This produces, in `target/release/`:

| Platform | Shared library | Link target |
|----------|----------------|-------------|
| Linux    | `libwickra_radar.so`     | `-lwickra_radar` |
| macOS    | `libwickra_radar.dylib`  | `-lwickra_radar` |
| Windows (MSVC) | `wickra_radar.dll` | `wickra_radar.dll.lib` (import lib) |

A static library (`libwickra_radar.a` / `wickra_radar.lib`) is emitted alongside.

## Build and run the examples

### With CMake (portable, used by CI)

```sh
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

### Directly with a compiler

```sh
# Linux / macOS
cc examples/c/scan.c -I bindings/c/include -L target/release -lwickra_radar -lm -o scan
LD_LIBRARY_PATH=target/release ./scan        # macOS: DYLD_LIBRARY_PATH

# Windows (MinGW gcc, linking the DLL directly)
gcc examples/c/scan.c -I bindings/c/include target/release/wickra_radar.dll -lm -o scan.exe
```

## The examples

| Example | What it does |
|---------|--------------|
| `scan.c` | A minimal C example: scan a perp universe through the wickra-radar C ABI. |
| `scan.cpp` | A minimal C++ example: scan a perp universe, then feed the same events one at a time and read the alerts back -- both through the C++ hull. |

## Usage shape

Every call follows the same handle discipline: construct from a spec JSON, drive
with command JSON, read the response, free the handle exactly once. `wickra_radar.h` is
the whole contract; the C++ header, where one ships, wraps the handle in a
move-only RAII type. See [`bindings/c/README.md`](../../bindings/c/README.md).
