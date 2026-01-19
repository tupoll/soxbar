use gtk4 as gtk;
use gtk::glib;
use gtk::prelude::*;
use vte4 as vte;
use vte::prelude::*; 
use gtk::subclass::prelude::*;
use std::process::Command;
use std::process::Stdio;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::path::Path;
use std::fs;
use std::fs::OpenOptions;

// --- 1. РЕАЛИЗАЦИЯ ВИДЖЕТА PlayInfoWidget ---

glib::wrapper! {
    pub struct PlayInfoWidget(ObjectSubclass<imp::PlayInfoWidget>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl PlayInfoWidget {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn start(&self) {
        let imp = self.imp();
        imp.terminal.spawn_async(
            vte::PtyFlags::DEFAULT,
            None,
            &["/usr/bin/fish", "-c", "clear; exec /usr/local/bin/play_info"],
            &[],
            gtk::glib::SpawnFlags::DEFAULT, 
            || {},
            -1,
            None::<&gtk::gio::Cancellable>,
            |_| {},
        );
    }
}

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct PlayInfoWidget {
        pub terminal: vte::Terminal,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PlayInfoWidget {
        const NAME: &'static str = "PlayInfoWidget";
        type Type = super::PlayInfoWidget;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for PlayInfoWidget {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.set_orientation(gtk::Orientation::Vertical);

            self.terminal.set_size(80, 1);
            self.terminal.set_vexpand(false);
            self.terminal.set_size_request(-1, 35);
            obj.append(&self.terminal);
        }
    }
    impl WidgetImpl for PlayInfoWidget {}
    impl BoxImpl for PlayInfoWidget {}
}

// --- 2. ГЛАВНАЯ ФУНКЦИЯ ПРИЛОЖЕНИЯ ---

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.sox.playinfo.v2026")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

// --- 3. ПОСТРОЕНИЕ ИНТЕРФЕЙСА ---

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Sox Control Center 2026")
        .default_width(580)
        .default_height(450)
        .build();

    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
    main_box.set_margin_end(12);

    let top_layout = gtk::Box::new(gtk::Orientation::Horizontal, 15);
    let button_bar = gtk::Box::new(gtk::Orientation::Vertical, 6);
    button_bar.set_hexpand(true);

    // --- КАРТИНКА (ОБЛОЖКА) ---
    let cover_image = gtk::Image::builder()
        .pixel_size(180)
        .icon_name("audio-x-generic")
        .build();

    // Кнопки
    let btn_setup_playlist = gtk::Button::builder()
        .label("📂 СОБРАТЬ ПЛЕЙЛИСТ-THUNAR")
        .build();
    btn_setup_playlist.connect_clicked(|_| { let _ = Command::new("fish").arg("-c").arg("setup_playlist").status(); });
    
    let btn_playlist = gtk::Button::with_label("📁 ОБНОВИТЬ ПЛЕЙЛИСТ");
    btn_playlist.connect_clicked(|_| { let _ = generate_playlist(); });

    let btn_bin_play = gtk::Button::builder().label("▶️ СТАРТ").css_classes(["suggested-action"]).build();
    btn_bin_play.connect_clicked(|_| { let _ = Command::new("play_sox_bin").spawn(); });

    let btn_auto = gtk::ToggleButton::with_label("🔄 АВТО: ВЫКЛ");
    btn_auto.connect_toggled(|btn| {
        btn.set_label(if btn.is_active() { "🔄 АВТО: ВКЛ" } else { "🔄 АВТО: ВЫКЛ" });
    });

    let btn_stop = gtk::Button::builder()
        .label("⏹️ СТОП")
        .css_classes(["destructive-action"])
        .build();
    btn_stop.connect_clicked(|_| { 
        let _ = Command::new("pkill").arg("-x").arg("play").status(); 
        let _ = Command::new("fish").arg("-c").arg("stop_sox").status(); 
    });
    
    let btn_play_next = gtk::Button::builder()
        .label("⏩ СЛЕДУЮЩИЙ ТРЕК")
        .build();
    let img_c1 = cover_image.clone();
    btn_play_next.connect_clicked(move |_| { 
        if let Ok(path) = switch_to_next_track() { update_cover(&img_c1, &path); }
    });
     
    let btn_prev = gtk::Button::with_label("⏪ НАЗАД (ИСТОРИЯ)");
    let img_c2 = cover_image.clone();
    btn_prev.connect_clicked(move |_| { 
        if let Ok(path) = rewind_to_previous() { update_cover(&img_c2, &path); }
    });
       
    button_bar.append(&btn_setup_playlist);
    button_bar.append(&btn_playlist);
    button_bar.append(&btn_play_next);
    button_bar.append(&btn_bin_play);
    button_bar.append(&btn_prev);
//    button_bar.append(&btn_auto);
    button_bar.append(&btn_stop);

    top_layout.append(&button_bar);
    top_layout.append(&cover_image);

    // Терминал (инфо)
    let info_display = PlayInfoWidget::new();
    let list_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    let scrolled = gtk::ScrolledWindow::builder()
        .child(&list_box)
        .vexpand(true)
        .build();

    main_box.append(&top_layout);
    main_box.append(&info_display);
    main_box.append(&top_layout);
    main_box.append(&scrolled);
    window.set_child(Some(&main_box));
    
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
   

    // Таймер обновления обложки из /var/tmp/wm/
    let img_clone = cover_image.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(1000), move || {
        let cover_path = "/var/tmp/wm/sox_current_cover.jpg"; 
        if std::path::Path::new(cover_path).exists() {
            img_clone.set_from_file(Some(cover_path));
        }
        glib::ControlFlow::Continue
    });

    window.present();
    info_display.start();
    
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

      fn play_audio(path: &str) {
    let _ = Command::new("play")
        .arg("-q").arg(path)
        .arg("rate").arg("-v").arg("48000")
        .arg("bass").arg("+15").arg("treble").arg("+12")
        .spawn();
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

}
