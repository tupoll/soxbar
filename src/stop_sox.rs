use std::process::Command;
use std::io::Error;

fn main() -> Result<(), Error> {
    //println!("Остановка воспроизведения Sox и скриптов...");

    // 1. Останавливаем сам процесс 'play' (Sox)
    // Мы используем pkill, чтобы найти все процессы по имени
    let _ = Command::new("pkill")
        .arg("-x") // Искать точное совпадение имени
        .arg("play")
        .status();

    // 2. Останавливаем скрипт-обертку, если он завис или запущен в цикле
    let _ = Command::new("pkill")
        .arg("-f") // Искать по всей строке аргументов (включая путь к .sh)
        .arg("play.sh")
        .status();

    // 3. Дополнительно: останавливаем рекурсивный вызов, если используется play_sox
    let _ = Command::new("pkill")
        .arg("-f")
        .arg("play_sox")
        .status();

   // println!("Все процессы остановлены.");
    Ok(())
}
