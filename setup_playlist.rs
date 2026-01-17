use std::env;
use std::fs::{self, File};
//use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

fn main() -> std::io::Result<()> {
    // 1. Определяем пути
    let home = env::var("HOME").expect("Переменная HOME не найдена");
    let mut tmp_path = PathBuf::from(&home);
    tmp_path.push("tmp");

    let playlist1 = tmp_path.join("playlist1");
    let sox_dir = tmp_path.join("sox");
    let playlist_main = tmp_path.join("playlist");
    let sox_play_script = sox_dir.join("play");

    // 2. Удаление и очистка (аналог rm -rf и touch)
    let _ = fs::remove_file(&playlist1);
    let _ = fs::remove_dir_all(&sox_dir);
    let _ = fs::remove_file(&playlist_main);

    // Создаем заново пустой файл playlist1
    File::create(&playlist1)?;

    // 3. Создание структуры папок
    fs::create_dir_all(&sox_dir)?;

    // 4. Создание файла ~/tmp/sox/play и установка прав 755
    {
        let file = File::create(&sox_play_script)?;
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(&sox_play_script, perms)?;
    }

    // 5. Запуск Thunar в папке Музыка
    let music_dir = format!("{}/Музыка", home);
    Command::new("thunar")
        .arg(&music_dir)
        .spawn()
        .expect("Не удалось запустить Thunar");

    println!("Окружение для музыки подготовлено.");
    Ok(())
}
