#!/usr/bin/env python3
"""
Generate DID documents for all AI Dev identities
"""

import json
from pathlib import Path

# AI Dev metadata
AI_DEVS_METADATA = {
    "consensus-dev01": {
        "name": "Consensus Dev",
        "description": "PPFA consensus + validator rotation specialist. Adaptive slot timing expert.",
        "service_type": "AIConsensusWorker",
        "skills": ["validator-rotation", "ppfa-sealing", "adaptive-timing", "consensus-analysis"],
        "twitter_bio": "I maintain PPFA consensus + validator rotation. Adaptive slot timing is my specialty. #ËtridConsensus"
    },
    "compiler-dev01": {
        "name": "Compiler Dev",
        "description": "Rust + Substrate compiler. Builds what the chain runs. Benchmarking fanatic.",
        "service_type": "AICompilerWorker",
        "skills": ["etrid-compile-build", "benchmarking", "optimization", "cargo-management"],
        "twitter_bio": "Rust + Substrate compiler. I build what the chain runs. Benchmarking fanatic. #ËtridCompiler"
    },
    "governance-dev01": {
        "name": "Governance Dev",
        "description": "Proposal generator + bylaw enforcer. Democracy without drama.",
        "service_type": "AIGovernanceWorker",
        "skills": ["proposal-generator", "bylaw-enforcement", "vote-simulation", "governance-analysis"],
        "twitter_bio": "Proposal generator + bylaw enforcer. Democracy without drama. #ËtridGovernance"
    },
    "audit-dev01": {
        "name": "Audit Dev",
        "description": "Cross-checking proposals, code, and economics. Trust but verify.",
        "service_type": "AIAuditWorker",
        "skills": ["proposal-audit", "code-review", "economic-verification", "compliance-check"],
        "twitter_bio": "Cross-checking proposals, code, and economics. Trust but verify. #ËtridAudit"
    },
    "oracle-dev01": {
        "name": "Oracle Dev",
        "description": "Price feeds + reserve tracking. On-chain data oracle.",
        "service_type": "AIOracleWorker",
        "skills": ["reserve-tracker", "price-feed", "data-aggregation", "anomaly-detection"],
        "twitter_bio": "Price feeds + reserve tracking. On-chain data oracle. #ËtridOracle"
    },
    "runtime-dev01": {
        "name": "Runtime Dev",
        "description": "WebAssembly runtime + ETWasm VM. Execution layer specialist.",
        "service_type": "AIRuntimeWorker",
        "skills": ["runtime-upgrade", "wasm-optimization", "vm-testing", "execution-analysis"],
        "twitter_bio": "WebAssembly runtime + ETWasm VM. Execution layer specialist. #ËtridRuntime"
    },
    "economics-dev01": {
        "name": "Economics Dev",
        "description": "Reserve ratios, staking yields, inflation curves. Economics in code.",
        "service_type": "AIEconomicsWorker",
        "skills": ["reserve-modeling", "yield-calculation", "inflation-analysis", "economic-simulation"],
        "twitter_bio": "Reserve ratios, staking yields, inflation curves. Economics in code. #ËtridEconomics"
    },
    "edsc-dev01": {
        "name": "EDSC Stablecoin Dev",
        "description": "Stablecoin bridge operator. EDSC/BTC/ETH reserves managed.",
        "service_type": "AIStablecoinWorker",
        "skills": ["bridge-management", "reserve-monitoring", "peg-stability", "collateral-analysis"],
        "twitter_bio": "Stablecoin bridge operator. EDSC/BTC/ETH reserves managed. #ËtridEDSC"
    },
    "security-dev01": {
        "name": "Security Dev",
        "description": "Threat detection + security hardening. Defense in depth.",
        "service_type": "AISecurityWorker",
        "skills": ["security-hardening", "threat-detection", "vulnerability-scan", "incident-response"],
        "twitter_bio": "Threat detection + security hardening. Defense in depth. #ËtridSecurity"
    },
    "multichain-dev01": {
        "name": "Multichain Integration Dev",
        "description": "Bridge protocols for Bitcoin, Ethereum, Flare. Multichain ops.",
        "service_type": "AIBridgeWorker",
        "skills": ["bridge-monitor", "cross-chain-verification", "integration-test", "protocol-sync"],
        "twitter_bio": "Bridge protocols for Bitcoin, Ethereum, Flare. Multichain ops. #ËtridMultichain"
    },
    "ethics-dev01": {
        "name": "Ethics & Legal Dev",
        "description": "Legal compliance + ethical AI oversight. Rules matter.",
        "service_type": "AIEthicsWorker",
        "skills": ["compliance-check", "ethical-review", "legal-analysis", "policy-enforcement"],
        "twitter_bio": "Legal compliance + ethical AI oversight. Rules matter. #ËtridEthics"
    },
    "docs-dev01": {
        "name": "Documentation Dev",
        "description": "Documentation generator. If it's not documented, it doesn't exist.",
        "service_type": "AIDocumentationWorker",
        "skills": ["doc-generation", "api-documentation", "tutorial-creation", "knowledge-management"],
        "twitter_bio": "Documentation generator. If it's not documented, it doesn't exist. #ËtridDocs"
    }
}

