# mdbook-selfpath

A Rust preprocessor for [mdBook](https://github.com/rust-lang/mdBook) that allows you to insert the current file path into your book's Markdown content. This is useful for referencing the source file location within your documentation, tutorials, or codebooks.

## Features

- **Self Path Injection**: Insert the current Markdown file's path using a custom syntax or directive.
- **Easy Integration**: Works as an mdBook preprocessor, so it fits seamlessly into your existing mdBook workflow.
- **Configurable**: Can be enabled or disabled per book or per file.

## Usage

### 1. Install the Preprocessor

```sh
cargo install mdbook-selfpath
```

This will produce the binary in `target/release/mdbook-filepath`.

### 2. Register the Preprocessor in `book.toml`

Add the following to your `book.toml`:

```toml
[preprocessor.selfpath]
```

Here is the configuration available:

- `include-file-ext` if true, file extension will be included (i.e. `.md`)

### 3. Use in Your Markdown

Insert a directive in your Markdown file where you want the file path to appear. For example:

```handlebars
[Feedback](https://github.com/howlowck/example-repo/files/blob/main/{{ selfpath }})
```

The output will be

```md
[Feedback](https://github.com/howlowck/example-repo/files/blob/main/src/intro.md)
```

When you build your book, this will be replaced with the relative path to the current Markdown file.

### 4. Build Your Book

```sh
mdbook build
```

## Development

- Source code is in `src/`.
- Example book for testing is in `testbook/`.
- Use `make` or the provided `makefile` for common tasks (if available).

## License

This project is licensed under the MIT License.

## Acknowledgements

- [mdBook](https://github.com/rust-lang/mdBook)
- [Rust](https://www.rust-lang.org/)
