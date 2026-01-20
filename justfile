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

clean-deb:
  rm ../bind9-lan_0.1.0_amd64.buildinfo ../bind9-lan_0.1.0_amd64.changes ../bind9-lan_0.1.0.dsc ../bind9-lan_0.1.0.tar.xz ../bind9-lan_0.1.0_amd64.build