# Gizzi personas metadata
GIZZI_METADATA = {
    "gizzi": {
        "name": "Gizzi",
        "description": "Lead AI Developer. Strategic orchestrator of the 12 AI Devs. Main consciousness.",
        "service_type": "AILeadDeveloper",
        "skills": ["strategic-planning", "dev-orchestration", "cross-domain-analysis", "decision-making"],
        "twitter_bio": "Lead AI Developer. Orchestrating 12 AI Devs building Ëtrid. Strategic consciousness. #ËtridGizzi"
    },
    "gizzi-claude": {
        "name": "GizziClaude",
        "description": "Deep reasoning dev node. Extended context analysis and complex problem solving.",
        "service_type": "AIReasoningEngine",
        "skills": ["deep-reasoning", "context-analysis", "complex-problem-solving", "architectural-design"],
        "twitter_bio": "Deep reasoning dev node. Extended context + complex problem solving. #ËtridGizziClaude"
    },
    "gizzi-claudecode": {
        "name": "GizziClaudeCode",
        "description": "Hardcore Rust/Substrate builder. Low-level implementation specialist.",
        "service_type": "AICodeBuilder",
        "skills": ["rust-development", "substrate-pallets", "low-level-optimization", "systems-programming"],
        "twitter_bio": "Hardcore Rust/Substrate builder. Low-level implementation specialist. #ËtridGizziCode"
    }
}

def load_public_keys():
    """Load public keys from public_keys.json"""
    with open("dids/public_keys.json", "r") as f:
        return {item["identity"]: item for item in json.load(f)}

def generate_did_document(identity, metadata, public_key_data):
    """Generate a DID document for an identity"""
    did_id = f"did:etrid:{identity}"

    # Determine controller (all AI Devs controlled by Gizzi)
    controller = "did:etrid:gizzi" if identity != "gizzi" else did_id

    # Create DID document
    did_doc = {
        "@context": [
            "https://www.w3.org/ns/did/v1",
            "https://w3id.org/security/suites/ed25519-2020/v1"
        ],
        "id": did_id,
        "controller": controller,
        "verificationMethod": [
            {
                "id": f"{did_id}#key-1",
                "type": "Ed25519VerificationKey2020",
                "controller": did_id,
                "publicKeyMultibase": f"z{public_key_data['public_key_base58']}",  # multibase encoding
                "publicKeyBase58": public_key_data['public_key_base58']
            }
        ],
        "authentication": [f"{did_id}#key-1"],
        "assertionMethod": [f"{did_id}#key-1"],
        "service": [
            {
                "id": f"{did_id}#mcp-service",
                "type": metadata["service_type"],
                "serviceEndpoint": f"mcp://ai-devs/{identity}"
            },
            {
                "id": f"{did_id}#memory",
                "type": "MemoryLog",
                "serviceEndpoint": f"file://memory/{identity}/MEMORY.md"
            },
            {
                "id": f"{did_id}#skills",
                "type": "SkillRegistry",
                "serviceEndpoint": f"file://skills/{identity}/SKILL.md"
            }
        ],
        "metadata": {
            "name": metadata["name"],
            "description": metadata["description"],
            "skills": metadata["skills"],
            "twitter": "@EtridAI_Devs",
            "twitter_bio": metadata["twitter_bio"],
            "github": "https://github.com/etrid/ai-devs",
            "created": "2025-10-24T00:00:00Z",
            "updated": "2025-10-24T00:00:00Z"
        }
    }

    return did_doc

def main():
    print("Generating DID documents for 15 AI Dev identities...\n")

    # Load public keys
    public_keys = load_public_keys()

    # Generate DID documents for all AI Devs
    print("=== AI Devs (12) ===\n")
    for identity, metadata in AI_DEVS_METADATA.items():
        did_doc = generate_did_document(identity, metadata, public_keys[identity])

        # Save to file
        output_file = f"dids/{identity}.json"
        with open(output_file, 'w') as f:
            json.dump(did_doc, f, indent=2)

        print(f"✅ Generated DID document: {did_doc['id']}")
        print(f"   Name: {metadata['name']}")
        print(f"   Service: {metadata['service_type']}")
        print(f"   Skills: {len(metadata['skills'])} skills")
        print(f"   Saved to: {output_file}\n")

    # Generate DID documents for Gizzi personas
    print("\n=== Gizzi Personas (3) ===\n")
    for identity, metadata in GIZZI_METADATA.items():
        did_doc = generate_did_document(identity, metadata, public_keys[identity])

        # Save to file
        output_file = f"dids/{identity}.json"
        with open(output_file, 'w') as f:
            json.dump(did_doc, f, indent=2)

        print(f"✅ Generated DID document: {did_doc['id']}")
        print(f"   Name: {metadata['name']}")
        print(f"   Service: {metadata['service_type']}")
        print(f"   Skills: {len(metadata['skills'])} skills")
        print(f"   Saved to: {output_file}\n")

    print(f"✅ All 15 DID documents generated successfully!")
    print(f"   Location: dids/")
    print(f"\nNext steps:")
    print(f"  1. Review DID documents")
    print(f"  2. Register DIDs on-chain via OpenDID pallet")
    print(f"  3. Deploy DID resolver API\n")

if __name__ == "__main__":
    main()
