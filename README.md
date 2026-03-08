# Cosmic Pomodoro

Minimal Pomodoro applet for COSMIC desktop.

## Features

- Work / break timer flow with configurable durations
- Phase switch controls (`Start`, `Pause`, `Forward`, `Restart`)
- Session indicators and cycle counter in the popup UI
- Panel icon with compact progress bar while a session is running
- Desktop notifications + sound on phase transition

## Requirements

- Rust toolchain (`rustup`, `cargo`)
- `just` command runner
- COSMIC desktop session (for applet integration)
- Linux audio backend (`paplay` or `aplay`) for notification sound

## Quick Start

```sh
just run
```

Useful commands:

- `just` or `just build-release` – build release binary
- `just run` – run locally from source tree
- `just install` – install binary + desktop metadata + icons + sound
- `just uninstall` – remove installed files
- `just check` – run clippy
- `just flatpak-cargo-sources` – regenerate Flatpak cargo source list
- `just flatpak-builder` – local Flatpak build + install
- `just flatpak-bundle` – generate `io.github.petar030.cosmic-pomodoro-master.flatpak`

## Install (system)

```sh
just build-release
sudo just install
```

After install, restart panel/session if icon cache is stale.

## Localization

Translations use Fluent. Files are under [i18n](i18n). You can copy [i18n/en](i18n/en) and create a new locale directory.

## Packaging

For distro/offline builds:

```sh
just vendor
just build-vendored
just rootdir=debian/cosmic-pomodoro prefix=/usr install
```

## Publishing Checklist

Before tagging a release:

- Update version in [Cargo.toml](Cargo.toml)
- Ensure [resources/app.desktop](resources/app.desktop) metadata is final
- Ensure [resources/app.metainfo.xml](resources/app.metainfo.xml) points to your real repo/icon URLs
- Run `cargo check` and `just check`
- Test both `just run` and installed mode (`just install`)

## Repository

Project repository: https://github.com/petar030/cosmic-pomodoro
