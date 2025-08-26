{pkgs}: let
  buildInputs = with pkgs; [
    fenix.minimal.toolchain
    cargo-expand # todo: with fenix
    rust-analyzer # todo: above
    pkg-config

    udev
    alsa-lib-with-plugins
    vulkan-loader
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    libxkbcommon
    wayland
  ];
in
  pkgs.mkShell {
    inherit buildInputs;

    shellHook = ''
      export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath buildInputs}:$LD_LIBRARY_PATH
    '';
  }
