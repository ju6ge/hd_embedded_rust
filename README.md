# `HD Embedded SoC, Rust environment`

This project is a test how far i can get in running an Embedded Soc, that was created during a university class, with rust only code.

Warning all of this is experimental in nature and not intendet for production use! But since this is a very custom SoC that was specific
to a university course the is not really any chance for that.

## Dependencies

To build embedded programs you'll need:

- currently you will need to use the nightly toolchain in order for this code to work since inline asm is not yet supported in stable

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```
- `openocd` to be able to debug the code on the SoC and to be able to flash the final program for usage

- The arm-none-eabi toolchain in particlar the `arm-none-eabi-gdb` for debbuing 

# Building and Debugging 

To comfortably test your code start the openocd server:

``` console
$ openocd -f openocd.cfg
```

Now running `cargo run` will result in a gdb session being attached and the newly compiled code to be loaded. 

# License

This template is licensed under

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

