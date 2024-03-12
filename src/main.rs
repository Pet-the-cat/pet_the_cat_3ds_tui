use pet_the_cat_3ds_tui::{app::App, handler, localization, ui};
use std::time::Instant;
use ctru::{
        applets::error,
        console::Console,
        services::{
            apt::Apt,
            hid::Hid,
            gfx::{Gfx, Side, TopScreen3D},
            cfgu::{Cfgu, Language}
        }
    };

fn main() {
    // Set a panic hook so that if the application panics,
    // it displays the error in the system error applet popup instead of quitting the application without notice.
    error::set_panic_hook(false);

    // Initialize the application runtime
    let mut apt = Apt::new().expect("Couldn't obtain GFX controller");

    // Initialize the controller inputs
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");

    // Initialize the graphics
    let gfx = Gfx::new().expect("Couldn't obtain APT controller");

    // Set cpu time limit
    apt.set_app_cpu_time_limit(5).expect("Couldn't set CPU time limit");

    // Set the top screen to 3D mode
    let top_screen = TopScreen3D::from(&gfx.top_screen);

    // Split the TopScreen3D to get references to the two render surfaces.
    let (left, right) = top_screen.split_mut();

    // Start a console on the top left/right and bottom screen
    let mut left_console = Console::new(left, Side::Left);
    let right_console = Console::new(right, Side::Right);
    let bottom_console = Console::new(gfx.bottom_screen.borrow_mut(), Side::Left);

    // Set the left console window for the 3D effect.
    left_console.set_window(1, 1, left_console.max_width() - 1, 28).expect("Couldn't set window");

    // Initialize the CFGU service to retrieve all wanted information.
    let cfgu = Cfgu::new().expect("Couldn't obtain CFGU controller");

    // Set the current localization to the system's locale.
    localization::set_to_system(cfgu.language().unwrap_or(Language::English));
  
    // === Main loop ===
    let mut app = get_app();
    let mut time = Instant::now();
    let mut refresh = false;

    while apt.main_loop() && app.running {
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
            ui::print_3d(&app, &left_console, &right_console, &bottom_console);

            refresh = false;
            gfx.wait_for_vblank();
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

#[cfg(not(debug_assertions))]
fn get_app() -> App {    
    // Initialize the application.
    let mut app = App::new();
    
    // Load the saved game state.
    app.load();
    app
}

#[cfg(debug_assertions)]
fn get_app() -> App {
    // Initialize the application.
    let mut app = App::new();
    
    // Load the saved game state.
    app.load();
    app.game.multiplier = 25;
    app
}