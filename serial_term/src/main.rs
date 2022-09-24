use fltk::{app, menu::Choice, output::MultilineOutput, prelude::*, window::Window};
use serial2::SerialPort;
fn main() {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(400, 200)
        .center_screen()
        .with_label("SerialTerm");
    let output_widget = MultilineOutput::default().with_size(wind.width(), 100);
    let mut serial_port_selector = Choice::default()
        .with_size(100, 20)
        .below_of(&output_widget, 32);
    let serial_port_list = SerialPort::available_ports();

    assert!(serial_port_list.is_ok());

    serial_port_list.unwrap().iter().for_each(move |item| {
        let choice_to_add = item.to_str();
        if choice_to_add.is_some() {
            serial_port_selector.add_choice(choice_to_add.unwrap());
        }
    });

    wind.make_resizable(true);
    wind.end();
    wind.show();
    /* Event handling */
    app.run().unwrap();
}
