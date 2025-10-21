# Ëtrid Virtual Machine Watts (VMw) Test Cases

## Purpose:
These are test vectors to verify the accuracy of VMw accounting logic inside the ËtwasmVM. Each test case defines the transaction context and expected outcomes.

---

## Test Case 1: Basic Contract Call
- `vmw_limit`: 10,000
- `op_price`: 2
- Operations: [contract_call (500), storage_read (100), storage_write (300)]

Expected:
- `vmw_used`: 900
- `cost`: 1800 ÉTR units

---

## Test Case 2: Over-Limit Rejection
- `vmw_limit`: 600
- `op_price`: 3
- Operations: [contract_call (500), storage_write (300)]

Expected:
- Error: "VMw limit exceeded" after second op

---

## Test Case 3: Full Refund Path (EDSC payer)
- `vmw_limit`: 1000
- `op_price`: 1
- Operations: [storage_read (100), storage_write (300)]
- VMw used: 400

Expected:
- Cost: 400 EDSC worth of ÉTR
- Refund of 600 VMw (converted to ÉTR from reward pool)
