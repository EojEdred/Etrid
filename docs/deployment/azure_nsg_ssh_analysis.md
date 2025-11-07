# Azure VM SSH Connectivity Analysis Report
**Date:** 2025-11-02
**Analyzed VMs:** 17 VMs across 4 Resource Groups

## Executive Summary

**SSH TIMEOUT ISSUE IS NOT CAUSED BY NSG RULES**

All 17 Azure VMs have proper Network Security Group (NSG) rules configured to allow SSH access on port 22. The effective security rules show that SSH is permitted from all sources (0.0.0.0/0) with Allow rules at priority 100-1000, which take precedence over the default Deny rule at priority 65500.

## VM Inventory & Status

### Working VMs (SSH Access Confirmed)
1. **etrid-audit-dev-secondary** (ETRID-VALIDATORS-UK-SOUTH)
   - Public IP: 51.142.203.160
   - NSG: etrid-nsg
   - SSH Rules: 3 (SSH-Access priority 100, Allow-SSH-Management priority 1000, Allow-SSH-From-New-IP priority 310)

### VMs Reported as Timing Out (But NSG Rules Are Correct)

#### ETRID-VALIDATORS-EU-NORTH (2 VMs)
1. **etrid-compiler-dev-secondary**
   - Public IP: 98.71.91.84
   - NSG: etrid-nsg
   - SSH Rules: 3 (SAME AS WORKING VM)

2. **etrid-multichain-dev-primary**
   - Public IP: 68.219.230.63
   - NSG: etrid-multichain-dev-primaryNSG
   - SSH Rules: 1 (default-allow-ssh priority 1000)

#### ETRID-VALIDATORS-EU-WEST (5 VMs)
3. **etrid-compiler-dev-primary**
   - Public IP: 4.180.59.25
   - NSG: etrid-nsg
   - SSH Rules: 3

4. **etrid-consensus-dev-secondary**
   - Public IP: 20.224.104.239
   - NSG: etrid-nsg
   - SSH Rules: 3

5. **etrid-multichain-dev-secondary**
   - Public IP: 98.71.219.106
   - NSG: etrid-nsg
   - SSH Rules: 3

6. **etrid-runtime-dev-primary**
   - Public IP: 108.142.205.177
   - NSG: etrid-nsg
   - SSH Rules: 3

7. **etrid-runtime-dev-secondary**
   - Public IP: 4.180.238.67
   - NSG: etrid-nsg
   - SSH Rules: 3

#### ETRID-VALIDATORS-UK-SOUTH (4 VMs - excluding working one)
8. **etrid-flarenode-15**
   - Public IP: 172.166.164.19
   - NSG: etrid-nsg
   - SSH Rules: 3

9. **etrid-flarenode-16**
   - Public IP: 172.166.187.180
   - NSG: etrid-nsg
   - SSH Rules: 3

10. **etrid-flarenode-17**
    - Public IP: 172.166.210.244
    - NSG: etrid-nsg
    - SSH Rules: 3

11. **etrid-oracle-dev**
    - Public IP: 172.167.8.217
    - NSG: etrid-nsg
    - SSH Rules: 3

#### ETRID-VALIDATORS-EU-FR (5 VMs)
12. **etrid-flarenode-18**
    - Public IP: 4.251.115.186
    - NSG: etrid-nsg
    - SSH Rules: 3

13. **etrid-flarenode-19**
    - Public IP: 52.143.191.232
    - NSG: etrid-nsg
    - SSH Rules: 3

14. **etrid-flarenode-20**
    - Public IP: 4.211.206.210
    - NSG: etrid-nsg
    - SSH Rules: 3

15. **etrid-flarenode-21**
    - Public IP: 4.178.181.122
    - NSG: etrid-nsg
    - SSH Rules: 3

16. **etrid-flarenode-22**
    - Public IP: 4.233.88.42
    - NSG: etrid-flarenode-22NSG
    - SSH Rules: 1 (default-allow-ssh priority 1000)

## NSG Configuration Details

### Standard NSG Configuration (etrid-nsg)
All VMs using "etrid-nsg" have the following SSH rules:

1. **SSH-Access** (Priority: 100)
   - Direction: Inbound
   - Access: Allow
   - Protocol: TCP
   - Source: * (0.0.0.0/0)
   - Destination Port: 22

2. **Allow-SSH-Management** (Priority: 1000)
   - Direction: Inbound
   - Access: Allow
   - Protocol: TCP
   - Source: * (0.0.0.0/0)
   - Destination Port: 22

3. **Allow-SSH-From-New-IP** (Priority: 310)
   - Direction: Inbound
   - Access: Allow
   - Protocol: TCP
   - Source: 172.58.12.128
   - Destination Port: 22

### Custom NSG Configurations

#### etrid-multichain-dev-primaryNSG
- **default-allow-ssh** (Priority: 1000)
  - Direction: Inbound
  - Access: Allow
  - Protocol: TCP
  - Source: *
  - Destination Port: 22

