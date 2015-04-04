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
use std::process::Command;

#[derive(Debug)]
pub struct Interface {
    address: String,
    iface: String,
}

impl Interface {
  pub fn new(path: &PathBuf, noop: bool) -> Option<Interface> {
      let iface = path.as_path().file_name().unwrap();
      if iface == "lo" {
          return None;
      }
      let assign_type = match get_file_value(&path.join("addr_assign_type")) {
        None => return None,
        Some(value) => value,
      };
      if assign_type != "0" {
          return None
      }

      if ! is_interface_live(&path, noop) {
          return None
      }

      let address = match get_file_value(&path.join("address")) {
        None => return None,
        Some(value) => value,
      };

      Some(Interface {
         address: address,
         iface: String::from_str(iface.to_str().unwrap()),
      })
  }
}

fn get_file_value(path: &PathBuf) -> Option<String> {
  let newline = '\n';
  let mut file_obj = match File::open(path) {
    Err(why) => { debug!("{}", why); return None},
    Ok(file) => file,
  };
  let mut raw_value = String::new();
  match file_obj.read_to_string(&mut raw_value) {
    Err(why) => { debug!("{}", why); return None},
    Ok(_) => {},
  };
  let value = raw_value.trim_matches(newline);
  debug!("File {} value is: ::{}::", path.display(), value);
  return Some(String::from_str(value));
}

fn is_interface_live(path: &PathBuf, noop: bool) -> bool {
  // Check to see if it's a live interface
  match get_file_value(&path.join("carrier")) {
    None => {}
    Some(value) => {
        if value == "1" {
            return true;
        }
    },
  };
  // This will be there so do things with ip link set dev {} up
  if noop {
      return false;
  } else {
      let iface = path.as_path().file_name().unwrap();
      let mut range = 0..10;
      loop {
          match range.next() {
              Some(x) => {
                  debug!("ip link number {}", x);
                  let output = Command::new("ip")
                      .arg("link").arg("set").arg("dev").arg(iface).arg("up")
                      .status();
                  match get_file_value(&path.join("carrier")) {
                    None => {}
                    Some(value) => {
                        if value == "1" {
                            return true;
                        }
                    },
                  };
              },
              None => return false,
          }
      }
  }
}
