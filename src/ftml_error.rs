/*
 * ftml_error.rs
 *
 * ftml-json - JSON server to convert Wikidot code to HTML
 * Copyright (C) 2019 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use ftml::Error::*;
use jsonrpc_core::{Error, Value};
use pest::error::{ErrorVariant, InputLocation, LineColLocation};
use std::fmt::Debug;

fn convert_variant<R: Debug>(variant: ErrorVariant<R>) -> Value {
    macro_rules! map_str {
        ($map:expr) => {
            $map.iter()
                .map(|rule| format!("{:?}", rule))
                .collect::<Vec<_>>()
        };
    }

    match variant {
        ErrorVariant::ParsingError {
            positives,
            negatives,
        } => json!({
            "positives": map_str!(positives),
            "negatives": map_str!(negatives),
        }),
        ErrorVariant::CustomError { message } => Value::String(message),
    }
}

fn convert_location(location: InputLocation) -> Value {
    use self::InputLocation::*;

    let (start, end) = match location {
        Pos(start) => (start, None),
        Span((start, end)) => (start, Some(end)),
    };

    json!({
        "start": start,
        "end": end,
    })
}

fn convert_line_col(line_col: LineColLocation) -> Value {
    use self::LineColLocation::*;

    let (start, end) = match line_col {
        Pos(start) => (start, None),
        Span(start, end) => (start, Some(end)),
    };

    let end: Value = match end {
        Some((line, col)) => json!({
            "line": line,
            "column": col,
        }),
        None => json!(null),
    };

    json!({
        "start": {
            "line": start.0,
            "column": start.0,
        },
        "end": end,
    })
}

#[cold]
pub fn convert(error: ftml::Error) -> Error {
    match error {
        StaticMsg(msg) => make_err!(100, msg),
        Msg(msg) => make_err!(101, msg),
        Io(err) => make_err!(
            102,
            err,
            json!({
                "kind": format!("{:?}", err.kind()),
            })
        ),
        Utf8(err) => make_err!(
            103,
            err,
            json!({
                "valid_up_to": err.valid_up_to(),
                "error_len": err.error_len(),
            })
        ),
        Parse(err) => make_err!(
            104,
            err,
            json!({
                "variant": convert_variant(err.variant),
                "location": convert_location(err.location),
                "line_col": convert_line_col(err.line_col),
            })
        ),
        Remote(err) => {
            let msg: String = err.into();
            make_err!(105, msg)
        }
        Fmt(err) => make_err!(106, err),
    }
}
