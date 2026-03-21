# Cosmic Pomodoro

A minimal, distraction-free Pomodoro applet for the COSMIC desktop.
![Main](img/Main.png)

## About
Cosmic Pomodoro is a small and simple pomodoro timer made for the COSMIC desktop.  
It is fully based on libcosmic and follows the COSMIC system theme, using native components so it blends naturally into the desktop.  
It shows a basic work/break cycle and keeps the interface minimal.  
The goal is just to provide a straightforward pomodoro timer integrated with the COSMIC desktop.




## 🔽 Download & Installation



## 1) COSMIC Store (recommended)

Cosmic Pomodoro is available for install directly in the **COSMIC Store** under  
**Applets → Cosmic Pomodoro**.

### Add the applet to the COSMIC panel

Open COSMIC panel settings and add **Cosmic Pomodoro** to your panel’s applets list.  
After adding, the indicator appears in the panel.

---

## 2) Manual Flatpak bundle (alternative)

You can also install the `.flatpak` bundle manually.

### Latest Release

- **Release page:**  
  https://github.com/petar030/cosmic-pomodoro/releases/tag/v0.1.0-flatpak-20260308

- **Direct download (.flatpak):**  
  https://github.com/petar030/cosmic-pomodoro/releases/download/v0.1.0-flatpak-20260308/io.github.petar030.cosmic-pomodoro-master.flatpak

### Install (user scope)

> **Important:** Run the command from the same folder where the `.flatpak` file is located  
> (e.g. your project root or `~/Downloads`).

```sh
flatpak install --user -y ./io.github.petar030.cosmic-pomodoro-master.flatpakl --user -y ./io.github.petar030.cosmic-pomodoro-master.flatpak
```

### Add the applet to the COSMIC panel

Open COSMIC panel settings and add **Cosmic Pomodoro** to your panel’s applets list.  
After adding, the indicator appears in the panel.

### Uninstall

```sh
flatpak uninstall --user io.github.petar030.cosmic-pomodoro
```



## Features

- Native COSMIC applet
- Simple popup with work/break timer
- Configurable session lengths
- Panel indicator showing progress
- Notifications with sound when a session ends


## Screenshots

| Theme | Preview |
|---|---|
| Catppuccin | ![Catppuccin](img/Catpuccin.png) |
| Tokyo Night | ![TokyoNight](img/TokyoNight.png) |
| Gruvbox Dark | ![GruvboxDark](img/GruvboxDark.png) |
| Gruvbox Light | ![GruvboxLight](img/GruvboxLight.png) |
| Pop!_OS Classic | ![PopOsClassic](img/PopOsClassic.png) |
| Mono Dark | ![MonoDark](img/MonoDark.png) |
| Settings | ![Config](img/Config.png) |



## Requirements

- Rust (`cargo`)
- https://github.com/casey/just
- `flatpak` + `org.flatpak.Builder`
- COSMIC session for full applet integration testing


## Local development

```sh
just run
```


## Flatpak build (local)

This project is prepared for the **COSMIC Flatpak ecosystem** (not Flathub-specific metadata/process).

```sh
# 1) Regenerate cargo sources used by manifest
just flatpak-cargo-sources

# 2) Build + install Flatpak locally
just flatpak-builder

# 3) Create distributable .flatpak bundle
just flatpak-bundle
```

Generated bundle:

```text
io.github.petar030.cosmic-pomodoro-master.flatpak
```

---

## Test installed Flatpak

```sh
flatpak run io.github.petar030.cosmic-pomodoro
```



