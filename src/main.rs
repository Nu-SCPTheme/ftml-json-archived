/*
 * main.rs
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

#![deny(missing_debug_implementations)]

extern crate clap;
extern crate color_backtrace;
extern crate ftml;
extern crate jsonrpc_core;
extern crate jsonrpc_core_client;
extern crate jsonrpc_derive;
extern crate jsonrpc_http_server;

#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate pest;
extern crate pretty_env_logger;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate str_macro;
extern crate toml;

#[macro_use]
mod macros;

mod config;
mod ftml_error;
mod handle;
mod server;

use self::config::Config;
use self::server::FtmlServer;
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};

pub type StdResult<T, E> = std::result::Result<T, E>;

fn main() {
    color_backtrace::install();

    let Config {
        address,
        threads,
        log_level,
    } = Config::parse_args();

    pretty_env_logger::formatted_builder()
        .filter_level(log_level)
        .init();

    info!("Server starting");
    let api = FtmlServer::new();
    let io = api.to_handler();
    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .threads(threads)
        .start_http(&address)
        .expect("Unable to start RPC server");

    info!("Running main loop, serving on {}", &address);
    server.wait();
}
