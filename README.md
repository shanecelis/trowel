# Trowel

This is a sister project to the [spade Sprig
engine](https://github.com/hackclub/spade). It is a sprig firmware
implementation in rust.

## Install rust

Follow [these instructions](https://www.rust-lang.org/tools/install) to install
rust on your computer.

## Setup rust

``` sh
$ rustup target add thumbv6m-none-eabi
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
$ cargo install elf2uf2-rs
$ cargo install probe-run
```

These need to be tested.

## Setup probe

TODO: Fill in.

The probe needs to have [this
uf2](https://github.com/raspberrypi/picoprobe/releases/latest/download/picoprobe.uf2)
installed. See [this
page](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html)
and the [Getting Started
Guide](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf)
for more information.

## Simulate on your PC

![maze on the pc simulator](/assets/maze_pc.png)

``` sh
cargo run --example maze
```

## Run on your sprig

![maze on the sprig](/assets/maze_sprig.png)

``` sh
cargo run --example maze --target thumbv6m-none-eabi
```

## Examples

* hello_world
* maze
* draw_ferris

## Notes

See the `.cargo/config.toml` for various build settings. You can generate uf2
files, write directly to the sprig if it's in BOOTSEL mode, or you can use a
debug probe.

