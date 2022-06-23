{
  config,
  lib,
  pkgs,
  ...
}:
pkgs.stdenv.mkDerivation rec {
  name = "wpilib-cross-compiler";
  version = "2022-1";

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
    url = "https://github.com/wpilibsuite/roborio-toolchain/releases/download/v2022-1/FRC-2022-Linux-Toolchain-7.3.0.tar.gz";
    sha256 = "sha256-snzeMC5G0RUkrt9mQSm8OsffAqeND55Ks/H+tA1merQ=";
  };
  sourceRoot = ".";

  installPhase = ''
    cp -r frc2022/roborio $out
  '';
}
