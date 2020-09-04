# GPGPU Experiments in Rust with ArrayFire

Toying around with / getting to know [arrayfire-rust](https://github.com/arrayfire/arrayfire-rust).
Don't expect anything crazy here. 🙌

## Example applications

- [trivial.rs](src/trivial.rs): The simple example from the `arrayfire-rust` [README](https://github.com/arrayfire/arrayfire-rust/blob/master/README.md):
  ```bash
  cargo run --bin trivial
  ```

- [conway.rs](src/conway.rs): GPU-enabled [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) through
  kernel convolutions, taken from the `arrayfire-rust` examples with added documentation.
  Uses ArrayFire's graphics module to provide a window.
  ```bash
  cargo run --bin conway
  ```
  
- [half_precision.rs](src/half_precision.rs): FP16 example using [half::f16](https://docs.rs/half/1.6.0/half/).
  Doesn't work on my GTX 980 Ti (Compute 5.2) with CUDA 10, driver 450 so I cannot actually test it. I keeping it as
  a bookmark anyways.
  ```bash
  cargo run --bin half-precision
  ```

## Installing ArrayFire

From the [arrayfire-rust](https://github.com/arrayfire/arrayfire-rust) instructions:

> To use the rust bindings for ArrayFire from crates.io, the following requirements are to be met first.
>
>  1. [Download and install ArrayFire binaries](https://arrayfire.com/download) based on your operating system.
>  2. Set the evironment variable `AF_PATH` to point to ArrayFire installation root folder.
>  3. Make sure to add the path to lib files to your path environment variables.
>      - On Linux: do `export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$AF_PATH/lib64`
>      - On OSX: do `export DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:$AF_PATH/lib64`
>      - On Windows: Add `%AF_PATH%\lib` to your PATH environment variable.
>  4. Add `arrayfire = "3.7"` to the dependencies section of your project's Cargo.toml file. Make sure
>     to change the version to latest available.
>  
>  Once step (4) is over, you should be able to use ArrayFire in your Rust project.

## License

- [ArrayFire](https://arrayfire.com/the-arrayfire-library/) and [arrayfire-rust](https://github.com/arrayfire/arrayfire-rust)
  are licensed under a [BSD 3-Clause License](https://tldrlegal.com/license/bsd-3-clause-license-(revised)).
