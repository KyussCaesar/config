#!/bin/bash
set -euo pipefail

# users and groups
groupadd -o -g "$GID" host
useradd -o -u "$UID" -g "$GID" -s /bin/bash -m "$USER"
