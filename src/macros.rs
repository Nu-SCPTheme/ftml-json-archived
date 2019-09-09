/*
 * macros.rs
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

macro_rules! make_err {
    ($code:expr) => {{
        use jsonrpc_core::{Error, ErrorCode};

        Error::new(ErrorCode::ServerError($code))
    }};
    ($code:expr, $message:expr) => {{
        use jsonrpc_core::{Error, ErrorCode};

        let mut error = Error::new(ErrorCode::ServerError($code));
        error.message = str!($message);
        error
    }};
    ($code:expr, $message:expr, $data:expr) => {{
        use jsonrpc_core::{Error, ErrorCode};

        let mut error = Error::new(ErrorCode::ServerError($code));
        error.message = str!($message);
        error.data = Some($data);
        error
    }};
}
