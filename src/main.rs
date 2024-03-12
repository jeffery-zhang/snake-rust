extern crate piston_window;
extern crate rand;

use chrono::Local;
use piston_window::*;
use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::Instant;

mod game;
use game::*;
mod gui;
use gui::create_menu_buttons;
mod food;
mod snake;

static WINDOW_WIDTH: u32 = 1280;
static WINDOW_HEIGHT: u32 = 720;
static POINT_UNIT: u32 = 20;

fn main() {
    write_to_debug_file(format!("Game started - {}", format_date()).as_str());
    let mut window: PistonWindow = WindowSettings::new("Snake", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(WINDOW_WIDTH, WINDOW_HEIGHT, POINT_UNIT);
    let mut events = Events::new(EventSettings::new()).ups(1);
    let mut last_update = Instant::now();

    while let Some(e) = events.next(&mut window) {
        if game.state == GameState::GameOver {
            continue;
        }

        if let Some(_) = e.render_args() {
            game.render(&e, &mut window);
        }

        game.add_control(e);

        if game.eating_check() {
            last_update = Instant::now();
        }

        if game.impact_check() {
            game.over(|| {
                last_update = Instant::now();
            });
        }

        {
            let food = &mut game.food;
            if last_update.elapsed().as_secs() > 10 {
                (*food).set_position(vec![]);
                last_update = Instant::now();
            }
        }
    }
}

fn write_to_debug_file(message: &str) {
    let mut file_path = PathBuf::from("log");
    // 确保log目录存在
    if !file_path.exists() {
        fs::create_dir_all(&file_path).expect("Failed to create log directory");
    }
    // 设定日志文件名称
    file_path.push("debug_log.txt");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open log file");
    if let Err(e) = writeln!(file, "{}", message) {
        // 如果写入文件失败，将错误信息打印到控制台
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn format_date() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}
