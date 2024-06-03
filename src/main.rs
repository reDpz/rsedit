use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Box as GtkBox, Button, DropDown, HeaderBar, Label,
    Orientation, Statusbar, TextBuffer, TextView,
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
    // vbox holding everything
    let vbox = GtkBox::new(Orientation::Vertical, 0);

    // button for opening files
    let open_button = Button::builder()
        .label("Open file")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // add the functionlity
    open_button.connect_clicked(|_| println!("Opening file"));

    // add it into a dropdown
    let dropdown_opener = DropDown::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    open_button.set_parent(&dropdown_opener);

    // Make header bar
    let title_label = Label::builder().label("rsedit").build();
    let headerbar = HeaderBar::builder()
        .decoration_layout("menu:close")
        .title_widget(&title_label)
        .build();

    let statusbar_buffer_text: &str = "[Untitled buffer]";
    // Create our status bar to display name of buffer
    let statusbar = Statusbar::builder().build();
    statusbar.push(0, &statusbar_buffer_text);
    statusbar.set_height_request(50);

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

    // add our vbox elements
    vbox.append(&headerbar);
    vbox.append(&dropdown_opener);
    vbox.append(&text_view);
    vbox.append(&statusbar);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&vbox)
        .build();

    // Present window
    window.present();
}
