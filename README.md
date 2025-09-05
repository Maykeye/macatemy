# Magical cats academy

Hobby project not intended to ever be finished (as everything I dump to githubs)

## Controls:

- `Left Ctrl` + move mouse around changes the camera input
- `Alt`+`Q` quits the game

## Running tests

- `cargo test` will not work as it reuses the same process (see https://github.com/bevyengine/bevy/discussions/20843). For testing use nextest, i.e.
`cargo nextest run` instead
