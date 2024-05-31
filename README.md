# Roguelike

This project is an ongoing 2D turn-based roguelike game that is still in its
early stages of development. The scope is not well-defined at the moment, but I
view it as a game prototype, serving as an opportunity to enhance my skills in
Rust, Bevy, and game development in general.

<img src="https://boreec.github.io/img/blog/devlog/roguelike-0015.png" width=400/>

# Devlog

This project development is covered by a written devlog with the following
entries:

1. 29/12/2023: [devlog #0 - A Roguelike for 2024 ?](https://boreec.github.io/posts/devlog-0000/)
2. 31/01/2024: [devlog #1 - Procedural Map Generation](https://boreec.github.io/posts/devlog-0001/)
3. 28/02/2024: [devlog #2 - Palette and Tileset update](https://boreec.github.io/posts/devlog-0002/)
4. 29/04/2024: [devlog #3 - Switching between maps](https://boreec.github.io/posts/devlog-0003/)
5. 31/05/2024: [devlog #4 - NPC movements](https://boreec.github.io/posts/devlog-0004/)

# Running the game

## Cloning repository

First of all, clone the repository:

```console
git clone https://github.com/boreec/roguelike.git
```

Move to the cloned repository folder:

```console
cd roguelike
```

## Fetching the assets

Assets are provided in a zip archive aside from the repository. It is available
via the following link: https://mega.nz/folder/h68WgZgS#wVV9Hj5B5O265B3UC4DYpQ

Once downloaded, move `assets.zip` to the current directory and unzip it:

```console
mv path/to/assets.zip .
unzip assets.zip
```

## Building the game

Requirements:

- [Rust](https://www.rust-lang.org/)

The source code can be compiled with cargo (be patient!):

```console
cargo run -r
```

Note: XUbuntu is the only tested system so far, others may require further
adjustments (see Rust and Bevy documentation).

## Unit tests (optional)

You can also run unit tests with:

```console
cargo test
```

# Credits

The following people participated to this project directly or indirectly.

- Font: [GABOED](https://www.dafont.com/gaboed.font), by Greentrik6789
- Palette: [RESURRECT 64](https://lospec.com/palette-list/resurrect-64), by Kerrie Lake
