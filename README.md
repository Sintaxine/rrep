# rrep

This is a lightweight Rust CLI tool that reads a text file and formats it in the terminal by interpreting some basic Markdown syntax:

- Lines starting with `#` are treated as headers and shown in **red**.
- Words wrapped in `**` are rendered **bold** using terminal formatting.

## Features

- Reads a file line-by-line from user input.
- Parses and formats Markdown-like syntax for headers and bold text.
- Outputs colored and styled text in the terminal using ANSI escape codes.

## How to Use

1. Clone the repository:
    ```bash
    git clone https://github.com/mikudev/rrep.git
    cd markdown-formatter
    ```

2. Build the project:
    ```bash
    cargo build --release
    ```

3. Run the formatter:
    ```bash
    cargo run
    ```

4. When prompted, enter the path to your Markdown file (e.g., `README.md`).

## Example

Given a file `sample.md`:

```md
# This is a header
This is a **bold** statement.
```


The formatter will output:

- The header line in **red**.
- The word wrapped with `**` in **bold**.

## Requirements

- Rust installed (https://rustup.rs/)
- Terminal that supports ANSI colors

## Limitations

- Only supports headers starting with `#` and bold wrapped in `**`.
- No support yet for other Markdown syntax (italic, lists, links, etc.).

## License

MIT License © mikuudev