#### etrid-flarenode-22NSG
- **default-allow-ssh** (Priority: 1000)
  - Direction: Inbound
  - Access: Allow
  - Protocol: TCP
  - Source: *
  - Destination Port: 22

## Effective Security Rules Analysis

Verified the effective security rules on etrid-compiler-dev-secondary (non-working VM) and confirmed:
- SSH-Access rule (Priority 100) is ACTIVE and ALLOWING traffic
- No conflicting Deny rules exist before the default DenyAllInBound (Priority 65500)
- All SSH allow rules have precedence over default deny rules

## Network Configuration Comparison

Compared working VM (etrid-audit-dev-secondary) vs non-working VM (etrid-compiler-dev-secondary):
- Both have public IPs properly assigned
- Both have identical NSG configurations
- Both have IP Forwarding: false
- Both have Accelerated Networking: false
- Both have no subnet-level NSGs
- Both have no route tables attached

## VM Status
All 17 VMs are:
- **Power State:** VM running
- **Provisioning State:** Succeeded
- **Public IP Allocation:** Dynamic/Static (properly allocated)

## Root Cause Analysis

**The SSH timeout issue is NOT caused by NSG rules.** Possible root causes:

1. **SSH Daemon Not Running**
   - The sshd service may not be started on the VMs
   - Service may have crashed or failed to start on boot

2. **OS-Level Firewall Rules**
   - iptables/firewalld inside the VM may be blocking port 22
   - ufw (Ubuntu) or firewalld (CentOS/RHEL) configurations

3. **VM Guest Agent Issues**
   - Azure VM Agent may not be functioning properly
   - Guest OS may have networking issues

4. **Public IP Propagation Delay**
   - Recently allocated public IPs may not be fully propagated
   - DNS resolution issues

5. **Azure Platform Issues**
   - Transient Azure networking issues
   - Backend connectivity problems

6. **SSH Configuration Issues**
   - sshd_config may have incorrect settings
   - SSH keys may not be properly configured
   - Password authentication may be disabled

## Recommended Actions

### Immediate Diagnostic Steps

1. **Check SSH Daemon Status** (Use Azure Run Command):
```bash
az vm run-command invoke -g <RESOURCE_GROUP> -n <VM_NAME> \
  --command-id RunShellScript \
  --scripts "systemctl status sshd || systemctl status ssh"
```

2. **Check iptables Rules** (Use Azure Run Command):
```bash
az vm run-command invoke -g <RESOURCE_GROUP> -n <VM_NAME> \
  --command-id RunShellScript \
  --scripts "iptables -L -n | grep 22"
```

3. **Check if SSH Port is Listening** (Use Azure Run Command):
```bash
az vm run-command invoke -g <RESOURCE_GROUP> -n <VM_NAME> \
  --command-id RunShellScript \
  --scripts "netstat -tuln | grep ':22' || ss -tuln | grep ':22'"
```

4. **Enable Boot Diagnostics**:
```bash
# For each VM
az vm boot-diagnostics enable -g <RESOURCE_GROUP> -n <VM_NAME>
```

5. **Use Azure Serial Console**:
   - Navigate to VM in Azure Portal → Support + Troubleshooting → Serial Console
   - Login directly to check SSH service

### Remediation Steps (If SSH Daemon is Down)

For each non-working VM, run:

```bash
az vm run-command invoke -g <RESOURCE_GROUP> -n <VM_NAME> \
  --command-id RunShellScript \
  --scripts "systemctl start sshd && systemctl enable sshd && systemctl status sshd"
```

### Check and Fix Firewall Rules

```bash
az vm run-command invoke -g <RESOURCE_GROUP> -n <VM_NAME> \
  --command-id RunShellScript \
  --scripts "
    # For Ubuntu/Debian with UFW
    ufw allow 22/tcp || true

    # For CentOS/RHEL with firewalld
    firewall-cmd --permanent --add-service=ssh || true
    firewall-cmd --reload || true

    # For iptables
    iptables -A INPUT -p tcp --dport 22 -j ACCEPT || true
    iptables-save || true
  "
```

## NSG Rules Status: ✅ ALL CORRECT

**No NSG rule modifications are needed.** All VMs already have proper SSH allow rules configured.

## Next Steps

1. Use Azure Run Command to diagnose SSH daemon status on all non-working VMs
2. Check system logs via Serial Console or Boot Diagnostics
3. Verify SSH daemon is running and configured correctly
4. Check for OS-level firewall rules blocking SSH
5. If issues persist, consider restarting VMs or redeploying

## Conclusion

The Network Security Group configurations are correct across all 17 VMs. SSH on port 22 is allowed from all sources. The SSH timeout issue must be investigated at the VM operating system level or Azure platform level, not at the NSG level.

**NO NSG RULES NEED TO BE ADDED OR MODIFIED.**
