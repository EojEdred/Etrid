#!/bin/bash
# Documentation Restructuring Script

echo "=== Ëtrid Documentation Restructuring ==="
echo ""

# Move specifications
echo "Moving specifications..."
mv "***ETRID_IVORY_PAPER_v2.0.md" "docs/specifications/ivory-paper.md" 2>/dev/null
mv "***ETRID_PROTOCOL_CHARTER.md" "docs/specifications/protocol-charter.md" 2>/dev/null
mv "etrid-protocol-governance-appendix.md" "docs/specifications/governance-appendix.md" 2>/dev/null
mv "***Etrid_project_roadmap.md" "docs/history/original-roadmap.md" 2>/dev/null

# Move guides
echo "Moving guides..."
mv "DEVELOPER_GUIDE.md" "docs/guides/developer-guide.md" 2>/dev/null
mv "USER_GUIDE.md" "docs/guides/user-guide.md" 2>/dev/null
mv "DEPLOYMENT_GUIDE.md" "docs/guides/deployment.md" 2>/dev/null
mv "TESTING_GUIDE.md" "docs/guides/testing.md" 2>/dev/null
mv "LOCAL_TESTING_GUIDE.md" "docs/guides/local-testing.md" 2>/dev/null

# Move operations
echo "Moving operations docs..."
mv "OPERATIONAL_READINESS.md" "docs/operations/readiness.md" 2>/dev/null
mv "OPERATIONS.md" "docs/operations/runbook.md" 2>/dev/null
mv "INCIDENT_RESPONSE.md" "docs/operations/incident-response.md" 2>/dev/null
mv "SECURITY_CHECKLIST.md" "docs/operations/security-checklist.md" 2>/dev/null
mv "DOCKER_SETUP.md" "docs/operations/docker.md" 2>/dev/null
mv "EMBER_DEPLOYMENT_CHECKLIST.md" "docs/operations/ember-checklist.md" 2>/dev/null
mv "EMBER_TESTNET_README.md" "docs/operations/ember-testnet.md" 2>/dev/null

# Move API docs
echo "Moving API documentation..."
mv "API_REFERENCE.md" "docs/api/reference.md" 2>/dev/null

# Move architecture docs
echo "Moving architecture docs..."
mv "MULTICHAIN_TEST_RESULTS.md" "docs/architecture/multichain-testing.md" 2>/dev/null

# Move history
echo "Moving history docs..."
mv "PROJECT_HISTORY.md" "docs/history/project-history.md" 2>/dev/null

# Archive session reports
echo "Archiving session reports..."
mv "APPLY_PBC_COMMON_TO_ALL.md" "docs/archive/sessions/" 2>/dev/null
mv "ARCHITECTURE_AUDIT_COMPLETE_OCT20.md" "docs/archive/sessions/" 2>/dev/null
mv "ARCHITECTURE_CORRECTIONS_REPORT.md" "docs/archive/sessions/" 2>/dev/null
mv "AUDIT_INDEX.md" "docs/archive/sessions/" 2>/dev/null
mv "CODEBASE_AUDIT_DETAILED.md" "docs/archive/sessions/" 2>/dev/null
mv "CODEBASE_AUDIT_OCT20.md" "docs/archive/sessions/" 2>/dev/null
mv "CODEBASE_AUDIT_REPORT.md" "docs/archive/sessions/" 2>/dev/null
mv "CODEBASE_CONSOLIDATION_REPORT.md" "docs/archive/sessions/" 2>/dev/null
mv "CONSOLIDATION_SUMMARY.txt" "docs/archive/sessions/" 2>/dev/null
mv "CURRENT_STATUS.md" "docs/archive/sessions/" 2>/dev/null
mv "DOCUMENTATION_CLEANUP_COMPLETE.md" "docs/archive/sessions/" 2>/dev/null
mv "DOCUMENTATION_COMPLETE_OCT20.md" "docs/archive/sessions/" 2>/dev/null
mv "EDSC_BRIDGE_STATUS.md" "docs/archive/sessions/" 2>/dev/null
mv "EDSC_IMPLEMENTATION_PLAN.md" "docs/archive/sessions/" 2>/dev/null
mv "EDSC_PALLET_ARCHITECTURE.md" "docs/archive/sessions/" 2>/dev/null
mv "EDSC_PBT_INTEGRATION_GAMEPLAN.md" "docs/archive/sessions/" 2>/dev/null
mv "eddc-pbt:update.md" "docs/archive/sessions/" 2>/dev/null
mv "edsc-pbt.md" "docs/archive/sessions/" 2>/dev/null
mv "FRONTEND_IMPLEMENTATION_STATUS.md" "docs/archive/sessions/" 2>/dev/null
mv "FRONTEND_INTEGRATION_PLAN.md" "docs/archive/sessions/" 2>/dev/null
mv "INTEGRATION_SUMMARY_OCT20_CONTINUED.md" "docs/archive/sessions/" 2>/dev/null
mv "LIGHTNING_BLOC_COMPLETE.md" "docs/archive/sessions/" 2>/dev/null
mv "PARALLEL_PHASES_COMPLETION_REPORT.md" "docs/archive/sessions/" 2>/dev/null
mv "PBC_COMMON_FUTURE_UTILITIES.md" "docs/archive/sessions/" 2>/dev/null
mv "PBC_COMMON_ROLLOUT_COMPLETE.md" "docs/archive/sessions/" 2>/dev/null
mv "PBC_DUPLICATION_ANALYSIS.md" "docs/archive/sessions/" 2>/dev/null
mv "PBC_REFACTORING_ANALYSIS.md" "docs/archive/sessions/" 2>/dev/null
mv "PBC_REFACTORING_COMPLETE.md" "docs/archive/sessions/" 2>/dev/null
mv "PBC_TEMPLATE_SYSTEM_DESIGN.md" "docs/archive/sessions/" 2>/dev/null
mv "PHASE3_CCTP_BRIDGE_PLAN.md" "docs/archive/sessions/" 2>/dev/null
mv "PROJECT_COMPLETION_SUMMARY.md" "docs/archive/sessions/" 2>/dev/null
mv "REPOSITORY_ARCHITECTURE_AUDIT.md" "docs/archive/sessions/" 2>/dev/null
mv "SESSION_OCT20_CONTINUED_FINAL.md" "docs/archive/sessions/" 2>/dev/null
mv "VALUE_REFERENCE.md" "docs/archive/sessions/" 2>/dev/null
mv "DEVELOPMENT_ROADMAP.md" "docs/archive/sessions/" 2>/dev/null

echo ""
echo "✅ Restructuring complete!"
echo ""
echo "Files remaining in root:"
find . -maxdepth 1 -name "*.md" -type f | wc -l
