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

#![feature(custom_derive)]
extern crate rustc_serialize;
use rustc_serialize::{json, Decodable, Decoder};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::env;

mod options;

// Automatically generate `Decodable` trait implementations
// Don't generate `Encodable` because we don't use them
// Do Debug traits so that we can print things

#[derive(Debug)]
pub struct Service {
    service_type: String,
    address: String,
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


#[derive(RustcDecodable, Debug)]
pub struct Route {
    netmask: String,
    network: String,
    gateway: String,
}

#[derive(Debug)]
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
    networks: Vec<Network>,
    links: Vec<Link>,
}

#[derive(RustcDecodable, Debug)]
pub struct VendorData {
    network_info: NetworkInfo,
}

fn get_network_info(data_path: &PathBuf) -> Option<NetworkInfo> {
    let display = data_path.display();
    // Needs to be mutable because reading from it apparently involves change
    let mut file = match File::open(&data_path) {
        Err(why) => panic!(
            "couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!(
            "couldn't read {}: {}", display, Error::description(&why)),
        Ok(ret) => ret
    };

    if data_path.file_name().unwrap() == "vendor_data.json" {
        let data: VendorData = match json::decode(&s) {
            Err(why) => panic!(
                "Could not decode {}: {}", display, Error::description(&why)),
            Ok(data) => data
        };
        let netinfo = data.network_info;
        return Some(netinfo);
    } else if data_path.file_name().unwrap() == "network_info.json" {
        let data: NetworkInfo = match json::decode(&s) {
            Err(why) => panic!(
                "Could not decode {}: {}", display, Error::description(&why)),
            Ok(data) => data
        };
        return Some(data);
    }
    return None;
}


#[cfg(not(test))]
fn main() {

    let opts = options::get_options();
    if opts.help {
        println!("{}", opts.usage);
        return;
    }

    let vendor_data_path = env::current_dir().unwrap().join(
        Path::new("samples/rax/openstack/latest/vendor_data.json"));
    let vendor_display = vendor_data_path.display();
    println!("{}", vendor_display);
    let vendor_netinfo = get_network_info(&vendor_data_path);
    println!("{:?}", vendor_netinfo.unwrap().networks[0]);

    let network_info_path = env::current_dir().unwrap().join(
        Path::new("samples/liberty/openstack/latest/network_info.json"));
    let network_display = network_info_path.display();
    println!("{}", network_display);
    let netinfo = get_network_info(&network_info_path);

    println!("{:?}", netinfo.unwrap().networks[0]);
}
