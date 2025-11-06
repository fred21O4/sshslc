{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "sshslc";
  version = "0.0.2";

  src = ./.;

  cargoHash = "sha256-ZxZ1cemk0vjKnDEMt6MXaiqs7kihQs1YM7dAfCgQFVI=";

  meta = {
    mainProgram = "sshslc";
    description = "A dead simple short lived ssh certificate issuing server";
    license = lib.licenses.mit;
    maintainers = [];
  };
})
