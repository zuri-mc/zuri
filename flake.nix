{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    utils,
  }:
    utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};
      toolchain = pkgs.rust-bin.stable.latest.default;
    in {
      formatter = pkgs.alejandra;

      devShell = with pkgs;
        mkShell {
          nativeBuildInputs = [
            clang
            lld
            pkg-config
            toolchain
          ];
          buildInputs = [
            alsa-lib
            libudev-zero
            vulkan-loader
            xorg.libX11
            xorg.libXrandr
            xorg.libXcursor
            xorg.libXi
          ];
          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
              pkgs.lib.makeLibraryPath [
                udev
                alsaLib
                vulkan-loader
              ]
            }"
          '';
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
    });
}
