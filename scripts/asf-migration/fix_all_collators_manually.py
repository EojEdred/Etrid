#!/usr/bin/env python3
"""
Comprehensive fix for all PBC collators.
Uses the working btc-pbc-collator as a template.
"""

import re
from pathlib import Path

COLLATORS_TO_FIX = [
    "doge-pbc-collator",
    "xlm-pbc-collator",
    "bnb-pbc-collator",
    "trx-pbc-collator",
    "ada-pbc-collator",
    "link-pbc-collator",
    "matic-pbc-collator",
    "sc-usdt-pbc-collator",
    "sol-pbc-collator",
    "xrp-pbc-collator",
]

BASE_PATH = Path("05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes")

def get_runtime_name(collator_name):
    """Get runtime name from collator name"""
    return collator_name.replace("-collator", "_runtime").replace("-", "_")

def get_pbc_name(collator_name):
    """Get PBC display name (e.g., DOGE-PBC from doge-pbc-collator)"""
    return collator_name.replace("-collator", "").upper()

def fix_service_imports(content, runtime_name):
    """Fix imports section"""
    # Find the imports section (from //! comment to first pub type)
    import_pattern = r'(//!.*?\n\n)(use.*?)(pub type FullClient)'

    new_imports = f'''use futures::FutureExt;
use sc_client_api::{{Backend, HeaderBackend}};
use sc_consensus_asf::{{import_queue as asf_import_queue, run_asf_worker, AsfWorkerParams}};
use sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging;
use sc_service::{{error::Error as ServiceError, Configuration, TaskManager, TFullBackend, TFullClient}};
use sc_telemetry::{{Telemetry, TelemetryWorker}};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_runtime::traits::Header as HeaderT;
use std::{{marker::PhantomData, sync::Arc, time::Duration}};

use {runtime_name}::{{self, opaque::Block, RuntimeApi, AccountId}};

'''

    match = re.search(import_pattern, content, re.DOTALL)
    if match:
        content = content[:match.start(2)] + new_imports + content[match.start(3):]

    return content

def fix_import_queue(content):
    """Replace AURA import queue with ASF"""
    # Find everything from "let cidp_client" or "let import_queue = sc_consensus_aura"
    # to the end of the import_queue construction

    # Pattern: match from start of import queue to the ?; that ends it
    pattern = r'(let cidp_client.*?)?let import_queue = sc_consensus_aura::import_queue.*?\)\?;'

    replacement = '''let import_queue = asf_import_queue::<_, _, _, AccountId>(
        client.clone(),
        client.clone(),
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
    )
    .map_err(|e| ServiceError::Other(format!("ASF import queue error: {}",e)))?;'''

    content = re.sub(pattern, replacement, content, flags=re.DOTALL)

    return content

def fix_block_authoring(content):
    """Replace AURA block authoring with ASF"""
    # Pattern: from "let slot_duration" to the spawn_blocking for "aura"
    pattern = r'let slot_duration = sc_consensus_aura.*?task_manager\.spawn_essential_handle\(\)\.spawn_blocking\(\s*"aura".*?\);'

    replacement = '''// ASF consensus worker parameters
    let backoff_authoring_blocks = Some(BackoffAuthoringOnFinalizedHeadLagging::default());

    let asf_params = AsfWorkerParams {
        client: client.clone(),
        block_import: client.clone(),
        env: proposer_factory,
        sync_oracle: sync_service.clone(),
        backoff_authoring_blocks,
        keystore: keystore_container.keystore(),
        create_inherent_data_providers: move |_, ()| async move {
            let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
            Ok((timestamp,))
        },
        force_authoring: config.force_authoring,
        block_proposal_slot_portion: 2f32 / 3f32,
        max_block_proposal_slot_portion: None,
        justification_sync_link: sync_service.clone(),
        _phantom: PhantomData,
    };

    // Start ASF block authoring worker
    let asf_worker = run_asf_worker(asf_params);
    task_manager.spawn_essential_handle().spawn_blocking(
        "asf-worker",
        Some("block-authoring"),
        asf_worker.map(|res| {
            if let Err(e) = res {
                log::error!("ASF worker error: {}", e);
            }
        }),
    );'''

    content = re.sub(pattern, replacement, content, flags=re.DOTALL)

    return content

def fix_collator(collator_name):
    """Fix a single collator"""
    service_path = BASE_PATH / collator_name / "src" / "service.rs"

    if not service_path.exists():
        print(f"  ❌ service.rs not found")
        return False

    content = service_path.read_text()
    runtime_name = get_runtime_name(collator_name)

    # Apply fixes
    content = fix_service_imports(content, runtime_name)
    content = fix_import_queue(content)
    content = fix_block_authoring(content)

    # Write back
    service_path.write_text(content)
    return True

def main():
    print("🔧 Comprehensive Collator Fix Script")
    print(f"Fixing {len(COLLATORS_TO_FIX)} collators...\n")

    success = 0
    failed = []

    for collator in COLLATORS_TO_FIX:
        print(f"📦 Fixing {collator}...")
        try:
            if fix_collator(collator):
                print(f"  ✅ Fixed {collator}")
                success += 1
            else:
                failed.append(collator)
        except Exception as e:
            print(f"  ❌ Error: {e}")
            failed.append(collator)

    print(f"\n{'='*60}")
    print(f"✅ Successfully fixed: {success}/{len(COLLATORS_TO_FIX)}")
    if failed:
        print(f"❌ Failed: {', '.join(failed)}")
    print(f"{'='*60}")

if __name__ == "__main__":
    main()
