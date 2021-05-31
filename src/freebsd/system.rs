//
// Sysinfo
//
// Copyright (c) 2021 Guillaume Gomez
//

use crate::sys::component::Component;
use crate::sys::process::*;
use crate::sys::processor::*;
use crate::sys::Disk;
use crate::sys::Networks;
use crate::{LoadAvg, Pid, User, RefreshKind, SystemExt};
use sysctl::Sysctl;

use std::collections::HashMap;

/// Structs containing system's information.
pub struct System {
    processes_list: HashMap<Pid, Process>,
    networks: Networks,
    global_processor: Processor,
}

impl SystemExt for System {
    fn new_with_specifics(refreshes: RefreshKind) -> System {
        let mut s = System {
            processes_list: Default::default(),
            networks: Networks::new(),
            global_processor: Processor::new(),
        };
		s.refresh_specifics(refreshes);

		s
    }

    fn refresh_memory(&mut self) {}

    fn refresh_cpu(&mut self) {
		self.global_processor.frequency = get_cpu_frequency();
		self.global_processor.brand = get_cpu_brand();
		self.global_processor.vendor_id = get_cpu_vendor();
	}

    fn refresh_components_list(&mut self) {}

    fn refresh_processes(&mut self) {}

    fn refresh_process(&mut self, _pid: Pid) -> bool {
        false
    }

    fn refresh_disks_list(&mut self) {}

    fn refresh_users_list(&mut self) {}

    // COMMON PART
    //
    // Need to be moved into a "common" file to avoid duplication.

    fn get_processes(&self) -> &HashMap<Pid, Process> {
        &self.processes_list
    }

    fn get_process(&self, _pid: Pid) -> Option<&Process> {
        None
    }

    fn get_networks(&self) -> &Networks {
        &self.networks
    }

    fn get_networks_mut(&mut self) -> &mut Networks {
        &mut self.networks
    }

    fn get_global_processor_info(&self) -> &Processor {
        &self.global_processor
    }

    fn get_processors(&self) -> &[Processor] {
        &[]
    }

    fn get_physical_core_count(&self) -> Option<usize> {
        None
    }

    fn get_total_memory(&self) -> u64 {
        0
    }

    fn get_free_memory(&self) -> u64 {
        0
    }

    fn get_available_memory(&self) -> u64 {
        0
    }

    fn get_used_memory(&self) -> u64 {
        0
    }

    fn get_total_swap(&self) -> u64 {
        0
    }

    fn get_free_swap(&self) -> u64 {
        0
    }

    fn get_used_swap(&self) -> u64 {
        0
    }

    fn get_components(&self) -> &[Component] {
        &[]
    }

    fn get_components_mut(&mut self) -> &mut [Component] {
        &mut []
    }

    fn get_disks(&self) -> &[Disk] {
        &[]
    }

    fn get_disks_mut(&mut self) -> &mut [Disk] {
        &mut []
    }

    fn get_uptime(&self) -> u64 {
        0
    }

    fn get_boot_time(&self) -> u64 {
        0
    }

    fn get_load_average(&self) -> LoadAvg {
        LoadAvg {
            one: 0.,
            five: 0.,
            fifteen: 0.,
        }
    }

    fn get_users(&self) -> &[User] {
        &[]
    }

    fn get_name(&self) -> Option<String> {
        Some("FreeBSD".to_owned())
    }

    fn get_long_os_version(&self) -> Option<String> {
        None
    }

    fn get_kernel_version(&self) -> Option<String> {
        let ctl = sysctl::Ctl::new("kern.version").unwrap();
        let osver = ctl.value().unwrap();
        Some(osver.to_string())
    }

    fn get_os_version(&self) -> Option<String> {
        let ctl = sysctl::Ctl::new("kern.osrelease").unwrap();
        let osrel = ctl.value().unwrap();
        Some(osrel.to_string())
    }

    fn get_host_name(&self) -> Option<String> {
        let ctl = sysctl::Ctl::new("kern.hostname").unwrap();
        let hostname = ctl.value().unwrap();
        Some(hostname.to_string())
    }
}

impl Default for System {
    fn default() -> System {
        System::new()
    }
}