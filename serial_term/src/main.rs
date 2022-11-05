use eframe::egui;
use serialport;
use std::string::String;

#[derive(Default)]
struct SerialAppCore {
    serial_port: Option<Box<dyn serialport::SerialPort>>,
    textBuffer: String,
    selectedIndex: usize,
}

impl SerialAppCore {
    pub fn open_port(&mut self, port_name: &str, baud_rate: u32) {
        // TODO assert baud rate is standard baud rate.
        // assert port_name not empty.
        let serial_port_builder = serialport::new(port_name, baud_rate);
        let serial_port_err_code = serial_port_builder.open();
        if serial_port_err_code.is_ok() {
            self.serial_port = Some(serial_port_err_code.unwrap());
        }
    }
}

impl eframe::App for SerialAppCore {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Serial Monitor");
            ui.add(egui::TextEdit::multiline(&mut self.textBuffer).interactive(false));
            let serial_port_list = serialport::available_ports().unwrap();
            let mut serial_port_choice = egui::ComboBox::from_label("Serial port:");
            /*           if self.serial_port.is_none()
                       {
            serial_port_choice.selected_text ("none");
                       }
                       else
                       {
            serial_port_choice.selected_text (serial_port.unwrapped ().);
                       }
            */
            serial_port_choice.show_index(
                ui,
                &mut self.selectedIndex,
                serial_port_list.len(),
                |i| serial_port_list[i].port_name.clone(),
            );
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Serial Monitor",
        options,
        Box::new(|_cc| Box::new(SerialAppCore::default())),
    );
}
