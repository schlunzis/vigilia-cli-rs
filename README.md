# Vigilia CLI

Vigilia CLI is a command-line interface for the Vigilia service.
The service, which contains the logic for the Vigilia service, is located [here](https://github.com/schlunzis/vigilia).

## Build

To build the Vigilia CLI, you need to have Rust and Cargo installed.

You can install those using `rustup`:

```bash
rustup toolchain install stable
```

Then, you can build the CLI using the following command:

```bash
cargo build
```

This will build the CLI and create an executable in the `target/debug` directory.

## Run

To build and run the CLI, you can use the following command:

```bash
cargo run -- <args>
```

Where `<args>` are the args to pass to the CLI.

## Build DEB-Package

To build a DEB package, you need to have `cargo-deb` installed.
You can install it using the following command:

```bash
cargo install cargo-deb
```

Then, you can build the DEB package using the following command:

```bash
cargo deb
```

This will create a DEB package in the `target/debian` directory.

## Update API Implementation

To update the API implementation, you need to run the `generate.sh` script.
This script will generate the API implementation based on the OpenAPI document.
The OpenAPI document is located in the `vigilia` repository with the service.

After the generation review the generated code and make sure it is correct.

> **Note:** Revert the changes setting the body of the request in `src/apis/default_api.rs` as the generated code
> sends the body as a json not as text/plain.
