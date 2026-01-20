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
mv -f $HOME/soxbar/target/release/list_sox $HOME/.local/bin/list_sox
mv -f $HOME/soxbar/target/release/play_sox_bin $HOME/.local/bin/play_sox_bin
mv -f $HOME/soxbar/target/release/setup_playlist $HOME/.local/bin/setup_playlist
mv -f $HOME/soxbar/target/release/stop_sox $HOME/.local/bin/stop_sox
mv -f $HOME/soxbar/target/release/soxbar $HOME/.local/bin/soxbar
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

If you closed the application window, but it is running!:
```
In the window manager panel (ironbar), click stop ⏹️
cmd = stop_sox

```
As root, compile play_info.rs in /usr/local/bin:
```
sudo rustc --out-dir /usr/local/bin $HOME/soxbar/play_info.rs

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
To run from terminal:
```
soxbar
```

Dependencies:
```
lua5.4(versions may be 5.1 ... 5.3);
rust1.92.0(gentoo:dev-lang/rust-bin);
thunar4.20.6(optional);
fish(versions can be any)
ffmpeg6.1.2
vte(gentoo:gui-libs/vte gtk4);
```
