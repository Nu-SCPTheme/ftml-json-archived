/*
 * config.rs
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

use clap::{App, Arg};
use log::LevelFilter;
use num_cpus;
use std::fs::File;
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
    pub address: SocketAddr,
    pub threads: usize,
    pub log_level: LevelFilter,
}

impl Config {
    #[cold]
    pub fn parse_args() -> Self {
        debug!("Parsing arguments");

        let matches = App::new("ftml-json")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Ammon Smith")
            .about("Daemon serving ftml transforms via JSONRPC")
            .max_term_width(110)
            .arg(
                Arg::with_name("config")
                    .value_name("FILE")
                    .required(true)
                    .help("Use the given configuration file"),
            )
            .get_matches();

        let path = Path::new(matches.value_of("config").unwrap());

        ConfigFile::read(path).into()
    }
}

#[serde(rename_all = "kebab-case")]
#[derive(Deserialize, Debug, Clone)]
struct ConfigFile {
    pub port: u16,
    pub use_ipv6: bool,
    pub log_level: Option<String>,
    pub threads: Option<usize>,
}

impl ConfigFile {
    #[cold]
    fn read(path: &Path) -> Self {
        let mut file = File::open(path).expect("Unable to open config file");
        let mut contents = String::new();
        let _ = file
            .read_to_string(&mut contents)
            .expect("Unable to read config file");
        let obj: Self = toml::from_str(&contents).expect("Unable to parse TOML in config file");

        obj
    }

    #[cold]
    fn parse_log_level(&self) -> LevelFilter {
        const LEVELS: [(&str, LevelFilter); 9] = [
            ("", LevelFilter::Info),
            ("off", LevelFilter::Off),
            ("none", LevelFilter::Off),
            ("trace", LevelFilter::Trace),
            ("debug", LevelFilter::Debug),
            ("warn", LevelFilter::Warn),
            ("warning", LevelFilter::Warn),
            ("err", LevelFilter::Error),
            ("error", LevelFilter::Error),
        ];

        let log_level = match self.log_level {
            Some(ref log_level) => log_level,
            None => return LevelFilter::Info,
        };

        for (text, level) in &LEVELS {
            if log_level.eq_ignore_ascii_case(text) {
                return *level;
            }
        }

        panic!("No such log level for '{}'", log_level);
    }
}

impl Into<Config> for ConfigFile {
    #[cold]
    fn into(self) -> Config {
        let ip_address = if self.use_ipv6 {
            IpAddr::V6(Ipv6Addr::UNSPECIFIED)
        } else {
            IpAddr::V4(Ipv4Addr::UNSPECIFIED)
        };

        let address = SocketAddr::new(ip_address, self.port);
        let log_level = self.parse_log_level();
        let threads = match self.threads {
            Some(0) | None => num_cpus::get(),
            Some(threads) => threads,
        };

        Config {
            address,
            threads,
            log_level,
        }
    }
}
