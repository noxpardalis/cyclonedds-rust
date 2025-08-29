{
  pkgs,
  perSystem,
  ...
}:
pkgs.mkShell {
  packages = with pkgs; [
    cargo-deny
    cargo-llvm-cov
    perSystem.self.cyclonedds-c
  ];
  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
}
