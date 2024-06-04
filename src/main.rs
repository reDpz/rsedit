mod savebar;

use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Box as GtkBox, Orientation, Statusbar, TextBuffer,
    TextView,
};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create header
    let bar = savebar::SaveBar::new("rsedit");
    // vbox holding everything
    let vbox = GtkBox::new(Orientation::Vertical, 0);

    let statusbar_buffer_text: &str = "[Untitled buffer]";
    // Create our status bar to display name of buffer
    let statusbar = Statusbar::builder().build();
    statusbar.push(0, &statusbar_buffer_text);
    // dont expand
    statusbar.set_vexpand(false);

    // Create a button with label and margins
    // example button

    // create buffer to store our text in
    let text_buffer = TextBuffer::builder().build();

    // Create the text view
    let text_view = TextView::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .buffer(&text_buffer)
        .build();
    text_view.set_vexpand(true);
    text_view.set_hexpand(true);

    // add our vbox elements
    vbox.append(&text_view);
    vbox.append(&statusbar);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&vbox)
        .build();
    window.set_titlebar(Some(bar.get_bar()));

    // Present window
    window.present();
}
