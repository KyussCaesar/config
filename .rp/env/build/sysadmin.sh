#!/bin/bash
set -euo pipefail

trace () {
  set -x
  "$@"
  set +x
}

export DEBIAN_FRONTEND=noninteractive

pkgs=(
)

if [ ${#pkgs[@]} -ne 0 ]
then
  # update the package lists
  apt-get update

  # upgrade all installed packages
  # the base is debian, if your shit breaks it means *you* have a problem
  apt-get upgrade

  trace apt-get --yes --no-install-recommends install "${pkgs[@]}"

  apt-get --yes autoremove
  apt-get --yes clean autoclean

  rm -rf /var/lib/apt/lists/*

else
  echo >&2 "No apt-get packages to install."
fi
