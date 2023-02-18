use crate::egui::Color32;
use crate::egui::RichText;
use eframe::egui;
use std::string::String;

#[derive(Default)]
struct SerialAppCore {
    serial_port: Option<Box<dyn serialport::SerialPort>>,
    text_buffer: String,
    selected_index: usize,
}

impl SerialAppCore {
    pub fn open_port(&mut self, port_name: &str, baud_rate: u32) -> Result<(), String> {
        // TODO assert baud rate is standard baud rate.
        // assert port_name not empty.
        let serial_port_builder = serialport::new(port_name, baud_rate);
        let serial_port_err_code = serial_port_builder.open();
        if let Ok(serial_port) = serial_port_err_code {
            self.serial_port = Some(serial_port);
            return Ok(());
        }
        Err(serial_port_err_code.err().unwrap().description)
    }
}

impl eframe::App for SerialAppCore {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Serial Monitor");
            ui.add(egui::TextEdit::multiline(&mut self.text_buffer).interactive(false));
            let serial_port_list = serialport::available_ports().unwrap();
            let serial_port_choice = egui::ComboBox::from_label("Serial port");

            serial_port_choice.show_index(
                ui,
                &mut self.selected_index,
                serial_port_list.len(),
                |i| serial_port_list[i].port_name.clone(),
            );

            if serial_port_list.is_empty() {
                self.serial_port = None;
                return;
            }

            if self.serial_port.is_none() {
                let defaultbaudrate = 9600;
                let result = self.open_port(
                    serial_port_list[self.selected_index].port_name.as_str(),
                    defaultbaudrate,
                );
                if result.is_err() {
                    ui.colored_label(Color32::RED, RichText::new(result.err().unwrap()));
                    return;
                }
            }

            let serial_port_ptr = self.serial_port.as_ref().unwrap().as_ref();
            let serial_port_name = serial_port_ptr.name().unwrap();
            if serial_port_list[self.selected_index].port_name != serial_port_name {
                let defaultbaudrate = 9600;
                let result = self.open_port(
                    serial_port_list[self.selected_index].port_name.as_str(),
                    defaultbaudrate,
                );
                if result.is_err() {
                    ui.colored_label(Color32::RED, RichText::new(result.err().unwrap()));
                    return;
                }
            }

            let _result = self
                .serial_port
                .as_mut()
                .unwrap()
                .as_mut()
                .read_to_string(&mut self.text_buffer);
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _error_code = eframe::run_native(
        "Serial Monitor",
        options,
        Box::new(|_cc| Box::<SerialAppCore>::default()),
    );
}
