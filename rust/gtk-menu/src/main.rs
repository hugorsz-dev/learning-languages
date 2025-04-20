use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, gio, glib};
use gio::{Menu, MenuItem, SimpleAction};

fn main() {
    let application = Application::builder()
        .application_id("com.github.yourname.menu1")
        .build();

    application.connect_startup(|app| {
        build_menu(app);
    });

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Menu Example"));
        window.set_default_size(400, 300);
        window.set_show_menubar(true);
        window.present();
    });

    application.run();
}

fn build_menu(app: &Application) {
    // Acción para programar el quit
    let quit_action = SimpleAction::new("quit", None);
    quit_action.connect_activate(glib::clone!(@weak app => move |_, _| {
        app.quit();
    }));
    app.add_action(&quit_action);

    // Menú principal
    let menubar = Menu::new(); // Menú principal, del que penden el resto de menús (File, Edit...)
    
    // Submenú 
    let menu = Menu::new(); // Subḿenú, donde irán los diferentes items (File, Edit...)

    // Creación del ítem "File" para el submenú, y asociacion del mismo con el menú y el submenú
    let menu_item_file = MenuItem::new(Some("File"), None);
    menu_item_file.set_submenu(Some(&menu));
    menubar.append_item(&menu_item_file);
        // Elementos pendientes de "File"
        let menu_item_quit = MenuItem::new(Some("Quit"), Some("app.quit"));
        menu.append_item(&menu_item_quit);

    // Set the menubar
    app.set_menubar(Some(&menubar));
}