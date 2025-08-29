{
  pkgs,
  inputs,
  perSystem,
  ...
}:

let
  src = ../.;
  rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile "${src}/rust-toolchain.toml";
  crane-lib = (inputs.crane.mkLib pkgs).overrideToolchain rust-toolchain;
in
crane-lib.devShell {
  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  inputsFrom = [ perSystem.self.default ];

  packages =
    with pkgs;
    [
      cargo-audit
      cargo-deny
      cargo-nextest

      perSystem.self.cyclonedds-c
    ]
    ++ lib.optionals stdenv.isLinux [ cargo-llvm-cov ];

}
