{
  config,
  lib,
  pkgs,
  ...
}:
pkgs.stdenv.mkDerivation rec {
  name = "wpilib-cross-compiler-old";
  version = "2021-2";

  nativeBuildInputs = with pkgs; [
    # Patch our binaries!
    autoPatchelfHook

    # Binary dependencies (patched during build)
    ncurses5.dev
    zlib.dev
    expat.dev
    xz.dev
    python27Full
    libclang.dev
  ];

  src = pkgs.fetchurl {
    url = "https://github.com/wpilibsuite/roborio-toolchain/releases/download/v2021-2/FRC-2021-Linux-Toolchain-7.3.0.tar.gz";
    sha256 = "sha256-AMxYvwYH1x5yWRnSjiJxTOGSBpLEhkvIY1P8gTnL97c=";
  };
  sourceRoot = ".";

  installPhase = ''
    cp -r frc2021/roborio $out
    rm -rfv $out/share # remove share to avoid collisions with newer versions.
  '';
}
