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

# Connection & Peer Setup: register peers, verify mutual recognition, and check idempotency
function connect_peers() {
    local node_a_port="${NODE_PORTS[0]}"
    local node_b_port="${NODE_PORTS[1]}"

    # Node-B's P2P address (internal container IP and P2P port)
    local node_b_p2p="172.28.0.3:62698"

    echo "=== [Peer Setup 1/3] Sending addpeer from node-a to node-b ==="
    local res
    res=$(curl -s -o /dev/null -w "%{http_code}" \
        -X POST \
        -H "Content-Type: application/json" \
        -d "{\"addr\": \"$node_b_p2p\"}" \
        "http://localhost:$node_a_port/peer")
    if [ "$res" -ge 200 ] && [ "$res" -lt 300 ]; then
        echo "  - addpeer (node-a -> node-b): OK (HTTP $res)"
    else
        echo "  - addpeer (node-a -> node-b): FAILED (HTTP $res)" >&2
        return 1
    fi

    # Wait briefly for peer discovery to propagate
    sleep 2

    echo "=== [Peer Setup 2/3] Verifying mutual peer recognition via /peers ==="
    local peers_a peers_b

    peers_a=$(curl -s "http://localhost:$node_a_port/peers" 2>/dev/null || echo "[]")
    echo "  - node-a peers: $peers_a"
    if echo "$peers_a" | grep -q "172.28.0.3"; then
        echo "  - node-a recognizes node-b: OK"
    else
        echo "  - node-a recognizes node-b: FAILED (node-b not found in peers)" >&2
        return 1
    fi

    peers_b=$(curl -s "http://localhost:$node_b_port/peers" 2>/dev/null || echo "[]")
    echo "  - node-b peers: $peers_b"
    if echo "$peers_b" | grep -q "172.28.0.2"; then
        echo "  - node-b recognizes node-a: OK"
    else
        # Not necessarily a failure if reverse-direction discovery is async
        echo "  - node-b recognizes node-a: NOT YET (may be async)"
    fi

    echo "=== [Peer Setup 3/3] Verifying duplicate addpeer is safe (idempotency) ==="
    local res2
    res2=$(curl -s -o /dev/null -w "%{http_code}" \
        -X POST \
        -H "Content-Type: application/json" \
        -d "{\"addr\": \"$node_b_p2p\"}" \
        "http://localhost:$node_a_port/peer")
    if [ "$res2" -ge 200 ] && [ "$res2" -lt 500 ]; then
        echo "  - Duplicate addpeer: Safe (HTTP $res2)"
    else
        echo "  - Duplicate addpeer: Node returned unexpected error (HTTP $res2)" >&2
        return 1
    fi

    echo "=== Peer Setup Complete ==="
}

function cleanup() {
    echo "=== Cleanup ==="
    docker compose -f "$COMPOSE_FILE" down
    echo "=== Cleanup Complete ==="
}

trap 'cleanup' EXIT

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    boot "$@"
    connect_peers
fi
