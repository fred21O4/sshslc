{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
  };

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    platforms = ["x86_64-linux"];
    forEachPlatform = f:
      nixpkgs.lib.genAttrs platforms (
        system:
          f {
            pkgs = import nixpkgs {inherit system;};
          }
      );
  in {
    packages = forEachPlatform ({pkgs}: {
      default = pkgs.callPackage ./build.nix {};
      sshslc = pkgs.callPackage ./build.nix {};
    });

    devShells = forEachPlatform ({pkgs}: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          alejandra

          rust-analyzer
          rustfmt
        ];

        inputsFrom = [self.packages.${pkgs.stdenv.hostPlatform.system}.default];
      };
    });
    formatter = forEachPlatform ({pkgs}: pkgs.alejandra);
  };
}
