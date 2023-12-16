# Havoc Resurgence

This project is a work-in-progress 2D turn-based roguelike game, still in the
early stages. The project's scope is not well defined at this point. I consider
it as a prototype for my own amusement.

The game is built with Rust and Bevy.

# Inspirations

- [Dofus](https://en.wikipedia.org/wiki/Dofus)
- [Dwarf Fortress](https://en.wikipedia.org/wiki/Dwarf_Fortress)
- [Final Fantasy Tactics Advance](https://en.wikipedia.org/wiki/Final_Fantasy_Tactics_Advance)
- [Into the Breach](https://en.wikipedia.org/wiki/Into_the_Breach)
- [NetHack](https://en.wikipedia.org/wiki/NetHack)
- [Tales of Maj'Eyal](https://en.wikipedia.org/wiki/Tales_of_Maj%27Eyal)

# Running the game

## Cloning repository

For those not familiar with `git`, you will need to clone the repository:
```console
git clone https://github.com/boreec/havoc-resurgence.git
```

Move to the cloned repository folder and follow the instructions below:
```console
cd havoc-resurgence
```

## Fetching the assets

The repository contains only game code. For the assets, download files from the
following MEGA link: https://mega.nz/folder/h68WgZgS#wVV9Hj5B5O265B3UC4DYpQ

In order to make the assets accessible by the game program, create a folder
`assets/img` and move the downloaded files in that folder.

For example:
```console
mkdir -p assets/img
mv ~/Downloads/tileset.png assets/img
```

## Bulding the program

The game is written in Rust. Make sure to install it and run the following:

```console
cargo run -r
```

## Unit tests (optional)

You can also run unit tests with:

```console
cargo test
```
