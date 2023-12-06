with import <nixpkgs> {};
mkShell {
 packages = [
    bashInteractive
    rustc
    cargo
    cargo-edit
  ];
}
