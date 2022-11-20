use std::ffi::{CString, CStr};

use crate::surface::Surface;

#[derive(Default)]
pub struct InstanceBuilder<'a> {
    /// The application name.
    app_name: Option<&'static str>,
    surface: Option<&'a Surface>,
}

impl<'a> InstanceBuilder<'a> {
    /// Create a new [vulkan instance builder](InstanceBuilder).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the application name.
    ///
    /// # Arguments
    ///
    /// * `app_name` - The application name.
    ///
    pub fn set_app_name(mut self, app_name: &'static str) -> Self {
        self.app_name = Some(app_name);
        self
    }

    /// Set the surface.
    ///
    /// # Arguments
    ///
    /// * `surface` - A reference to the surface.
    pub fn set_surface(mut self, surface: &'a Surface) -> Self {
        self.surface = Some(surface);
        self
    }

    /// Create a new [Vulkan Instance](VkInstance).
    pub fn build(&self) -> Result<Instance, ash::vk::Result> {
        // We can call unwrap() for create a new CString because
        // we know that the app name string don't contains the null
        // string character '\0'.
        let app_name = CString::new(
            self.app_name.unwrap_or("No application name !")
        ).unwrap();

        let surface = self.surface.expect("Surface is missing");

        Instance::new(app_name, surface, &[], &[])
    }
}

/// The vulkan debug callback.
unsafe extern "system" fn vulkan_debug_callback(
    message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> ash::vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number: i32 = callback_data.message_id_number as i32;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        std::borrow::Cow::from("")
    } else {
        std::ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        std::borrow::Cow::from("")
    } else {
        std::ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{:?}:\n{:?} [{} ({})] : {}\n",
        message_severity,
        message_type,
        message_id_name,
        &message_id_number.to_string(),
        message,
    );

    ash::vk::FALSE
}

/// A wrapper on top of the [ash::Instance].
pub struct Instance {
    _raw_entry: ash::Entry,

    /// The raw vulkan instance.
    raw_instance: ash::Instance,

    /// The raw surface.
    raw_surface: ash::vk::SurfaceKHR,

    /// The raw surface lader.
    raw_surface_loader: ash::extensions::khr::Surface,

    /// The debug utilities loader (only in debug mode)
    #[cfg(debug_assertions)]
    debug_utils_loader: ash::extensions::ext::DebugUtils,

    /// The debug utilities callback (only in debug mode)
    #[cfg(debug_assertions)]
    debug_call_back: ash::vk::DebugUtilsMessengerEXT
}

impl Instance {
    /// Create a new [Vulkan Instance](Instance).
    ///
    /// # Arguments
    ///
    /// * `app_name`    - The name of the application.
    /// * `layers`      - An array that contains the vulkan layer names to load.
    /// * `extensions`  - An array that contains the vulkan extension names to load.
    ///
    /// # Panics
    ///
    /// The [instance](Instance) creation can panics if the vulkan
    /// library can't loading or the entry point is missing.
    ///
    fn new(
        app_name: CString,
        surface: &Surface,
        layers: &[CString],
        extensions: &[CString]
    ) -> Result<Self, ash::vk::Result> {
        use ash::{vk, Entry};

        // We can called the .unwrap() here because we know that the
        // string haven't the null byte '\0' at the end...
        let engine_name = CString::new("VoxelEngine").unwrap();
        let engine_version = vk::make_api_version(0, 1, 0, 0);

        let entry = match unsafe { Entry::load() } {
            Ok(entry) => entry,
            Err(e) => panic!("Failed to load Vulkan entry: {}", e),
        };

        let app_info = vk::ApplicationInfo::builder()
            .api_version(vk::make_api_version(0, 1, 3, 0))
            .engine_version(engine_version)
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_name(engine_name.as_c_str())
            .application_name(&app_name);

        let mut layer_names: Vec<&CStr> = Vec::new();

        for layer_name in layers {
            layer_names.push(layer_name.as_c_str())
        }

        if cfg!(debug_assertions) {
            let validation_layer_name = CStr::from_bytes_with_nul(b"VK_LAYER_KHRONOS_validation\0").unwrap();
            layer_names.push(validation_layer_name);
        }

        let mut extension_names: Vec<&CStr> = Vec::new();

        for extension_name in extensions {
            extension_names.push(extension_name.as_c_str());
        }

        if cfg!(debug_assertions) {
            extension_names.push(ash::extensions::ext::DebugUtils::name());
        }

        // Add the surface required extensions to the instance
        // extension list...
        let surface_extensions = ash_window::enumerate_required_extensions(surface.display)
            .unwrap();

        let mut surface_extensions = surface_extensions
            .iter()
            .map(|c_str| unsafe { CStr::from_ptr(*c_str) })
            .collect();

        extension_names.append(&mut surface_extensions);

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            use ash::vk::{ KhrPortabilityEnumerationFn, KhrGetPhysicalDeviceProperties2Fn };

            extension_names.push(KhrPortabilityEnumerationFn::name().as_ptr());
            // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
            extension_names.push(KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
        }

        let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::default()
        };

