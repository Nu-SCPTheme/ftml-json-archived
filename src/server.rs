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

use crate::handle::FtmlHandle;
use crate::{ftml_error, json};
use ftml::html::HtmlOutput;
use ftml::prelude::*;
use jsonrpc_core::{IoHandler, Result, Value};
use jsonrpc_derive::rpc;
use std::time::SystemTime;

const PROTOCOL_VERSION: &str = "0";

#[rpc]
pub trait FtmlApi {
    // Misc
    #[rpc(name = "protocol", alias("protocolVersion"))]
    fn protocol(&self) -> Result<String>;

    #[rpc(name = "ping")]
    fn ping(&self) -> Result<String>;

    #[rpc(name = "error")]
    fn error(&self, message: Option<String>) -> Result<()>;

    #[rpc(name = "time")]
    fn time(&self) -> Result<SystemTime>;

    // Core
    #[rpc(name = "prefilter")]
    fn prefilter(&self, input: String) -> Result<String>;

    #[rpc(name = "parse")]
    fn parse(&self, input: String) -> Result<Value>;

    #[rpc(name = "render", alias("transform"))]
    fn render(&self, page_info: PageInfoOwned, input: String) -> Result<HtmlOutput>;
}

#[derive(Debug)]
pub struct FtmlServer {
    handle: FtmlHandle,
}

impl FtmlServer {
    #[inline]
    pub fn new() -> Self {
        FtmlServer { handle: FtmlHandle }
    }

    pub fn to_handler(self) -> IoHandler {
        debug!("Creating IoHandler with FtmlApi");

        let mut io = IoHandler::default();
        io.extend_with(self.to_delegate());
        io
    }
}

impl FtmlApi for FtmlServer {
    // Misc
    fn protocol(&self) -> Result<String> {
        info!("Method: protocol");
        Ok(str!(PROTOCOL_VERSION))
    }

    fn ping(&self) -> Result<String> {
        info!("Method: ping");
        Ok(str!("pong!"))
    }

    fn error(&self, message: Option<String>) -> Result<()> {
        info!("Method: error");

        let error = match message {
            Some(message) => make_err!(-1, message),
            None => make_err!(-1),
        };

        Err(error)
    }

    fn time(&self) -> Result<SystemTime> {
        info!("Method: time");

        Ok(SystemTime::now())
    }

    // Core
    fn prefilter(&self, input: String) -> Result<String> {
        info!("Method: prefilter");

        let mut text = input;
        match prefilter(&mut text, &self.handle) {
            Ok(_) => Ok(text),
            Err(error) => Err(ftml_error::convert(error)),
        }
    }

    fn parse(&self, input: String) -> Result<Value> {
        info!("Method: parse");

        let tree = parse(&input).map_err(ftml_error::convert)?;
        json::to(&tree)
    }

    fn render(&self, page_info: PageInfoOwned, mut input: String) -> Result<HtmlOutput> {
        info!("Method: render");

        let html = HtmlRender::new(&self.handle);
        let info = page_info.as_borrow();
        let output = html
            .transform(&mut input, info, &self.handle)
            .map_err(ftml_error::convert)?;
        Ok(output)
    }
}
