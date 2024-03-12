use crate::app::App;
use ctru::services::hid::{Hid, KeyPad};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(hid: &mut Hid, app: &mut App) -> bool {
    hid.scan_input();

    let pressed_keys = hid.keys_down();

    if pressed_keys.contains(KeyPad::START) {
        app.quit();
        return true;
    }
    if pressed_keys.contains(KeyPad::A) {
        app.game.pet_cat();
        return true;
    }
    if pressed_keys.contains(KeyPad::B) {
        app.game.buy_multiplier();
        return true;
    }
    if pressed_keys.contains(KeyPad::X) {
        app.game.buy_petting_machine();
        return true;
    }

    if pressed_keys.contains(KeyPad::TOUCH) {
        let touch: (u16, u16) = hid.touch_position();

        // First row
        if touch.1 >= 20 && touch.1 < 60 {
            app.game.pet_cat();
            return true;
        }
        // Second row
        if touch.1 >= 100 && touch.1 < 120 {
            app.game.buy_multiplier();
            return true;
        }
        // Third row
        if touch.1 >= 180 && touch.1 < 220{
            app.game.buy_petting_machine();
            return true;
        }
    }

    false
}
