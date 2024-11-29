# GH-Spray

A Rust implementation to draw custom patterns on your GitHub contribution graph.
Based on github-spray (https://github.com/Annihil/github-spray).

## Features

- Draw custom text on your GitHub contribution graph
- Customize start date
- Adjust contribution intensity with multiplier
- Terminal visualization while creating commits
- Simple CLI interface

## Installation

`cargo install gh-spray`

### Usage

`gh-spray -t "rust" [options]`

```
Options:
  -t, --text <TEXT>          Text to draw on the contribution graph
  -s, --startdate <DATE>     Start date (YYYY-MM-DD or RFC3339 format)
  -m, --multiplier <NUMBER>  Contribution intensity multiplier [default: 1]
  -f, --font <FONT>         Font to use [default: default]
  -h, --help                Print help
  -V, --version             Print version
```

### Examples

`gh-spray --text "rust" --startdate "2024-01-01" --multiplier 2`

## License

[MIT](LICENSE)
