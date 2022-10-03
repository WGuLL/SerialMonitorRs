use fltk::{
    app,
    enums::Shortcut,
    menu::{Choice, MenuFlag},
    output::MultilineOutput,
    prelude::*,
    window::Window,
};
use serialport;
use std::rc::Rc;

struct SerialAppCore {
    serial_port: Option<Box<dyn serialport::SerialPort>>,
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

fn main() {
    // backend:
    let serial_port_list = serialport::available_ports();
    let mut core = Rc::<SerialAppCore>::new(SerialAppCore { serial_port: None });

    // GUI
    let app = app::App::default();
    let mut wind_rc = Rc::new(
        Window::default()
            .with_size(400, 200)
            .center_screen()
            .with_label("SerialTerm"),
    );
    let wind = Rc::get_mut(&mut wind_rc).unwrap();
    let mut output_widget = MultilineOutput::default().with_size(wind.width(), 100);
    let mut serial_port_selector = Choice::default()
        .with_size(100, 20)
        .below_of(&output_widget, 32);

    assert!(serial_port_list.is_ok());

    serial_port_list.unwrap().iter().for_each(move |item| {
        let choice_to_add = &item.port_name;
        let mut core_clone = Rc::clone(&mut core);
        serial_port_selector.add(
            choice_to_add.as_str(),
            Shortcut::None,
            MenuFlag::Normal,
            move |choice| {
                let serial_port_to_open = choice.choice();
                assert!(serial_port_to_open.is_some());
                let mut_core = Rc::get_mut(&mut core_clone);
                if mut_core.is_some() {
                    mut_core
                        .unwrap()
                        .open_port(serial_port_to_open.unwrap().as_str(), 9600);
                }
            },
        );
    });

    wind.make_resizable(true);
    wind.end();
    wind.show();

    let window_update_timer = timer::Timer::new();
    let _guard =
        window_update_timer.schedule_repeating(chrono::Duration::milliseconds(500), move || {
            let _result = output_widget.append("test\n");
        });

    /* Event handling */
    app.run().unwrap();
}
