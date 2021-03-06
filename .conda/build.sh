#!/bin/bash

set -u

# Set CARGO linker and archiver
mkdir .cargo
cat <<EOF > .cargo/config
[target.x86_64-unknown-linux-gnu]
linker = "${CC}"
ar = "${AR}"
EOF
# Alternatively, we could have used environment variables
# export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="${CC}"
# export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_ARCHIVER="${AR}"

cargo build --release

mkdir -p "${PREFIX}/bin"
cp target/release/uniparc_xml_parser ${PREFIX}/bin/
