# Rust-lang
Tutorial Rust


## Tutorial using Book Rust
https://doc.rust-lang.org/book/

### 1 - Getting started:

1.2 - Hello World

> $ rustc main.rs

>$ ./main

>Hello, world!


1.3 - Hello cargo

> $ cargo --version

Creating a Project with Cargo

> $ cargo new hello_cargo

> $ cd hello_cargo

> $ cargo build

```
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```


> $ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!

Building and Running a Cargo Project


> $ cargo run
```
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

> $ cargo check

```
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Build release:

> cargo build --release