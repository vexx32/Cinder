#![allow(dead_code)]
use mira::vulkan::{VkDevice, VkAllocationCallbacks};

use crate::vulkan::safe::{functions::create_device, };

use super::{physical_device::PhysicalDevice, device_create_info::DeviceCreateInfo};

/// LogicalDevice is a software representation of the GPU
pub struct LogicalDevice {
    device: VkDevice,
}

/// LogicalDevice implementation
impl LogicalDevice {
    /// Creates a new LogicalDevice from the PhysicalDevice
    /// 
    /// # Example
    /// 
    /// ```rs
    /// let logical_device = LogicalDevice::new(physical_device, create_info, allocator);
    /// ```
    pub fn new<T: Clone>(physical_device: PhysicalDevice, create_info: DeviceCreateInfo<T>, allocator: Option<VkAllocationCallbacks>) -> Self {
        LogicalDevice {
            device: create_device(physical_device.clone(), Some(create_info), allocator).unwrap(),
        }
    }
}