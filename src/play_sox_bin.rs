use std::process::{Command, Stdio};
use std::io::{Write, Error, BufRead, BufReader};
use std::fs::{self, File, OpenOptions};
use std::env;
//use std::path::Path;

fn main() -> Result<(), Error> {
    // 2. Определяем пути
    let home = env::var("HOME").expect("HOME not found");
    let tmp_path = format!("{}/tmp", home);
    let playlist_file = format!("{}/playlist", tmp_path);
    let name_file = format!("{}/sox/name", tmp_path);
    let history_file = format!("{}/playlist1", tmp_path);
    let wm_playlist = "/var/tmp/wm/playlist";

    fs::create_dir_all(format!("{}/sox", tmp_path))?;

    // 3. Читаем первую строку плейлиста
    let file = File::open(&playlist_file)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    if lines.is_empty() {
        println!("Плейлист пуст.");
        return Ok(());
    }

    let current_track = &lines[0];
    let remaining_tracks = &lines[1..];

    // --- НОВОЕ: ИЗВЛЕЧЕНИЕ ОБЛОЖКИ ПЕРЕД ЗАПУСКОМ ---
    extract_cover(current_track);

    // 4. Логика перемещения строк
    fs::write(&playlist_file, remaining_tracks.join("\n") + "\n")?;
    fs::write(&name_file, current_track)?;

    let mut history = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&history_file)?;
    writeln!(history, "{}", current_track)?;

    let short_list: Vec<String> = remaining_tracks
        .iter()
        .map(|s| if s.len() > 26 { s[26..].to_string() } else { s.clone() })
        .collect();
    fs::write(wm_playlist, short_list.join("\n") + "\n")?;

    // 5. ЗАПУСК PLAY
    println!("Запуск: {}", current_track);
    Command::new("play")
        .arg(current_track)
        .arg("rate").arg("-v").arg("48000")
        .arg("bass").arg("+15")
        .arg("treble").arg("+12")
        .spawn()?;

    // 6. Рекурсивный вызов (мониторинг завершения)
    let current_exe = env::current_exe()?;
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "sleep 1 && while pgrep -x play > /dev/null; do sleep 2; done; {} ", 
            current_exe.display()
        ))
        .spawn()?;

    Ok(())
}

// ФУНКЦИЯ ДЛЯ ОБНОВЛЕНИЯ ОБЛОЖКИ (читается GTK интерфейсом)
fn extract_cover(track_path: &str) {
    let cover_path = "/var/tmp/wm/sox_current_cover.jpg";
    // Удаляем старую обложку
    let _ = fs::remove_file(cover_path);

    // Используем ffmpeg для извлечения встроенного изображения
    let _ = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(track_path)
        .arg("-an")
        .arg("-vcodec")
        .arg("copy")
        .arg(cover_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    
    println!("Обложка обновлена в {}", cover_path);
}
