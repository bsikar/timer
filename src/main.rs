use macroquad::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};

#[macroquad::main("Stopwatch")]
async fn main() {
    let mut start_time: Option<Instant> = None;
    let mut elapsed_time = Duration::from_secs(0);
    let mut is_running = false;
    let mut should_write_file = false;
    let mut written = false;

    loop {
        clear_background(Color::from_rgba(77, 71, 48, 255));

        let mut text = String::from("00:00");

        if let Some(start) = start_time {
            if is_running {
                elapsed_time = start.elapsed();
            }
            let seconds = elapsed_time.as_secs() % 60;
            let minutes = elapsed_time.as_secs() / 60;
            text = format!("{minutes:02}:{seconds:02}");
        }

        let text_size = measure_text(&text, None, 90, 1.0);

        draw_text(
            &text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. - text_size.height / 2.,
            90.0,
            Color::from_rgba(36, 24, 19, 255),
        );

        if is_running {
            if is_key_pressed(KeyCode::Space) {
                should_write_file = true;
                is_running = false;
                written = false;
            }
        } else if is_key_pressed(KeyCode::Space) {
            start_time = Some(Instant::now() - elapsed_time);
            is_running = true;
            should_write_file = false;
        }

        next_frame().await;

        if should_write_file && !written {
            let date = chrono::Local::now().format("%m/%d/%Y %H:%M").to_string();
            let elapsed = format!(
                "{:02}:{:02}",
                elapsed_time.as_secs() / 60,
                elapsed_time.as_secs() % 60
            );

            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("times.txt")
                .unwrap();

            writeln!(file, "{} {}", date, elapsed).unwrap();
            written = true;
        }
    }
}
