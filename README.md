## ftml-json
[![Build Status](https://travis-ci.org/Nu-SCPTheme/ftml-json.svg?branch=master)](https://travis-ci.org/Nu-SCPTheme/ftml-json)

See the [ftml](https://github.com/Nu-SCPTheme/ftml) repo for information on "Foundation Text Markup Language".

This is a Rust application which enables parsing and rendering Wikidot sources into HTML, receiving requests via JSON and making client calls as needed to fetch remote information.

Available under the terms of the GNU Affero General Public License. See [LICENSE.md](LICENSE).

### Compilation
This library targets the latest stable Rust. At time of writing, that is 1.40.0

```sh
$ cargo build --release
```

This will create the final `ftml-json` binary, which can be executed using the following:

```sh
$ cargo run --release -- <config-file>
```

An example configuration file is available at `misc/config.toml`.

### Testing

```sh
$ cargo test
```

Add `-- --nocapture` to the end if you want to see test output.

### Requests

Utilizes the [JSONRPC 2.0](https://www.jsonrpc.org/specification) protocol. The methods currently supported are:

`protocol`, `protocolVersion`: Gets the API version of ftml-json. Currently `0`.

`ping`: Returns `pong!`. For testing that the server is up.

`error`: Returns an optional message as an error. For testing error handling.

`time`: Returns the current system time.

`prefilter`: Transforms the given input text according to ftml's preprocessing rules. Performs no parsing or rendering.

Example:
```json
{
    "jsonrpc": "2.0",
    "method": "prefilter",
    "id": (request ID),
    "params": ["<< TEST >>"]
}
```
yields
```json
{
    "jsonrpc": "2.0",
    "id": (request ID),
    "result": "« TEST »"
}
```

`parse`: Transforms the given input text to a JSON parse tree. The prefilter is not performed.

Example:
```json
{
    "jsonrpc": "2.0",
    "method": "parse",
    "id": (request ID),
    "params": ["**test** word"]
}
```
yields
```json
{
    "jsonrpc": "2.0",
    "id": (request ID),
    "result": {
        "paragraphs": [
            {
                "Words": {
                    "centered": false,
                    "words": [
                        {
                            "Bold": {
                                "words": [
                                    "Text": { "contents": "test" }
                                ]
                            }
                        },
                        {
                            "Text": { "contents": " word" }
                        }
                    ]
                }
            }
        ]
    }
}
```

`render`, `transform`: Performs prefilter, parsing, and HTML rendering.

Input parameters are `[page_info, input]`, where `input` is a string with the source, and `page_info` is an object with the following schema:

```json
{
    "title": "SCP-1000",
    "alt_title": "Bigfoot",
    "header": null,
    "subheader": null,
    "rating": 500,
    "tags": ["scp", "keter", "species", ...]
}
```

(`header` and `subheader` affect the "SCP Foundation" and "Secure, Contain, Protect" text above the page. If `null` they remain unchanged)

And returns an html object with the following schema:

```json
{
    "html": " ... generated HTML body here ...",
    "style": " ... all custom CSS here ...",
    "meta": [
        {
            "tag_type": "name",
            "name": "generator",
            "value": "ftml"
        }
        {
            ... other tags...
        }
    ]
}
```
