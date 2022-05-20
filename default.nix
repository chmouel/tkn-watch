{ lib
, naersk
, stdenv
, clangStdenv
, hostPlatform
, targetPlatform
, pkg-config
, libiconv
, rustfmt
, cargo
, rustc
, openssl
}:

let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
in

naersk.lib."${targetPlatform.system}".buildPackage rec {
  src = ./.;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [
    rustfmt
    pkg-config
    cargo
    rustc
    libiconv
    openssl
  ];
  checkInputs = [ cargo rustc ]; # just for the host building the package

  doCheck = true;
  CARGO_BUILD_INCREMENTAL = "false";
  RUST_BACKTRACE = "full";
  copyLibs = true;

  name = cargoToml.package.name;
  version = cargoToml.package.version;

  meta = with lib; {
    description = cargoToml.package.description;
    homepage = cargoToml.package.homepage;
    license = with licenses; [ asl20 ];
    maintainers = with maintainers; [ vdemeester chmouel ];
  };
}
