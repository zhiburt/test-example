# test-example

```bash
$ cargo test-example README.md 
"./README.md" 0 "   Compiling rust-example v0.1.0 (/tmp/.tmpmthIl6/rust-example)\n     Running `rustc --crate-name rust_example --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=54392270b532f757 -C extra-filename=-54392270b532f757 --out-dir /tmp/.tmpmthIl6/rust-example/target/debug/deps -C incremental=/tmp/.tmpmthIl6/rust-example/target/debug/incremental -L dependency=/tmp/.tmpmthIl6/rust-example/target/debug/deps`\n    Finished dev [unoptimized + debuginfo] target(s) in 0.24s\n       Fresh rust-example v0.1.0 (/tmp/.tmpmthIl6/rust-example)\n    Finished dev [unoptimized + debuginfo] target(s) in 0.25s\n"
```

##### TODO:

- Add running option (fork issue)
- Add status field (success/failure)
- Return file line/column information

##### Example

It's used in a usage section.

```rust,no_run
fn main() {
    println!("Hello World!");
}
```