#!/bin/bash

# Generate Node Authorization Genesis Configuration
# This script helps create the nodeAuthorization section for genesis presets

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PRESET_DIR="${SCRIPT_DIR}/../runtime/presets"

echo "========================================="
echo "Node Authorization Genesis Generator"
echo "========================================="
echo ""

# Function to extract AccountId from subkey output
extract_account_id() {
    local uri="$1"
    subkey inspect "$uri" 2>/dev/null | grep "SS58 Address" | awk '{print $3}'
}

# Function to generate a node key and extract PeerId
generate_peer_id() {
    local output=$(subkey generate-node-key 2>&1)
    echo "$output" | tail -1
}

# Function to create node authorization entry
create_node_entry() {
    local name="$1"
    local uri="$2"

    echo "Processing: $name ($uri)"

    # Get AccountId
    account_id=$(extract_account_id "$uri")

    # Generate PeerId (in production, use actual node keys)
    # For testing with well-known keys, derive deterministically
    if [[ "$uri" == "//"* ]]; then
        # Development key - use hardcoded PeerIds for Alice, Bob, etc.
        case "$uri" in
            "//Alice")
                peer_id="12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"
                ;;
            "//Bob")
                peer_id="12D3KooWHdiAxVd8uMQR1hGWXccidmfCwLqcMpGwR6QcTP6QRMuD"
                ;;
            "//Charlie")
                peer_id="12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2"
                ;;
            "//Dave")
                peer_id="12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust"
                ;;
            "//Eve")
                peer_id="12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ"
                ;;
            *)
                echo "Warning: Unknown dev key $uri, generating random PeerId"
                peer_id=$(generate_peer_id)
                ;;
        esac
    else
        # Production key - generate new PeerId
        echo "  Generating PeerId..."
        peer_id=$(generate_peer_id)
    fi

    echo "  AccountId: $account_id"
    echo "  PeerId: $peer_id"
    echo ""

    # Return as JSON array element
    echo "      [\"$peer_id\", \"$account_id\"]"
}

# Mode selection
echo "Select mode:"
echo "1. Development (Alice & Bob)"
echo "2. Local Testnet (Alice, Bob, Charlie)"
echo "3. Custom validators from file"
echo ""
read -p "Enter choice [1-3]: " mode

case $mode in
    1)
        echo ""
        echo "Generating development config..."
        echo ""
        echo "{"
        echo "  \"nodeAuthorization\": {"
        echo "    \"nodes\": ["
        create_node_entry "Alice" "//Alice"
        echo ","
        create_node_entry "Bob" "//Bob"
        echo ""
        echo "    ]"
        echo "  }"
        echo "}"
        ;;

    2)
        echo ""
        echo "Generating local testnet config..."
        echo ""
        echo "{"
        echo "  \"nodeAuthorization\": {"
        echo "    \"nodes\": ["
        create_node_entry "Alice" "//Alice"
        echo ","
        create_node_entry "Bob" "//Bob"
        echo ","
        create_node_entry "Charlie" "//Charlie"
        echo ""
        echo "    ]"
        echo "  }"
        echo "}"
        ;;

    3)
        read -p "Enter path to validators JSON file: " validators_file

        if [ ! -f "$validators_file" ]; then
            echo "Error: File not found: $validators_file"
            exit 1
        fi

        echo ""
        echo "Processing validators from $validators_file..."
        echo ""
        echo "{"
        echo "  \"nodeAuthorization\": {"
        echo "    \"nodes\": ["

        # Read validators from JSON (expects format: [{"name": "...", "accountId": "...", "peerId": "..."}])
        first=true
        while IFS= read -r line; do
            name=$(echo "$line" | jq -r '.name')
            account_id=$(echo "$line" | jq -r '.accountId')
            peer_id=$(echo "$line" | jq -r '.peerId')

            if [ "$first" = false ]; then
                echo ","
            fi
            first=false

            echo "      [\"$peer_id\", \"$account_id\"]"
        done < <(jq -c '.[]' "$validators_file")

        echo ""
        echo "    ]"
        echo "  }"
        echo "}"
        ;;

    *)
        echo "Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "========================================="
echo "Generation complete!"
echo ""
echo "Next steps:"
echo "1. Copy the JSON output above"
echo "2. Add it to your genesis preset file"
echo "3. Make sure PeerIds match actual node keys"
echo "4. Rebuild chain spec: ./primearc-core-node build-spec --chain <preset> --raw > chainspec.json"
echo "========================================="
