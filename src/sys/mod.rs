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

use glob::glob;
use std::path::PathBuf;

mod interface;

#[derive(Debug)]
pub struct SysInterfaces {
    root: PathBuf,
    interfaces: Vec<interface::Interface>,
}

impl SysInterfaces {
  pub fn new(root: &Option<String>, interface: &Option<String>) -> SysInterfaces {

      let base_root_path = match root {
          &Some(ref path) => PathBuf::from(path),
          &None => PathBuf::from("/"),
      };
      let root_path = base_root_path.join("sys/class/net");
      debug!("Root Path {:?}", root_path);
      SysInterfaces {
          root: PathBuf::from(&root_path),
          interfaces: get_interfaces(&root_path, interface),
      }
  }
}

fn get_interfaces(root_path: &PathBuf, interface: &Option<String>) -> Vec<interface::Interface> {
    let interface_paths = match interface {
        &Some(ref iface) => vec![PathBuf::from(iface)],
        &None => read_interfaces(root_path),
    };
    let mut interfaces = Vec::new();
    for path in interface_paths {
        let interface = interface::Interface::new(&path);
        match interface {
            None => {}
            Some(iface) => interfaces.push(iface),
        }
    }
    return interfaces;
}

fn read_interfaces(root_path: &PathBuf) -> Vec<PathBuf> {
   let mut interfaces = Vec::new();
   let root_glob_path = root_path.join("*");
   let glob_path = root_glob_path.to_str().unwrap();
   for entry in glob(glob_path).unwrap() {
       match entry {
         Ok(path) => interfaces.push(PathBuf::from(&path.to_str().unwrap())),
         _ => {}
       }
   }
   return interfaces;
}
