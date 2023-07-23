# Maintainer: Erica Marigold <hi@devcomp.xyz>

pkgname=lune-git
pkgver=0.7.5
pkgrel=1
pkgdesc="GIT - A standalone Luau script runtime"
arch=('any')
url="https://github.com/filiptibell/lune"
license=('MPL')
makedepends=('just' 'rust')
provides=('lune')
conflicts=('lune')
source=("git+$url.git")
sha256sums=('SKIP')

prepare() {
  cd "$srcdir/lune"
  git submodule update --init
}

pkgver() {
  cd "$srcdir/lune"
  git rev-parse v$pkgver
}

build() {
  cd "$srcdir/lune"

  git checkout $pkgver
  just build
}

check() {
  cd "$srcdir/lune"
  
  just test
}

package() {
  cd "$srcdir/lune"
  mkdir -p "$pkgdir/usr/bin"

  mv ./target/release/lune "$pkgdir/usr/bin"
}
