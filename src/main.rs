use hidapi::*;
use std::error::Error;
use egui::*;
use eframe::{egui, App};

const BEYOND_VID: u16 = 0x35BD;
const BEYOND_PID: u16 = 0x0101;
const SET_LED_COLOR_CMD: u8 = 0x4C;
const SET_FAN_SPEED_CMD: u8 = 0x46;

const MIN_FAN_SPEED: u8 = 40;
const MAX_FAN_SPEED: u8 = 100;

// Break command into struct
// Create gui

fn main() -> Result<(), Box<dyn Error>> {
    let api = HidApi::new()?;
    let device = api.open(BEYOND_VID, BEYOND_PID)?;
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native("Beyond^2", options, Box::new(|cc| { Box::new(MyApp::new(device)) }))?;
    Ok(())
}

struct MyApp {
    current_color: Color32,
    device: Option<HidDevice>,
    fan_speed: u8,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_color: Color32::default(),
            device: None,
            fan_speed: 40,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let current_fan_speed = self.fan_speed;
        egui::CentralPanel::default().show(ctx, |ui|{
            if egui::color_picker::color_picker_color32(ui, &mut self.current_color, color_picker::Alpha::Opaque) {
                if let Some(dev) = self.device.as_mut() {
                    set_beyond_led_color(dev, self.current_color.to_array());
                }
            }
            ui.add(egui::Slider::new(&mut self.fan_speed, 40..=100).text("Fan Speed"));
        });
        if self.fan_speed != current_fan_speed {
            if let Some(dev) = self.device.as_mut() {
                set_beyond_fan_speed(&dev, self.fan_speed);
            }
        }
    }
}

impl MyApp {
    pub fn new(dev: HidDevice) -> Self {
        Self {
            current_color: Color32::default(),
            device: Some(dev),
            fan_speed: 40,
        }
    }
}

fn set_beyond_led_color(dev: &HidDevice, rgba: [u8; 4]) {
    let mut command = [0; 65];
    command[1] = SET_LED_COLOR_CMD;
    command[2] = rgba[0];
    command[3] = rgba[1];
    command[4] = rgba[2];
    let _ = dev.send_feature_report(&command);
}

fn set_beyond_fan_speed(dev: &HidDevice, speed: u8) {
    let mut command = [0; 65];
    command[1] = SET_FAN_SPEED_CMD;
    if speed > MAX_FAN_SPEED {
        command[2] = MAX_FAN_SPEED;
    } else if speed < MIN_FAN_SPEED {
        command[2] = MIN_FAN_SPEED;
    } else {
        command[2] = speed;
    }
    let _ = dev.send_feature_report(&command);
}
