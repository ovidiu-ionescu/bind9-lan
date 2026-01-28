clean-build: build clean clean-deb
  @echo "The deb file is ready in the parent directory."

build:
  #dpkg-buildpackage -us -uc
  debuild -us -uc

install-dependencies:
  sudo apt install -y debhelper devscripts build-essential

clean:
  #cd debian && rm -rf debhelper-build-stamp bind9-lan.substvars bind9-lan files .debhelper
  debclean
  rm -rf debian/man

clean-deb:
  rm ../bind9-lan_*_amd64.buildinfo ../bind9-lan_*_amd64.changes ../bind9-lan_*.dsc ../bind9-lan_*.tar.xz ../bind9-lan_*_amd64.build

check-man-page:
  dpkg-deb -c ../bind9-lan_0.1.*_amd64.deb | grep man

