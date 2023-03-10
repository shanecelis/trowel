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

## Advanced: Setup probe

If you have an extra pico, you can set one up as a debug probe. It is super
useful. You can run your code through the debugger. You'll get stacktraces and
panic messages when your code fails.

### Requirements

* sprig
* extra pico
* breadboard
* wires

### Load probe firmware

The probe needs to have [this
uf2](https://github.com/raspberrypi/picoprobe/releases/latest/download/picoprobe.uf2)
installed. 

### Wire the probe to the target sprig

See [this
page](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html)
and the [Getting Started
Guide](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf)
for more information.

### Change to use probe-run

Once you have that setup, you can change your runner in the `.cargo/config.toml` file.

``` sh
runner = "probe-run --chip RP2040"
```

## Acknowledgments

* Thanks to [Leo McElroy](https://github.com/leomcelroy), [Cedric
  Hutchings](https://github.com/cedric-h),
  [Kognize](https://github.com/kognise), and the whole Hack Club team for
  creating the Sprig and for making it an open platform in both hardware and
  software.

* Thanks to [Zach Latta](https://zachlatta.com), [Christina
  Asquith](https://christinaasquith.com), and the [Hack Club
  donors](https://hackclub.com/philanthropy/) who have made the Sprig free for
  teenagers, making it an accessible platform.

* Thanks to the hundreds of Hack Club members who have made [hundreds of
  games](https://sprig.hackclub.com/gallery) for the Sprig, making it a vibrant
  platform.

* Thanks to Andrew Christiansen for the rp2040-examples in
  [st7735-lcd-examples](https://github.com/sajattack/st7735-lcd-examples) that
  provided a good basis for this project.

* Thanks to Grant Shandy for this [article on
  raytracing](https://grantshandy.github.io/posts/raycasting/) in rust and the
  [maze
  code](https://github.com/grantshandy/wasm4-raycaster/blob/main/src/lib.rs)
  ported as an example in this project.

