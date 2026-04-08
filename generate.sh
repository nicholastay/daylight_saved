#!/bin/sh

[ -z "$1" ] && set -- \
	Australia/Sydney \
	Australia/Melbourne \
	Australia/Adelaide \
	America/Chicago \
	America/New_York \
	Europe/Amsterdam

set -xe

f="./docs/index.html"
echo '<h1>daylight_saved</h1>' > $f
echo '<p>Code/README: <a href="https://github.com/nicholastay/daylight_saved">https://github.com/nicholastay/daylight_saved</a></p>' >> $f
echo "<p>Generated at: $(date)</p>" >> $f

echo "<p>.ics files to subscribe to for DST:<ul>" >> $f
for tz in "$@"; do
	cargo run $tz 50 150 > "./docs/$(basename "$tz").ics"
	echo "<li><a href=\"/$(basename "$tz").ics\">$tz</a></li>" >>$f
done
echo "</ul></p>" >> $f
