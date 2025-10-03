{
  pkgs,
  inputs,
  ...
}:
pkgs.stdenv.mkDerivation {
  name = "cyclonedds";
  src = inputs.cyclonedds-sources;
  nativeBuildInputs = with pkgs; [
    cmake
    ninja
  ];
  cmakeFlags = [ "-DEXPORT_ALL_SYMBOLS=true" ];
}
