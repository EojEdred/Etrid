# FlareChain ASF Genesis Update & Deployment Log

## Context
- Validators had ASF `sr25519` keys inserted but remained idle at block #0.
- Root cause: genesis chainspec lacked `initialAuthorities`, so PPFA had no authorized proposers.
- Goal: collect all validator ASF keys (20 Contabo + Gizzi + Audit Dev) and generate a new genesis/chain spec that includes them.

## Key Outputs & Files
- ASF key collection log: `/tmp/asf-public-keys-collection.log`
- Authority list (hex): `/tmp/asf-authorities.json`
- Extended preset with 22 validators: `05-multichain/flare-chain/runtime/presets/flarechain_mainnet_v1_pure_asf.json`
- Human-readable chainspec: `/tmp/flarechain-mainnet-v108-readable.json`
  - `consensus.validators` length = 22
  - `validatorCommittee.validators` length = 22
- Raw chainspec (to generate): `/tmp/flarechain-mainnet-v108-raw.json`

## Validator Keys (22 Total)
```
0x36edba289334c21d8c78b81d56dba974dd49ebe709c81dfc5b05469fcf6d772b
0x941afdfa66338714710e8f1b2b829162a1ac132efe56d665aad81e9b5f12c617
0x6a15f2f70d19046b1c13eb61fa64f473c38294d43aa6a399962a94c67bcb9272
0xa2fb2b030b1b4cf9984e7755b4d828f41158d120087dbf7f28c5722ba2358600
0xa42a1bfc7e9baf5bed45e5363b3d0a1e4ed26e9034d69b701c2be571db709a68
0x64c7c52bd7dbf2b41e1ebe2f608a6e8d2db60c813a5f1aa5b77a89786167e658
0x0c09522384290ee11ab1cbc520100902859eafed683f7b8efb1959d69104d348
0x48e99e1406036a1a1f914bae7512c89c7d0500572f7b01568154a2ac6a8baa59
0xf4020d26fb3062efa9964c48da1b6ccbb2a89a5bfea9631800fe389d32b4d014
0xd2867b993988c533bb0c920ee314c7ea715e987861d3644cb1c9720f2882903a
0xe86f6f6a2b11e51daebe856f66e324e6f155178686a9d865364c26acb8fb9b3e
0x084b9c663309503874fcd7370365bcc1e5154a186beb43ad7e4da761200c6209
0x7e46e1e6a70cad911da865921e9f1b0d1c24cd4135661f0176ab916cd6a6372a
0x8a2e45c85d5ca6488bf983d5f7ab1babc33f4748335cf95b8ddbb05e28f68664
0x18b82e4b3f6c0971ebaa6b8ba5c1397c551ebaa872749411fa49718b51188241
0x32f084b23c41246103cd9164442e976701c9b41bc34bd6f2b9aab64ad438346a
0xd84b7d5e75ca46712492d1293c15ddeef6a3ca92f3f6bcd8e6314cecea2aaa43
0xc4e2e74bf658aea912538932982f66ef5aa0104a3a099ca5097249a943464a71
0x8cf7dc3cdc82ec92a51dc15e6177312bc5ea832b869a6e481a66717d5d81c21f
0xee9eb32281843f77137276416619e25449d921f66415a1b45d74eee789780d0b
0x8230ef2ffd67b375f0cf3658c44e67650b2cde9c51a7525810586637edd8201d
0xc4f9e488c7f3cc487bf0e8619dc94d05ab0f12488745659a560af9d03bf1dd64
```
- First two entries are Gizzi (director) and Audit Dev (director).
- Remaining entries are the 20 Contabo validators (9 directors + 11 validity nodes).

## Procedure Executed
1. Collected ASF keys from all Contabo validators with `/tmp/COLLECT-ASF-PUBLIC-KEYS.sh`.
2. Added Gizzi/Audit Dev keys:
   - Audit Dev seed retrieved (`runway inform weird ... glow`) → derived hex key.
   - Generated new sr25519 key pair for Gizzi (stored in `/tmp/gizzi-asf-key.json`).
3. Updated preset JSON to include 22 validators under `consensus.validators` and `validatorCommittee.validators` (committee size now 22).
4. Generated chainspec:
   ```bash
   ./target/release/flarechain-node build-spec \
     --chain 05-multichain/flare-chain/runtime/presets/flarechain_mainnet_v1_pure_asf.json \
     --disable-default-bootnode \
     > /tmp/flarechain-mainnet-v108-readable.json
   ```
   (Requires full chainspec structure; used existing chainspec as base and replaced `genesis.runtimeGenesis.patch` with updated preset content.)
5. Verified validator counts via `jq` (both lists length 22).

## Next Required Steps
### Raw Chainspec Status (Resolved)
- Instead of patching the old raw file, generated a fresh raw chainspec directly from `flarechain_mainnet.json` while only replacing the validator sets.
- Steps:
  ```bash
  # Convert desired validator lists back to SS58 (script in log)
  jq --slurpfile cons /tmp/consensus_validators_ss58.json \
     --slurpfile comm /tmp/committee_validators_ss58.json \
     '.name = "Ëtrid FlareChain Mainnet v108 (ASF 22)" |
      .id = "flarechain_mainnet_v108_asf" |
      .genesis.runtimeGenesis.patch.consensus.validators = $cons[0] |
      .genesis.runtimeGenesis.patch.validatorCommittee.validators = $comm[0] |
      .genesis.runtimeGenesis.patch.validatorCommittee.committeeSize = ($comm[0] | length)' \
      flarechain_mainnet.json \
      > /tmp/flarechain-mainnet-v108-readable.json

  ./target/release/flarechain-node build-spec \
    --chain /tmp/flarechain-mainnet-v108-readable.json \
    --raw --disable-default-bootnode \
    > /tmp/flarechain-mainnet-v108-raw.json

  cp /tmp/flarechain-mainnet-v108-raw.json flarechain_mainnet_v108_raw.json
  ```
- Result: `/Users/macbook/Desktop/etrid/flarechain_mainnet_v108_raw.json` (size ~2.1 MB) is **valid** and ready for deployment.

### Deployment Steps
1. Copy `flarechain_mainnet_v108_raw.json` to each validator as `/root/flarechain-mainnet-v108.json`.
2. Stop `flarechain-validator`, clear DBs (`rm -rf /root/.local/share/flarechain-node/chains/*/db`), restart with new `--chain`.
3. Verify block production via `journalctl -u flarechain-validator -n 50 --no-pager` and ensure blocks > #0.

## Reference Commands
```bash
# Check validator keys on a host
flarechain-node key inspect \
  --key-type asfk \
  --scheme sr25519 \
  --keystore /root/.local/share/flarechain-node/chains/*/keystore

# View ASF committee info once running
substrate rpc call \
  state_call \
  validatorCommittee_validatorCommittee
```

## Validator Inventory
- Contabo (20 nodes): 157.173.200.{86,84,81,80}, 154.12.250.{18,17,15}, 154.12.249.{223,182}, 85.239.239.{194,193,190,189,188}, 80.190.82.{186,185,184,183}, 158.220.83.{146,66}
- Gizzi (Director): 64.181.215.19 (ubuntu)
- Audit Dev (Director): 129.80.122.34 (ubuntu)

## Outstanding Questions / Notes
- Gizzi currently shows `ERROR: invalid number at line 1 column 2` due to corrupted chainspec; deploying new raw spec should fix it.
- Ensure `/tmp/gizzi-asf-key.json` private seed is secured or re-generated if compromised.
- After network restart, run monitoring to confirm PPFA proposer rotation and ASF finality certificates propagate.

