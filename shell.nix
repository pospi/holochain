{ ... }:

let
  default = import (builtins.toString ./default.nix);
  holonix = default.holonix;

in default.shells.legacy
