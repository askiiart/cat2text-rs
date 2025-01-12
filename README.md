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

## Limitations

This currently only supports lowercase text in the latin alphabet without numbers or anything - however, using `cat2text::core`, you can convert anything to meows, as long as you can put it in integers - which, you can.

## TODO

- Add functionality for converting `Vec<u8>` to cat.
- Add more bases - adaptable base option?
