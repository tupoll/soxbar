use gtk4 as gtk;
use gtk::prelude::*;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.sox.controller.full.v2")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Sox Control Center 2026")
        .default_width(560)
        .default_height(750)
        .build();

    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    main_box.set_margin_end(12);

    // --- ВЕРХНИЙ БЛОК: КНОПКИ + ОБЛОЖКА ---
    let top_layout = gtk::Box::new(gtk::Orientation::Horizontal, 15);

    // Левая часть: Вертикальный стек кнопок
    let button_bar = gtk::Box::new(gtk::Orientation::Vertical, 0);
    button_bar.add_css_class("linked");
    button_bar.set_hexpand(true);

    // Правая часть: Виджет обложки
    let cover_image = gtk::Image::builder()
        .pixel_size(170)
        .icon_name("audio-x-generic")
        .build();
    cover_image.add_css_class("cover-art");

    // --- СОЗДАНИЕ ВСЕХ КНОПОК ---
    
    let btn_playlist = gtk::Button::with_label("📁 ОБНОВИТЬ ПЛЕЙЛИСТ");
    btn_playlist.connect_clicked(|_| { let _ = generate_playlist(); });

    // --- ВНУТРИ build_ui ---

// Создаем виджет для обложки (если еще не создан)
let cover_image = gtk::Image::builder()
    .pixel_size(170)
    .icon_name("audio-x-generic")
    .build();
cover_image.add_css_class("cover-art");

// КНОПКА ЗАПУСКА БИНАРНИКА
let btn_bin_play = gtk::Button::builder()
    .label("▶️ СТАРТ (БИНАРНИК)")
    .css_classes(["suggested-action"])
    .build();

btn_bin_play.connect_clicked(|_| {
    // Запускаем ваш скомпилированный бинарный файл
    let _ = Command::new("play_sox_bin").spawn(); 
});

// ТАЙМЕР ОБНОВЛЕНИЯ ОБЛОЖКИ ИЗ /var/tmp/wm/
let img_clone = cover_image.clone();
glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
    let cover_path = "/var/tmp/wm/sox_current_cover.jpg";
    
    if std::path::Path::new(cover_path).exists() {
        // Устанавливаем изображение из файла, который обновил бинарник
        img_clone.set_from_file(Some(cover_path));
    } else {
        // Если файла нет, показываем иконку по умолчанию
        img_clone.set_icon_name(Some("audio-x-generic"));
    }
    glib::ControlFlow::Continue
});

// Не забудьте добавить cover_image в ваш layout (например, в top_layout)


    let btn_play_next = gtk::Button::builder()
        .label("⏩ СЛЕДУЮЩИЙ ТРЕК")
        .build();
    let img_c1 = cover_image.clone();
    btn_play_next.connect_clicked(move |_| { 
        if let Ok(path) = switch_to_next_track() { update_cover(&img_c1, &path); }
    });

    let btn_auto = gtk::ToggleButton::builder()
        .label("🔄 АВТО: ВЫКЛ")
        .build();
    btn_auto.connect_toggled(|btn| {
        btn.set_label(if btn.is_active() { "🔄 АВТО: ВКЛ" } else { "🔄 АВТО: ВЫКЛ" });
    });

    let btn_prev = gtk::Button::with_label("⏪ НАЗАД (ИСТОРИЯ)");
    let img_c2 = cover_image.clone();
    btn_prev.connect_clicked(move |_| { 
        if let Ok(path) = rewind_to_previous() { update_cover(&img_c2, &path); }
    });

    let btn_stop = gtk::Button::builder()
        .label("⏹️ СТОП")
        .css_classes(["destructive-action"])
        .build();
    btn_stop.connect_clicked(|_| { let _ = Command::new("pkill").arg("-x").arg("play").status(); });
    
    let btn_setup_playlist = gtk::Button::builder()
        .label("📂 СОБРАТЬ ПЛЕЙЛИСТ-THUNAR")
        .build();
    btn_setup_playlist.connect_clicked(|_| { let _ = Command::new("fish").arg("-c").arg("setup_playlist").status(); });

    // Сборка бара кнопок
    button_bar.append(&btn_playlist);
    button_bar.append(&btn_setup_playlist);
    button_bar.append(&btn_bin_play);
    button_bar.append(&btn_play_next);    
    button_bar.append(&btn_auto);
    button_bar.append(&btn_prev);
    button_bar.append(&btn_stop);
    
    top_layout.append(&button_bar);
    top_layout.append(&cover_image);

    // --- НИЖНИЙ БЛОК: СПИСОК ---
    let list_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    let scrolled = gtk::ScrolledWindow::builder()
        .child(&list_box)
        .vexpand(true)
        .build();

    main_box.append(&top_layout);
    main_box.append(&scrolled);

    // --- ТАЙМЕРЫ ---

    // 1. Обновление списка (раз в секунду)
    let lb_clone = list_box.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(1000), move || {
        let home = env::var("HOME").unwrap_or_default();
        let p_path = PathBuf::from(home).join("tmp/playlist");
        if let Ok(content) = fs::read_to_string(p_path) {
            while let Some(child) = lb_clone.first_child() { lb_clone.remove(&child); }
            for line in content.lines().take(35) {
                let name = line.split('/').last().unwrap_or(line);
                lb_clone.append(&gtk::Label::builder().label(name).xalign(0.0).build());
            }
        }
        glib::ControlFlow::Continue
    });

    // 2. Авто-проигрывание (мониторинг play)
    let btn_a_clone = btn_auto.clone();
    let img_a_clone = cover_image.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(2), move || {
        if btn_a_clone.is_active() {
            let status = Command::new("pgrep").arg("-x").arg("play").stdout(Stdio::null()).status();
            if let Ok(s) = status {
                if !s.success() {
                    if let Ok(path) = switch_to_next_track() {
                        update_cover(&img_a_clone, &path);
                    }
                }
            }
        }
        glib::ControlFlow::Continue
    });

    window.set_child(Some(&main_box));
    window.present();
}

