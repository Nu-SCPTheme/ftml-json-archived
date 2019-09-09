## ftml-json
[![Build Status](https://travis-ci.org/Nu-SCPTheme/ftml-json.svg?branch=master)](https://travis-ci.org/Nu-SCPTheme/ftml-json)

See the [ftml](https://github.com/Nu-SCPTheme/ftml) repo for information on "Foundation Text Markup Language".

This is a Rust application which enables parsing and rendering Wikidot sources into HTML, receiving requests via JSON and making client calls as needed to fetch remote information.

Available under the terms of the GNU Affero General Public License. See [LICENSE.md](LICENSE).

### Compilation
This library targets the latest stable Rust. At time of writing, that is 1.37.0

```sh
$ cargo build --release
```

This will create the final `ftml-json` binary, which can be executed using the following:

```sh
$ cargo run -- [arguments]
```

### Testing
```sh
$ cargo test
```

Add `-- --nocapture` to the end if you want to see test output.
