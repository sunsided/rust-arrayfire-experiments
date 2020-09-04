# ArrayFire Experiments in Rust

Toying around with / getting to know [arrayfire-rust](https://github.com/arrayfire/arrayfire-rust).
Don't expect anything crazy here. 🙌

## Example applications

- [trivial.rs](src/trivial.rs): The simple example from the `arrayfire-rust` README:
  ```bash
  cargo run --bin trivial
  ```

- [conway.rs](src/conway.rs): [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) taken from the `arrayfire-rust` examples.
  Uses ArrayFire's graphics module to provide a window.
  ```bash
  cargo run --bin conway
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