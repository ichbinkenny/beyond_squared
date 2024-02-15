use hidapi::*;
use std::error::Error;
use egui::*;
use eframe::{egui, App};

const BEYOND_VID: u16 = 0x35BD;
const BEYOND_PID: u16 = 0x0101;
const SET_LED_COLOR_CMD: u8 = 0x4C;

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
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_color: Color32::default(),
            device: None,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            if egui::color_picker::color_picker_color32(ui, &mut self.current_color, color_picker::Alpha::Opaque) {
                if let Some(dev) = self.device.as_mut() {
                    set_beyond_led_color(dev, self.current_color.to_array());
                }
            }
        });
    }
}

impl MyApp {
    pub fn new(dev: HidDevice) -> Self {
        Self {
            current_color: Color32::default(),
            device: Some(dev),
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
