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

use std::collections::HashMap;
use std::path::PathBuf;

mod network_info;

#[derive(Debug, Clone)]
pub struct ConfigDrive {
    pub interface_map: HashMap<String, network_info::Network>,
    pub dns: Vec<String>,
}

impl ConfigDrive {
    pub fn new(root: &Option<String>) -> Option<ConfigDrive> {
        let paths = vec![
            "openstack/latest/network_info.json",
            "openstack/latest/vendor_data.json",
            ];
        for path in paths {
            let data_path = PathBuf::from(path);
            debug!("Trying Path: {}", path);
            match network_info::NetworkInfo::new(root, &data_path) {
                Some(info) => return Some(ConfigDrive{
                    interface_map: info.get_interface_map(),
                    dns: info.get_dns()
                }),
                None => {},
            };
        }
        return None;
    }
}
