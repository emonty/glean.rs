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

extern crate gleam;
use std::path::Path;
use std::env;
use gleam::{options, get_network_info};

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
