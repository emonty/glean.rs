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

use std::path::Path;
use std::fs::PathExt;

use ::config_drive::ConfigDrive;
use ::sys::SysInterfaces;
use self::interface_type::InterfaceType;
use self::interface_type::InterfaceType::{Static, Dhcp};
use self::Platform::{RedHat, Debian};

mod interface_type;

#[derive(Debug)]
pub struct LiveNetworks {
    interfaces: Vec<InterfaceType>,
    platform: Platform,
}

impl LiveNetworks {

    pub fn new(interfaces: &SysInterfaces, config_drive: Option<ConfigDrive>) -> LiveNetworks {

      let mut live_ifaces = Vec::new();
      for iface in interfaces.interfaces.iter() {
          match config_drive {
              Some(ref drive) =>
                  match drive.interface_map.get(&iface.address) {
                      Some(network) => {
                          let mut new_net = network.clone();
                          new_net.id = iface.iface.clone();
                          live_ifaces.push(Static(new_net));
                      },
                      None => live_ifaces.push(Dhcp(iface.iface.clone())),
                  },
              None => live_ifaces.push(Dhcp(iface.iface.clone())),
          }
      }
      live_ifaces.sort();

      let platform;
      if Path::new("/etc/network").is_dir() {
          platform = RedHat;
      } else {
          platform = Debian;
      }
      LiveNetworks {
          interfaces: live_ifaces,
          platform: platform,
      }
    }

    pub fn get_output(&self) -> FileList {
        let files = match self.platform {
            RedHat => {
                let w = RedHatWriter;
                return get_output_files(&w as &Writer, &self.interfaces);
            }
            Debian => {
                let w = DebianWriter;
                return get_output_files(&w as &Writer, &self.interfaces);
            }
        };
    }
}

fn get_output_files(writer: &Writer, interfaces: &Vec<InterfaceType>) -> FileList {
    let mut file_list = Vec::<FileToWrite>::new();
    for interface in interfaces.iter() {
        let iface = match(interface) {
            &Static(ref network) => network.id.clone(),
            &Dhcp(ref iface) => iface.clone(),
        };
        if ! writer.config_exists(&iface) {
            file_list.push(writer.generate_config(&interface));
        }
    }
    FileList { files: file_list }
}

#[derive(Debug)]
pub struct FileList {
   files: Vec<FileToWrite>,
}

impl FileList {
   pub fn write(&self) {
      println!("{:?}", self.files);
   }
}


#[derive(Debug)]
struct FileToWrite {
    filename: String,
    content: String,
}

#[derive(Debug)]
enum Platform {
  RedHat,
  Debian,
}

struct RedHatWriter;
struct DebianWriter;

trait Writer {
    fn config_exists(&self, iface: &String) -> bool;
    fn generate_config(&self, interface: &InterfaceType) -> FileToWrite;
}

impl Writer for RedHatWriter {
    fn config_exists(&self, iface: &String) -> bool {
        return true;
    }
    fn generate_config(&self, interface: &InterfaceType) -> FileToWrite {
        return FileToWrite { filename: String::from(""), content: String::from("") };
    }
}

impl Writer for DebianWriter {
    fn config_exists(&self, iface: &String) -> bool {
        return true;
    }
    fn generate_config(&self, interface: &InterfaceType) -> FileToWrite {
        return FileToWrite { filename: String::from(""), content: String::from("") };
    }
}

