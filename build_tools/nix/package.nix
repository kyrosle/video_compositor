{ rustPlatform
, openssl
, pkg-config
, ffmpeg_6-headless
, llvmPackages_16
, libGL
, cmake
, libopus
, lib
, vulkan-loader
, mesa
, darwin
, stdenv
, makeWrapper
, x264
}:
let
  ffmpeg =
    (if stdenv.isDarwin then
      (ffmpeg_6-headless.override {
        x264 = x264.overrideAttrs (old: {
          postPatch = old.postPatch + ''
            substituteInPlace Makefile --replace '$(if $(STRIP), $(STRIP) -x $@)' '$(if $(STRIP), $(STRIP) -S $@)'
          '';
        });
      })
    else
      ffmpeg_6-headless
    );
  buildInputs = [
    ffmpeg
    openssl
    libopus
    libGL
    mesa.drivers
    vulkan-loader
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Metal
    darwin.apple_sdk.frameworks.Foundation
    darwin.apple_sdk.frameworks.QuartzCore
    darwin.libobjc
  ];
  rpath = lib.makeLibraryPath buildInputs;
in
rustPlatform.buildRustPackage {
  pname = "video_compositor";
  version = "0.2.0-rc.1";
  src = ../..;
  cargoSha256 = "sha256-DrAKb/URNMoQTJ/K8qiUCd5n12mN+QVNhxcAhIoroJ8=";

  buildNoDefaultFeatures = true;
  doCheck = false;

  inherit buildInputs;
  nativeBuildInputs = [ pkg-config llvmPackages_16.clang cmake makeWrapper ];

  env.LIBCLANG_PATH = "${llvmPackages_16.libclang.lib}/lib";

  postFixup =
    ''
      rm -f $out/bin/generate_docs
      rm -f $out/bin/generate_json_schema
      rm -f $out/bin/video_compositor
      rm -f $out/bin/package_for_release
      rm -f $out/bin/update_snapshots

      mv $out/bin/main_process $out/bin/live_compositor
    '' + (
      lib.optionalString stdenv.isLinux ''
        patchelf --set-rpath ${rpath} $out/bin/live_compositor
        wrapProgram $out/bin/live_compositor \
        --prefix XDG_DATA_DIRS : "${mesa.drivers}/share" \
        --prefix LD_LIBRARY_PATH : "${mesa.drivers}/lib"
      ''
    );
}

