{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "sshslc";
  version = "0.0.1";

  src = ./.;

  cargoHash = "sha256-92M6KcS4xyAo5FXJ8GKZIAf2S01CiMDUeFgowSwWod8=";

  meta = {
    mainProgram = "sshslc";
    description = "A dead simple short lived ssh certificate issuing server";
    license = lib.licenses.mit;
    maintainers = [];
  };
})
