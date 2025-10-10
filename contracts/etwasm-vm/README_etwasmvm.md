# ËtwasmVM – Gas Metering & VMw Logic

## Overview
ËtwasmVM is the WASM-based runtime engine powering smart contract execution on the Ëtrid Multichain.

## Virtual Machine Watts (VMw)
VMw is a non-tradable unit representing computational effort. It is:
- Converted from ÉTR or paid in EDSC
- Consumed per opcode based on a fee schedule
- Refunded (in ÉTR) if unused

## VMw Cost Equation
`Total Cost = VMwUsed * VMw_OP_Price`

## Example Operation Costs:
| Operation        | VMw Cost |
|------------------|----------|
| contract_init     | 2000     |
| contract_call     | 500      |
| storage_read      | 100      |
| storage_write     | 300      |
| state_verify      | 150      |
| address_check     | 50       |

## Refund Logic
Unused VMw is reverted to ÉTR. When EDSC is used, refund is sourced from the block’s reward pool and returned in ÉTR.

See `vmw_test_vectors.md` for test cases.
