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

#[macro_use]
extern crate log;
extern crate glean;
use glean::config_drive::ConfigDrive;
use glean::sys::SysInterfaces;
use glean::live_networks::LiveNetworks;

mod options;

#[cfg(not(test))]
fn main() {

    let opts = options::Opt::new();
    if opts.help {
        println!("{}", opts.usage);
        return;
    }

    let config_drive = ConfigDrive::new(&opts.root);
    match config_drive {
        Some(ref interface) => {
            debug!("Network {:?}", interface.interface_map);
            debug!("DNS {:?}", interface.dns);
        },
        None => {}
    }

    let sys_interfaces = SysInterfaces::new(&opts.root, &opts.interface, opts.noop);

    let live_networks = LiveNetworks::new(&sys_interfaces, config_drive, &opts.root);
    println!("{:?}", live_networks.get_output());
}
