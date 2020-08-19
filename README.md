# sprite_animations_demo

## How to use

First, clone the repo and navigate to the project's directory:

```
git clone https://github.com/tigleym/sprite_animations_demo.git
cd sprite_animations_demo
```

Next, download the spritesheet at https://0x72.itch.io/dungeontileset-ii. Name the file "sprite_sheet.png" and place in the `assets/texture` directory.

To run the game, use

```
cargo run --features "vulkan"
```

on Windows and Linux, and

```
cargo run --features "metal"
```

on macOS.

For building without any graphics backend, you can use

```
cargo run --features "empty"
```

but be aware that as soon as you need any rendering you won't be able to run your game when using
the `empty` feature.
