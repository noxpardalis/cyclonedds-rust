{
  description = ''
    Rust binding for Eclipse Cyclone DDS
  '';

  inputs = {
    # Nix
    nixpkgs-stable.url = "github:nixos/nixpkgs?ref=nixos-25.11";
    nixpkgs-unstable.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    nixpkgs-latest.url = "github:nixos/nixpkgs";
    nixpkgs.follows = "nixpkgs-stable";

    # Tools
    blueprint = {
      url = "github:numtide/blueprint";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Programming
    ## Rust
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
    };

    # External sources
    cyclonedds-sources = {
      url = "github:eclipse-cyclonedds/cyclonedds";
      flake = false;
    };
  };

  outputs =
    inputs:
    inputs.blueprint {
      inherit inputs;
      nixpkgs.overlays = [
        inputs.rust-overlay.overlays.default
      ];
      prefix = "nix/";
    };
}
