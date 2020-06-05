use vulkano::instance::{
    Instance,
    InstanceExtensions,
    PhysicalDevice,
};


use crate::font::*;

pub fn start_render() {
    //initialize stuff
    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("failed to create instance!");
    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available!");
    println!("Using device: {} (type: {:?})", physical.name(), physical.ty());

}
