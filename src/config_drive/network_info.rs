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

use rustc_serialize::{json, Decodable, Decoder};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::ascii::OwnedAsciiExt;


// Automatically generate `Decodable` trait implementations
// Don't generate `Encodable` because we don't use them
// Do Debug traits so that we can print things

#[derive(Debug)]
pub struct Service {
    pub service_type: String,
    pub address: String,
}

impl Decodable for Service {
  fn decode<D: Decoder>(decoder: &mut D) -> Result<Service, D::Error> {
    decoder.read_struct("root", 0, |decoder| {
         Ok(Service{
          service_type: try!(decoder.read_struct_field("type", 0, |decoder| Decodable::decode(decoder))),
          address: try!(decoder.read_struct_field("address", 0, |decoder| Decodable::decode(decoder)))
        })
    })
  }
}


#[derive(RustcDecodable, Debug, Clone)]
pub struct Route {
    netmask: String,
    network: String,
    gateway: String,
}

#[derive(Debug, Clone)]
pub struct Network {
    network_id: String,
    network_type: String,
    netmask: String,
    link: String,
    routes: Vec<Route>,
    ip_address: String,
    id: String,
}


// Custom class because type can't be a struct member
impl Decodable for Network {
  fn decode<D: Decoder>(decoder: &mut D) -> Result<Network, D::Error> {
    decoder.read_struct("root", 0, |decoder| {
         Ok(Network{
          network_id: try!(decoder.read_struct_field("network_id", 0, |decoder| Decodable::decode(decoder))),
          network_type: try!(decoder.read_struct_field("type", 0, |decoder| Decodable::decode(decoder))),
          netmask: try!(decoder.read_struct_field("netmask", 0, |decoder| Decodable::decode(decoder))),
          link: try!(decoder.read_struct_field("link", 0, |decoder| Decodable::decode(decoder))),
          routes: try!(decoder.read_struct_field("routes", 0, |decoder| Decodable::decode(decoder))),
          ip_address: try!(decoder.read_struct_field("ip_address", 0, |decoder| Decodable::decode(decoder))),
          id: try!(decoder.read_struct_field("id", 0, |decoder| Decodable::decode(decoder))),
        })
    })
  }
}

#[derive(RustcDecodable, Debug)]
pub struct Link {
    ethernet_mac_address: String,
    mtu: u16,
    id: String,
    vif_id: String,
}

#[derive(RustcDecodable, Debug)]
pub struct NetworkInfo {
    services: Vec<Service>,
    pub networks: Vec<Network>,
    links: Vec<Link>,
}

#[derive(RustcDecodable, Debug)]
pub struct VendorData {
    network_info: NetworkInfo,
}

impl NetworkInfo {
    pub fn new(root: &Option<String>, data_path: &PathBuf) -> Option<NetworkInfo> {
        let base_root_path = match root {
            &Some(ref path) => PathBuf::from(path),
            &None => PathBuf::from("/"),
        };
        let root_path = base_root_path.join("mnt/config").join(data_path);
        let display = root_path.display();

        // Needs to be mutable because reading from it involves change
        let mut file = match File::open(&root_path) {
            Err(why) => {
                debug!("couldn't open {}: {}", display,
                       Error::description(&why));
                return None;
            },
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => {
                debug!("couldn't read {}: {}", display,
                       Error::description(&why));
                return None;
            },
            Ok(ret) => ret,
        };

        // First, try decoding at NetworkInfo
        match json::decode::<NetworkInfo>(&s) {
            Err(why) => {
                debug!("Could not decode as NetworkInfo {}: {}",
                       display, Error::description(&why));
            },
            Ok(data) => return Some(data),
        };
        // Then as VendorData
        match json::decode::<VendorData>(&s) {
            Err(why) => {
                debug!("Could not decode as VendorData {}: {}",
                       display, Error::description(&why));
                return None;
            },
            Ok(data) => return Some(data.network_info),
        };
    }

    pub fn get_interface_map(&self) -> HashMap<String, Network> {

        let mut interfaces = HashMap::new();
        for link in self.links.iter() {
            for net in self.networks.iter() {
                if net.link == link.id {
                    let mac_addr = String::from_str(&link.ethernet_mac_address);
                    let lower_mac = mac_addr.into_ascii_lowercase();
                    interfaces.insert(lower_mac, net.clone());
                }
            }
        }
        return interfaces;
    }
}
