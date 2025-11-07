#!/usr/bin/env python3
"""
Validator Key Deployment Script for Azure VMs
Distributes 21 validators across 17 Azure VMs
"""

import json
import subprocess
import sys
from typing import List, Dict, Tuple

# Azure VM Configuration
VMS = [
    ("98.71.91.84", 1),
    ("68.219.230.63", 2),
    ("4.180.59.25", 3),
    ("20.224.104.239", 4),
    ("98.71.219.106", 5),
    ("108.142.205.177", 6),
    ("4.180.238.67", 7),
    ("51.142.203.160", 8),
    ("172.166.164.19", 9),
    ("172.166.187.180", 10),
    ("172.166.210.244", 11),
    ("172.167.8.217", 12),
    ("4.251.115.186", 13),
    ("52.143.191.232", 14),
    ("4.211.206.210", 15),
    ("4.178.181.122", 16),
    ("4.233.88.42", 17)
]

SSH_KEY = "~/.ssh/etrid_vm1"
SSH_USER = "audit-dev01"
BASE_PATH = "~/.local/share/flarechain"
CHAIN_SPEC = "mainnet-raw.json"
NODE_BINARY = "~/etrid/target/release/flarechain-node"

def load_keys(json_path: str) -> List[Dict]:
    """Load validator keys from JSON file"""
    print(f"Loading keys from: {json_path}")
    with open(json_path, 'r') as f:
        data = json.load(f)
    validators = data.get('validators', [])
    print(f"Loaded {len(validators)} validators")
    return validators

def distribute_validators(validators: List[Dict], vms: List[Tuple[str, int]]) -> Dict[int, List[Dict]]:
    """
    Distribute validators across VMs:
    - VMs 1-11: One validator each (validators 1-11)
    - VMs 12-17: Distribute remaining 10 validators (validators 12-21) - ~2 per VM
    """
    distribution = {}

    # VMs 1-11 get validators 1-11 (one each)
    for i in range(11):
        vm_num = i + 1
        validator = validators[i]
        distribution[vm_num] = [validator]
        print(f"VM {vm_num}: Validator {validator['validatorIndex']} ({validator['name']})")

    # VMs 12-17 get validators 12-21 (remaining 10 validators)
    remaining_validators = validators[11:21]  # validators 12-21
    remaining_vms = list(range(12, 18))  # VMs 12-17

    # Distribute approximately 2 validators per VM
    validators_per_vm = len(remaining_validators) // len(remaining_vms)
    extra_validators = len(remaining_validators) % len(remaining_vms)

    validator_idx = 0
    for i, vm_num in enumerate(remaining_vms):
        count = validators_per_vm + (1 if i < extra_validators else 0)
        distribution[vm_num] = remaining_validators[validator_idx:validator_idx + count]
        for v in distribution[vm_num]:
            print(f"VM {vm_num}: Validator {v['validatorIndex']} ({v['name']})")
        validator_idx += count

    return distribution

def create_key_insert_command(validator: Dict, key_type: str) -> str:
    """Create the key insert command for a specific key type"""
    session_keys = validator['sessionKeys']

    if key_type == 'aura':
        seed = session_keys['auraKey']
        scheme = "Sr25519"
        key_type_flag = "aura"
    elif key_type == 'grandpa':
        seed = session_keys['grandpaKey']
        scheme = "Ed25519"
        key_type_flag = "gran"
    elif key_type == 'asf':
        seed = session_keys['asfKey']
        scheme = "Sr25519"
        key_type_flag = "asf"
    else:
        raise ValueError(f"Unknown key type: {key_type}")

    # Insert keys without chainspec - they go directly to keystore
    cmd = f"""{NODE_BINARY} key insert \\
  --base-path {BASE_PATH} \\
  --scheme {scheme} \\
  --suri "{seed}" \\
  --key-type {key_type_flag}"""

    return cmd

def create_deployment_script(vm_ip: str, validators: List[Dict]) -> str:
    """Create a deployment script for a single VM"""
    script_lines = [
        "#!/bin/bash",
        f"# Deployment script for VM {vm_ip}",
        f"# Deploying {len(validators)} validator(s)",
        "",
        "set -e",  # Exit on error
        "",
        "echo '========================================='",
        f"echo 'Deploying keys to {vm_ip}'",
        "echo '========================================='",
        ""
    ]

    for validator in validators:
        validator_idx = validator['validatorIndex']
        validator_name = validator['name']

        script_lines.append(f"echo 'Installing keys for Validator {validator_idx} ({validator_name})...'")
        script_lines.append("")

        # Insert AURA key
        script_lines.append(f"echo '  - Inserting AURA key...'")
        script_lines.append(create_key_insert_command(validator, 'aura'))
        script_lines.append("")

        # Insert GRANDPA key
        script_lines.append(f"echo '  - Inserting GRANDPA key...'")
        script_lines.append(create_key_insert_command(validator, 'grandpa'))
        script_lines.append("")

        # Note: ASF keys are handled by the AURA consensus mechanism in this runtime
        # The asfKey in the JSON is the same as the auraKey for this implementation

        script_lines.append(f"echo 'Completed Validator {validator_idx}'")
        script_lines.append("echo '-----------------------------------------'")
        script_lines.append("")

    script_lines.append("echo 'All keys deployed successfully!'")
    script_lines.append("")

    return "\n".join(script_lines)