        let layer_names = layer_names.iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect::<Vec<*const i8>>();

        let extension_names = extension_names.iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect::<Vec<*const i8>>();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&layer_names)
            .enabled_extension_names(&extension_names)
            .flags(create_flags)
            .build();

        let raw_instance = unsafe {
            entry.create_instance(&create_info, None)?
        };

        #[cfg(debug_assertions)]
        let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            // All severity...
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
              | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
              | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
            //   | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
            )
            // All type...
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
              | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
              | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(vulkan_debug_callback))
            .build();

        #[cfg(debug_assertions)]
        let debug_utils_loader = ash::extensions::ext::DebugUtils::new(&entry, &raw_instance);

        #[cfg(debug_assertions)]
        let debug_call_back = unsafe {
            debug_utils_loader
                .create_debug_utils_messenger(&debug_info, None)
                .unwrap()
        };

        let surf = unsafe {
            let surface = ash_window::create_surface(
                &entry,
                &raw_instance,
                surface.display,
                surface.window,
                None
            ).unwrap();

            (Some(surface), Some(ash::extensions::khr::Surface::new(&entry, &raw_instance)))
        };

        Ok(Self {
            _raw_entry: entry,
            raw_instance,
            raw_surface: surf.0.expect("Failed to create surface"),
            raw_surface_loader: surf.1.expect("Failed to create surface loader"),

            #[cfg(debug_assertions)]
            debug_utils_loader,

            #[cfg(debug_assertions)]
            debug_call_back,
        })
    }

    pub fn request_device_queue(&self) {

    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {

            // Destroy the surface properly...
            self.raw_surface_loader.destroy_surface(self.raw_surface, None);

            // Destroy the debug utilities...
            self.debug_utils_loader.destroy_debug_utils_messenger(
                self.debug_call_back,
                None
            );

            // Destroy the instance properly...
            self.raw_instance.destroy_instance(None);
        }
    }
}

// impl Device {
//     /// Create a new [device](Device) from an [instance](Instance).
//     ///
//     /// # Arguments
//     ///
//     /// * `instance` - A reference to the instance.
//     ///
//     pub fn new(instance: &Instance) -> Result<Device, String> {
//         let surface = instance.raw_surface;
//         let surface_loader = &instance.raw_surface_loader;
//         let instance = &instance.raw_instance;

//         let devices = unsafe {
//             instance.enumerate_physical_devices().unwrap()
//         };

//         let device_queue = devices.iter().find_map(|device| {
//             let queue_family_properties = unsafe {
//                 instance.get_physical_device_queue_family_properties(*device)
//             };

//             queue_family_properties
//                 .iter()
//                 .enumerate()
//                 .find_map(|(index, properties)| {

//                     let support_surface = unsafe {
//                         surface_loader.get_physical_device_surface_support(
//                             *device,
//                             index as u32,
//                             surface
//                         ).unwrap()
//                     };

//                     if properties.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS | ash::vk::QueueFlags::COMPUTE)
//                      && support_surface {
//                         Some((*device, index))
//                     } else {
//                         None
//                     }
//                 })
//         });

//         let (device, queue_family_index) = device_queue
//             .ok_or("Couldn't find suitable physical device.".to_string())?;

//         let mut device_extension_names = Vec::<&CStr>::new();

//         device_extension_names.push(ash::extensions::khr::Swapchain::name());

//         #[cfg(any(target_os = "macos", target_os = "ios"))]
//         device_extension_names.push(ash::vk::KhrPortabilitySubsetFn::name());

//         let features = ash::vk::PhysicalDeviceFeatures::builder()
//             .shader_clip_distance(true);

//         let queue_info = ash::vk::DeviceQueueCreateInfo::builder()
//             .queue_family_index(queue_family_index as u32)
//             .queue_priorities(&[1.0]);

//         let device_extension_names = device_extension_names.iter()
//             .map(|e| e.as_ptr())
//             .collect::<Vec<*const i8>>();

//         let device_create_info = ash::vk::DeviceCreateInfo::builder()
//             .queue_create_infos(std::slice::from_ref(&queue_info))
//             .enabled_extension_names(&device_extension_names)
//             .enabled_features(&features);

//         let raw_device = unsafe {
//             instance
//                 .create_device(
//                     device,
//                     &device_create_info,
//                     None
//                 ).unwrap()
//         };

// //        let present_queue = unsafe {
// //            raw_device.get_device_queue(queue_family_index as u32, 0);
// //        };

//         let surface_format = unsafe {
//             surface_loader
//                 .get_physical_device_surface_formats(device, surface)
//                 .unwrap()[0]
//         };

//         println!("Surface format: {:#?}", surface_format);

//         Ok(Device { raw_device })
//     }
// }

// impl Drop for Device {
//     fn drop(&mut self) {
//         unsafe {
//             self.raw_device.destroy_device(None);
//         }
//     }
// }

// pub struct Queue { }

// impl Queue {
//     /// Create a new [queue](Queue)
//     pub fn new(_device: &Device) -> Self {
//         // device.raw_device.get_device_queue(queue_family_index, queue_index)
//         Self { }
//     }
// }