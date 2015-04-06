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

use std::cmp::Ordering;

use ::config_drive::network_info::Network;
use self::InterfaceType::{Static, Dhcp};

#[derive(Debug)]
pub enum InterfaceType {
    Static(Network),
    Dhcp(String),
}

impl InterfaceType {
    fn get_key(&self) -> String {
        match *self {
            Static(ref network) => return network.id.clone(),
            Dhcp(ref network) => return network.clone(),
        }
    }
}

impl PartialOrd for InterfaceType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.get_key().partial_cmp(&other.get_key());
    }
}

impl PartialEq for InterfaceType {
    fn eq(&self, other: &Self) -> bool {
        return self.get_key().eq(&other.get_key());
    }
}

impl Eq for InterfaceType {}

impl Ord for InterfaceType {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.get_key().cmp(&other.get_key());
    }
}