def deploy_to_vm(vm_ip: str, vm_num: int, validators: List[Dict]) -> Tuple[bool, str]:
    """Deploy keys to a single VM via SSH"""
    print(f"\n{'='*60}")
    print(f"Deploying to VM {vm_num} ({vm_ip})")
    print(f"Validators: {', '.join([str(v['validatorIndex']) for v in validators])}")
    print(f"{'='*60}")

    # Create deployment script
    script_content = create_deployment_script(vm_ip, validators)

    # Save script locally for reference
    script_filename = f"/tmp/deploy_vm{vm_num}_{vm_ip.replace('.', '_')}.sh"
    with open(script_filename, 'w') as f:
        f.write(script_content)
    print(f"Script saved to: {script_filename}")

    # Execute via SSH
    ssh_cmd = [
        "ssh",
        "-i", SSH_KEY.replace("~", f"/Users/{subprocess.getoutput('whoami')}"),
        "-o", "StrictHostKeyChecking=no",
        "-o", "UserKnownHostsFile=/dev/null",
        f"{SSH_USER}@{vm_ip}",
        "bash -s"
    ]

    try:
        print(f"Executing deployment via SSH...")
        result = subprocess.run(
            ssh_cmd,
            input=script_content.encode(),
            capture_output=True,
            timeout=300  # 5 minute timeout
        )

        output = result.stdout.decode() + result.stderr.decode()

        if result.returncode == 0:
            print(f"✓ SUCCESS: VM {vm_num} deployment completed")
            return True, output
        else:
            print(f"✗ FAILED: VM {vm_num} deployment failed")
            print(f"Error output:\n{output}")
            return False, output

    except subprocess.TimeoutExpired:
        error_msg = f"Timeout: Deployment to VM {vm_num} took too long"
        print(f"✗ {error_msg}")
        return False, error_msg
    except Exception as e:
        error_msg = f"Error deploying to VM {vm_num}: {str(e)}"
        print(f"✗ {error_msg}")
        return False, error_msg

def main():
    """Main deployment orchestrator"""
    print("="*60)
    print("Etrid Validator Key Deployment to Azure VMs")
    print("="*60)

    # Load keys
    keys_path = "/Users/macbook/Desktop/etrid/secrets/validator-keys/generated-keys/generated-keys-gizzi-eoj/MASTER_COMPLETE_ALL_KEYS.json"
    validators = load_keys(keys_path)

    if len(validators) < 21:
        print(f"ERROR: Expected 21 validators, found {len(validators)}")
        sys.exit(1)

    # Distribute validators across VMs
    print("\n" + "="*60)
    print("Validator Distribution Plan")
    print("="*60)
    distribution = distribute_validators(validators, VMS)

    # Deployment summary
    print("\n" + "="*60)
    print("Starting Deployment")
    print("="*60)

    results = {}
    successful = 0
    failed = 0

    # Deploy to each VM
    for vm_ip, vm_num in VMS:
        if vm_num not in distribution:
            print(f"Warning: No validators assigned to VM {vm_num}")
            continue

        validators_for_vm = distribution[vm_num]
        success, output = deploy_to_vm(vm_ip, vm_num, validators_for_vm)

        results[vm_num] = {
            'ip': vm_ip,
            'validators': [v['validatorIndex'] for v in validators_for_vm],
            'success': success,
            'output': output
        }

        if success:
            successful += 1
        else:
            failed += 1

    # Final summary
    print("\n" + "="*60)
    print("DEPLOYMENT SUMMARY")
    print("="*60)
    print(f"Total VMs: {len(VMS)}")
    print(f"Successful: {successful}")
    print(f"Failed: {failed}")
    print("")

    print("Detailed Results:")
    print("-"*60)
    for vm_num in sorted(results.keys()):
        result = results[vm_num]
        status = "✓ SUCCESS" if result['success'] else "✗ FAILED"
        validators_str = ", ".join([f"V{v}" for v in result['validators']])
        print(f"VM {vm_num:2d} ({result['ip']:15s}): {status} | Validators: {validators_str}")

    print("="*60)

    # Save detailed results
    results_file = "/Users/macbook/Desktop/etrid/deployment_results.json"
    with open(results_file, 'w') as f:
        json.dump(results, f, indent=2)
    print(f"\nDetailed results saved to: {results_file}")

    # Exit with appropriate code
    sys.exit(0 if failed == 0 else 1)

if __name__ == "__main__":
    main()
