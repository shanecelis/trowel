# Trowel

This is a sister project to the [spade Sprig
engine](https://github.com/hackclub/spade). It is a sprig firmware
implementation in rust.

## Setup rust

``` sh

```

## Simulate on your PC

``` sh
cargo run --example raycaster
```

## Run on your sprig

``` sh
cargo run --example raycaster --target thumbv6m-none-eabi
```

See the `.cargo/config.toml` for various build settings. You can generate uf2
files, write directly to the sprig if it's in BOOTSEL mode, or you can use a
pico probe.

