# lacam3-rust-wrapper

Rust FFI bindings for [Kei18/lacam3](https://github.com/Kei18/lacam3), an
anytime LaCAM3 multi-agent path finding (MAPF) solver written in C++.

The original C++ implementation is vendored under [`lacam3/`](lacam3/) with
small additions (`lacam_ffi_wrappers.{hpp,cpp}` and minor changes to
`planner.{hpp,cpp}`) to expose a stable C++ surface for the Rust bridge.

## Acknowledgement

This crate wraps [Kei18/lacam3](https://github.com/Kei18/lacam3) by AIST,
distributed under the MIT License. The original copyright notice is retained
in [`lacam3/LICENCE.txt`](lacam3/LICENCE.txt) and [`LICENCE.txt`](LICENCE.txt).

## Requirements

- A C++17 toolchain (clang or gcc)
- CMake (>= 3.16)
- Cargo / rustc

## Build

```sh
cargo build --release
```

The build script (`build.rs`) invokes CMake to compile the vendored
`lacam3/` static library, then links it through a `cxx`-generated bridge.

## Usage

```rust
use lacam3_rust_wrapper::{
    LacamGraph, LacamInstance, LacamDeadline,
    configure_planner_via_ffi_defaults, set_flg_star, solve, convert_solution,
};

configure_planner_via_ffi_defaults();
set_flg_star(true);

let graph = LacamGraph::new("path/to/map.map")?;
let starts: Vec<i32> = /* vertex indices */;
let goals: Vec<i32> = /* vertex indices */;

let instance = LacamInstance::new(&graph, &starts, &goals)?;
let deadline = LacamDeadline::new(/* seconds */ 30.0)?;

let raw_solution = solve(instance, /* verbose */ 0, &deadline, /* seed */ 0)?;
let solution_paths = convert_solution(&raw_solution)?;
```

## License

MIT. See [`LICENCE.txt`](LICENCE.txt).
