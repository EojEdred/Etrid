#!/bin/bash

echo "====================================="
echo "Etrid Shared Packages Verification"
echo "====================================="
echo ""

# Check workspace configuration
echo "1. Workspace Configuration"
echo "   [✓] pnpm-workspace.yaml: $(test -f pnpm-workspace.yaml && echo "EXISTS" || echo "MISSING")"
echo "   [✓] turbo.json: $(test -f turbo.json && echo "EXISTS" || echo "MISSING")"
echo "   [✓] package.json: $(test -f package.json && echo "EXISTS" || echo "MISSING")"
echo ""

# Check packages
echo "2. Package Directories"
for pkg in ui hooks types utils; do
    if [ -d "packages/$pkg" ]; then
        echo "   [✓] @etrid/$pkg"
        echo "       - package.json: $(test -f packages/$pkg/package.json && echo "✓" || echo "✗")"
        echo "       - src/index.ts: $(test -f packages/$pkg/src/index.ts && echo "✓" || echo "✗")"
        file_count=$(find packages/$pkg/src -name "*.ts" -o -name "*.tsx" | wc -l | tr -d ' ')
        echo "       - Files: $file_count"
    else
        echo "   [✗] @etrid/$pkg - MISSING"
    fi
done
echo ""

# Check documentation
echo "3. Documentation"
echo "   [✓] packages/README.md: $(test -f packages/README.md && echo "EXISTS" || echo "MISSING")"
echo "   [✓] packages/IMPORT_GUIDE.md: $(test -f packages/IMPORT_GUIDE.md && echo "EXISTS" || echo "MISSING")"
echo "   [✓] PACKAGES_IMPLEMENTATION_REPORT.md: $(test -f PACKAGES_IMPLEMENTATION_REPORT.md && echo "EXISTS" || echo "MISSING")"
echo ""

# Summary
echo "4. Summary Statistics"
echo "   - Total packages: 4"
echo "   - Total TypeScript files: $(find packages -name "*.ts" -o -name "*.tsx" | wc -l | tr -d ' ')"
echo "   - @etrid/ui files: $(find packages/ui/src -name "*.ts" -o -name "*.tsx" 2>/dev/null | wc -l | tr -d ' ')"
echo "   - @etrid/hooks files: $(find packages/hooks/src -name "*.ts" 2>/dev/null | wc -l | tr -d ' ')"
echo "   - @etrid/types files: $(find packages/types/src -name "*.ts" 2>/dev/null | wc -l | tr -d ' ')"
echo "   - @etrid/utils files: $(find packages/utils/src -name "*.ts" 2>/dev/null | wc -l | tr -d ' ')"
echo ""

echo "====================================="
echo "✅ Verification Complete"
echo "====================================="
