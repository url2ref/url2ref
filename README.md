# url2ref &ndash; generate a reference from a web address

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) ![build](https://img.shields.io/github/actions/workflow/status/url2ref/url2ref/build_and_test.yml)

## Motivation

Thoroughly citing a web resource can be a tedious process. This application aims to make the process automatic for those web resources which have annotated their content using the [Open Graph](https://ogp.me/) protocol or the [Schema.org](https://schema.org/) vocabulary.

## User interfaces

* CLI &ndash; [``url2ref-cli``](./src/bin/cli) provides a command-line interface powered by [clap](https://crates.io/crates/clap)
* GUI &ndash; [``url2ref-web``](./src/bin/web) provides an in-browser graphical user interface powered by [Rocket](https://rocket.rs/) and [Bootstrap](https://getbootstrap.com/)

## Build instructions

### ``url2ref-cli``

To build and run the CLI application, execute

```console
cargo run --bin url2ref-cli -- --url <URL>
```

where URL points to a web resource.

### ``url2ref-web``

To build and run the web application, first install the required dependencies via [npm](https://www.npmjs.com/) by executing [``build.sh``](./npm/build.sh) from within the [``npm``](./npm) directory.

Then execute

```console
cargo run --bin url2ref-web
```

or simply

```console
cargo run
```

as ``url2ref-web`` is the default binary.

## Documentation

The documentation can be generated using [``rustdoc``](https://doc.rust-lang.org/rustdoc/index.html). To generate it using a simple [``cargo``](https://doc.rust-lang.org/cargo/) command, execute

```console
cargo doc --no-deps
```

from the project root.

## Contributing

See [``CONTRIBUTING.md``](CONTRIBUTING.md).

## License

The code in this project is licensed under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0).
