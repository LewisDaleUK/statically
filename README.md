# Statically

An extremely simple Static Site Generator, written in Rust.

Very much a work-in-progress, and _heavily_ inspired by (read: copied indiscriminately) [Eleventy](https://github.com/11ty/eleventy/).

## Running

`cargo run`

## Building

`cargo build --profile release`

The binary has a few optimisations to get the size down - right now it's still close to 2mb on an M1 Macbook. I've kept the optimisation profile for run speed though, it only saved ~400mb, and I value speed more.

## Running tests

`cargo test`
