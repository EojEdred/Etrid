"""
Validator Monitor - Collects validator status from multiple sources
Part of AI Dev Blockchain Monitoring System
"""

import paramiko
import requests
from typing import Dict, List
import json


class ValidatorMonitor:
    """
    Collects validator status from multiple sources:
    - Prometheus metrics
    - Blockchain RPC
    - SSH logs
    """

    def __init__(self,
                 validator_ips_path: str,
                 ssh_key_path: str,
                 prometheus_url: str):
        """
        Args:
            validator_ips_path: Path to validator-ips.json
            ssh_key_path: Path to SSH private key (~/.ssh/gizzi-validator)
            prometheus_url: Prometheus server URL (http://localhost:9090)
        """
        self.ssh_key_path = ssh_key_path
        self.prometheus_url = prometheus_url

        # Load validator IPs
        with open(validator_ips_path, 'r') as f:
            data = json.load(f)
            self.validators = {v['number']: v for v in data['validators']}

    def get_validators_by_aidevid(self, aidev_id: str) -> List[Dict]:
        """Get all validators assigned to an AI dev"""
        return [v for v in self.validators.values()
                if v.get('aiDevId') == aidev_id]

    def check_validator_status(self, validator_num: int) -> Dict:
        """
        Check complete validator status

        Returns:
            {
                "validator_num": 1,
                "name": "Gizzi",
                "ip": "64.181.215.19",
                "metrics": {...},
                "rpc_status": {...},
                "process_status": {...},
                "healthy": True,
                "issues": []
            }
        """
        validator = self.validators[validator_num]

        # 1. Get Prometheus metrics
        metrics = self._query_prometheus_metrics(validator['ip'])

        # 2. Get RPC status
        rpc_status = self._query_rpc(validator['ip'])

        # 3. Get process status via SSH
        process_status = self._check_process_ssh(validator)

        # 4. Determine health
        healthy, issues = self._analyze_health(metrics, rpc_status, process_status)

        return {
            "validator_num": validator_num,
            "name": validator['name'],
            "ip": validator['ip'],
            "metrics": metrics,
            "rpc_status": rpc_status,
            "process_status": process_status,
            "healthy": healthy,
            "issues": issues
        }

    def _query_prometheus_metrics(self, ip: str) -> Dict:
        """Query Prometheus for validator metrics"""
        queries = {
            "block_height": f'substrate_block_height{{instance="{ip}:9615"}}',
            "peers": f'substrate_sub_libp2p_peers_count{{instance="{ip}:9615"}}',
            "finalized_height": f'substrate_block_height{{status="finalized",instance="{ip}:9615"}}'
        }

        metrics = {}
        for metric_name, query in queries.items():
            try:
                response = requests.get(
                    f"{self.prometheus_url}/api/v1/query",
                    params={"query": query},
                    timeout=5
                )
                data = response.json()
                if data['data']['result']:
                    metrics[metric_name] = float(data['data']['result'][0]['value'][1])
                else:
                    metrics[metric_name] = None
            except Exception as e:
                metrics[metric_name] = None

        return metrics

    def _query_rpc(self, ip: str) -> Dict:
        """Query validator RPC endpoint"""
        try:
            response = requests.post(
                f"http://{ip}:9944",
                json={
                    "jsonrpc": "2.0",
                    "method": "system_health",
                    "params": [],
                    "id": 1
                },
                timeout=5
            )
            result = response.json().get('result', {})
            result['online'] = True
            return result
        except:
            return {"online": False}

    def _check_process_ssh(self, validator: Dict) -> Dict:
        """Check validator process via SSH"""
        try:
            ssh = paramiko.SSHClient()
            ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())

            # Determine username from validator number
            # Oracle Cloud (1, 5) = ubuntu
            # Azure = dev-specific usernames
            username = self._get_username(validator)

            ssh.connect(
                validator['ip'],
                username=username,
                key_filename=self.ssh_key_path,
                timeout=10
            )

            # Check if flarechain-node is running
            stdin, stdout, stderr = ssh.exec_command(
                "pgrep -f flarechain-node || echo 'not_running'"
            )
            output = stdout.read().decode().strip()

            running = output != 'not_running' and output != ''

            ssh.close()

            return {
                "running": running,
                "checked_via": "ssh",
                "username": username
            }
        except Exception as e:
            return {
                "running": None,
                "error": str(e)
            }

    def _get_username(self, validator: Dict) -> str:
        """Determine SSH username based on validator"""
        validator_num = validator['number']

        # Oracle Cloud VMs
        if validator_num in [1, 5]:
            return 'ubuntu'

        # Azure VMs - use aiDevId as username
        aidev_id = validator.get('aiDevId', '')
        if aidev_id:
            return aidev_id

        # Fallback
        return 'ubuntu'

    def _analyze_health(self, metrics, rpc, process) -> tuple:
        """
        Analyze validator health from collected data

        Returns:
            (healthy: bool, issues: List[str])
        """
        issues = []

        # Check process running
        if not process.get('running'):
            issues.append("Process not running")

        # Check RPC connectivity
        if not rpc.get('online'):
            issues.append("RPC not responding")

        # Check peer count
        peers = metrics.get('peers')
        if peers is not None and peers < 3:
            issues.append(f"Low peer count: {int(peers)}")

        # Check block height
        block_height = metrics.get('block_height')
        finalized = metrics.get('finalized_height')
        if block_height and finalized:
            lag = block_height - finalized
            if lag > 100:
                issues.append(f"Finalization lag: {int(lag)} blocks")

        healthy = len(issues) == 0

        return healthy, issues

    def restart_validator(self, validator_num: int, reason: str):
        """Restart validator via SSH"""
        validator = self.validators[validator_num]
        username = self._get_username(validator)

        try:
            ssh = paramiko.SSHClient()
            ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())
            ssh.connect(
                validator['ip'],
                username=username,
                key_filename=self.ssh_key_path,
                timeout=10
            )

            # Try systemd first, then pkill/start
            stdin, stdout, stderr = ssh.exec_command(
                "sudo systemctl restart flarechain-validator || "
                "(pkill -9 flarechain-node && nohup ~/flarechain-node --validator > /dev/null 2>&1 &)"
            )

            ssh.close()

            print(f"✅ Restarted validator {validator_num}: {reason}")
            return True
        except Exception as e:
            print(f"❌ Failed to restart validator {validator_num}: {e}")
            return False
