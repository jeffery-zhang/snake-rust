use crate::game::Difficulty;

pub struct Button {
    rect: [f64; 4],
    color: [f64; 4],
    label: String,
    action: ButtonAction,
}

enum ButtonAction {
    ChooseDifficulty(Difficulty),
    StartGame,
}

pub enum GUI {
    Menu(Vec<Button>),
    GameOver(Vec<Button>),
}

pub fn create_menu_buttons(window_width: u32, window_height: u32) -> Vec<Button> {
    let left = (window_width / 2) as f64;
    let top = (window_height / 2) as f64;
    vec![
        Button {
            rect: [left - 150.0, top - 100.0, 100.0, 50.0],
            color: [0.0, 1.0, 0.0, 1.0],
            label: "Easy".to_string(),
            action: ButtonAction::ChooseDifficulty(Difficulty::Easy),
        },
        Button {
            rect: [left - 50.0, top - 100.0, 100.0, 50.0],
            color: [1.0, 1.0, 0.0, 1.0],
            label: "Medium".to_string(),
            action: ButtonAction::ChooseDifficulty(Difficulty::Medium),
        },
        Button {
            rect: [left + 50.0, top - 100.0, 100.0, 50.0],
            color: [1.0, 0.0, 0.0, 1.0],
            label: "Hard".to_string(),
            action: ButtonAction::ChooseDifficulty(Difficulty::Hard),
        },
        Button {
            rect: [left - 50.0, top + 100.0, 100.0, 50.0],
            color: [0.0, 0.0, 1.0, 1.0],
            label: "Start".to_string(),
            action: ButtonAction::StartGame,
        },
    ]
}
