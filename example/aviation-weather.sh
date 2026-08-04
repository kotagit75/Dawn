#!/usr/bin/env bash

USER_AGENT="btfy-temperature-fetcher/1.0"

fetch_temperature() {
    local icao_code="${1^^}"
    local timestamp="$2"

    if ! [[ "$icao_code" =~ ^[A-Z0-9]{4}$ ]]; then
        echo "Error: invalid ICAO code" >&2
        return 1
    fi
    time=$(date -u -d "@${timestamp}" '+%Y-%m-%dT%H:%M:%SZ')

    local response
    response=$(
        curl --fail --silent --show-error \
            --get 'https://aviationweather.gov/api/data/metar' \
            --user-agent "$USER_AGENT" \
            --data-urlencode "ids=${icao_code}" \
            --data-urlencode "date=${time}" \
            --data-urlencode 'format=json'
    ) || return 1

    echo "$response" | jq ".[0].temp"
}

while read -r lat lon icao_code ts; do
    temp=$(fetch_temperature "$icao_code" "$ts")

    temp10=$(awk -v t="$temp" 'BEGIN { printf("%d", int(t*10+0.5)) }')

    echo "{\"temperature\": $temp10}"
done
