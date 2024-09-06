name := 'anylist'
export APPID := 'com.luzynski.anylist'

rootdir := ''
prefix := '/usr'
flatpak-prefix := '/app'

base-dir := absolute_path(clean(rootdir / prefix))
flatpak-base-dir := absolute_path(clean(rootdir / flatpak-prefix))

export INSTALL_DIR := base-dir / 'share'

bin-src := 'target' / 'release' / name
bin-dst := base-dir / 'bin' / name
flatpak-bin-dst := flatpak-base-dir / 'bin' / name

desktop := APPID + '.desktop'
desktop-src := 'res' / desktop
desktop-dst := clean(rootdir / prefix) / 'share' / 'applications' / desktop
flatpak-desktop-dst := clean(rootdir / flatpak-prefix) / 'share' / 'applications' / desktop

metainfo := APPID + '.metainfo.xml'
metainfo-src := 'res' / metainfo
metainfo-dst := clean(rootdir / prefix) / 'share' / 'metainfo' / metainfo
flatpak-metainfo-dst := clean(rootdir / flatpak-prefix) / 'share' / 'metainfo' / metainfo

icons-src := 'res' / 'icons' / 'hicolor'
icons-dst := clean(rootdir / prefix) / 'share' / 'icons' / 'hicolor'
flatpak-icons-dst := clean(rootdir / flatpak-prefix) / 'share' / 'icons' / 'hicolor'

# Default recipe which runs `just build-release`
default: build-release

# Runs `cargo clean`
clean:
    cargo clean

# Compiles with debug profile
build-debug *args:
    cargo tauri build --debug {{args}}

# Compiles with release profile
build-release *args: 
    cargo tauri build {{args}}

# Runs a clippy check
check *args:
    cargo clippy --all-features {{args}} -- -W clippy::pedantic

dev *args:
    cargo tauri dev

# Installs files
install:
    install -Dm0755 {{bin-src}} {{bin-dst}}
    install -Dm0644 {{desktop-src}} {{desktop-dst}}
    install -Dm0644 {{metainfo-src}} {{metainfo-dst}}
    for size in `ls {{icons-src}}`; do \
        install -Dm0644 "{{icons-src}}/$size/apps/{{APPID}}.svg" "{{icons-dst}}/$size/apps/{{APPID}}.svg"; \
    done

# Installs files
flatpak:
    install -Dm0755 {{bin-src}} {{flatpak-bin-dst}}
    install -Dm0644 {{desktop-src}} {{flatpak-desktop-dst}}
    install -Dm0644 {{metainfo-src}} {{flatpak-metainfo-dst}}
    for size in `ls {{icons-src}}`; do \
        install -Dm0644 "{{icons-src}}/$size/apps/{{APPID}}.svg" "{{flatpak-icons-dst}}/$size/apps/{{APPID}}.svg"; \
    done

# Uninstalls installed files
uninstall:
    rm {{bin-dst}}
    rm {{desktop-dst}}
    rm {{metainfo-dst}}
    for size in `ls {{icons-src}}`; do \
        rm "{{icons-dst}}/$size/apps/{{APPID}}.svg"; \
    done

