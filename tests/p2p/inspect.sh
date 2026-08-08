#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPOSE_FILE="$SCRIPT_DIR/docker-compose.yaml"

NODE_PORTS=(8080 8081)
NODE_NAMES=("node-a" "node-b")

function boot() {
    echo "=== [Preparation 1/3] Starting Node A/B and checking ports/IPs ==="
    docker compose -f "$COMPOSE_FILE" up -d --build

    for i in "${!NODE_NAMES[@]}"; do
        local name="${NODE_NAMES[$i]}"
        local port="${NODE_PORTS[$i]}"
        local ip
        ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "btfy-$name" 2>/dev/null || echo "N/A")
        echo "  - $name: API Port = $port, Container IP = $ip"
    done

    echo "=== [Preparation 2/3] Checking health status for Node A/B (health) ==="
    for port in "${NODE_PORTS[@]}"; do
        local url="http://localhost:$port/health"
        echo -n "  - Waiting for $url ... "
        local retries=30
        local healthy=false
        while [ $retries -gt 0 ]; do
            if res=$(curl -s "$url" 2>/dev/null || true) && [ "$res" = "ok" ]; then
                healthy=true
                break
            fi
            sleep 1
            ((retries--))
        done

        if [ "$healthy" = true ]; then
            echo "OK"
        else
            echo "FAILED"
            echo "Error: Node on port $port failed health check." >&2
            return 1
        fi
    done

    echo "=== [Preparation 3/3] Fetching and recording node addresses ==="
    for i in "${!NODE_NAMES[@]}"; do
        local name="${NODE_NAMES[$i]}"
        local port="${NODE_PORTS[$i]}"
        local address
        address=$(curl -s "http://localhost:$port/address" 2>/dev/null || echo "N/A")
        echo "  - $name (Port $port) Address: $address"
    done

    echo "=== Preparation Complete ==="
}

function cleanup() {
    echo "=== Cleanup ==="
    docker compose -f "$COMPOSE_FILE" down
    echo "=== Cleanup Complete ==="
}

function inspect() {
    echo "=== Inspect ==="
    echo "=== Inspect Complete ==="
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    boot "$@"
    inspect
    cleanup
fi
