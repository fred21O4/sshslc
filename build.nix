{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "sshslc";
  version = "0.0.1";

  src = ./.;

  cargoHash = "sha256-TAqPwKsfSfrVQC3gKavUBmt/qvvVLHvKDsTo+lDT7Mo=";

  meta = {
    description = "A short lived ssh certificate issuing server";
    license = lib.licenses.mit;
    maintainers = [];
  };
})
