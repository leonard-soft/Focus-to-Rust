# maintainer: leonard-soft leoberfo@gmail.com
pkgname=focus
pkgver=1.0.0
pkgrel=1
pkgdesc="a simple pomodoro manager written in rust"
arch=('x86_64')
url="https://github.com/leonard-soft/Focus-to-Rust"
license=('MIT')
depends=('rust' 'cargo')
makedepends=('rust' 'cargo')

original_dir="$PWD"

source=( "file://$original_dir/focus")

sha256sums=('SKIP')

build() { 
    cd "$original_dir/focus"
    cargo build --release
}

package() {
    install -Dm755 "$original_dir/focus/target/release/focus" "$pkgdir/usr/bin/focus"
    install -Dm644 "$original_dir/focus/static/sound.wav" "$pkgdir/usr/share/focus/static/sound.wav"
    install -Dm644 "$original_dir/focus/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}