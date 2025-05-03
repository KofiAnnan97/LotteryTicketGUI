use gtk::{prelude::*, Button};
use gtk::gio::{Menu, MenuItem, MenuModel, ActionEntry};
use gtk::{
    glib, Application, ApplicationWindow, PopoverMenuBar,
};

mod ticket;

const APP_ID: &str = "org.gtk_rs.LotteryTicketGUI";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_menu);
    app.connect_activate(build_ui);
    
    // Run the application
    app.run()
}

fn build_menu(app: &Application) {
    let generate =  ActionEntry::builder("generate")
        .activate(|_, _, _| println!("Go to ticket generator panel"))
        .build();

    let history =  ActionEntry::builder("history")
        .activate(|_, _, _| println!("Go to ticket history panel"))
        .build();
    
    let new_preset =  ActionEntry::builder("new_preset")
        .activate(|_, _, _| println!("Go to preset creator panel"))
        .build();

    let default_preset =  ActionEntry::builder("default_preset")
        .activate(|_, _, _| println!("Set default preset"))
        .build();

    let saved_presets = ActionEntry::builder("saved_presets")
        .activate(|_, _, _| println!("Go to saved preset panel"))
        .build();

    let quit = ActionEntry::builder("quit")
        .activate(|app: &gtk::Application, _, _| app.quit())
        .build();

    app.add_action_entries([generate, history, new_preset, default_preset, saved_presets, quit]);

    let menubar = {
        let tickets_menu = {
            let generate_item = MenuItem::new(Some("Generate"), Some("app.generate"));
            let history_item = MenuItem::new(Some("History"), Some("app.history"));
            let quit_item = MenuItem::new(Some("Quit"), Some("app.quit"));

            let tickets_menu = Menu::new();
            tickets_menu.append_item(&generate_item);
            tickets_menu.append_item(&history_item);
            tickets_menu.append_item(&quit_item);
            tickets_menu
        };
        let presets_menu: Menu = {
            let new_item = MenuItem::new(Some("New..."), Some("app.new_preset"));
            let default_item = MenuItem::new(Some("Default"), Some("app.default_preset"));
            let saved_item = MenuItem::new(Some("Saved"), Some("app.saved_presets"));

            let presets_menu = Menu::new();
            presets_menu.append_item(&new_item);
            presets_menu.append_item(&default_item);
            presets_menu.append_item(&saved_item);
            presets_menu
        };

        let menubar = Menu::new();
        menubar.append_submenu(Some("Tickets"), &tickets_menu);
        menubar.append_submenu(Some("Presets"), &presets_menu);
        menubar
    };
    app.set_menubar(Some(&menubar));
}

fn build_ui(app: &Application) {
    let start_button = Button::builder()
        .label("Get Started!")
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    start_button.connect_clicked(|start_button: &Button|{
        let limits = vec!(70, 70, 70, 70, 70, 24);
        let ticket_nums = ticket::generate(limits);
        println!("{:?}", ticket_nums);
    });

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Lottery Ticket Generator")
        .default_width(400)
        .default_height(600)
        .show_menubar(true)
        .child(&start_button)
        .build();

    // Present window
    window.present();
}