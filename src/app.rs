use std::error;
use pet_the_cat::game::Game;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

const SAVE_FILE_NAME: &str = "pet_the_cat.save.toml";

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    // Game instance
    pub game: Game,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            game: Game::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Called every seconds to update the application.
    pub fn update(&mut self) {

        // Pet cat by factory every second
        if self.game.petting_machine > 0 {
            self.game.pet_cat_with_machine();
        }
    }

    // Load the `Game` save file in TOML format
    pub fn load(&mut self) {
        let game_str_result = std::fs::read_to_string(SAVE_FILE_NAME);

        let game_str = match game_str_result {
            Ok(game_str) => game_str,
            Err(_) => {
                // Check if save file exists
                let save_exists = std::path::Path::new(SAVE_FILE_NAME).exists();

                if save_exists {
                    // If it does, but we can't read it, then something is wrong
                    panic!("{}", t!("read_save_fail"));
                }

                // If it doesn't, then create a new save file
                self.save();
                return;
            }
        };

        self.game = toml::from_str(&game_str).unwrap();
    }

    // Save the `Game` to a TOML file
    pub fn save(&self) {
        let save: String = toml::to_string(&self.game).unwrap();
        std::fs::write(SAVE_FILE_NAME, save).unwrap();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.save();
        
        self.running = false;
    }
}
