# Rust Grep

A clone of the grep command using rust.

## Introduciton

The `grep` command matches a `pattern` within the lines of a file and prints the matching lines. In its most basic form, grep takes two arguments: `pattern` and `path`.

The `pattern`, as previously mentioned, is of type String and it is what the computer is told to match within the file.

The `path` must be given as the relative path to the location in which Grep Rust (abbreviated grrs) is saved.

## Example

Here we have a local document stored in the same location as grss rust named `test.txt`. We are searching `test.txt` for the `pattern` "foo"

```bash
cargo run grss -- foo ./text.txt
```
