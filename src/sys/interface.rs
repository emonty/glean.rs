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

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Interface {
    address: String,
    iface: String,
}

impl Interface {
  pub fn new(path: &PathBuf) -> Option<Interface> {
      let iface = path.as_path().file_name().unwrap();
      if iface == "lo" {
          return None;
      }
      let mut assign_type_file = match File::open(path.join("addr_assign_type")) {
        Err(why) => { debug!("{}", why); return None},
        Ok(file) => file,
      };
      let newline = '\n';
      let mut assign_type = String::new();
      match assign_type_file.read_to_string(&mut assign_type) {
        Err(why) => { debug!("{}", why); return None},
        Ok(_) => {},
      };
      let trimmed_assign_type = assign_type.trim_matches(newline);
      debug!("Type is: ::{}::", trimmed_assign_type);
      if trimmed_assign_type != "0" {
          return None
      }
      let mut address_file = match File::open(path.join("address")) {
        Err(why) => { debug!("{}", why); return None},
        Ok(file) => file,
      };
      let mut address = String::new();
      match address_file.read_to_string(&mut address) {
        Err(why) => { debug!("{}", why); return None},
        Ok(_) => {},
      };
      let trimmed_address = address.trim_matches(newline);
      debug!("Address is: ::{}::", trimmed_address);
      Some(Interface {
         address: String::from_str(trimmed_address),
         iface: String::from_str(iface.to_str().unwrap()),
      })
  }
}
