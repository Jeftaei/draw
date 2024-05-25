#![allow(unused_imports, dead_code)]

use std::error::Error;

use app::{
    apphandler::{TrayEvent, UserEvent},
    program::Application,
};
use softbuffer::{Context, Surface};
use trayicon::{Icon, MenuBuilder, TrayIcon, TrayIconBuilder};
use winit::{
    event::{DeviceEvent, ElementState},
    event_loop::{DeviceEvents, EventLoop, EventLoopBuilder},
    window::Window,
};

mod app;
mod art;
mod modules;

fn main() -> Result<(), Box<dyn Error>> {
    let icon = include_bytes!("../assets/pencil.ico");

    let event_loop = EventLoop::<UserEvent>::with_user_event().build()?;
    event_loop.listen_device_events(DeviceEvents::Always);

    let _tray_proxy = event_loop.create_proxy();
    let _loop_proxy = event_loop.create_proxy();

    let _ = _loop_proxy.send_event(UserEvent::StartMinimized);

    let tray = TrayIconBuilder::new()
        .sender(move |e: &UserEvent| {
            let _ = _tray_proxy.send_event(*e);
        })
        .icon_from_buffer(icon)
        .on_right_click(TrayEvent::RightClick.into())
        .menu(MenuBuilder::new().item("E&xit", TrayEvent::Exit.into()))
        .build()
        .unwrap();

    std::thread::spawn(move || loop {
        let _ = _loop_proxy.send_event(UserEvent::Redraw);

        // sleep for ~1/30th of a second
        // i Know its not 30fps whatever i just want to limit the amount of times its updating to a reasonable amount
        std::thread::sleep(std::time::Duration::from_millis(16));
    });

    let mut state = Application::new(&event_loop, tray);

    event_loop.run_app(&mut state).map_err(Into::into)
}
