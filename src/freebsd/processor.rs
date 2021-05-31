//
// Sysinfo
//
// Copyright (c) 2021 Guillaume Gomez
//

use crate::ProcessorExt;
use raw_cpuid::{CpuId};

/// Dummy struct that represents a processor.
pub struct Processor {
	pub(crate) frequency: u64,
	pub(crate) vendor_id: String,
	pub(crate) brand: String
}

impl Processor {
    pub(crate) fn new() -> Processor {
        Processor {
			frequency: 0u64,
			vendor_id: String::default(),
			brand: String::default()
		}
    }
}

impl ProcessorExt for Processor {
    fn get_cpu_usage(&self) -> f32 {
        0.0
    }

    fn get_name(&self) -> &str {
        ""
    }

    fn get_frequency(&self) -> u64 {
        self.frequency
    }

    fn get_vendor_id(&self) -> &str {
        &self.vendor_id
    }

    fn get_brand(&self) -> &str {
        &self.brand
    }
}

pub fn get_cpu_frequency() -> u64 {
	let cpuid = CpuId::new();

	match cpuid.get_processor_frequency_info() {
		Some(f) => u64::from(f.processor_base_frequency()),
		None => 0u64
	}
}

pub fn get_cpu_vendor() -> String {
	let cpuid = CpuId::new();

	match cpuid.get_vendor_info() {
		Some(v) => v.as_string().to_string(),
		None => String::default()
	}
}

pub fn get_cpu_brand() -> String {
	let cpuid = CpuId::new();

	match cpuid.get_extended_function_info() {
		Some(f) => {
			let cpu_brand = f.processor_brand_string().unwrap_or("");

			cpu_brand.to_string()
		},
		None => String::default()
	}
}
