//! Module for the content widget

use gtk::{ActionBarExt, BoxExt, ButtonExt, WidgetExt};

use crate::ui::WIDGET_NAME_CONTENT;
use super::cardnotebook::CardNotebook;


/// The application's content widget
pub struct Content;

/// The implementation of the application's content widget
impl Content {
    /// Build the action bar
    ///
    /// # Arguments
    ///
    /// * `window` - The GTK application window
    fn build_action_bar(window: &gtk::ApplicationWindow) -> gtk::ActionBar {
        let action_bar = gtk::ActionBar::new();
        let next = gtk::Button::from_icon_name(
            Some("go-next"), gtk::IconSize::Button);
        next.connect_clicked(glib::clone!(@weak window => move |_| {
            CardNotebook::replace(&window, 0);
        }));
        action_bar.pack_start(&next);
        let label = gtk::Label::new(Some("Press button or type <n> for next random card."));
        action_bar.pack_start(&label);
        action_bar
    }

    /// Build the application's content area with flash card and action bar
    ///
    /// # Arguments
    ///
    /// * `window` - The GTK application window
    pub fn build(window: &gtk::ApplicationWindow) -> gtk::Box {
        let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
        content.set_widget_name(WIDGET_NAME_CONTENT);

        let card = CardNotebook::build(window, 0);
        content.pack_start(&card, true, true, 10);

        let action_bar = Content::build_action_bar(window);
        content.pack_end(&action_bar, false, false, 0);
        content
    }
}
