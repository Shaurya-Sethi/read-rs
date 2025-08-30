# read-rs

A simple Rust command-line tool to read and display the contents of a text file.
Supports **UTF-8 text files only**.

## Usage

```bash
cargo run -- <FILE_PATH>
```

Example:

```bash
cargo run -- fixtures/hello.txt
```

Output:

```
Hello, world!
```

## Behavior

* Prints the file contents to **stdout**.
* Prints errors to **stderr**.
* Exit codes:

  * `0` → success
  * `1` → user error (file not found, not readable, not UTF-8, or path is a directory)
  * `2` → unexpected/internal error

## Limitations

* Only works with **UTF-8 encoded text files**.
* Binary files and non-UTF-8 text are not supported.

## License

MIT