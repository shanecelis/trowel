# Trowel

This is a sister project to the [spade Sprig
engine](https://github.com/hackclub/spade). It is a
[Sprig](https://sprig.hackclub.com) firmware implementation in rust.

## Install rust

Follow [these instructions](https://www.rust-lang.org/tools/install) to install
rust on your computer.

## Setup rust for cross platform development

``` sh
$ rustup target add thumbv6m-none-eabi
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
$ cargo install elf2uf2-rs
$ cargo install probe-run
$ cargo install flip-link
```

### Ancillary Dependencies

For the most part `cargo` will handle all the dependencies. However, the rust
`sdl2` crate does require SDL2 to be installed for the PC simulator. Follow
[these instructions](https://crates.io/crates/sdl2) for your operating system to
install SDL2.

## Simulate on your PC

![maze on the pc simulator](/assets/maze-pc.png)

``` sh
cargo run --example maze
```

## Run on your sprig

![maze on the sprig](/assets/maze-sprig.png)

``` sh
cargo run --example maze --target thumbv6m-none-eabi
```

## Examples

* hello_world
* maze
* draw_ferris

## Notes

See the `.cargo/config.toml` for various build and run settings. You can generate uf2
files, write directly to the sprig if it's in BOOTSEL mode, or you can use a
debug probe.

## TODOs

* Add sound support
* Add SD card read support
* Buffer the output to increase responsiveness

> "The framerate is ok on this. But it's the tip of the iceberg. There's a ton
> that can be done to improve it. The screen is the big bottleneck, we can get
> maybe 10 or 20 fps redrawing the whole thing. But the screen also remembers
> everything, so if you don't draw to a part of it, you don't need to update it.
> So if we buffered the output with two framebuffers, we can check them for
> differences. Pretend we had only a 1-bit display, then we could XOR our
> buffers, that'd give us a table of all the pixels that would need to change.
> So some neat opportunities there to make it more performant."

* Add a [runty8](https://github.com/jjant/runty8) adapter

