"""
AI Dev Orchestrator - Main loop coordinating all 12 AI dev workers
Part of AI Dev Blockchain Monitoring System
"""

import os
import time
import sys
from pathlib import Path
from dotenv import load_dotenv

# Load environment variables from .env file
env_path = Path('/opt/ai-monitoring/.env')
if env_path.exists():
    load_dotenv(dotenv_path=env_path)
    print(f"✓ Loaded environment from {env_path}")
else:
    print(f"⚠ Warning: .env file not found at {env_path}")

from validator_monitor import ValidatorMonitor
from ai_dev_workers import AIDevWorker


class AIDevOrchestrator:
    """
    Orchestrates all 12 AI dev workers
    Runs continuous monitoring cycles
    """

    def __init__(self,
                 api_key: str,
                 validator_ips_path: str,
                 ssh_key_path: str,
                 prometheus_url: str,
                 memory_path: str = '/opt/ai-monitoring/GLOBAL_MEMORY.md',
                 optimized: bool = True):
        """
        Args:
            api_key: Anthropic API key (shared by all workers)
            validator_ips_path: Path to validator-ips.json
            ssh_key_path: Path to SSH private key
            prometheus_url: Prometheus server URL
            memory_path: Path to GLOBAL_MEMORY.md
            optimized: Use cost-optimized monitoring (only call Claude on issues)
        """
        self.api_key = api_key

        # Initialize validator monitor
        print("Initializing validator monitor...")
        self.monitor = ValidatorMonitor(
            validator_ips_path=validator_ips_path,
            ssh_key_path=ssh_key_path,
            prometheus_url=prometheus_url
        )

        # Create all 12 AI dev workers
        print(f"Creating AI dev workers (optimized={optimized})...")
        self.workers = {}

        ai_dev_ids = [
            'governance-dev01',
            'security-dev01',
            'audit-dev01',
            'consensus-dev01',
            'runtime-dev01',
            'compiler-dev01',
            'multichain-dev01',
            'oracle-dev01',
            'edsc-dev01',
            'economics-dev01',
            'ethics-dev01',
            'docs-dev01'
        ]

        for aidev_id in ai_dev_ids:
            try:
                self.workers[aidev_id] = AIDevWorker(
                    aidev_id,
                    api_key,
                    self.monitor,
                    memory_path=memory_path,
                    optimized=optimized
                )
            except Exception as e:
                print(f"WARNING: Failed to create worker {aidev_id}: {e}")

        print(f"✅ Initialized {len(self.workers)} AI dev workers")

    def run_monitoring_cycle(self):
        """Run one monitoring cycle for all AI devs"""
        print("\n" + "="*60)
        print(f"AI DEV ORCHESTRATOR - Monitoring Cycle")
        print("="*60)

        results = {}
        for aidev_id, worker in self.workers.items():
            try:
                results[aidev_id] = worker.monitoring_cycle()
            except Exception as e:
                print(f"[{aidev_id}] ERROR: {e}")
                results[aidev_id] = {"error": str(e)}

        # Summary
        healthy_count = sum(1 for r in results.values() if r.get('healthy', False))
        total_count = len(results)

        print("\n" + "="*60)
        print(f"Monitoring cycle complete: {healthy_count}/{total_count} AI devs report healthy validators")
        print("="*60)

        return results

    def run_forever(self, interval_seconds=300):
        """
        Run continuous monitoring

        Args:
            interval_seconds: Seconds between monitoring cycles
                             300 = 5 minutes (recommended for production)
                             60 = 1 minute (responsive but higher cost)
        """
        print(f"\n🚀 Starting AI Dev Monitoring")
        print(f"   Interval: {interval_seconds} seconds ({interval_seconds/60:.1f} minutes)")
        print(f"   Workers: {len(self.workers)}")
        print(f"   Total Validators: 21")
        print(f"   Optimized Mode: {self.workers[list(self.workers.keys())[0]].optimized}")
        print()

        cycle_count = 0

        while True:
            try:
                cycle_count += 1
                print(f"\n{'='*60}")
                print(f"CYCLE #{cycle_count} - {time.strftime('%Y-%m-%d %H:%M:%S')}")
                print(f"{'='*60}")

                self.run_monitoring_cycle()

                print(f"\n💤 Sleeping for {interval_seconds} seconds...")
                time.sleep(interval_seconds)

            except KeyboardInterrupt:
                print("\n\n⏹️  Stopping AI dev monitoring...")
                print(f"Total cycles completed: {cycle_count}")
                break
            except Exception as e:
                print(f"\n❌ Orchestrator error: {e}")
                print(f"Retrying in {interval_seconds} seconds...")
                time.sleep(interval_seconds)


# Main entry point
if __name__ == '__main__':
    print("="*60)
    print("AI DEV BLOCKCHAIN MONITORING SYSTEM")
    print("="*60)
    print()

    # Configuration
    API_KEY = os.getenv('ANTHROPIC_API_KEY')
    if not API_KEY:
        print("❌ Error: ANTHROPIC_API_KEY environment variable not set")
        print()
        print("To set it:")
        print("  export ANTHROPIC_API_KEY='your-api-key-here'")
        print()
        print("Get your API key from: https://console.anthropic.com/settings/keys")
        sys.exit(1)

    # Paths
    VALIDATOR_IPS_PATH = os.getenv('VALIDATOR_IPS_PATH', '/opt/ai-monitoring/validator-ips.json')
    SSH_KEY_PATH = os.getenv('SSH_KEY_PATH', '/home/ubuntu/.ssh/gizzi-validator')
    PROMETHEUS_URL = os.getenv('PROMETHEUS_URL', 'http://localhost:9090')
    MEMORY_PATH = os.getenv('MEMORY_PATH', '/opt/ai-monitoring/GLOBAL_MEMORY.md')

    # Monitoring interval (default 5 minutes = 300 seconds)
    INTERVAL = int(os.getenv('MONITOR_INTERVAL', '300'))

    # Optimized mode (default True = only call Claude when issues detected)
    OPTIMIZED = os.getenv('OPTIMIZED', 'true').lower() == 'true'

    print(f"Configuration:")
    print(f"  Validator IPs: {VALIDATOR_IPS_PATH}")
    print(f"  SSH Key: {SSH_KEY_PATH}")
    print(f"  Prometheus: {PROMETHEUS_URL}")
    print(f"  Memory Log: {MEMORY_PATH}")
    print(f"  Interval: {INTERVAL}s ({INTERVAL/60:.1f} minutes)")
    print(f"  Optimized: {OPTIMIZED}")
    print()

    # Create orchestrator
    try:
        orchestrator = AIDevOrchestrator(
            api_key=API_KEY,
            validator_ips_path=VALIDATOR_IPS_PATH,
            ssh_key_path=SSH_KEY_PATH,
            prometheus_url=PROMETHEUS_URL,
            memory_path=MEMORY_PATH,
            optimized=OPTIMIZED
        )
    except Exception as e:
        print(f"❌ Failed to initialize orchestrator: {e}")
        sys.exit(1)

    # Run forever
    orchestrator.run_forever(interval_seconds=INTERVAL)
