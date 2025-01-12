# Cat2Text-rs

This is a port of [Cat2Text](https://github.com/askiiart/Cat2Text) to Rust, with extra functionality, better documentation, and support for using it as a library as well.

## Base4 Format

Using the original base4 format, it works like this. First off, each word is equal to a value in base 4:

| Value | Cat sound |
| ----- | --------- |
| 0     | meow      |
| 1     | mrrp      |
| 2     | mreow     |
| 3     | mrow      |

Then, the text is converted into lowercase

## Limitations

This currently only supports lowercase text in the latin alphabet without numbers or anything - however, using `cat2text::core`, you can convert anything to meows, as long as you can put it in integers - which, you can.

## TODO

- Add functionality for converting `Vec<u8>` to cat.
- Add more bases - adaptable base option?
