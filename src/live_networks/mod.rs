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

pub use super::config_drive::ConfigDrive;
pub use super::config_drive::network_info::Network;
pub use super::sys::SysInterfaces;
pub use self::interface_type::InterfaceType;

mod interface_type;

#[derive(Debug)]
pub struct LiveNetworks {
    interfaces: Vec<InterfaceType>,
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
                          live_ifaces.push(InterfaceType::Static(new_net));
                      },
                      None => live_ifaces.push(InterfaceType::Dhcp(iface.iface.clone())),
                  },
              None => live_ifaces.push(InterfaceType::Dhcp(iface.iface.clone())),
          }
      }
      live_ifaces.sort();


      LiveNetworks {
          interfaces: live_ifaces,
      }
    }
}
