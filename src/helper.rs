use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;
use std::env;

fn main() -> io::Result<()> {
    // 1. Пути
    let fstab_path = "/etc/fstab";
    let target_dir = "/var/tmp/wm";
    let line_to_add = "tmpfs  /var/tmp/wm  tmpfs size=10M  0 0";

    // 2. Добавление записи в /etc/fstab
    let mut already_exists = false;
    if let Ok(file) = fs::File::open(fstab_path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(l) = line {
                if l.contains("/var/tmp/wm") {
                    already_exists = true;
                    break;
                }
            }
        }
    }

    if !already_exists {
        match OpenOptions::new().append(true).open(fstab_path) {
            Ok(mut file) => {
                writeln!(file, "{}", line_to_add)?;
                println!("✅ Запись добавлена в {}", fstab_path);
            }
            Err(e) => {
                eprintln!("❌ Ошибка доступа к {}: {}. Запустите программу от root.", fstab_path, e);
                std::process::exit(1);
            }
        }
    } else {
        println!("ℹ️ Запись для {} уже есть в fstab.", target_dir);
    }

    // 3. Создание директории
    if !fs::metadata(target_dir).is_ok() {
        fs::create_dir_all(target_dir)?;
        println!("✅ Директория {} создана.", target_dir);
    }

    // 4. Передача прав текущему пользователю (не root)
    // Если программа запущена через sudo, переменная SUDO_USER содержит имя обычного юзера
    let user_name = env::var("SUDO_USER").unwrap_or_else(|_| {
        env::var("USER").expect("Не удалось определить пользователя")
    });

    let status = Command::new("chown")
        .arg(format!("{}:{}", user_name, user_name))
        .arg(target_dir)
        .status()?;

    if status.success() {
        println!("✅ Права на {} переданы пользователю {}", target_dir, user_name);
    } else {
        eprintln!("❌ Не удалось изменить владельца директории.");
    }

    Ok(())
}
