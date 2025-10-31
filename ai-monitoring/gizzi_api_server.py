#!/usr/bin/env python3
"""
Gizzi Network API Server
Provides REST API for validators to query network status
"""

from flask import Flask, jsonify, request
import requests
import json
import subprocess
from typing import Dict, List

app = Flask(__name__)

# Configuration
PROMETHEUS_URL = "http://localhost:9090"
VALIDATOR_IPS_PATH = "/opt/ai-monitoring/validator-ips.json"


def load_validators() -> List[Dict]:
    """Load validator configuration"""
    try:
        with open(VALIDATOR_IPS_PATH, 'r') as f:
            data = json.load(f)
            return data.get('validators', [])
    except Exception as e:
        return []


def query_prometheus(query: str) -> Dict:
    """Query Prometheus"""
    try:
        url = f"{PROMETHEUS_URL}/api/v1/query"
        response = requests.get(url, params={'query': query}, timeout=5)
        return response.json()
    except Exception as e:
        return {"status": "error", "error": str(e)}


@app.route('/')
def index():
    """API documentation"""
    return jsonify({
        "name": "Gizzi Network API",
        "version": "1.0",
        "endpoints": [
            "/api/network/status - Network-wide status",
            "/api/validator/<id> - Single validator details",
            "/api/validators/list - List all validators",
            "/api/metrics/query?q=<query> - Raw Prometheus query",
            "/api/network/summary - Quick network summary"
        ]
    })


@app.route('/api/network/status')
def network_status():
    """
    Get status of all validators

    Returns:
        {
          "validators": [
            {
              "number": 1,
              "name": "Gizzi",
              "ip": "64.181.215.19",
              "block_height": 12345,
              "peers": 8,
              "finalized_height": 12340,
              "health": "healthy"
            },
            ...
          ],
          "network_summary": {
            "total_validators": 21,
            "online": 20,
            "average_peers": 7.5,
            "max_block_height": 12345
          }
        }
    """
    validators = load_validators()
    validator_statuses = []

    total_online = 0
    total_peers = 0
    max_block_height = 0

    for validator in validators:
        validator_num = validator['number']

        # Query metrics
        block_query = f'substrate_block_height{{instance=~".*{validator_num}.*"}}'
        peers_query = f'substrate_peers_count{{instance=~".*{validator_num}.*"}}'
        finalized_query = f'substrate_finalized_height{{instance=~".*{validator_num}.*"}}'

        block_result = query_prometheus(block_query)
        peers_result = query_prometheus(peers_query)
        finalized_result = query_prometheus(finalized_query)

        # Extract values
        block_height = 0
        peers = 0
        finalized_height = 0
        is_online = False

        if block_result.get('status') == 'success' and block_result['data']['result']:
            block_height = int(float(block_result['data']['result'][0]['value'][1]))
            is_online = True
            total_online += 1

        if peers_result.get('status') == 'success' and peers_result['data']['result']:
            peers = int(float(peers_result['data']['result'][0]['value'][1]))
            total_peers += peers

        if finalized_result.get('status') == 'success' and finalized_result['data']['result']:
            finalized_height = int(float(finalized_result['data']['result'][0]['value'][1]))

        max_block_height = max(max_block_height, block_height)

        # Determine health
        health = "offline"
        if is_online:
            if peers < 2 or (finalized_height > 0 and block_height - finalized_height > 100):
                health = "warning"
            elif peers >= 3 and (finalized_height == 0 or block_height - finalized_height < 50):
                health = "healthy"
            else:
                health = "warning"

        validator_statuses.append({
            "number": validator_num,
            "name": validator['name'],
            "ip": validator.get('ip', 'NEEDS_IP'),
            "block_height": block_height,
            "peers": peers,
            "finalized_height": finalized_height,
            "finalization_lag": block_height - finalized_height if finalized_height > 0 else 0,
            "health": health,
            "aiDevId": validator.get('aiDevId'),
            "role": validator.get('role')
        })

    return jsonify({
        "validators": validator_statuses,
        "network_summary": {
            "total_validators": len(validators),
            "online": total_online,
            "average_peers": round(total_peers / max(total_online, 1), 1),
            "max_block_height": max_block_height,
            "timestamp": subprocess.check_output(['date', '+%Y-%m-%d %H:%M:%S']).decode().strip()
        }
    })


