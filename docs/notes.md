- Rust macro
- Using a ! means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.
- Rust is an ahead-of-time compiled language

- The file is in the [TOML](https://toml.io/en/) (Tom’s Obvious, Minimal Language) format, which is Cargo’s configuration format.
- Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There’s a place for everything, and everything is in its place.

- By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude.
- In Rust, variables are immutable by default


```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

This expression:

```rust
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, evaluates to 4. That value gets bound to y as part of the let statement. Note that the x + 1 line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value


`Remember`: Statements don’t evaluate to a value.