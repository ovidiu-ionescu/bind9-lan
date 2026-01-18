install-nvim: build
  install -D libtree-sitter-orgncf.so ~/.local/share/nvim/site/parser/orgncf.so
  mkdir -p ~/.local/share/nvim/site/queries/orgncf
  cp -r queries/* ~/.local/share/nvim/site/queries/orgncf/
  nvim ../netconfig.ncf -c 'InspectTree'

build:
  tree-sitter generate
  make

