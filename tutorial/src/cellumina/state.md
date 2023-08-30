# Initializing State


## Representation

In Cellumina, the **state** of an automaton is represented by grid of characters. This has both advantages and disadvantages:

Advantages:
 * Converting to and from text files is very simple, allowing users to simply load in .txt documents.
 * Writing rules can also directly depend on easy-to-understand characters instead of anonymous numbers.
Disadvantages:
 * A ```char``` takes up 4 bytes of space compared to ```u8``` at 1 byte, but for most automat 128 different states are more than enough.
 * Arithmetic operations in rules become much more difficult.

 For these reasons, a conversion of the state grid to a ```Grid<u8>``` is planned.

## Initialization

The ```AutomatonBuilder``` provides a variety of different ways to supply an automaton with an initial state grid.

### By Hand

As mentioned, the state grid is internally represented as ```Grid<char>```.
It is therefore possible to create a char grid with the methods provided by the [grid library] and pass this to the automaton using ```from_grid```:

[grid library]: https://docs.rs/grid/latest/grid.html

```rust,noplayground
    cellumina::AutomatonBuilder::new()
        .from_grid(
            Grid::from_vec(
                vec![
                    '0', '1', '1',
                    '1', '0', '0',
                    '1', '0', '0',
                    '0', '1', '1',
                ], 3)
        )
        .build();
```

If you are initializing your grid from a ```vec```, there is a shortcut via ```from_vec```:

```rust,noplayground
    cellumina::AutomatonBuilder::new()
        .from_vec(
            vec![
                '0', '1', '1',
                '1', '0', '0',
                '1', '0', '0',
                '0', '1', '1',
            ], 3)
        .build();
```

### From a text file

Typing these characters explicitly inside the code file is somewhat cumbersome.
In order to take a more data-driven approach, automaton states can be directly initialized from a text file with ```from_text_file```:

```rust,noplayground
    cellumina::AutomatonBuilder::new()
        .from_text_file(
            "./example.txt"
        )
        .build();
```

where the argument -- in this case ```"./example.txt"``` -- anything implementing the ```AsRef<Path>``` trait. 
Usually this will be a string containing the relative path to a file, but you may choose to get a path from a file picker or similar.
The state of your automaton will have one row for each line in the text file, and as many columns as the longest had characters.
Newlines are of course discarded, and shorter lines are filled with spaces.

### From an image

