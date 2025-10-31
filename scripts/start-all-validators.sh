#!/bin/bash
# Start All 21 Validators for Mainnet Launch
# Can do immediate start or scheduled start

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "╔════════════════════════════════════════════════════════════╗"
echo "║         Ëtrid FlareChain Mainnet Launch                   ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check validator list
if [ ! -f "validator-vms-numbered.txt" ]; then
    echo -e "${RED}❌ Error: validator-vms-numbered.txt not found${NC}"
    exit 1
fi

VALIDATOR_COUNT=$(wc -l < validator-vms-numbered.txt)
echo -e "${BLUE}Validators ready to start: $VALIDATOR_COUNT${NC}"
echo ""

# Ask for start method
echo "Choose start method:"
echo "  1) Immediate start (all validators start now)"
echo "  2) Scheduled start (all validators start at specified time)"
echo ""
read -p "Select option (1 or 2): " START_METHOD

case $START_METHOD in
    1)
        # Immediate start
        echo ""
        echo -e "${YELLOW}⚠️  MAINNET WILL START IMMEDIATELY${NC}"
        echo ""
        read -p "Are you absolutely sure? (type 'YES' to confirm): " CONFIRM

        if [ "$CONFIRM" != "YES" ]; then
            echo "Cancelled."
            exit 0
        fi

        echo ""
        echo -e "${GREEN}🚀 Starting all validators NOW...${NC}"
        echo ""

        start_count=0
        while read num vm; do
            VALIDATOR_NAME=$(jq -r ".validators[$((num-1))].name" mainnet-deployment-package/validator-keys-complete.json 2>/dev/null || echo "Validator-$num")
            echo -e "${GREEN}[$((start_count+1))/$VALIDATOR_COUNT]${NC} Starting $VALIDATOR_NAME on $vm..."
            ssh -i ~/.ssh/gizzi-validator -o StrictHostKeyChecking=no "$vm" "sudo systemctl start flarechain-validator" &
            start_count=$((start_count+1))
        done < validator-vms-numbered.txt

        # Wait for all background jobs to complete
        wait

        echo ""
        echo -e "${GREEN}✅ All $start_count validators started!${NC}"
        ;;

    2)
        # Scheduled start
        echo ""
        echo "Enter launch time (UTC):"
        read -p "  Minutes from now: " MINUTES

        if ! [[ "$MINUTES" =~ ^[0-9]+$ ]]; then
            echo -e "${RED}❌ Invalid number${NC}"
            exit 1
        fi

        START_TIME=$(date -u -d "+$MINUTES minutes" "+%Y-%m-%d %H:%M:%S" 2>/dev/null || date -u -v+${MINUTES}M "+%Y-%m-%d %H:%M:%S")
        START_TIME_AT=$(date -u -d "+$MINUTES minutes" "+%H:%M %Y-%m-%d" 2>/dev/null || date -u -v+${MINUTES}M "+%H:%M %Y-%m-%d")

        echo ""
        echo -e "${YELLOW}⚠️  MAINNET WILL START AT: $START_TIME UTC${NC}"
        echo ""
        read -p "Confirm scheduled start? (y/N) " -n 1 -r
        echo

        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 0
        fi

        echo ""
        echo -e "${GREEN}⏰ Scheduling validators to start at $START_TIME UTC...${NC}"
        echo ""

        schedule_count=0
        while read num vm; do
            VALIDATOR_NAME=$(jq -r ".validators[$((num-1))].name" mainnet-deployment-package/validator-keys-complete.json 2>/dev/null || echo "Validator-$num")
            echo -e "${GREEN}[$((schedule_count+1))/$VALIDATOR_COUNT]${NC} Scheduling $VALIDATOR_NAME..."

            # Schedule with 'at' command
            ssh -i ~/.ssh/gizzi-validator -o StrictHostKeyChecking=no "$vm" bash << EOSSH 2>/dev/null
                echo 'sudo systemctl start flarechain-validator' | at $START_TIME_AT 2>/dev/null || \
                (echo 'sudo systemctl start flarechain-validator' > /tmp/start_validator.sh && \
                 chmod +x /tmp/start_validator.sh && \
                 echo '/tmp/start_validator.sh' | at $START_TIME_AT)
EOSSH
            schedule_count=$((schedule_count+1))
        done < validator-vms-numbered.txt

        echo ""
        echo -e "${GREEN}✅ All $schedule_count validators scheduled!${NC}"
        echo ""
        echo "⏰ Countdown to mainnet launch: $MINUTES minutes"
        echo "📅 Launch time: $START_TIME UTC"
        ;;

    *)
        echo -e "${RED}❌ Invalid option${NC}"
        exit 1
        ;;
esac

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║              Mainnet Launch Initiated!                    ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Next steps:"
echo "  1. Monitor launch: ./monitor-mainnet.sh"
echo "  2. Check validators: ./check-validator-status.sh"
echo "  3. Watch logs: ssh <validator> 'sudo journalctl -fu flarechain-validator'"
echo ""
echo -e "${YELLOW}💡 Tip: Keep monitoring for the first hour to catch any issues${NC}"
echo ""
