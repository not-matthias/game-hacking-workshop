{pkgs, ...}: {
  env.RUST_LOG = "info";
  env.NIXPKGS_ALLOW_UNFREE = "1";

  packages = with pkgs; [
    assaultcube
  ];

  languages.rust.enable = true;
}
