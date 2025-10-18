#!/usr/bin/env python3
"""
Automated deployment script for ASF consensus to PBC collators.
Replaces AURA consensus with ASF in service.rs, chain_spec.rs, and Cargo.toml.
"""

import re
import sys
from pathlib import Path

# List of PBC collators to update (excluding btc and eth which are already done)
COLLATORS = [
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

def update_service_rs(collator_name: str) -> bool:
    """Update service.rs to replace AURA with ASF"""
    service_path = BASE_PATH / collator_name / "src" / "service.rs"

    if not service_path.exists():
        print(f"  ❌ service.rs not found for {collator_name}")
        return False

    content = service_path.read_text()

    # Get runtime name (e.g., "doge_pbc_runtime" from "doge-pbc-collator")
    runtime_name = collator_name.replace("-collator", "_runtime").replace("-", "_")

    # 1. Update imports - build patterns without f-strings for complex braces
    import_old = """use sc_consensus_aura::{ImportQueueParams, SlotProportion, StartAuraParams};
use sc_service::{error::Error as ServiceError, Configuration, TaskManager, TFullBackend, TFullClient};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_consensus_aura::sr25519::AuthorityPair as AuraPair;
use sp_runtime::traits::Header as HeaderT;
use std::{sync::Arc, time::Duration};

use """ + runtime_name + """::{self, opaque::Block, RuntimeApi};"""

    import_new = """use sc_consensus_asf::{import_queue as asf_import_queue, run_asf_worker, AsfWorkerParams};
use sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager, TFullBackend, TFullClient};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_runtime::traits::Header as HeaderT;
use std::{marker::PhantomData, sync::Arc, time::Duration};

use """ + runtime_name + """::{self, opaque::Block, RuntimeApi, AccountId};"""

    content = content.replace(import_old, import_new)

    # 2. Replace import queue (more flexible pattern matching)
    # Find and replace the entire AURA import queue block
    import_queue_pattern = r'let cidp_client = client\.clone\(\);\s+let import_queue = sc_consensus_aura::import_queue::<AuraPair.*?\)\?;'
    import_queue_new = """let import_queue = asf_import_queue::<_, _, _, AccountId>(
        client.clone(),
        client.clone(),
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
    )
    .map_err(|e| ServiceError::Other(format!("ASF import queue error: {}", e)))?;"""

    content = re.sub(import_queue_pattern, import_queue_new, content, flags=re.DOTALL)

    # 3. Replace AURA consensus with ASF
    aura_pattern = r'let slot_duration = sc_consensus_aura::slot_duration.*?task_manager\.spawn_essential_handle\(\)\.spawn_blocking\(\s+"aura".*?\);'
    asf_replacement = """// ASF consensus worker parameters
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
    );"""

    content = re.sub(aura_pattern, asf_replacement, content, flags=re.DOTALL)

    service_path.write_text(content)
    return True

def update_chain_spec_rs(collator_name: str) -> bool:
    """Update chain_spec.rs to remove AURA references"""
    chain_spec_path = BASE_PATH / collator_name / "src" / "chain_spec.rs"

    if not chain_spec_path.exists():
        print(f"  ❌ chain_spec.rs not found for {collator_name}")
        return False

    content = chain_spec_path.read_text()

    # Remove AuraId import
    content = content.replace(
        "use sp_consensus_aura::sr25519::AuthorityId as AuraId;\n", ""
    )
    content = content.replace(
        "use sp_consensus_grandpa::AuthorityId as GrandpaId;\n",
        ""
    )

    # Remove authority_keys_from_seed function
    authority_pattern = r'/// Generate authority keys\npub fn authority_keys_from_seed\(s: &str\) -> \(AuraId, GrandpaId\) \{\s+\(get_from_seed::<AuraId>\(s\), get_from_seed::<GrandpaId>\(s\)\)\s+\}\s+'
    content = re.sub(authority_pattern, '', content)

    chain_spec_path.write_text(content)
    return True

def update_cargo_toml(collator_name: str) -> bool:
    """Update Cargo.toml to replace AURA with ASF dependencies"""
    cargo_path = BASE_PATH / collator_name / "Cargo.toml"

    if not cargo_path.exists():
        print(f"  ❌ Cargo.toml not found for {collator_name}")
        return False

    content = cargo_path.read_text()

    # Replace sc-consensus-aura with sc-consensus-asf and sc-consensus-slots
    content = content.replace(
        'sc-consensus-aura = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506" }',
        '''sc-consensus-asf = { path = "../../../../../09-consensus/client/consensus-asf" }
sc-consensus-slots = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506" }'''
    )

    # Replace sp-consensus-aura with sp-consensus-asf
    content = content.replace(
        'sp-consensus-aura = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506" }',
        'sp-consensus-asf = { path = "../../../../../09-consensus/primitives/consensus-asf" }'
    )

    cargo_path.write_text(content)
    return True

def main():
    print("🚀 ASF Collator Deployment Script")
    print(f"Deploying to {len(COLLATORS)} collators...\n")

    success_count = 0
    failed = []

    for collator in COLLATORS:
        print(f"📦 Processing {collator}...")

        try:
            service_ok = update_service_rs(collator)
            chain_spec_ok = update_chain_spec_rs(collator)
            cargo_ok = update_cargo_toml(collator)

            if service_ok and chain_spec_ok and cargo_ok:
                print(f"  ✅ {collator} updated successfully")
                success_count += 1
            else:
                print(f"  ⚠️  {collator} partially updated")
                failed.append(collator)
        except Exception as e:
            print(f"  ❌ {collator} failed: {e}")
            failed.append(collator)

    print(f"\n{'='*60}")
    print(f"✅ Successfully updated: {success_count}/{len(COLLATORS)}")

    if failed:
        print(f"❌ Failed: {', '.join(failed)}")
    else:
        print("🎉 All collators updated successfully!")

    print(f"{'='*60}")

if __name__ == "__main__":
    main()
