#!/bin/bash

set -u

cargo build --release

mkdir -p "${PREFIX}/bin"
cp target/release/uniparc_xml_parser ${PREFIX}/bin/
