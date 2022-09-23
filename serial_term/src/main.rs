use fltk::{app, button::Button, output::MultilineOutput, prelude::*, window::Window};
fn main() {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(400, 200)
        .center_screen()
        .with_label("SerialTerm");
    let mut output_widget = MultilineOutput::default().with_size(wind.width(), 100);
    let _test_button = Button::default()
        .with_size(100, 20)
        .below_of(&output_widget, 32)
        .with_label("test")
        .set_callback(move |_| {
            let _error_code = output_widget.append("test");
        });

    wind.make_resizable(true);
    wind.end();
    wind.show();
    /* Event handling */
    app.run().unwrap();
}
