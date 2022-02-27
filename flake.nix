{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs";
    };
  };

  outputs = { self, nixpkgs }: {
    devShell = {
      "x86_64-linux" =
        let
          pkgs = import nixpkgs {
            system = "x86_64-linux";
          };

        in
        pkgs.mkShell {
          buildInputs = with pkgs; [
            alsaLib
            alsaLib
            libudev
            libxkbcommon
            pkg-config
            udev
            vulkan-loader
            wayland
            xlibs.libxcb
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${with pkgs; lib.makeLibraryPath [
              alsaLib
              libxkbcommon
              udev
              vulkan-loader
              wayland
              xlibs.libxcb
            ]}"
          '';
        };
    };
  };
}
