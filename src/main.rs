extern crate winit;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
const WINDOW_SIZE: [u32; 2] = [1920, 1080];
const APP_NAME: &'static str = "Cinema";

fn main() {

    let event_loop = EventLoop::new();

    let (logical_window_size, physical_window_size) = {
        use winit::dpi::{LogicalSize, PhysicalSize};

        let dpi = event_loop.primary_monitor().unwrap().scale_factor();
        let logical: LogicalSize<u32> = WINDOW_SIZE.into();
        let physical: PhysicalSize<u32> = logical.to_physical(dpi);

        (logical, physical)
    };

    let window = WindowBuilder::new()
        .with_title(APP_NAME)
        .with_inner_size(logical_window_size)
        .build(&event_loop)
        .expect("failed to open window");
    //VULKAN

    Vulkan::new();

    //END VULKAN

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(dims) => { }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => { }
                _ => ()
            },
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(_) => {

            },
            _ => ()
        }
    });
}
