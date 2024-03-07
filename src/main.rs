use pet_the_cat_3ds_tui::{app::App, handler, localization, ui};
use std::time::Instant;
use ctru::prelude::*;
use ctru::services::cfgu::Cfgu;

fn main() {
    // Initialize the application runtime
    let apt = Apt::new().unwrap();

    // Initialize the controller inputs
    let mut hid = Hid::new().unwrap();

    // Initialize the graphics
    let gfx = Gfx::new().unwrap();

    //gfx.top_screen.borrow_mut().set_wide_mode(true); // Set the screen to wide mode

    // Start a console on the top and bottom screen
    let top_console = Console::new(gfx.top_screen.borrow_mut());
    let bottom_console = Console::new(gfx.bottom_screen.borrow_mut());

    // Initialize the CFGU service to retrieve all wanted information.
    let cfgu = Cfgu::new().expect("Couldn't obtain CFGU controller");

    // Set the current localization to the system's locale.
    localization::set_to_system(cfgu.language().unwrap());
  
    // Initialize the application.
    let mut app = App::new();

    // Load the saved game state.
    app.load();
    
    // Initialize the timer
    let mut time = Instant::now();

    let mut refresh = false;

    while apt.main_loop() && app.running {
        gfx.wait_for_vblank();

        if is_time_elapsed(&mut time) {
            refresh = true;

            // Update the application
            app.update();
        }

        // Handle the key events
        if handler::handle_key_events(&mut hid, &mut app) {
            refresh = true;
        }

        if refresh {            
            // Print the user interface
            ui::print(&app, &top_console, &bottom_console);

            refresh = false;
        }
    }
}

fn is_time_elapsed(time: &mut Instant) -> bool {
    let seconds = time.elapsed().as_secs();

    if seconds > 0 {
        *time = Instant::now();
        return true;
    }

    false
}
