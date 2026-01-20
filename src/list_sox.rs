use std::env;
use std::fs; 
use std::path:: Path;
use std::io::Write;
use std::fs::File;
use std::process::Command;

fn main() -> Result<(), std::io::Error> { 
	let _dir = env::temp_dir();
	fs::remove_file("tmp/playlist1").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
	});
	fs::remove_file("tmp/playlist").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
     });
             
   let path = "/var/tmp/wm/helper.lua";
   let mut output = File::create(path)?;
    write!(output, "#!/usr/bin/lua\ncmd1 = string.format('ls %s', '~/Музыка/*.mp3 |sort -u>~/tmp/playlist');\ncmd2 = string.format('ls %s', '~/Музыка/*/*.mp3 |sort -u>>~/tmp/playlist');\ncmd3 = string.format('ls %s', '~/Музыка/*/*.flac |sort -u>>~/tmp/playlist');\ncmd4 = string.format('ls %s', '~/Музыка/*/*.ape |sort -u>>~/tmp/playlist');\nio.popen(cmd1);\nio.popen(cmd2);\nio.popen(cmd3);\nio.popen(cmd4);")?;
    
    let _output = Command::new("lua")
        .arg("/var/tmp/wm/helper.lua")
        .output().unwrap_or_else(|e| {
            panic!("Ошибка выполнения процесса {}", e)
    }); 
    
    fs::remove_file("/var/tmp/wm/helper.lua").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    
    let _output = Command::new("notify-send")
        .arg("'Плэйлист для sox готов'")
        .output().unwrap_or_else(|e| {
            panic!("Ошибка выполнения процесса {}", e)
    }); 
    
    let dir_path = "tmp/sox";  // название создаваемой папки 
    // если папка не существует
    if !Path::new(dir_path).exists() {
        fs::create_dir(dir_path)?; // создаем папку
    }
           Ok(())
}
