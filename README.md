# Cat2Text-rs

This is a port of [Cat2Text](https://github.com/askiiart/Cat2Text) to Rust, with extra functionality, better documentation, and support for using it as a library as well.

## Base 4 Format

Using the original base4 format, it works like this. First off, each word is equal to a value in base 4:

| Value | Cat sound |
| ----- | --------- |
| 0     | meow      |
| 1     | mrrp      |
| 2     | mreow     |
| 3     | mrow      |

Then, the text is converted into lowercase, 96 is subtracted from its ASCII value (i.e. "a" (97) -> 1), and it's converted to base 4, which is then replaced by the cat sounds above; Each cat sound is separated by a space (" "), then each word is delimited by a semicolon and a space ("; ").

For example, "i love cats" is translated into:

> meow mreow mrrp; meow mrow meow meow mrow mrow mrrp mrrp mreow meow mrrp mrrp; meow meow mrow meow meow mrrp mrrp mrrp meow mrrp meow mrow

Separating out the letters and words:

> [
> ["meow mreow mrrp"],
> [
> "meow mrow meow",
> "meow mrow mrow",
> "mrrp mrrp mreow",
> "meow mrrp mrrp"
> ],
> [
> "meow meow mrow",
> "meow meow mrrp",
> "mrrp mrrp meow",
> "mrrp meow mrow"
> ]
> ]

## Library usage

To use the library, first off add it as a dependency:

```sh
cargo add cat2text
```

Then just import the relevant functions, and run it like this:

```rust
use cat2text::base4::{encode, decode};

let encoded = encode("i love cats".to_string());
assert_eq!(encoded, "meow mreow mrrp; meow mrow meow meow mrow mrow mrrp mrrp mreow meow mrrp mrrp; meow meow mrow meow meow mrrp mrrp mrrp meow mrrp meow mrow");
let decoded = decode(encoded);
assert_eq!(decoded, "i love cats");
```

Or to encode binary in cat2text's expanded base16 alphabet:

```rust
use cat2text::{anybase::bytes::encode, core::bytes::char_length};

let bytes = &[243, 10];
let base = 16;
let char_length = char_length(base);

assert_eq!("mrow~ mrow meow purrrr", encode(bytes, base, char_length));
```

You can use the library to encode anything up to base 16 - for details, see the [docs](https://docs.rs/cat2text/latest/cat2text/)

## Binary usage

To install cat2text, just run the following:

```sh
cargo install cat2text
```

**Usage**:

```yaml
A port of Cat2Text to Rust, with extra functionality, better documentation, and support for using it as a library as well.

Usage: cat2text <COMMAND>

Commands:
  gen-completion  Generate shell completions
  encode          Encodes text/data to mrow~
  decode          Decodes mrow~ to text/data
  benchmark       Benchmark cat2text-rs
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

For example, `cat2text encode 'i love cats'` to encode `i love cats` in text mode using the default of base 4.

### Encode/decode arguments

- `-b`, `--base` (int): What base to encode/decode using - up to base 16
- `--bytes` (flag): Whether to use byte encoding or text encoding
- `-h`, `--help`: Print help

### Benchmark arguments

- `-b`, `--base` (integer): What base to encode/decode using - up to base 16
- `--bytes` (flag): Whether to use byte encoding or text encoding
- `-i`, `--iterations` (int): How many iterations to run each benchmark for
- `-h`, `--help`: Print help

### Shell completions

To generate shell completions, you can run `cat2text gen-completion $(basename $SHELL) | source` on *nix systems using bash, zsh, or fish.
