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

pub use super::config_drive::ConfigDrive;
pub use super::sys::SysInterfaces;

pub struct NetworkWriter {
    config_drive: Option<ConfigDrive>,
    interfaces: SysInterfaces,
    root: PathBuf,
    noop: bool,
}

impl NetworkWriter {

    pub fn new(root: &Option<String>, interfaces: &SysInterfaces, config_drive: Option<ConfigDrive>, noop: bool) -> NetworkWriter {
      let base_root_path = match root {
          &Some(ref path) => PathBuf::from(path),
          &None => PathBuf::from("/"),
      };
      NetworkWriter {
          config_drive: config_drive,
          interfaces: interfaces.clone(),
          root: base_root_path,
          noop: noop,
      }
    }
}
