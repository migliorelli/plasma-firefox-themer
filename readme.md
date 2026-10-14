# Plasma Firefox Themer

Makes Firefox follow your Plasma color scheme. I wrote it because real reason Firefox never matched my KDE theme (real reason: I got tired of editing the CSS by hand).

Needs to enable `toolkit.legacyUserProfileCustomizations.stylesheets` to work. See [r/FirefoxCSS tutorial](https://www.reddit.com/r/FirefoxCSS/wiki/index/tutorials/) to more info.

Only the CSS template is AI generated.

## Todo

- Watch: watch for kdeglobal changes and auto apply theme

## Install

Needs Rust.

```sh
git clone https://github.com/migliorelli/plasma-firefox-themer
cd plasma-firefox-themer
cargo install --path .
```

## Usage

```sh
plasma-firefox-themer
```

See `plasma-firefox-themer --help` for the available flags.

## How it works

Reads the colors from `~/.config/kdeglobals` and writes/import a plasma-firefox-themer.css to your Firefox profile.

## Limitations

- Only tested on plasma 6.7
- Some colors may be weird

## License

GPL-3.0