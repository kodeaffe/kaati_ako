//! Contains code to build the user interface

mod dialogs;
mod widgets;

use gdk::Screen;
use gio::ActionMapExt;
use gtk::{ContainerExt, GtkApplicationExt, GtkWindowExt, WidgetExt};

use dialogs::about::About;
use dialogs::addcard::AddCard;
use dialogs::deletecard::DeleteCard;
use widgets::card::Card;
use widgets::content::Content;


/// The name of content widget which contains the flash card
const WIDGET_NAME_CONTENT: &str = "content";
/// The name of flash card's widget
const WIDGET_NAME_CARD: &str = "card";


/// The application's user interface
pub struct UI;

/// The implementation of the application's user interface
impl UI {
    /// Add accelerators for the application
    ///
    /// Currently it handles 'F1', 'n' and 'Ctrl-Q'
    fn add_accelerators(app: &gtk::Application) {
        app.set_accels_for_action("app.about", &["F1"]);
        app.set_accels_for_action("app.next_card", &["n"]);
        // `Primary` is a platform-agnostic accelerator modifier.
        // On Windows and Linux, `Primary` maps to the `Ctrl` key,
        // and on macOS it maps to the `command` key.
        app.set_accels_for_action(
            "app.add_card", &["<Primary>A"]);
        app.set_accels_for_action(
            "app.delete_card", &["<Primary>D"]);
        app.set_accels_for_action("app.quit", &["<Primary>Q"]);
    }

    /// Add actions for the application
    fn add_actions(app: &gtk::Application, window: &gtk::ApplicationWindow) {
        let quit = gio::SimpleAction::new("quit", None);
        quit.connect_activate(glib::clone!(@weak window => move |_, _| {
            window.close();
        }));
        app.add_action(&quit);

        let about = gio::SimpleAction::new("about", None);
        about.connect_activate(glib::clone!(@weak window => move |_, _| {
            About::show(&window);
        }));
        app.add_action(&about);

        let add_card = gio::SimpleAction::new("add_card", None);
        add_card.connect_activate(glib::clone!(@weak window => move |_, _| {
            AddCard::show(&window);
        }));
        app.add_action(&add_card);

        let delete_card = gio::SimpleAction::new(
            "delete_card", None);
        delete_card.connect_activate(glib::clone!(@weak window => move |_, _| {
            DeleteCard::show(&window);
        }));
        app.add_action(&delete_card);

        let next_card = gio::SimpleAction::new("next_card", None);
        next_card.connect_activate(glib::clone!(@weak window => move |_, _| {
            Card::replace(&window, 0);
        }));
        app.add_action(&next_card);
    }

    /// Build the application's user interface with window parametrisation and widgets
    pub fn build(app: &gtk::Application) {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Kaati Ako");
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);
        match Screen::get_default() {
            Some(screen) => {
                let width = screen.get_width() / 2;
                let height = screen.get_height() / 2;
                window.set_default_size(width, height);
            },
            None => { window.set_default_size(350, 70); }
        }
        window.add(&Content::build(&window));
        UI::build_menu(app);
        UI::add_accelerators(app);
        UI::add_actions(app, &window);
        window.show_all();
    }

    /// Build the application's menu
    fn build_menu(app: &gtk::Application) {
        let menu = gio::Menu::new();
        menu.append(Some("Quit"), Some("app.quit"));
        app.set_app_menu(Some(&menu));

        let card_menu = gio::Menu::new();
        card_menu.append(Some("Add card"), Some("app.add_card"));
        card_menu.append(Some("Edit current card"), Some("app.edit_card"));
        card_menu.append(
            Some("Delete current card"), Some("app.delete_card"));

        let about_menu = gio::Menu::new();
        about_menu.append(Some("About"), Some("app.about"));

        let menu_bar = gio::Menu::new();
        menu_bar.append_submenu(Some("Card"), &card_menu);
        menu_bar.append_submenu(Some("?"), &about_menu);
        app.set_menubar(Some(&menu_bar));
    }
}