#![allow(unused_imports, dead_code)]

use std::error::Error;

use app::{apphandler::UserEvent, program::Application};
use softbuffer::{Context, Surface};
use winit::{
    event::{DeviceEvent, ElementState},
    event_loop::{DeviceEvents, EventLoop, EventLoopBuilder},
    platform::run_on_demand::EventLoopExtRunOnDemand,
    window::Window,
};

mod app;
mod modules;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build()?;
    let _loop_proxy = event_loop.create_proxy();

    let _ = _loop_proxy.send_event(UserEvent::StartMinimized);

    std::thread::spawn(move || loop {
        let _ = _loop_proxy.send_event(UserEvent::Redraw);

        // sleep for 1/60th of a second
        // i Know its not 60fps whatever i just want to limit the amount of times its updating to a reasonable amount
        std::thread::sleep(std::time::Duration::from_millis(8));
    });

    let mut state = Application::new(&event_loop);

    event_loop.run_app(&mut state).map_err(Into::into)
}
