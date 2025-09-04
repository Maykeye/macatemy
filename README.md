# Magical cats academy

Hobby project not intended to ever be finished (as everything I dump to githubs)

## Running tests

* `cargo test` will not work as it reuses the same process (see https://github.com/bevyengine/bevy/discussions/20843). For testing use nextest, i.e.
`cargo nextest run` instead
