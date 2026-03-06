#!/usr/bin/env bash

mkdir -p out

TIMEZONES=(
	Australia/Sydney
	Australia/Adelaide
	America/Chicago
	America/New_York
	Europe/Amsterdam
)

set -xe

for tz in "${TIMEZONES[@]}"; do
	cargo run $tz > "./out/$(basename "$tz").ics"
done
