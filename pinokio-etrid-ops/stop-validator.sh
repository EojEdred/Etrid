#!/bin/bash
#
# Stop Etrid Validator + Operations Center
#

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo ""
echo "🛑 Stopping Etrid Validator..."
echo ""

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Stop Operations Center
if [ -f ".ops.pid" ]; then
    OPS_PID=$(cat .ops.pid)
    if ps -p $OPS_PID > /dev/null 2>&1; then
        echo "Stopping Operations Center (PID: $OPS_PID)..."
        kill $OPS_PID
        echo -e "${GREEN}✓ Operations Center stopped${NC}"
    else
        echo -e "${YELLOW}⚠ Operations Center not running${NC}"
    fi
    rm .ops.pid
else
    # Try to find and kill by process name
    if pgrep -f "dashboard/server.js" > /dev/null; then
        pkill -f "dashboard/server.js"
        echo -e "${GREEN}✓ Operations Center stopped${NC}"
    else
        echo -e "${YELLOW}⚠ Operations Center not running${NC}"
    fi
fi

# Stop Validator
if [ -f ".validator.pid" ]; then
    VALIDATOR_PID=$(cat .validator.pid)
    if ps -p $VALIDATOR_PID > /dev/null 2>&1; then
        echo "Stopping Validator (PID: $VALIDATOR_PID)..."
        kill $VALIDATOR_PID

        # Wait for graceful shutdown
        sleep 2

        # Force kill if still running
        if ps -p $VALIDATOR_PID > /dev/null 2>&1; then
            kill -9 $VALIDATOR_PID
        fi

        echo -e "${GREEN}✓ Validator stopped${NC}"
    else
        echo -e "${YELLOW}⚠ Validator not running${NC}"
    fi
    rm .validator.pid
else
    # Try to find and kill by process name
    if pgrep -f "etrid-node" > /dev/null; then
        pkill -f "etrid-node"
        echo -e "${GREEN}✓ Validator stopped${NC}"
    else
        echo -e "${YELLOW}⚠ Validator not running${NC}"
    fi
fi

echo ""
echo -e "${GREEN}✓ All services stopped${NC}"
echo ""
