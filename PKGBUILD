# PKGBUILD

pkgname=wayruler-git
pkgver=r1.1234567
pkgrel=1
pkgdesc="A native Wayland smart screen measurement tool for KDE Plasma (PowerToys Screen Ruler clone)"
arch=('x86_64')
url="https://github.com/Mahmoud-walid/WayRuler"
license=('Apache')
depends=('gtk4' 'gtk4-layer-shell' 'cairo' 'dbus' 'glibc')
makedepends=('cargo' 'git' 'rust')
provides=('wayruler')
conflicts=('wayruler')
source=("git+https://github.com/Mahmoud-walid/WayRuler.git")
md5sums=('SKIP')

pkgver() {
  cd "$srcdir/wayruler"
  printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
  cd "$srcdir/wayruler"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --release --locked
}

package() {
  cd "$srcdir/wayruler"
  install -Dm 755 target/release/wayruler -t "$pkgdir/usr/bin"
  
  # Optional: Install desktop file for autostart
  # install -Dm 644 wayruler.desktop -t "$pkgdir/usr/share/applications"
}