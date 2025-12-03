#!/bin/bash

# Apply RPC initialization fix to 13 PBC collators
# Template: BTC-PBC Collator

set -e

BASE_PATH="/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes"
TEMPLATE_RPC="$BASE_PATH/btc-pbc-collator/src/rpc.rs"

# List of collators to fix
COLLATORS=("xrp" "bnb" "sol" "ada" "doge" "trx" "matic" "xlm" "link" "sc-usdt" "edsc" "eth" "ai-compute")

SUCCESS_COUNT=0
FAILED_COLLATORS=()

echo "========================================"
echo "RPC FIX AUTOMATION SCRIPT"
echo "========================================"
echo "Template: btc-pbc-collator"
echo "Collators to fix: ${#COLLATORS[@]}"
echo ""

for pbc in "${COLLATORS[@]}"; do
    echo "----------------------------------------"
    echo "Processing: $pbc-pbc-collator"
    echo "----------------------------------------"

    COLLATOR_DIR="$BASE_PATH/${pbc}-pbc-collator"

    # Check if collator directory exists
    if [ ! -d "$COLLATOR_DIR" ]; then
        echo "❌ ERROR: Directory not found: $COLLATOR_DIR"
        FAILED_COLLATORS+=("$pbc (dir not found)")
        continue
    fi

    # Convert hyphenated names to underscores for runtime names
    RUNTIME_NAME="${pbc//-/_}_pbc_runtime"
    PBC_UPPER=$(echo "$pbc" | tr '[:lower:]' '[:upper:]')

    echo "  Runtime name: $RUNTIME_NAME"
    echo "  PBC uppercase: $PBC_UPPER"

    # Step 1: Create rpc.rs
    echo "  [1/3] Creating src/rpc.rs..."
    RPC_FILE="$COLLATOR_DIR/src/rpc.rs"

    # Copy template and replace names
    sed -e "s/BTC-PBC/${PBC_UPPER}-PBC/g" \
        -e "s/BTC Partition/${PBC_UPPER} Partition/g" \
        -e "s/btc_pbc_runtime/${RUNTIME_NAME}/g" \
        "$TEMPLATE_RPC" > "$RPC_FILE"

    if [ ! -f "$RPC_FILE" ]; then
        echo "  ❌ Failed to create rpc.rs"
        FAILED_COLLATORS+=("$pbc (rpc.rs creation failed)")
        continue
    fi
    echo "  ✅ Created rpc.rs"

    # Step 2: Update main.rs
    echo "  [2/3] Updating src/main.rs..."
    MAIN_FILE="$COLLATOR_DIR/src/main.rs"

    # Check if 'mod rpc;' already exists
    if grep -q "^mod rpc;" "$MAIN_FILE" 2>/dev/null; then
        echo "  ℹ️  'mod rpc;' already exists in main.rs"
    else
        # Add 'mod rpc;' after 'mod cli;'
        sed -i.bak '/^mod cli;/a\
mod rpc;
' "$MAIN_FILE"
        echo "  ✅ Added 'mod rpc;' to main.rs"
    fi

    # Step 3: Update service.rs
    echo "  [3/3] Updating src/service.rs..."
    SERVICE_FILE="$COLLATOR_DIR/src/service.rs"

    # Check if RPC initialization already exists
    if grep -q "RPC SERVER INITIALIZATION" "$SERVICE_FILE" 2>/dev/null; then
        echo "  ℹ️  RPC initialization already exists in service.rs"
    else
        # Find the line number of "Ok(task_manager)" (last occurrence)
        LINE_NUM=$(grep -n "Ok(task_manager)" "$SERVICE_FILE" | tail -1 | cut -d: -f1)

        if [ -z "$LINE_NUM" ]; then
            echo "  ❌ Could not find 'Ok(task_manager)' in service.rs"
            FAILED_COLLATORS+=("$pbc (service.rs pattern not found)")
            continue
        fi

        # Create the RPC initialization block
        RPC_BLOCK="
    // ═══════════════════════════════════════════════════════════════════════════
    // RPC SERVER INITIALIZATION - CRITICAL FIX
    // ═══════════════════════════════════════════════════════════════════════════
    log::info!(\"🔧 Initializing RPC server for ${PBC_UPPER}-PBC Collator...\");

    // Build RPC extensions
    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();

        Box::new(move |_| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: pool.clone(),
            };

            crate::rpc::create_full(deps).map_err(Into::into)
        })
    };

    // Spawn RPC server tasks - THIS STARTS THE JSON-RPC SERVER
    let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        network: network.clone(),
        client: client.clone(),
        keystore: keystore_container.keystore(),
        task_manager: &mut task_manager,
        transaction_pool: transaction_pool.clone(),
        rpc_builder: rpc_extensions_builder,
        backend: backend.clone(),
        system_rpc_tx,
        tx_handler_controller,
        sync_service: sync_service.clone(),
        config,
        telemetry: telemetry.as_mut(),
    })?;

    log::info!(\"✅ RPC server initialized successfully\");
"

        # Insert before "Ok(task_manager)"
        INSERT_LINE=$((LINE_NUM - 1))

        # Create temp file with inserted content
        head -n "$INSERT_LINE" "$SERVICE_FILE" > "$SERVICE_FILE.tmp"
        echo "$RPC_BLOCK" >> "$SERVICE_FILE.tmp"
        tail -n +"$INSERT_LINE" "$SERVICE_FILE" | tail -n +2 >> "$SERVICE_FILE.tmp"
        mv "$SERVICE_FILE.tmp" "$SERVICE_FILE"

        echo "  ✅ Added RPC initialization to service.rs"
    fi

    echo "✅ Successfully fixed: $pbc-pbc-collator"
    SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
    echo ""
done

echo "========================================"
echo "SUMMARY"
echo "========================================"
echo "Total collators: ${#COLLATORS[@]}"
echo "Successfully fixed: $SUCCESS_COUNT"
echo "Failed: ${#FAILED_COLLATORS[@]}"

if [ ${#FAILED_COLLATORS[@]} -gt 0 ]; then
    echo ""
    echo "Failed collators:"
    for failed in "${FAILED_COLLATORS[@]}"; do
        echo "  - $failed"
    done
fi

echo ""
echo "========================================"
