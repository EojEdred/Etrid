#!/bin/bash

echo "🔍 ETH PBC Build Progress Monitor"
echo "=================================="
echo ""

if [ -f /tmp/eth-pbc-build.log ]; then
    COMPILED=$(grep "Compiling" /tmp/eth-pbc-build.log | wc -l | tr -d ' ')
    echo "📦 Dependencies compiled: $COMPILED"
    echo ""
    echo "📝 Latest compilation activity:"
    tail -10 /tmp/eth-pbc-build.log
    echo ""

    if grep -q "Finished" /tmp/eth-pbc-build.log; then
        echo "✅ BUILD COMPLETE!"
        echo ""
        echo "ETH PBC collator is ready to start."
        echo "Binary location: target/release/eth-pbc-collator"
    else
        echo "⏳ Build is still in progress..."
        echo ""
        echo "Estimated: 10-15 minutes total"
        echo "Run this script again to check progress: ./check-build-progress.sh"
    fi
else
    echo "❌ Build log not found. Build may not have started."
fi

echo ""
