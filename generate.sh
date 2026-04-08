#!/bin/sh

mkdir -p out

[ -z "$1" ] && set -- \
	Australia/Sydney \
	Australia/Adelaide \
	America/Chicago \
	America/New_York \
	Europe/Amsterdam

set -xe

for tz in "$@"; do
	cargo run $tz 50 150 > "./out/$(basename "$tz").ics"
done
