use crate::app::App;
use pet_the_cat::game::{MULTIPLIER_COST, PETTING_MACHINE_COST};
use ctru::console::Console;

/// Print the user interface.
/// 
/// # Arguments
/// 
/// * `app` - The application.
/// * `top_console` - The top console.
/// * `bottom_console` - The bottom console.
pub fn print(app: &App, top_console: &Console, bottom_console: &Console) {
    print_top_screen(app, top_console);
    print_bottom_screen(app, bottom_console);
}

/// Print the user interface with 3D capabilities.
/// 
/// # Arguments
/// 
/// * `app` - The application.
/// * `top_screen` - The top 3D screen.
/// * `bottom_console` - The bottom console.
pub fn print_3d(app: &App, top_left: &Console, top_right: &Console, bottom: &Console) {
    print_top_screen(app, top_left);
    print_top_screen(app, top_right);

    print_bottom_screen(app, bottom);
}

// NOTE: 0: BLACK 1: RED, 2: GREEN, 3: YELLOW, 4: BLUE, 5: MAGENTA, 6: CYAN, 7: WHITE
// NOTE: 2: FAINT, 4: UNDERLINE
// NOTE: [y;xH

/// Print the top screen.
/// 
/// # Arguments
/// 
/// * `app` - The application.
/// * `console` - The console to print to.
/// * `clear` - If the console should be cleared.
///
/// # Notes
/// 
/// The maximum length of the text is 50 characters and 100 in wide mode and the maximum height is 30.
/// `console.max_width()`
fn print_top_screen(app: &App, console: &Console) {
    console.select();
    console.clear();

    print!("\x1b[1m\x1b[33m"); // Set the color to yellow
    print_center(t!("title").to_string(), 5, console);
    print!("\x1b[39m"); // Reset the color
 
    print_center(t!("cat_petted", cat_petted = app.game.cat_petted).to_string(), 10, console);
    print_center(t!("multiplier", multiplier = app.game.multiplier).to_string(), 15, console);
    print_center(t!("petting_machine", petting_machine = app.game.petting_machine).to_string(), 20, console);

    print_center(t!("save_quit_text").to_string(), 28, console);
    
    println!(); // To avoid weird behavior
}

/// Print the bottom screen.
/// 
/// # Arguments
///
/// * `app` - The application.
/// * `console` - The console to print to.
///
/// # Notes
/// 
/// The maximum length of the text is 40 characters and the maximum height is 30.
/// `console.max_width()`
fn print_bottom_screen(app: &App, console: &Console) {    
    console.select();
    console.clear();
  
    // Reset
    print!("\x1b[0m");
    print_ascii_button(format!("{} (A)", t!("pet")), 3);
    
    apply_faint_effect(app.game.cat_petted >= MULTIPLIER_COST);
    print_ascii_button(format!("{} (B)", t!("multiplier_buy_text")), 13);
    
    apply_faint_effect(app.game.cat_petted >= PETTING_MACHINE_COST);
    print_ascii_button(format!("{} (X)", t!("petting_machine_buy_text")), 23);

    println!(); // To avoid weird behavior
}

// TODO: Crash because the length of the text is greater than 40
fn print_ascii_button(text: String, heigth: u8) {
    /*  .-------------.
        |             |
        | Pet the cat |
        |             |
        '-------------' */

    let text_len = text.len();
    // The text need to be centered to the 28 characters width
    let text_start_pos = (28 - text_len) / 2;
    let text_end_pos = 28 - text_start_pos - text_len;
    let text_with_spaces = format!("{}{}{}", " ".repeat(text_start_pos), text, " ".repeat(text_end_pos));

    print!("\x1b[{};6H.----------------------------.", heigth);
    print!("\x1b[{};6H|                            |", heigth + 1);
    print!("\x1b[{};6H|{}|"                          , heigth + 2, text_with_spaces);
    print!("\x1b[{};6H|                            |", heigth + 3);
    print!(  "\x1b[{};6H'----------------------------'", heigth + 4);
 }

fn print_center(text: String, heigth: usize, console: &Console) {
    let max_size = usize::from(console.max_width());
    let text_start_pos = (max_size - text.len()) / 2;

    print!("\x1b[{};{}H{}", heigth, text_start_pos, text);
}

fn apply_faint_effect(condition: bool) {
    print!("{}", if condition { "\x1b[22m" } else { "\x1b[2m" }); // Faint effect
}