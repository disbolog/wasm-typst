{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        flake-parts.url = "github:hercules-ci/flake-parts";
        fenix = {
            url = "github:nix-community/fenix/monthly";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = inputs @ {
        flake-parts,
        fenix,
        ...
    }:
        flake-parts.lib.mkFlake {inherit inputs;} {
            systems = [
                "x86_64-linux"
                "aarch64-linux"
                "aarch64-darwin"
                "x86_64-darwin"
            ];

            perSystem = {pkgs, ...}: {
                devShells.default = pkgs.mkShell {
                    packages = with pkgs; [
                        # utilities
                        cargo-generate
                        cargo-leptos

                        # wasm
                        trunk
                        wasm-pack
                        openssl
                        # perl
                        wasm-bindgen-cli

                        # development
                        sccache # fast compilation
                        pkg-config # potentially required to be put in `buildInputs` and `nativeBuildInputs`
                        llvmPackages.bintools # potentially # it has the lld linker, for wasm plugin in typst for whatever reason it uses lld # https://matklad.github.io/2022/03/14/rpath-or-why-lld-doesnt-work-on-nixos.html
                        # openssl # potentially
                        (
                            fenix.packages.${system}.combine [
                                (
                                    fenix.packages.${system}.complete.withComponents [
                                        "cargo"
                                        "clippy"
                                        "rust-src"
                                        "rustc"
                                        "rustfmt"
                                        "rustc-codegen-cranelift-preview"
                                        # "llvm-tools-preview" # potentially for faster linker
                                    ]
                                    # rust-analyzer-nightly
                                    # fenix.packages.${system}.targets.wasm32-unknown-unknown.latest.rust-std
                                    # TODO vscode extension
                                )
                                fenix.packages.${system}.rust-analyzer
                                fenix.packages.${system}.targets.wasm32-unknown-unknown.latest.rust-std
                            ]
                        )
                        # inputs.fenix.packages.${system}.rust-analyzer-vscode-extension
                    ];

                    shellHook = ''
                        export RUSTC_WRAPPER=sccache
                        export CARGO_INCREMENTAL=0

                        # Cranelift and optimization settings
                        export CARGO_PROFILE_DEV_OPT_LEVEL=3
                        export CARGO_PROFILE_DEV_LTO=false

                        # Enable Cranelift and faster linking
                        # Temporarily remove cranelift for wasm compilation
                        export RUSTFLAGS=""

                        # -Z options are for cargo-nightly, -C are for usual cargo
                        # export RUSTFLAGS="-Z codegen-backend=cranelift -C link-arg=-fuse-ld=lld" # potentially for faster linker
                        # export RUSTFLAGS="-C link-args=-fuse-ld=lld"
                    '';
                };
            };
        };
}
