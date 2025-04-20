use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};
use gtk4::gio::{Menu, MenuItem, SimpleAction};

const APP_ID: &str = "org.gtk_rs.GObjectList";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    
    // Display of the menu (File, Edit...)

    let menubar = Menu::new(); 
    
    let menu = Menu::new(); 

    // File

    let menu_item_file = MenuItem::new(Some("File"), None);
    menu_item_file.set_submenu(Some(&menu));
    menubar.append_item(&menu_item_file);

        // New Window

        let section0_0 = Menu::new();

        let menu_item_new_window = MenuItem::new(Some("New Window"), Some("app.new_window"));
        section0_0.append_item(&menu_item_new_window);

        menu.append_section(None,&section0_0);

        // Open...
        
        let section0_1 = Menu::new();

        let menu_item_open = MenuItem::new(Some("Open..."), Some("app.open"));
        section0_1.append_item(&menu_item_open);

        // Open From Clipboard...
        
        let menu_item_open_from_clipboard = MenuItem::new(Some("Open From Clipboard..."), Some("app.open_clipboard"));
        section0_1.append_item(&menu_item_open_from_clipboard);

        // Open Recent
        
        let menu_item_open_recent = MenuItem::new(Some("Open Recent"), Some("app.open_recent"));
        section0_1.append_item(&menu_item_open_recent);

        menu.append_section(None, &section0_1);

        // Export Data As

        let section0_2 = Menu::new();

        // JSON 

        let export_submenu = Menu::new();

        let menu_item_export_json = MenuItem::new(Some("JSON"), Some("app.export_json"));
        export_submenu.append_item(&menu_item_export_json);

        // YAML

        let menu_item_export_yaml = MenuItem::new(Some("YAML"), Some("app.export_json"));
        export_submenu.append_item(&menu_item_export_yaml);

        // CSV

        let menu_item_export_csv = MenuItem::new(Some("CSV"), Some("app.export_json"));
        export_submenu.append_item(&menu_item_export_csv);

        // XML

        let menu_item_export_xml = MenuItem::new(Some("XML"), Some("app.export_json"));
        export_submenu.append_item(&menu_item_export_xml);

        // Create the Export Data As menu item and set its submenu
        let menu_item_export = MenuItem::new(Some("Export Data As"), None);
        menu_item_export.set_submenu(Some(&export_submenu));
        section0_2.append_item(&menu_item_export);

        menu.append_section(None, &section0_2);

        // Close

        let section0_3 = Menu::new();

        let menu_item_close = MenuItem::new(Some("Close"), Some("app.open"));
        section0_3.append_item(&menu_item_close);

        menu.append_section(None, &section0_3);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .build();

    // Imprescindible para mostrar el menú
    window.set_show_menubar(true);
    app.set_menubar(Some(&menubar));
    
    // Present window
    window.present();
}