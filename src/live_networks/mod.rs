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

use std::path::PathBuf;
use std::fs;
use std::process::Command;

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
    root: PathBuf,
}

impl LiveNetworks {

    pub fn new(interfaces: &SysInterfaces, config_drive: Option<ConfigDrive>, root: &Option<String>) -> LiveNetworks {

      let mut live_ifaces = Vec::new();
      for iface in interfaces.interfaces.iter() {
          match config_drive {
              Some(ref drive) =>
                  match drive.interface_map.get(&iface.address) {
                      Some(network) => {
                          let mut new_net = network.clone();
                          new_net.id = iface.iface.clone();
                          new_net.link = iface.address.clone();
                          live_ifaces.push(Static(new_net));
                      },
                      None => live_ifaces.push(Dhcp(iface.iface.clone())),
                  },
              None => live_ifaces.push(Dhcp(iface.iface.clone())),
          }
      }
      live_ifaces.sort();

      let base_root_path = match root {
          &Some(ref path) => PathBuf::from(path),
          &None => PathBuf::from("/"),
      };

      let platform;
      match fs::metadata(base_root_path.clone().join("etc/network")) {
          Ok(metadata) => {
              if metadata.is_dir() {
                  platform = Debian;
              } else {
                  platform = RedHat;
              }
          },
          Err(_) => {
              platform = RedHat;
          },
      };
      LiveNetworks {
          interfaces: live_ifaces,
          platform: platform,
          root: base_root_path,
      }
    }

    pub fn get_output(&self) -> FileList {
        println!("{:?}", self.platform);
        match self.platform {
            RedHat => {
                let w = RedHatWriter{ root: self.root.clone() };
                return get_output_files(&w as &Writer, &self.interfaces);
            }
            Debian => {
                let w = DebianWriter{ root: self.root.clone() };
                return get_output_files(&w as &Writer, &self.interfaces);
            }
        };
    }
}

fn get_output_files(writer: &Writer, interfaces: &Vec<InterfaceType>) -> FileList {
    let mut file_list = Vec::<FileToWrite>::new();
    for interface in interfaces.iter() {
        let iface = match interface {
            &Static(ref network) => network.id.clone(),
            &Dhcp(ref iface) => iface.clone(),
        };
        if ! writer.config_exists(&iface) {
            println!("Don't have: {}", iface);
            file_list.push(writer.generate_config(&iface, &interface));
        } else {
            println!("Already have: {}", iface);
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
    path: PathBuf,
    content: String,
}

#[derive(Debug)]
enum Platform {
  RedHat,
  Debian,
}

struct RedHatWriter {
    root: PathBuf,
}

struct DebianWriter {
    root: PathBuf,
}

static BULLET: &'static str = "Generated by glean";

trait Writer {
    fn interface_file(&self, iface: &String) -> PathBuf;
    fn config_exists(&self, iface: &String) -> bool;
    fn generate_config(&self, iface: &String, interface: &InterfaceType) -> FileToWrite;
}

impl Writer for RedHatWriter {
    fn interface_file(&self, iface: &String) -> PathBuf {
      let mut path_str = String::from("etc/sysconfig/network-scripts/ifcfg-");
      path_str.push_str(iface);
      return self.root.clone().join(path_str);
    }

    fn config_exists(&self, iface: &String) -> bool {
      match fs::metadata(self.interface_file(&iface)) {
          Ok(metadata) => return metadata.is_file(),
          Err(_) => return false,
      }
    }

    fn generate_config(&self, iface: &String, interface: &InterfaceType) -> FileToWrite {
        let filename = self.interface_file(&iface);
        let content;
        match interface {
            &Dhcp(_) => { content = format!("# {bullet}
DEVICE={iface}
BOOTPROTO=dhcp
ONBOOT=yes
NM_CONTROLLED=no
TYPE=Ethernet", bullet=BULLET, iface=iface) },
            &Static(ref network) => {
                content = format!("# {bullet}
DEVICE={iface}
BOOTPROTO=static
HWADDR={hwaddr}
IPADDR={ip_address}
NETMASK={netmask}
ONBOOT=yes
NM_CONTROLLED=no", bullet=BULLET, iface=iface, ip_address=network.ip_address, netmask=network.netmask, hwaddr=network.link);
            }
        };
        return FileToWrite {
            path: filename,
            content: content,
        };
    }
}

impl Writer for DebianWriter {
    fn interface_file(&self, iface: &String) -> PathBuf {
        return self.root.clone().join("etc/network/interfaces.d").join(iface);
    }

    fn config_exists(&self, iface: &String) -> bool {
      match Command::new("ifquery").arg(iface).status() {
        Err(why) => { debug!("ifquery error: {}", why); return false ; }
        Ok(status) => return status.success(),
      }
    }
    fn generate_config(&self, iface: &String, interface: &InterfaceType) -> FileToWrite {
        let mut content;
        match interface {
            &Dhcp(_) => { content = format!("# {bullet}
aut0 {iface}
iface {iface} inet dhcp", bullet=BULLET, iface=iface) },
            &Static(ref network) => {
                let link_type;
                if network.network_type == "ipv6" {
                    link_type = "inet6";
                } else {
                    link_type = "inet";
                }
                content = String::from(format!("# {bullet}
aut0 {iface}
iface {iface} {link_type} static
    address {ip_address}
    netmask {netmask}", bullet=BULLET, iface=iface, link_type=link_type, ip_address=network.ip_address, netmask=network.netmask));
                for ref route in network.routes.iter() {
                    if route.network == "0.0.0.0" && route.netmask == "0.0.0.0" {
                        content.push_str(&format!("
    gateway {gateway}", gateway=route.gateway));
                    } else {
                        content.push_str(&format!("
    post-up route add -net {net} netmask {mask} gw {gw} || true\n",
                            net=route.network, mask=route.netmask,
                            gw=route.gateway));
                        content.push_str(&format!("
    pre-down route add -net {net} netmask {mask} gw {gw} || true\n",
                            net=route.network, mask=route.netmask,
                            gw=route.gateway));
                    }
                }

            }
        };

        return FileToWrite {
            path: self.interface_file(&iface),
            content: content,
        };
    }
}
