{
  inputs = {
    nci.url = "github:yusdacra/nix-cargo-integration";
  };
  outputs = inputs:
    inputs.nci.lib.makeOutputs {
      root = ./.;

      overrides.shell = common: prev: {
        packages = prev.packages ++ (with common.pkgs; [(pkgs.callPackage ./nix/pkgs/wpilib-toolchain.nix {}) cargo-outdated jdk11 libcxxStdenv]);
        env =
          prev.env
          ++ [
            {
              name = "LIBCLANG_PATH";
              eval = "${common.pkgs.libclang.lib}/lib";
            }
            {
              name = "BINDGEN_EXTRA_CLANG_ARGS";
              eval = with common.pkgs; ''"$(< ${stdenv.cc}/nix-support/libc-crt1-cflags) \
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
