#![feature(custom_derive)]
extern crate rustc_serialize;
use rustc_serialize::json;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Service {
    service_type: String,
    address: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Route {
    netmask: String,
    network: String,
    gateway: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Network {
    network_id: String,
    network_type: String,
    netmask: String,
    link: String,
    routes: Vec<Route>,
    ip_address: String,
    id: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Link {
    ethernet_mac_address: String,
    mtu: u16,
    id: String,
    vif_id: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct NetworkInfo {
    services: Vec<Service>,
    networks: Vec<Network>,
    links: Vec<Link>,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct VendorData {
    network_info: NetworkInfo,
}

fn main() {

    let vendor_data_path = env::current_dir().unwrap().join(
        Path::new("samples/rax/openstack/latest/vendor_data.json"));
    let display = vendor_data_path.display();
    println!("{}", display);

    // Needs to be mutable because reading from it apparently involves change
    let mut file = match File::open(&vendor_data_path) {
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
    let vendor_data: VendorData = match json::decode(&s) {
        Err(why) => panic!(
            "Could not decode {}: {}", display, Error::description(&why)),
        Ok(vendor_data) => vendor_data
    };
    let netinfo = vendor_data.network_info;
    println!("{:?}", netinfo.networks[0]);
}
