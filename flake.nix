{
  description = "A very basic flake";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:/nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
      in
      {
        defaultPackage = pkgs.rustPlatform.buildRustPackage rec {
          pname = "rustlings";
          version = "4.6.0";

          src = pkgs.fetchFromGitHub {
            owner = "rust-lang";
            repo = pname;
            rev = version;
            sha256 = "sha256-Lb6JnF1jfnF4mlCYOZkmx/Ka2/jnZ3EEgfuWwrfO03E=";
          };

          cargoHash = "sha256-t29XFL0xbYZLwKCs+tLg92fgB46XO+cMYkEm325iJgw="; 

          meta = with pkgs.lib; {
            description = "Small exercises to get you used to reading and writing Rust code!";
            homepage = "https://github.com/rust-lang/${pname}";
            changelog = "https://github.com/rust-lang/${pname}/releases/tag/${version}";
            license = licenses.mit;
          };
        };
      });
}
