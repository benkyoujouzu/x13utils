#![windows_subsystem = "windows"]
use std::time::Duration;
use std::time::Instant;
use tao::{
    event::{Event, StartCause},
    event_loop::{ControlFlow, EventLoopBuilder},
};
use tray_icon::menu::MenuEvent;
use tray_icon::{
    menu::{Menu, MenuItem},
    Icon, TrayIconBuilder,
};
use windows::Devices::Power::Battery;
use x13utils::NumberImage;

fn get_charge_rate() -> i32 {
    let battery = Battery::AggregateBattery().unwrap();
    let report = battery.GetReport().unwrap();
    let charge_rate = report.ChargeRateInMilliwatts().unwrap().Value().unwrap();
    charge_rate
}

fn main() {
    let mut number_image = NumberImage::new();

    let tray_menu = Menu::new();
    let quit_item = MenuItem::new("Quit", true, None);
    tray_menu.append_items(&[&quit_item]).unwrap();

    let icon = Icon::from_rgba(number_image.image.as_raw().clone(), 64, 64).unwrap();
    let tray_icon = TrayIconBuilder::new()
        // .with_tooltip("system-tray - tray icon library!")
        .with_menu(Box::new(tray_menu))
        .with_icon(icon)
        .build()
        .unwrap();

    let menu_channel = MenuEvent::receiver();
    let event_loop = EventLoopBuilder::new().build();

    event_loop.run(move |event, _, control_flow| {
        let update_time = Duration::from_millis(5000);
        *control_flow = ControlFlow::WaitUntil(Instant::now() + update_time);

        match event {
            Event::NewEvents(StartCause::Init)
            | Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                let charge_rate = get_charge_rate() as f32 / 1000.0;
                number_image.update(charge_rate);
                let new_icon =
                    Icon::from_rgba(number_image.image.as_raw().clone(), 64, 64).unwrap();
                tray_icon.set_icon(Some(new_icon)).unwrap();
                // println!("{}", charge_rate);
            }
            _ => {
                if let Ok(event) = menu_channel.try_recv() {
                    if event.id == quit_item.id() {
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }
        }
    });
}
