use vulkano_win::VkSurfaceBuild;
use vulkano::instance::{
    Instance,
    InstanceExtensions,
    PhysicalDevice,
};

use winit::{
    event_loop::{
        ControlFlow,
        EventLoop,
    },
    event::{
        Event,
        WindowEvent,
    },
    window::{
        WindowBuilder,
        Window,
    },
};

use crate::font::*;

pub fn run(winsize: [u32; 2]) {
    //initialize vulkan things and gather information
    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("failed to create instance!");
    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available!");
    println!("Using device: {} (type: {:?})", physical.name(), physical.ty());

    //set up window and event loop
    let events_loop = EventLoop::new();
    let surface = WindowBuilder::new()




    events_loop.run(|event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            _ => ()
        }
    });
}
