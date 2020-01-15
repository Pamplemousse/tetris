with import <nixpkgs> { };

stdenv.mkDerivation rec {
  name = "tetris";

  buildInputs = [
    rustup
    SDL2
  ];
}