// --- ФУНКЦИИ ЛОГИКИ ---

fn update_cover(image_widget: &gtk::Image, audio_path: &str) {
    let tmp_cover = "/var/tmp/wm/sox_current_cover.jpg";
    let _ = fs::remove_file(tmp_cover);
    let _ = Command::new("ffmpeg")
        .args(["-y", "-i", audio_path, "-an", "-vcodec", "copy", tmp_cover])
        .stdout(Stdio::null()).stderr(Stdio::null()).status();

    if Path::new(tmp_cover).exists() {
        image_widget.set_from_file(Some(tmp_cover));
    } else {
        image_widget.set_icon_name(Some("audio-x-generic"));
    }
}

fn switch_to_next_track() -> Result<String, std::io::Error> {
    let home = env::var("HOME").expect("HOME not found");
    let tmp_path = PathBuf::from(&home).join("tmp");
    let p_main = tmp_path.join("playlist");
    let p_hist = tmp_path.join("playlist1");

    let content = fs::read_to_string(&p_main)?;
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    if lines.is_empty() { return Err(std::io::Error::new(std::io::ErrorKind::Other, "Empty")); }

    let track = lines.remove(0);
    fs::write(&p_main, lines.join("\n") + "\n")?;
    let mut hist = OpenOptions::new().append(true).create(true).open(&p_hist)?;
    writeln!(hist, "{}", track)?;

    let _ = Command::new("pkill").arg("-x").arg("play").status();
    play_audio(&track);
    Ok(track)
}

fn rewind_to_previous() -> Result<String, std::io::Error> {
    let home = env::var("HOME").expect("HOME not found");
    let tmp_path = PathBuf::from(&home).join("tmp");
    let p_main = tmp_path.join("playlist");
    let p_hist = tmp_path.join("playlist1");

    let content = fs::read_to_string(&p_hist)?;
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    if lines.len() < 2 { return Err(std::io::Error::new(std::io::ErrorKind::Other, "No history")); }

    let _current = lines.pop(); 
    let target = lines.pop().unwrap(); 

    fs::write(&p_hist, lines.join("\n") + "\n")?;
    let main_content = fs::read_to_string(&p_main).unwrap_or_default();
    fs::write(&p_main, format!("{}\n{}", target, main_content))?;

    let _ = Command::new("pkill").arg("-x").arg("play").status();
    play_audio(&target);
    Ok(target)
}

fn play_audio(path: &str) {
    let _ = Command::new("play")
        .arg("-q").arg(path)
        .arg("rate").arg("-v").arg("48000")
        .arg("bass").arg("+15").arg("treble").arg("+12")
        .spawn();
}

fn generate_playlist() -> std::io::Result<()> {
    let _home = env::var("HOME").unwrap();
    let helper = "/tmp/playlist_gen.lua";
    let mut f = File::create(helper)?;
    write!(f, "
        local h = os.getenv('HOME')
        local out = h .. '/tmp/playlist'
        os.execute('rm -f ' .. out)
        local function s(p) os.execute('ls ' .. p .. ' 2>/dev/null >> ' .. out) end
        s(h .. '/Музыка/*.mp3') s(h .. '/Музыка/*/*.mp3')
        s(h .. '/Музыка/*.flac') s(h .. '/Музыка/*/*.flac')
        s(h .. '/Музыка/*.ape') s(h .. '/Музыка/*/*.ape')
    ")?;
    let _ = Command::new("lua").arg(helper).status();
    let _ = Command::new("notify-send").arg("Sox").arg("Плейлист обновлен").spawn();
    Ok(())
}
