# `tt` (pronounced tiddy) 

A minimal typing test in the terminal. Written in Rust, as usual.

## here it is

![Typing tester](./tiddy.png)

## to run it 

```bash
cargo install --path .
```

Or run directly:

```bash
cargo run
```

## usage

```
Usage: tiddy [COMMAND]

Commands:
  words  Test typing with a specific number of words
  time   Test typing for a specific duration
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
defaults to 15 words

## todo
- [] use monkeytype language lists
- [] quotes
- [] plots? 
- [] build instructions into a proper cli tool with updates
- [] tests on different devices to make sure that it is rendering correctly
- [] use git lfs