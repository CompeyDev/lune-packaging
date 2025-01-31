Name:           lune
Version:        0.8.9
Release:        1%{?dist}
Summary:        A standalone Luau runtime


License:        MPL2
URL:            https://github.com/lune-org/lune
Source0:        https://github.com/lune-org/lune/archive/refs/tags/v%{version}.tar.gz


ExclusiveArch: x86_64 aarch64


BuildRequires: cmake
BuildRequires: rust
BuildRequires: cargo
BuildRequires: just
BuildRequires: clang
BuildRequires: perl
%ifarch aarch64
BuildRequires: binutils-aarch64-linux-gnu
BuildRequires: sysroot-aarch64-glibc
BuildRequires: gcc-aarch64-linux-gnu
BuildRequires: musl-devel
%endif


Requires: libstdc++
Requires: libgcc
Requires: openlibm
Requires: glibc


%description
%{summary}.


%prep
%setup -q -n lune-%{version}
# Removes the first line containing clippy lint attribute which is falsely flagged as a shebang in build
find . -type f -name "*.rs" -exec perl -i -ne 'print unless /^\#!\[/ ' {} +


%build
just build --locked --release


%install
mkdir -p %{buildroot}/%{_bindir}
install -m0755 ./target/release/%{name} %{buildroot}/%{_bindir}/%{name}


%files
%license LICENSE.txt
%{_bindir}/%{name}


%changelog
%autochangelog

