with import <nixpkgs> {
  config.android_sdk.accept_license = true;
  config.allowUnfree = true;
};

stdenv.mkDerivation {
  name = "qaul";
  buildInputs = with pkgs; [

    # General rust stuff
    rustracer rustup clangStdenv cargo-watch
    binutils
    
    # Required for the docs
    mdbook graphviz

    # Required for Android integration
    cmake

    # Required for libqaul-voice
    libopus pkg-config
    steam-run

    # webgui debugging and development
    httpie
    nodejs

    # Required for the code coverage and stuff
    openssl
  ] ++ (with androidenv.androidPkgs_9_0; [
    # Required for Android builds
    androidsdk
    build-tools
    ndk-bundle
    platform-tools

    pkgs.openjdk
  ]);
}
