## Update

Found some solutions and documentation:

- https://github.com/rust-lang/cargo/issues/8366#issuecomment-653091992
- https://doc.rust-lang.org/cargo/reference/features.html

## The Issue

Found some weird quirk of cargo that's breaking one of my projects.
The `default-features` flag seems to be ignored when running binaries in a workspace folder / outside of a package folder.

Explanation of the setup:

- `example_lib`
  - Defines features "a" and "b", with "a" included in the default features list.
  - Exports a function named test which calls `println` for each enabled feature.
- `example_bin` depends on `example_lib`, with default_features set to false and feature "b" enabled.

Using `cargo run` within the workspace root:

```
feature "a" is enabled!
feature "b" is enabled!
```

Using `cargo run` within the example_bin folder:

```
feature "b" is enabled!
```

Full output:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/example_bin`
feature "a" is enabled!
feature "b" is enabled!
$ cd example_bin
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/konst/Documents/Projects/Rust/features_test/target/debug/example_bin`
feature "b" is enabled!
```
