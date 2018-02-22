#!/bin/bash

set -u

export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="${CC}"

cargo build --release

mkdir -p "${PREFIX}/bin"
cp target/release/uniparc_xml_parser ${PREFIX}/bin/
