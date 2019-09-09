/*
 * server.rs
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

use jsonrpc_core::{IoHandler, Result};
use jsonrpc_derive::rpc;

#[rpc]
pub trait FtmlApi {
    #[rpc(name = "ping")]
    fn ping(&self) -> Result<String>;
}

#[derive(Debug)]
pub struct FtmlServer;

impl FtmlServer {
    pub fn to_handler(self) -> IoHandler {
        debug!("Creating IoHandler with FtmlApi");

        let mut io = IoHandler::new();
        io.extend_with(FtmlServer.to_delegate());
        io
    }
}

impl FtmlApi for FtmlServer {
    #[inline]
    fn ping(&self) -> Result<String> {
        Ok(str!("pong!"))
    }
}
