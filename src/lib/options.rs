// Copyright (c) 2015 Hewlett-Packard Development Company, L.P.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate getopts;
use self::getopts::Options;
use std::env;

#[derive(Debug)]
pub struct Opt {
  pub help: bool,
  pub noop: bool,
  pub root: String,
  pub distro: Option<String>,
  pub interface: Option<String>,
  pub usage: String,
}

pub fn get_options() -> Opt {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("n", "noop", "Do not write files");
    opts.optopt("", "root", "Mounted root for config drive info [default: '/mnt/config']", "ROOT");
    opts.optopt("", "distro", "Override detected distro", "DISTRO");
    opts.optopt("i", "interface", "Interface to process", "INTERFACE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let brief = format!("Usage: {} [options]", program);
    let ret = Opt {
        usage: opts.usage(&brief),
        help: matches.opt_present("help"),
        noop: matches.opt_present("noop"),
        root: match matches.opt_present("root") {
            true => matches.opt_str("root").unwrap(),
            false => "/mnt/config".to_string(),
        },
        distro: None,
        interface: None,
    };

    return ret;
}
