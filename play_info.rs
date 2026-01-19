use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::{thread, time::{Duration, Instant}};

// Проверка процесса
fn is_process_running(name: &str) -> bool {
    Command::new("pgrep")
        .arg("-x")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

// Чтение имени из файла с учетом домашней папки
fn get_name_from_file(path: &str) -> String {
    fs::read_to_string(path)
        .map(|content| {
            let line = content.trim().lines().next().unwrap_or("");
            if line.is_empty() { return String::new(); }
            
            // Берем только имя файла (SuperTrack.mp3)
            Path::new(line)
                .file_name()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| line.to_string())
        })
        .unwrap_or_default()
}

fn main() {
    // ДИНАМИЧЕСКИЙ ПУТЬ К $HOME/tmp/sox/name
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/tupoll".to_string());
    let file_path = format!("{}/tmp/sox/name", home);
    
    let mut current_name = String::new();
    let mut display_text = String::new();
    let mut start_time: Option<Instant> = None;
    let width = 25;

    // Скрыть курсор
    print!("\x1b[?25l");

    loop {
        if is_process_running("play") {
            if start_time.is_none() {
                start_time = Some(Instant::now());
            }

            let new_name = get_name_from_file(&file_path);

            if !new_name.is_empty() {
                if new_name != current_name {
                    current_name = new_name;
                    display_text = format!("{}    ", current_name);
                    start_time = Some(Instant::now()); // Сброс таймера
                }

                // Бегущая строка
                let view: String = display_text.chars().take(width).collect();
                let elapsed = start_time.unwrap().elapsed().as_secs();
                
                // Вывод: Зеленый текст и синее время
                print!(
                    "\r\x1b[32mИМЯ: {}\x1b[0m \x1b[34m{:02}:{:02}\x1b[0m\x1b[K", 
                    view, 
                    elapsed / 60, elapsed % 60
                );

                // Сдвиг
                let first = display_text.chars().next().unwrap_or(' ');
                display_text = display_text.chars().skip(1).collect();
                display_text.push(first);
            } else {
                print!("\r\x1b[33mОЖИДАНИЕ ТРЕКА В ФАЙЛЕ...\x1b[0m\x1b[K");
            }
        } else {
            // Если play не запущен
            start_time = None;
            current_name.clear();
            print!("\r\x1b[31mSTOPPED: play\x1b[0m\x1b[K");
        }

        // ОБЯЗАТЕЛЬНО ДЛЯ GTK/VTE
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(150));
    }
}