@app.route('/api/validator/<int:validator_id>')
def validator_details(validator_id: int):
    """
    Get detailed metrics for a single validator

    Returns:
        {
          "validator": {
            "number": 6,
            "name": "Runtime Dev",
            "ip": "20.224.104.239"
          },
          "metrics": {
            "block_height": 12345,
            "peers": 8,
            "finalized_height": 12340,
            "finalization_lag": 5
          },
          "health": "healthy"
        }
    """
    validators = load_validators()
    validator = next((v for v in validators if v['number'] == validator_id), None)

    if not validator:
        return jsonify({"error": f"Validator {validator_id} not found"}), 404

    # Query metrics
    block_query = f'substrate_block_height{{instance=~".*{validator_id}.*"}}'
    peers_query = f'substrate_peers_count{{instance=~".*{validator_id}.*"}}'
    finalized_query = f'substrate_finalized_height{{instance=~".*{validator_id}.*"}}'

    block_result = query_prometheus(block_query)
    peers_result = query_prometheus(peers_query)
    finalized_result = query_prometheus(finalized_query)

    # Extract values
    block_height = 0
    peers = 0
    finalized_height = 0

    if block_result.get('status') == 'success' and block_result['data']['result']:
        block_height = int(float(block_result['data']['result'][0]['value'][1]))

    if peers_result.get('status') == 'success' and peers_result['data']['result']:
        peers = int(float(peers_result['data']['result'][0]['value'][1]))

    if finalized_result.get('status') == 'success' and finalized_result['data']['result']:
        finalized_height = int(float(finalized_result['data']['result'][0]['value'][1]))

    # Determine health
    health = "offline"
    if block_height > 0:
        if peers < 2 or (finalized_height > 0 and block_height - finalized_height > 100):
            health = "warning"
        elif peers >= 3 and (finalized_height == 0 or block_height - finalized_height < 50):
            health = "healthy"
        else:
            health = "warning"

    return jsonify({
        "validator": {
            "number": validator['number'],
            "name": validator['name'],
            "ip": validator.get('ip', 'NEEDS_IP'),
            "aiDevId": validator.get('aiDevId'),
            "role": validator.get('role')
        },
        "metrics": {
            "block_height": block_height,
            "peers": peers,
            "finalized_height": finalized_height,
            "finalization_lag": block_height - finalized_height if finalized_height > 0 else 0
        },
        "health": health
    })


@app.route('/api/validators/list')
def validators_list():
    """List all validators with basic info"""
    validators = load_validators()
    return jsonify({
        "validators": [
            {
                "number": v['number'],
                "name": v['name'],
                "ip": v.get('ip', 'NEEDS_IP'),
                "aiDevId": v.get('aiDevId'),
                "role": v.get('role'),
                "region": v.get('region')
            }
            for v in validators
        ]
    })


@app.route('/api/metrics/query')
def prometheus_query():
    """
    Raw Prometheus query

    Query params:
        q: Prometheus query string

    Example:
        /api/metrics/query?q=substrate_block_height
    """
    query = request.args.get('q')
    if not query:
        return jsonify({"error": "Missing query parameter 'q'"}), 400

    result = query_prometheus(query)
    return jsonify(result)


@app.route('/api/network/summary')
def network_summary():
    """
    Quick network summary (optimized for Ollama)

    Returns:
        {
          "total_validators": 21,
          "online": 20,
          "healthy": 18,
          "warning": 2,
          "offline": 1,
          "average_block_height": 12345,
          "average_peers": 7.5
        }
    """
    validators = load_validators()

    online = 0
    healthy = 0
    warning = 0
    offline = 0
    total_block_height = 0
    total_peers = 0

    for validator in validators:
        validator_num = validator['number']

        # Quick query
        block_query = f'substrate_block_height{{instance=~".*{validator_num}.*"}}'
        peers_query = f'substrate_peers_count{{instance=~".*{validator_num}.*"}}'

        block_result = query_prometheus(block_query)
        peers_result = query_prometheus(peers_query)

        if block_result.get('status') == 'success' and block_result['data']['result']:
            block_height = int(float(block_result['data']['result'][0]['value'][1]))
            total_block_height += block_height
            online += 1

            if peers_result.get('status') == 'success' and peers_result['data']['result']:
                peers = int(float(peers_result['data']['result'][0]['value'][1]))
                total_peers += peers

                if peers >= 3:
                    healthy += 1
                else:
                    warning += 1
            else:
                warning += 1
        else:
            offline += 1

    return jsonify({
        "total_validators": len(validators),
        "online": online,
        "healthy": healthy,
        "warning": warning,
        "offline": offline,
        "average_block_height": round(total_block_height / max(online, 1)),
        "average_peers": round(total_peers / max(online, 1), 1)
    })


@app.route('/health')
def health_check():
    """Health check endpoint"""
    return jsonify({"status": "ok", "service": "Gizzi Network API"})


if __name__ == '__main__':
    # Run on all interfaces, port 8080
    app.run(host='0.0.0.0', port=8080, debug=False)
