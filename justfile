name := 'cosmic-pomodoro'
appid := 'io.github.petar030.cosmic-pomodoro'
manifest := appid + '.json'
branch := 'master'

rootdir := ''
prefix := '/usr'
flatpak-prefix := '/app'

base-dir := absolute_path(clean(rootdir / prefix))
flatpak-base-dir := absolute_path(clean(rootdir / flatpak-prefix))
cargo-target-dir := env('CARGO_TARGET_DIR', 'target')

bin-src := cargo-target-dir / 'release' / name
bin-dst := base-dir / 'bin' / name
flatpak-bin-dst := flatpak-base-dir / 'bin' / name

desktop-src := 'resources' / 'app.desktop'
desktop-dst := base-dir / 'share' / 'applications' / appid + '.desktop'
flatpak-desktop-dst := flatpak-base-dir / 'share' / 'applications' / appid + '.desktop'

metainfo-src := 'resources' / 'app.metainfo.xml'
metainfo-dst := base-dir / 'share' / 'metainfo' / appid + '.metainfo.xml'
flatpak-metainfo-dst := flatpak-base-dir / 'share' / 'metainfo' / appid + '.metainfo.xml'

icon-src := 'resources' / 'icon.svg'
icon-dst := base-dir / 'share' / 'icons' / 'hicolor' / 'scalable' / 'apps' / appid + '.svg'
flatpak-icon-dst := flatpak-base-dir / 'share' / 'icons' / 'hicolor' / 'scalable' / 'apps' / appid + '.svg'

icon-symbolic-src := 'resources' / 'icon-symbolic.svg'
icon-symbolic-dst := base-dir / 'share' / 'icons' / 'hicolor' / 'symbolic' / 'apps' / appid + '-symbolic.svg'
flatpak-icon-symbolic-dst := flatpak-base-dir / 'share' / 'icons' / 'hicolor' / 'symbolic' / 'apps' / appid + '-symbolic.svg'

icon-png-src := 'resources' / 'icon-128.png'
icon-png-dst := base-dir / 'share' / 'icons' / 'hicolor' / '128x128' / 'apps' / appid + '.png'
flatpak-icon-png-dst := flatpak-base-dir / 'share' / 'icons' / 'hicolor' / '128x128' / 'apps' / appid + '.png'

sound-src := 'resources' / 'sounds' / 'cosmic-pomodoro-notification.wav'
sound-dst := base-dir / 'share' / 'sounds' / name / 'cosmic-pomodoro-notification.wav'
flatpak-sound-dst := flatpak-base-dir / 'share' / 'sounds' / name / 'cosmic-pomodoro-notification.wav'

default: build-release

clean:
    cargo clean

build-debug *args:
    cargo build {{args}}

build-release *args: (build-debug '--release' args)

check *args:
    cargo clippy --all-features {{args}} -- -W clippy::pedantic

check-json: (check '--message-format=json')

run *args:
    env RUST_BACKTRACE=full cargo run --release {{args}}

install:
    install -Dm0755 {{bin-src}} {{bin-dst}}
    install -Dm0644 {{desktop-src}} {{desktop-dst}}
    install -Dm0644 {{metainfo-src}} {{metainfo-dst}}
    install -Dm0644 {{icon-src}} {{icon-dst}}
    install -Dm0644 {{icon-symbolic-src}} {{icon-symbolic-dst}}
    install -Dm0644 {{icon-png-src}} {{icon-png-dst}}
    install -Dm0644 {{sound-src}} {{sound-dst}}

flatpak-install:
    install -Dm0755 {{bin-src}} {{flatpak-bin-dst}}
    install -Dm0644 {{desktop-src}} {{flatpak-desktop-dst}}
    install -Dm0644 {{metainfo-src}} {{flatpak-metainfo-dst}}
    install -Dm0644 {{icon-src}} {{flatpak-icon-dst}}
    install -Dm0644 {{icon-symbolic-src}} {{flatpak-icon-symbolic-dst}}
    install -Dm0644 {{icon-png-src}} {{flatpak-icon-png-dst}}
    install -Dm0644 {{sound-src}} {{flatpak-sound-dst}}

uninstall:
    rm -f {{bin-dst}} {{desktop-dst}} {{metainfo-dst}} {{icon-dst}} {{icon-symbolic-dst}} {{icon-png-dst}} {{sound-dst}}

flatpak-cargo-sources:
    if [ ! -d .venv ]; then python3 -m venv .venv; fi
    ./.venv/bin/pip install --quiet aiohttp toml
    ./.venv/bin/python ./flatpak/flatpak-cargo-generator.py ./Cargo.lock -o ./cargo-sources.json

flatpak-builder:
    flatpak run org.flatpak.Builder \
        --force-clean \
        --verbose \
        --ccache \
        --user \
        --install \
        --install-deps-from=flathub \
        --repo=repo \
        flatpak-out \
        {{manifest}}

flatpak-bundle:
    flatpak build-bundle repo {{appid}}-{{branch}}.flatpak {{appid}} {{branch}}
