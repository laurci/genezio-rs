# genezio-rs

Get Rust running on [Genezio](https://genez.io) as a backend language.

## Getting started!

First, you will need Rust installed :) We will use both `cargo` and `rustup` in the next steps, so I recommend installing Rust using [rustup](https://rustup.rs/). If you have it installed, skip this step.

Next, we will need the `genezio-rs` CLI from this repo. You can install it using `cargo`:
```
cargo install --git http://github.com/laurci/genezio-rs.git cli
```

Now, just run `genezio-rs doctor` and see what it says. If everything is fine, you can start a new project using `genezio-rs new my-project` and start coding! If you have errors, don't worry, just continue reading this section.

## Troubleshooting

### `genezio-rs doctor` says it can't find `rustup`

Go to the previous step and install Rust using `rustup`. If you already have it installed, make sure it's in your `PATH` environment variable.

### `genezio-rs doctor` says it can't find `cargo`

Make sure you have Rust installed. If you already have it installed, make sure `cargo` and the other binaries are in your `PATH` environment variable.

### `genezio-rs doctor` says it can't find target `aarch64-unknown-linux-musl`

Now this is a bit tricky. If you want to know what it means, read [this](#cross-compilation). If you don't care, just run `rustup target add aarch64-unknown-linux-musl` and you should be good to go.

### `genezio-rs doctor` says it can't find toolchain `aarch64-linux-musl-gnu`

This is a bit more tricky. If you want to know what it means, read [this](#cross-compilation). If you don't care, here's how to fix it:

1. You are on MacOS. Install the `aarch64-linux-musl-gnu` toolchain using `brew`. First, you need the tap `brew tap messense/macos-cross-toolchains` and then you can install the toolchain using `brew install aarch64-unknown-linux-musl`.

2. You are on Linux. If you are on Debian/Ubuntu (and friends), install it using `apt` (the package you're looking for is probably `musl-dev`). If you're on other distros, look on the internet, it may be able to help you. If you need to build it from source, you're on your own (but you can still read [this](#cross-compilation)).

3. If you are on Windows, get linux. `genezio-rs` only supports unix systems for now.


### `genezio-rs doctor` says it can't find `genezio`

That's easy. Make sure you have `genezio` installed and in your `PATH` environment variable. You can install it using `npm install -g genezio`. If you don't know what I'm talking about, go to [genezio.io](https://genezio.io) and follow the instructions there.

## Genezio cloud

If you want to deploy your project to the Genezio cloud (if not, why are you reading this?), you will need to have a Genezio Account and be logged in with the Genezio CLI. You can use `genezio login` to log in; [more here] (https://docs.genez.io/genezio-documentation/cli-tool/cli-commands/login). Then check you are logged in using `genezio account`; [more here](https://docs.genez.io/genezio-documentation/cli-tool/cli-commands/account).

## Cross-compilation

Why is the setup so hard? Well, it's because we need to cross-compile our Rust code to run on arm64 linux with musl libc. This is because Genezio runs on AWS Lambdas on ARM. It's very unlikely that the system you're running this on is arm64 linux with musl libc, and that's why we need to cross-compile. This is what cross-compilation means: compiling code for a different architecture than the one you're running on.

For our setup, we need two things: the rust target and the gnu toolchain (we mostly use the toolchain for linking, but it's easy to install the complete thing rather than just the linker).

That's it :) You are now an expert on cross-compilation.

## State of the project

This project is still in its early stages.

You will be missing out on the auto-generated client library for now, but you can still use the API directly. I will be adding support for the JSON-RPC features (including the magic client generation), but for now, you get an HTTP API powered by `axum` at the `/` path of your service Lambda URL.

To get this URL, after deploying your poject, go to the Genezio dashboard and click on your project, then click on the `Test Project` button. You will see the URL there (with some text like `Requests will be sent to https://.....`). It won't change between deploys, so you only need to do this step once.

If you want to help, please do! I'm sure there are a lot of things that can be improved.
