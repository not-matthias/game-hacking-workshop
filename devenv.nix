{pkgs, ...}: {
  env.RUST_LOG = "info";
  # env.NIXPKGS_ALLOW_UNFREE = "1";

  packages = with pkgs; [
    # assaultcube
    chromium # Required for marp
  ];

  languages.rust = {
    enable = true;
    version = "latest";
  };

  languages.javascript.enable = true;

  enterShell = ''
    npm set prefix ~/.npm-global
    # npm i -g  @marp-team/marp-cli

    export PATH="$HOME/.npm-global/bin:$PATH"
  '';

  scripts.slides.exec = ''
    marp -s slides/
  '';
}
