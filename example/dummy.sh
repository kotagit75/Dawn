#!/usr/bin/env bash

set -euo pipefail

while read -r lat lon icao_code ts; do
    sleep 0.04
    echo "{\"temperature\": 10}"
done
