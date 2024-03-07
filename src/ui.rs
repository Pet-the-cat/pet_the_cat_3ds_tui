use crate::app::App;
use pet_the_cat::game::{MULTIPLIER_COST, PETTING_MACHINE_COST};
use ctru::prelude::Console;

/// Print the user interface.
/// 
/// # Arguments
/// 
/// * `app` - The application.
/// * `top_console` - The top console.
/// * `bottom_console` - The bottom console.
pub fn print(app: &App, top_console: &Console<'_>, bottom_console: &Console<'_>) {
    print_top_screen(app, top_console);
    print_bottom_screen(app, bottom_console);
}

/// Print the top screen.
/// 
/// # Arguments
/// 
/// * `app` - The application.
/// * `console` - The console to print to.
fn print_top_screen(app: &App, console: &Console<'_>) {
    console.select();
    console.clear();

    println!("\x1b[5;15H{}", t!("title"));

    println!("\x1b[10;15H{}", t!("cat_petted", cat_petted = app.game.cat_petted));
    println!("\x1b[15;15H{}", t!("multiplier", multiplier = app.game.multiplier));
    println!("\x1b[20;15H{}", t!("petting_machine", petting_machine = app.game.petting_machine));

    println!("\x1b[29;1H{}", t!("save_quit_text"));
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
/// The maximum length of the text is 40 characters.
fn print_bottom_screen(app: &App, console: &Console<'_>) {    
    console.select();
    console.clear();
  
    println!("\x1b[5;0H "); // Line space: 5
    print_ascii_button(format!("{} (A)", t!("pet_the_cat")));// Line space: 10
    
    println!("\x1b[15;0H "); // Line space: 15
    if app.game.cat_petted >= MULTIPLIER_COST {
        print_ascii_button(format!("{} (B)", t!("multiplier_buy_text"))); // Line space: 20
    }
    
    println!("\x1b[25;0H "); // Line space: 25
    if app.game.cat_petted >= PETTING_MACHINE_COST {
        // TODO: Crash because the length of the text is greater than 40
        print_ascii_button(format!("{} (X)", t!("petting_machine_buy_text"))); // Line space: 30
    }
}

fn print_ascii_button(text: String) {
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

    println!("     .----------------------------.");
    println!("     |                            |");
    println!("     |{}|", text_with_spaces);
    println!("     |                            |");
    println!("     '----------------------------'");
 }
