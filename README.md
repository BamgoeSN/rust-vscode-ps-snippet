# rust-vscode-ps-snippet
Rust PS Snippet Settings File Generator for VSCode (Usable even if it's not exclusively for Rust)

## Usage
The snippet generator part is written in Rust, and any release for it is not yet in plan. Hence, you would need to have `rustc` to be able to compile the generator. Using `cargo` for this is recommended.

First, clone the repository into any directory you'd like to work in.
```bash
git clone https://github.com/BamgoeSN/rust-vscode-ps-snippet.git
```
Then, move into the generated directory.

### Using contained snippets (for Rust)
If you're using Rust for competitive programming and you're going to use snippets in the `snippets/` directory, then simply run `cargo run --release`. The program will search through every `.txt` text files in `snippets/`, include them in snippet, and generate `output.txt` file containing json data for VSCode snippet settings. After the run, open up snippets setting for Rust, and overwrite the content in `output.txt` into the settings file.

### Using custom snippets
The generator searches through `snippets/` folder which is in the same directory with the generator project, finds every `.txt` files in it, convert them into json data for VSCode settings and combine them into `output.txt` file. No matter what the inner folder structure inside `snippets/` is, it will just search out every text files and generate a settings file. So, simply replace the contents in `snippets/` and run the generator to get your own snippets settings.

The title of each snippet will be used for its prefix, so make sure that none of them have duplicated file names.

## Limitations
Current implementation of handling escape `$` sign is to check if either alphabetic character follows after it, or the line terminates with it, and if so then replace it with `\\$` to make the sign literal dollar sign. In any other cases, the `$` will not be escaped and therefore used as a preset cursor for the snippet. In other words, any literal dollar signs included like `$0` or `${0:here}` will not be escaped, where the terminating `$` and strings like `$abc` will be escaped. This implementation is to cope with macro definition in Rust, but it isn't considering any other cases for other languages. To my knowledge, there may be an issue with dealing with PHP snippets, so keep this in mind.
