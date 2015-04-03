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
pub struct TestStruct  {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
}

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
    link_type: Option<String>,
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
    region: Option<String>,
    ip_whitelist: Option<Vec<String>>,
}

fn main() {

    let vendor_data_file = env::current_dir().unwrap().join(Path::new("samples/rax/openstack/latest/vendor_data.json"));
    println!("{}", vendor_data_file.display());
    let display = vendor_data_file.display();

    // Needs to be mutable because reading from it apparently involves change
    let mut file = match File::open(&vendor_data_file) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           Error::description(&why)),
        // HOW do I do nothing here
        Ok(_) => print!("decoded"), // {} contains:\n{}", display, s),
    }
    let vendor_data: VendorData = json::decode(&s).unwrap();
    let netinfo = vendor_data.network_info;
    println!("{:?}", netinfo.networks[0]);

    let object = TestStruct {
        data_int: 1,
        data_str: "homura".to_string(),
        data_vector: vec![2,3,4,5],
    };
    let raw_string = r#"{"data_int":1,"data_str":"homura","data_vector":[2,3,4,5]}"#;


    // Serialize using `json::encode`
    let encoded = json::encode(&object).unwrap();

    // Deserialize using `json::decode`
    let decoded: TestStruct = json::decode(&encoded).unwrap();
    let raw_decoded: TestStruct = json::decode(raw_string).unwrap();
    println!("{}", encoded);
    println!("{}", decoded.data_str);
    println!("{}", raw_decoded.data_str);
}
