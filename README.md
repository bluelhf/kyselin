# 💭 Kyselin
Kyselin is a Rust 🦀 command-line tool for studying using self-inflicted quizzes (like Quizlet, but in the terminal 💻)!

Kyselin is heavily inspired by [romeq/kyselija](https://github.com/romeq/kyselija), an awesome Python 🐍 program that achieves the same goal, albeit with a different question format.

## Usage
With [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed:
1. Clone the repository
2. Navigate to its root
3. Run `cargo run --release`
4. Profit!

## Changing the question set
The question set is read from `questions.krs` in the working directory of the executable (if you're following the usage steps, the repository root directory).

The format for `questions.krs` is as follows:
- The file is encoded with UTF-8.
- Each line starts with one of `Q: `, `A: `, or `#: `.
- Lines that start with `Q: ` are **questions**. They should be immediately followed by an **answer**.
- Lines that start with `A: ` are **answers**. They should be immediately _preceded_ by a **question**.
- Lines that start with `#: ` are **comments**. They are additional information that is displayed at the start of the program (e.g. the format of the answers.)
- Multiple answers should be split with `, `. If you want an answer to literally contain `, `, use `\, `.
    - For example, `A: foo, bar` accepts both `foo` and `bar`
    - `A: foo\, bar` only accepts `foo, bar`
    - `A: foo\\, bar` accepts both `foo\` and `bar`
    - `A: foo\\\, bar` only accepts `foo\, bar`
    - `A: foo\\\\, bar` accepts both `foo\\` and `bar`
    - etc.