# default.nix
let a=import /etc/nixos/overlays/pkgs.nix; in 
with import <nixpkgs> {
    overlays=[
    a    
    ];
};
stdenv.mkDerivation {
    name = "controller"; # Probably put a more meaningful name here
    buildInputs =  [
    (python3.withPackages (ps: with ps; [
    pip
    numpy
    scipy
    ipython
     ]))
    python39Packages.pybind11
    python39
    pkg-config
    ];
}
