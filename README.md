# read-rs

A simple Rust command-line tool to read and display the contents of a text file like `cat`.
Supports **UTF-8 text files only**.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Shaurya-Sethi/read-rs.git
   cd read-rs
   ```

2. Install the tool globally using Cargo:
   ```bash
   cargo install --path .
   ```

   This will make `read-rs` available in your system's PATH.

## Usage

Once installed, run the tool with a file path:

```bash
read-rs <FILE_PATH>
```

For development or local testing:

```bash
cargo run -- <FILE_PATH>
```

Or run the built binary directly:

```bash
./target/debug/read-rs <FILE_PATH>
```

### Help and Version

The tool automatically provides help and version information:

```bash
read-rs --help
read-rs --version
```

Example:

```bash
read-rs fixtures/hello.txt
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