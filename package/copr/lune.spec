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
BuildRequires: just
BuildRequires: musl-tools
BuildRequires: clang
BuildRequires: musl
BuildRequires: musl-devel


Requires: libstdc++
Requires: libgcc
Requires: openlibm
Requires: glibc


%description
%{summary}.


%prep
%setup -q -n lune-%{version}


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

