use std::env;
use std::fs;
use std::process::{Command};
use std::thread;
use std::time::Duration;

fn main() {
    // 1. Получаем домашнюю директорию
    let home = env::var("HOME").expect("Не установлена переменная $HOME");
    let soxbar_dir = format!("{}/soxbar", home);
    let bin_dir = format!("{}/.local/bin", home);
    let app_dir = format!("{}/.local/share/applications", home);

    println!("🚀 Начинаю сборку и установку Sox Control Center 2026...");

    // cmd1: Сборка проекта
    status_check(
        Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(&soxbar_dir)
            .status(),
        "Сборка cargo",
    );

    // Подготовка целевых папок
    let _ = fs::create_dir_all(&bin_dir);
    let _ = fs::create_dir_all(&app_dir);

    // cmd2 - cmd6: Перемещение пользовательских бинарников
    let binaries = ["list_sox", "play_sox_bin", "setup_playlist", "stop_sox", "soxbar"];
    for bin in binaries {
        let src = format!("{}/target/release/{}", soxbar_dir, bin);
        let dst = format!("{}/{}", bin_dir, bin);
        if fs::rename(&src, &dst).is_err() {
            // Если rename не сработал (разные разделы диска), пробуем copy + remove
            fs::copy(&src, &dst).expect("Ошибка копирования бинарника");
            fs::remove_file(&src).ok();
        }
        println!("✅ Установлен {}", bin);
    }

    // cmd8 - cmd9: Установка Desktop-файла
    let desktop_name = "Sox Control Center 2026.desktop";
    let src_desktop = format!("{}/{}", soxbar_dir, desktop_name);
    let dst_desktop = format!("{}/{}", app_dir, desktop_name);
    fs::copy(&src_desktop, &dst_desktop).expect("Ошибка установки .desktop файла");
    
    status_check(
        Command::new("chmod").args(["a+x", &dst_desktop]).status(),
        "Установка прав на .desktop",
    );

    // cmd10: Пауза 
    println!("⏳ Ожидание 5 секунд...");
    thread::sleep(Duration::from_secs(5));

    // cmd11 - cmd14: Системные бинарники через sudo
    println!("🔐 Требуются права root для системных компонентов...");

    // Перемещаем play_info
    sudo_move(&soxbar_dir, "play_info", "/usr/local/bin/play_info");
    
    // Перемещаем и запускаем helper
    sudo_move(&soxbar_dir, "helper", "/usr/local/bin/helper");

    status_check(
        Command::new("sudo").arg("/usr/local/bin/helper").status(),
        "Запуск helper",
    );

    status_check(
        Command::new("sudo").args(["rm", "-f", "/usr/local/bin/helper"]).status(),
        "Удаление helper",
    );

    println!("✨ Установка успешно завершена!");
}

// Помощник для перемещения через sudo
fn sudo_move(base_dir: &str, bin_name: &str, dst_path: &str) {
    let src_path = format!("{}/target/release/{}", base_dir, bin_name);
    status_check(
        Command::new("sudo")
            .args(["mv", "-f", &src_path, dst_path])
            .status(),
        &format!("Установка {}", bin_name),
    );
}

// Помощник для проверки статуса выполнения
fn status_check(status: std::io::Result<std::process::ExitStatus>, task: &str) {
    match status {
        Ok(s) if s.success() => println!("✅ {} - успешно", task),
        _ => {
            eprintln!("❌ Ошибка при выполнении: {}", task);
            std::process::exit(1);
        }
    }
}
