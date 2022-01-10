{
  description = "A rust CLI tool to control older Pioneer recievers over the internet";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs;
  };

  outputs = inputs:
    with inputs;
    let
      systems = [
        "aarch64-linux"
        "aarch64-darwin"
        "i686-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      config = system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        swayhide = pkgs.rustPlatform.buildRustPackage {
          name = "pioneerctl";
          version = "0.1.4";
          src = ./.;
          cargoSha256 = "sha256-FvqUXMChVpNEEnXDxswAxKDq04Bipwso8s3myeSmeWU=";
        };
      in {
        defaultPackage.${system} = swayhide;

        devShell.${system} = pkgs.mkShell {
          buildInputs = with pkgs; [ rustc cargo rustfmt ];
        };
      };
    in builtins.foldl' (acc: system: acc // (config system)) { } systems;
}
