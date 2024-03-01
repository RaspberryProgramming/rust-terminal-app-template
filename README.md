# rust-terminal-app-template
This template is made to show how to make a rust terminal application.
The main purpose of this application is to provide a basic template for writing command line applications.

This template provides logging and cli arguments. The user can enable/disable logging. The program will write a [lorem ipsum](https://www.lipsum.com/) snippet to the designated file.

## Prerequisites

You must install rust to build this project

See [rust-lang](https://www.rust-lang.org/learn/get-started) website

## Building

First clone the project

```bash
git clone https://www.github.com/RaspberryProgramming/rust-terminal-app-template
```

Next, cd into the folder you just cloned

```bash
cd rust-terminal-app-template
```

Use cargo to build the project

```bash
cargo build
```

You can build for release

```bash
cargo build --release
```

To install as a command run

```bash
cargo install --path .
```