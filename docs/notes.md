- Rust macro
- Using a ! means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.
- Rust is an ahead-of-time compiled language

- The file is in the [TOML](https://toml.io/en/) (Tom’s Obvious, Minimal Language) format, which is Cargo’s configuration format.
- Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There’s a place for everything, and everything is in its place.

- By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude.
- In Rust, variables are immutable by default