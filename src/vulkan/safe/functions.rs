use std::mem::zeroed;
use std::ptr;

use mira::mem::zeroed_vec;
use mira::vulkan::{VkPhysicalDevice, VK_SUCCESS, VkPhysicalDeviceProperties, VkQueueFamilyProperties, VkSurfaceKHR};
use mira::vulkan::{VkInstanceCreateInfo, VkAllocationCallbacks, VkInstance};

use crate::vulkan::r#unsafe::unsafe_functions::*;

pub(crate) fn create_instance(
    create_info: Option<VkInstanceCreateInfo>, 
    allocator: Option<VkAllocationCallbacks>, 
) -> Result<VkInstance, i32> {
    unsafe {
        let mut instance: VkInstance = zeroed();
        let result = vkCreateInstance(
            match create_info {
                Some(create_info) => &create_info as *const _,
                None => ptr::null(),
            },
            match allocator {
                Some(mut allocator) => &mut allocator as *mut _,
                None => ptr::null_mut(),
            },
            &mut instance,
            None
        );
        match result {
            VK_SUCCESS => Ok(instance),
            _ => Err(result),
        }
    }
}

pub(crate) fn get_physical_devices(
    instance: VkInstance,
) -> Result<Vec<VkPhysicalDevice>, i32> {
    let mut amount = 0u32;
    match vkEnumeratePhysicalDevices(instance, &mut amount, ptr::null_mut(), Some(instance)) {
        VK_SUCCESS => {},
        error => {
            return Err(error);
        }
    }

    let mut devices = unsafe { zeroed_vec::<VkPhysicalDevice>(amount as usize) };
    match vkEnumeratePhysicalDevices(instance, &mut amount, devices.as_mut_ptr(), Some(instance)) {
        VK_SUCCESS => {},
        error => {
            return Err(error);
        }
    }
    Ok(devices)
}

pub(crate) fn get_physical_device_properties(
    physical_device: VkPhysicalDevice,
    instance: VkInstance,
) -> VkPhysicalDeviceProperties {
    let mut properties = unsafe { zeroed::<VkPhysicalDeviceProperties>() };
    vkGetPhysicalDeviceProperties(physical_device, &mut properties, Some(instance));
    return properties;
}

pub(crate) fn get_physical_device_queue_family_properties(
    physical_device: VkPhysicalDevice,
    instance: VkInstance,
) -> Vec<VkQueueFamilyProperties> {
    let mut amount = 0u32;
    vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &mut amount, ptr::null_mut(), Some(instance));
    
    let mut properties = unsafe { zeroed_vec::<VkQueueFamilyProperties>(amount as usize) };
    vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &mut amount, properties.as_mut_ptr(), Some(instance));
    
    return properties;
}

#[test] 
pub fn optional_surface_test() {
    let instance = create_instance(None, None).unwrap();
    let physical_devices = get_physical_devices(instance).unwrap();
    let physical_device = physical_devices[0];
    let properties = get_physical_device_properties(physical_device, instance);
    println!("{:?}", properties);
}

pub(crate) fn physical_device_surface_support(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    surface: Option<VkSurfaceKHR>,
    instance: VkInstance,
) -> bool {
    let mut supported = 0u32;
    let result = vkGetPhysicalDeviceSurfaceSupportKHR(physical_device, queue_family_index, match surface {
        Some(surface) => surface,
        None => ptr::null_mut(),
    }, &mut supported, Some(instance));
    match result {
        VK_SUCCESS => {},
        _ => {
            panic!("Failed to get physical device surface support");
        }
    }
    return match supported {
        0 => false,
        _ => true,
    };
}