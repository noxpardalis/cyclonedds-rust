{
  flake,
  inputs,
  pkgs,
  ...
}:
let
  treefmtEval = inputs.treefmt-nix.lib.evalModule pkgs {
    projectRootFile = "flake.nix";

    programs = {
      # Nix
      # begin-sorted start
      deadnix.enable = true;
      nixf-diagnose.enable = true;
      nixfmt.enable = true;
      statix.enable = true;
      # begin-sorted end

      # Markdown
      mdformat.enable = true;

      # TOML
      taplo.enable = true;

      # Rust
      rustfmt.enable = true;

      # C
      clang-format.enable = true;

      # Spell-checking source code
      typos.enable = true;

      # Source-agnostic lexicographic sorting
      keep-sorted.enable = true;

      # GitHub actions.
      # begin-sorted start
      actionlint.enable = true;
      pinact.enable = true;
      # begin-sorted end
    };
  };
  formatter = treefmtEval.config.build.wrapper;
in
formatter
// {
  passthru = formatter.passthru // {
    tests = {
      check = treefmtEval.config.build.check flake;
    };
  };
}
