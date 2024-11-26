{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs = {
      url = "github:cachix/devenv-nixpkgs/rolling"; # https://github.com/cachix/devenv-nixpkgs
    };

    systems = {
      url = "github:nix-systems/default"; # https://github.com/nix-systems/default
    };
    flake-utils = {
      url = "github:numtide/flake-utils"; # https://github.com/numtide/flake-utils
      inputs.systems.follows = "systems";
    };
    nix-pre-commit = {
      url = "github:jmgilman/nix-pre-commit"; # https://github.com/jmgilman/nix-pre-commit
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay"; # https://github.com/oxalica/rust-overlay
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, systems, flake-utils, nix-pre-commit }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            self.overlays.default
          ];
        };
      });
    in
    {
      overlays.default = final: prev: {
        rustToolchain =
          let
            rust = prev.rust-bin;
          in
          if builtins.pathExists ./rust-toolchain.toml then
            rust.fromRustupToolchainFile ./rust-toolchain.toml
          else if builtins.pathExists ./rust-toolchain then
            rust.fromRustupToolchainFile ./rust-toolchain
          else
            rust.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [
                "rust-src"
                "rustfmt"
              ];
            });
      };

      devShells = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            # Nix
            nil

            # Rust
            rustToolchain
            openssl
            pkg-config
            cargo-deny
            cargo-edit
            cargo-watch
            rust-analyzer

            # Rust dev
            bacon
          ];

          env = {
            # Required by rust-analyzer
            RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
          };

          shellHook = (nix-pre-commit.lib.${pkgs.system}.mkConfig {
            inherit pkgs;
            config = {
              repos = [
                {
                  repo = "local";
                  hooks = [
                    {
                      id = "rust-fmt";
                      entry = "${pkgs.rustToolchain}/bin/cargo fmt";
                      language = "rust";
                      files = "\\.rs";
                    }
                    {
                      id = "nixpkgs-fmt";
                      entry = "${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt";
                      language = "system";
                      files = "\\.nix";
                    }
                  ];
                }
              ];
            };
          }).shellHook;
        };
      });
    };
}
