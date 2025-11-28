#!/bin/bash
# Example: Automated monitoring and alerting script
# Run this from cron every 5 minutes

# Configuration
VALIDATOR_ADDRESS="${1:-$(cat ~/.etrid/validator-address 2>/dev/null)}"
RPC_ENDPOINT="${2:-http://localhost:9944}"
ALERT_EMAIL="${ALERT_EMAIL:-ops@example.com}"
LOG_DIR="/var/log/etrid"
HEALTH_FILE="${LOG_DIR}/health-$(date +%Y%m%d).json"

# Create log directory if it doesn't exist
mkdir -p "${LOG_DIR}"

# Function to send alert
send_alert() {
  local subject="$1"
  local message="$2"

  echo "${message}" | mail -s "${subject}" "${ALERT_EMAIL}"
  logger -t etrid-monitor "${subject}: ${message}"
}

# Function to check metric threshold
check_threshold() {
  local metric="$1"
  local value="$2"
  local threshold="$3"
  local comparison="$4"  # "gt" or "lt"

  if [ "${comparison}" = "lt" ]; then
    if [ "${value}" -lt "${threshold}" ]; then
      return 0
    fi
  else
    if [ "${value}" -gt "${threshold}" ]; then
      return 0
    fi
  fi
  return 1
}

# Main health check
echo "Running health check at $(date)"

asf-health \
  --rpc "${RPC_ENDPOINT}" \
  --validator "${VALIDATOR_ADDRESS}" \
  --format json \
  --output "${HEALTH_FILE}"

HEALTH_STATUS=$?

if [ ${HEALTH_STATUS} -ne 0 ]; then
  send_alert \
    "CRITICAL: Validator Health Check Failed" \
    "Health check failed for validator ${VALIDATOR_ADDRESS}. Check ${HEALTH_FILE} for details."
  exit 1
fi

# Parse and check specific metrics
UPTIME=$(jq -r '.checks[] | select(.name=="System Resources") | .details.uptime_percentage // 0' "${HEALTH_FILE}")
PEER_COUNT=$(jq -r '.checks[] | select(.name=="P2P Peer Count") | .details.peer_count // 0' "${HEALTH_FILE}")
HEALTH_SCORE=$(jq -r '.checks[] | select(.name=="Validator Status") | .details.health_score // 0' "${HEALTH_FILE}")

# Check uptime
if check_threshold "uptime" "${UPTIME}" 95 "lt"; then
  send_alert \
    "WARNING: Low Validator Uptime" \
    "Validator uptime is ${UPTIME}% (threshold: 95%)"
fi

# Check peer count
if check_threshold "peers" "${PEER_COUNT}" 10 "lt"; then
  send_alert \
    "WARNING: Low Peer Count" \
    "Validator has only ${PEER_COUNT} peers (threshold: 10)"
fi

# Check health score
if check_threshold "health" "${HEALTH_SCORE}" 70 "lt"; then
  send_alert \
    "WARNING: Low Health Score" \
    "Validator health score is ${HEALTH_SCORE}/100 (threshold: 70)"
fi

# Log success
echo "Health check passed at $(date)" >> "${LOG_DIR}/monitor.log"

# Cleanup old health files (keep last 7 days)
find "${LOG_DIR}" -name "health-*.json" -mtime +7 -delete

exit 0
