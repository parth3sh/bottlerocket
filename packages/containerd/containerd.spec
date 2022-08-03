%global goproject github.com/containerd
%global gorepo containerd
%global goimport %{goproject}/%{gorepo}

%global gover 1.6.6
%global rpmver %{gover}
%global gitrev 10c12954828e7c7c9b6e0ea9b0c02b01407d3ae1

%global _dwz_low_mem_die_limit 0

Name: %{_cross_os}%{gorepo}
Version: %{rpmver}
Release: 1%{?dist}
Summary: An industry-standard container runtime
License: Apache-2.0
URL: https://%{goimport}
Source0: https://%{goimport}/archive/v%{gover}/%{gorepo}-%{gover}.tar.gz
Source1: containerd.service
Source2: containerd-config-toml_k8s
Source3: containerd-config-toml_basic
Source4: containerd-config-toml_k8s_nvidia
Source5: containerd-tmpfiles.conf

# For newer K8s variants (>=1.23) that uses `containerd.sock` as the container runtime socket
# TODO: remove once we converge all k8s variants and deprecate dockershim.sock
Source20: containerd-config-toml_k8s_containerd_sock
Source21: containerd-config-toml_k8s_nvidia_containerd_sock

# Mount for writing containerd configuration
Source100: etc-containerd.mount

# Create container storage mount point.
Source110: prepare-var-lib-containerd.service

Source1000: clarify.toml

# TODO: submit this upstream, including a unit test.
Patch1001: 1001-cri-set-default-RLIMIT_NOFILE.patch

BuildRequires: git
BuildRequires: %{_cross_os}glibc-devel
Requires: %{_cross_os}runc

%description
%{summary}.

%prep
%autosetup -Sgit -n %{gorepo}-%{gover} -p1
%cross_go_setup %{gorepo}-%{gover} %{goproject} %{goimport}

%build
%cross_go_configure %{goimport}
export BUILDTAGS="no_btrfs selinux"
export LD_VERSION="-X github.com/containerd/containerd/version.Version=%{gover}+bottlerocket"
export LD_REVISION="-X github.com/containerd/containerd/version.Revision=%{gitrev}"
for bin in \
  containerd \
  containerd-shim \
  containerd-shim-runc-v1 \
  containerd-shim-runc-v2 \
  ctr ;
do
  go build \
     -buildmode=pie \
     -ldflags="${GOLDFLAGS} ${LD_VERSION} ${LD_REVISION}" \
     -tags="${BUILDTAGS}" \
     -o ${bin} \
     %{goimport}/cmd/${bin}
done

%install
install -d %{buildroot}%{_cross_bindir}
for bin in \
  containerd \
  containerd-shim \
  containerd-shim-runc-v1 \
  containerd-shim-runc-v2 \
  ctr ;
do
  install -p -m 0755 ${bin} %{buildroot}%{_cross_bindir}
done

install -d %{buildroot}%{_cross_unitdir}
install -p -m 0644 %{S:1} %{S:100} %{S:110} %{buildroot}%{_cross_unitdir}

install -d %{buildroot}%{_cross_templatedir}
install -d %{buildroot}%{_cross_factorydir}%{_cross_sysconfdir}/containerd
install -p -m 0644 %{S:2} %{S:3} %{S:4} %{S:20} %{S:21} %{buildroot}%{_cross_templatedir}

install -d %{buildroot}%{_cross_tmpfilesdir}
install -p -m 0644 %{S:5} %{buildroot}%{_cross_tmpfilesdir}/containerd.conf

%cross_scan_attribution --clarify %{S:1000} go-vendor vendor

%files
%license LICENSE NOTICE
%{_cross_attribution_file}
%{_cross_attribution_vendor_dir}
%{_cross_bindir}/containerd
%{_cross_bindir}/containerd-shim
%{_cross_bindir}/containerd-shim-runc-v1
%{_cross_bindir}/containerd-shim-runc-v2
%{_cross_bindir}/ctr
%{_cross_unitdir}/containerd.service
%{_cross_unitdir}/etc-containerd.mount
%{_cross_unitdir}/prepare-var-lib-containerd.service
%dir %{_cross_factorydir}%{_cross_sysconfdir}/containerd
%{_cross_templatedir}/containerd-config-toml*
%{_cross_tmpfilesdir}/containerd.conf

%changelog
