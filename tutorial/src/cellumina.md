# Building Cellular Automata with Cellumina

**Cellumina** is a rust library designed to easily create and run cellular automata of the type we have discussed in the last chapter. 

In the following subchapters, we will discuss how these abstract concepts are implemented in Cellumina, and how you can use the ```AutomatonBuilder``` struct to create your own cellular automata.
A builder can be created as such:

```rust,noplayground
    use cellumina;

    let builder = cellumina::AutomatonBuilder::new();
```

The builder can then be augmented by initializing state and progression rule(s) and setting different configuration options using the [builder pattern].

[builder pattern]: https://rust-unofficial.github.io/patterns/patterns/creational/builder.html

```rust,noplayground
    let builder = cellumina::AutomatonBuilder::new()
        .from_image_file("./example.png")
        .with_rule(my_rule)
        .build();
```

Don't worry about these functions yet, we will go over them in the following chapters in greater detail.

Most of the code examples you will see are excerpts from the [examples].
You can run them by cloning the cellumina repo 
```bash
    git clone https://github.com/Linus-Mussmaecher/cellumina
```
and running 
```bash
    cargo run --examples <name> --features="display"
```
in the created repository. Example:
```bash
    cargo run --examples sand --features="display"
```

[examples]: https://github.com/Linus-Mussmaecher/cellumina/tree/master/examples