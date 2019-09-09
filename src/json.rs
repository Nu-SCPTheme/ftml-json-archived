/*
 * json.rs
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

use jsonrpc_core::{Error, Result};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Error as JsonError, Value};

fn json_error(error: JsonError) -> Error {
    make_err!(
        107,
        error,
        json!({
            "line": error.line(),
            "column": error.column(),
            "classify": format!("{:?}", error.classify()),
        })
    )
}

#[cold]
pub fn to<S: Serialize>(object: &S) -> Result<Value> {
    let value = serde_json::to_value(object).map_err(json_error)?;
    Ok(value)
}

#[cold]
pub fn from<D: DeserializeOwned>(value: Value) -> Result<D> {
    let object = serde_json::from_value(value).map_err(json_error)?;
    Ok(object)
}
