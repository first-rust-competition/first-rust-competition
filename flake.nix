{
  inputs = {
    nci.url = "github:yusdacra/nix-cargo-integration";
  };
  outputs = inputs: let
    wpilib-toolchain = pkgs:
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
      };
  in
    inputs.nci.lib.makeOutputs {
      root = ./.;
      pkgsOverlays = [
      ];

      overrides.shell = common: prev: {
        packages = prev.packages ++ (with common.pkgs; [(wpilib-toolchain pkgs) cargo-outdated jdk11 fmt.dev libcxxStdenv]);
        env =
          prev.env
          ++ [
            {
              name = "LIBCLANG_PATH";
              eval = "${common.pkgs.libclang.lib}/lib";
            }
            {
              name = "BINDGEN_EXTRA_CLANG_ARGS";
              eval = with common.pkgs; ''"
                $(< ${stdenv.cc}/nix-support/libc-crt1-cflags) \
                      $(< ${stdenv.cc}/nix-support/libc-cflags) \
                      $(< ${stdenv.cc}/nix-support/cc-cflags) \
                      $(< ${stdenv.cc}/nix-support/libcxx-cxxflags) \
                      ${lib.optionalString stdenv.cc.isClang "-idirafter ${stdenv.cc.cc}/lib/clang/${lib.getVersion stdenv.cc.cc}/include"} \
                      ${lib.optionalString stdenv.cc.isGNU "-isystem ${stdenv.cc.cc}/include/c++/${lib.getVersion stdenv.cc.cc} -isystem ${stdenv.cc.cc}/include/c++/${lib.getVersion stdenv.cc.cc}/${stdenv.hostPlatform.config} -idirafter ${stdenv.cc.cc}/lib/gcc/${stdenv.hostPlatform.config}/${lib.getVersion stdenv.cc.cc}/include"} \
              "'';
            }
          ];
      };
    }
    // {
      templates = {
        basic = {
          path = ./examples/basic;
          description = "A basic example.";
        };
      };
    };
}
