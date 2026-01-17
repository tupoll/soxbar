# soxbar
Gui for controlling the player of the SOX console program.

To install sox, use the standard repository or github:
```
wget https://github.com/rbouqueau/SoX.git
```

We build soxbar and move the binary file:
```
cd $HOME/soxbar
cargo build --release
mv -f $HOME/soxbar/target/release/soxbar $HOME/.local/bin/soxbar

```
We collect the remaining binaries:
```
rustc --out-dir $HOME/.local/bin $HOME/soxbar/list_sox.rs
rustc --out-dir $HOME/.local/bin $HOME/soxbar/play_sox_bin.rs
rustc --out-dir $HOME/.local/bin $HOME/soxbar/setup_playlist.rs
rustc --out-dir $HOME/.local/bin $HOME/soxbar/stop_sox.rs

```
Script for automatic file assembly and movement:
```
cd $HOME/soxbar
lua install.lua

```
Copy the "Sox Control Center 2026" launch icon and grant it permission to run:
```
cp -Rv $HOME/soxbar/'Sox Control Center 2026.desktop' $HOME/.local/share/applications
chmod 755 $HOME/.local/share/applications/'Sox Control Center 2026.desktop'

```
To stop binary play, press two keys!
```
"НАЗАД (ИСТОРИЯ)" then "СТОП"

```

If you closed the application window, but it is running!:
```
In the window manager panel (ironbar), click stop ⏹️
cmd = stop_sox

```

To work, you need to create a directory in tmpfs and set permissions on it:
```

sudo mkdir -p /var/tmp/wm
sudo echo "tmpfs  /var/tmp/wm  tmpfs size=10M  0 0">>/etc/fstab
sudo chown -R <имя пользователя>:<группа пользователя> /var/tmp/sway

```
Thunar file manager:

```
Edit-Special>Actions-Add Icon-Text-Command:
echo -n %f | /usr/bin/awk '{print $0}'>>~/tmp/playlist

```
Dependencies:

lua5.4(versions may be 5.1 ... 5.3);
rust1.92.0(gentoo:dev-lang/rust-bin);
thunar4.20.6(optional);
fish(versions can be any)
