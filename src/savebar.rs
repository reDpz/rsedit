use gtk::prelude::*;
use gtk::{Button, DropDown, HeaderBar, Label};
pub struct SaveBar {
    pub bar: HeaderBar,
    pub title_label: Label,
    pub save_button: Button,
}
impl SaveBar {
    pub fn new(label_text: &str) -> SaveBar {
        let bar = HeaderBar::builder().build();
        // create title
        let title_label = Label::new(Some(label_text));
        // insert title into header
        bar.set_title_widget(Some(&title_label));

        // Save button
        let save_button = Button::from_icon_name("media-floppy");
        save_button.set_tooltip_text(Some("Save"));
        save_button.connect_clicked(|_| println!("Saving..."));
        // add it to bar
        bar.pack_end(&save_button);

        // dropdown button with the option to open/new
        let open_btn = Button::builder().label("Open file").build();

        // dropdown_btn = DropDown::new(model, expression);

        SaveBar {
            bar,
            title_label,
            save_button,
        }
    }
    pub fn get_bar(&self) -> &HeaderBar {
        return &self.bar;
    }
}
