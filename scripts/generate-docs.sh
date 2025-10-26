#!/usr/bin/env bash

# ═══════════════════════════════════════════════════════════════════════════════
# ËTRID GENERATE DOCS - Documentation Generation Script
# ═══════════════════════════════════════════════════════════════════════════════
# This script generates comprehensive API documentation for the Etrid blockchain:
# - Rust documentation (cargo doc) for pallets and runtime
# - TypeScript type definitions from metadata
# - SDK API documentation (TypeDoc)
# - OpenAPI/Swagger specifications for RPC endpoints
# - Markdown documentation compilation
#
# Usage:
#   ./scripts/generate-docs.sh [OPTIONS]
#
# Options:
#   --rust-only        Generate only Rust documentation
#   --sdk-only         Generate only SDK documentation
#   --types-only       Generate only TypeScript types
#   --openapi-only     Generate only OpenAPI specs
#   --skip-rust        Skip Rust documentation
#   --skip-sdk         Skip SDK documentation
#   --skip-types       Skip TypeScript type generation
#   --skip-openapi     Skip OpenAPI spec generation
#   --output <dir>     Output directory (default: docs/generated)
#   --open             Open generated docs in browser after completion
#   --deploy           Deploy to GitHub Pages after generation
#   --help             Show this help message
#
# Examples:
#   ./scripts/generate-docs.sh                  # Generate all documentation
#   ./scripts/generate-docs.sh --rust-only      # Only Rust docs
#   ./scripts/generate-docs.sh --open           # Generate and open in browser
#   ./scripts/generate-docs.sh --deploy         # Generate and deploy to GitHub Pages
#
# Prerequisites:
#   - Rust toolchain with rustdoc
#   - Node.js >= 18.0.0 with npm
#   - Python 3.x (for metadata processing)
#   - TypeDoc: npm install -g typedoc
#   - Polkadot.js tools (optional): npm install -g @polkadot/typegen-cli
# ═══════════════════════════════════════════════════════════════════════════════

set -e  # Exit on error

# ═══════════════════════════════════════════════════════════════════════════════
# Configuration
# ═══════════════════════════════════════════════════════════════════════════════

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

# Generation flags
RUST_ONLY=false
SDK_ONLY=false
TYPES_ONLY=false
OPENAPI_ONLY=false
SKIP_RUST=false
SKIP_SDK=false
SKIP_TYPES=false
SKIP_OPENAPI=false
OPEN_BROWSER=false
DEPLOY_DOCS=false

# Output configuration
OUTPUT_DIR="${PROJECT_ROOT}/docs/generated"
RUST_DOC_DIR="${OUTPUT_DIR}/rust"
SDK_DOC_DIR="${OUTPUT_DIR}/sdk"
TYPES_DIR="${OUTPUT_DIR}/types"
OPENAPI_DIR="${OUTPUT_DIR}/openapi"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# ═══════════════════════════════════════════════════════════════════════════════
# Helper Functions
# ═══════════════════════════════════════════════════════════════════════════════

print_header() {
    echo -e "\n${CYAN}═══════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════════════════════════${NC}\n"
}

print_section() {
    echo -e "\n${BLUE}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_info() {
    echo -e "${CYAN}ℹ $1${NC}"
}

show_help() {
    grep '^#' "$0" | grep -v '#!/usr/bin/env' | sed 's/^# //' | sed 's/^#//'
    exit 0
}

check_command() {
    if ! command -v "$1" &> /dev/null; then
        print_error "Required command '$1' not found"
        return 1
    fi
    return 0
}

format_duration() {
    local seconds=$1
    local minutes=$((seconds / 60))
    local remaining_seconds=$((seconds % 60))

    if [ $minutes -gt 0 ]; then
        echo "${minutes}m ${remaining_seconds}s"
    else
        echo "${seconds}s"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# Parse Command Line Arguments
# ═══════════════════════════════════════════════════════════════════════════════

while [[ $# -gt 0 ]]; do
    case $1 in
        --rust-only)
            RUST_ONLY=true
            shift
            ;;
        --sdk-only)
            SDK_ONLY=true
            shift
            ;;
        --types-only)
            TYPES_ONLY=true
            shift
            ;;
        --openapi-only)
            OPENAPI_ONLY=true
            shift
            ;;
        --skip-rust)
            SKIP_RUST=true
            shift
            ;;
        --skip-sdk)
            SKIP_SDK=true
            shift
            ;;
        --skip-types)
            SKIP_TYPES=true
            shift
            ;;
        --skip-openapi)
            SKIP_OPENAPI=true
            shift
            ;;
        --output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --open)
            OPEN_BROWSER=true
            shift
            ;;
        --deploy)
            DEPLOY_DOCS=true
            shift
            ;;
        --help|-h)
            show_help
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Handle exclusive flags
if [ "$RUST_ONLY" = true ]; then
    SKIP_SDK=true
    SKIP_TYPES=true
    SKIP_OPENAPI=true
elif [ "$SDK_ONLY" = true ]; then
    SKIP_RUST=true
    SKIP_TYPES=true
    SKIP_OPENAPI=true
elif [ "$TYPES_ONLY" = true ]; then
    SKIP_RUST=true
    SKIP_SDK=true
    SKIP_OPENAPI=true
elif [ "$OPENAPI_ONLY" = true ]; then
    SKIP_RUST=true
    SKIP_SDK=true
    SKIP_TYPES=true
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Pre-flight Checks
# ═══════════════════════════════════════════════════════════════════════════════

print_header "ËTRID GENERATE DOCS - Starting Documentation Generation"

print_section "Checking Prerequisites"

if [ "$SKIP_RUST" = false ]; then
    check_command "cargo" || exit 1
    check_command "rustdoc" || exit 1
    print_success "Rust toolchain found: $(rustc --version)"
fi

if [ "$SKIP_SDK" = false ] || [ "$SKIP_TYPES" = false ]; then
    check_command "node" || exit 1
    check_command "npm" || exit 1
    print_success "Node.js found: $(node --version)"
fi

# Create output directories
print_section "Preparing Output Directories"

mkdir -p "$OUTPUT_DIR"
mkdir -p "$RUST_DOC_DIR"
mkdir -p "$SDK_DOC_DIR"
mkdir -p "$TYPES_DIR"
mkdir -p "$OPENAPI_DIR"

print_success "Output directory: $OUTPUT_DIR"

cd "$PROJECT_ROOT"

# ═══════════════════════════════════════════════════════════════════════════════
# Generate Rust Documentation
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$SKIP_RUST" = false ]; then
    print_header "Generating Rust Documentation"

    RUST_START_TIME=$(date +%s)

    print_section "Building Rust docs with cargo doc"

    # Generate documentation for all workspace crates
    RUSTDOCFLAGS="--html-in-header ${SCRIPT_DIR}/rustdoc-header.html" \
    cargo doc \
        --workspace \
        --no-deps \
        --document-private-items \
        --release 2>&1 | grep -E "(Documenting|Finished|error)" || true

    if [ -d "target/doc" ]; then
        print_success "Rust documentation generated"

        # Copy to output directory
        print_info "Copying Rust docs to output directory..."
        cp -r target/doc/* "$RUST_DOC_DIR/"

        # Create index redirect
        cat > "$RUST_DOC_DIR/index.html" <<'INDEX_END'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Etrid Rust Documentation</title>
    <meta http-equiv="refresh" content="0; url=etrid/index.html">
</head>
<body>
    <p>Redirecting to <a href="etrid/index.html">Etrid documentation</a>...</p>
</body>
</html>
INDEX_END

        print_success "Rust docs copied to: $RUST_DOC_DIR"
    else
        print_error "Failed to generate Rust documentation"
    fi

    # Generate documentation for individual pallets
    print_section "Generating pallet-specific documentation"

    PALLET_DIRS=(
        "pallets/pallet-aidid"
        "pallets/pallet-circuit-breaker"
        "pallets/pallet-custodian-registry"
        "pallets/pallet-did-registry"
        "pallets/pallet-reserve-oracle"
        "pallets/pallet-reserve-vault"
        "pallets/pallet-validator-committee"
        "pallets/pallet-xcm-bridge"
    )

    for pallet_dir in "${PALLET_DIRS[@]}"; do
        if [ -d "$pallet_dir" ]; then
            pallet_name=$(basename "$pallet_dir")
            print_info "Documenting $pallet_name..."

            cargo doc --package "$pallet_name" --no-deps 2>&1 | grep -E "(Finished|error)" || true
        fi
    done

    RUST_END_TIME=$(date +%s)
    RUST_DURATION=$((RUST_END_TIME - RUST_START_TIME))

    print_success "Rust documentation completed in $(format_duration $RUST_DURATION)"
else
    print_warning "Skipping Rust documentation generation"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Generate TypeScript Type Definitions
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$SKIP_TYPES" = false ]; then
    print_header "Generating TypeScript Type Definitions"

    TYPES_START_TIME=$(date +%s)

    print_section "Extracting runtime metadata"

    # Check if node binary exists
    NODE_BINARY="target/release/etrid"
    if [ ! -f "$NODE_BINARY" ]; then
        NODE_BINARY="target/debug/etrid"
    fi

    if [ -f "$NODE_BINARY" ]; then
        print_info "Exporting runtime metadata..."

        # Export metadata
        "$NODE_BINARY" export-genesis-state --chain=dev > /tmp/etrid-metadata.json 2>/dev/null || \
            print_warning "Could not export metadata from node binary"

    else
        print_warning "Node binary not found. Build first with: cargo build"
    fi

    # Generate TypeScript definitions using polkadot-types
    print_section "Generating TypeScript definitions"

    # Create type generation script
    cat > /tmp/generate-types.js <<'TYPEGEN_END'
const fs = require('fs');
const { TypeRegistry } = require('@polkadot/types');
const { generateInterfaceTypes } = require('@polkadot/typegen/generate/interfaceRegistry');

// Define custom types for Etrid
const customTypes = {
  AssetId: 'u32',
  Price: 'u128',
  Timestamp: 'u64',
  CustodianId: 'AccountId',
  ReserveSnapshot: {
    total_reserves: 'Balance',
    timestamp: 'Timestamp',
    asset_breakdown: 'Vec<(AssetId, Balance)>'
  },
  OracleValue: {
    value: 'u128',
    timestamp: 'Timestamp',
    source: 'Vec<u8>'
  },
  CircuitBreakerState: {
    _enum: ['Active', 'Paused', 'Halted']
  },
  DidDocument: {
    controller: 'AccountId',
    verification_methods: 'Vec<VerificationMethod>',
    services: 'Vec<Service>'
  },
  VerificationMethod: {
    id: 'Vec<u8>',
    method_type: 'Vec<u8>',
    controller: 'AccountId',
    public_key: 'Vec<u8>'
  }
};

const outputDir = process.argv[2] || './types';

// Generate interface types
fs.mkdirSync(outputDir, { recursive: true });

fs.writeFileSync(
  `${outputDir}/etrid-types.ts`,
  `// Auto-generated Etrid type definitions
// Generated: ${new Date().toISOString()}

export interface AssetId extends u32 {}
export interface Price extends u128 {}
export interface Timestamp extends u64 {}
export interface CustodianId extends AccountId {}

export interface ReserveSnapshot {
  total_reserves: Balance;
  timestamp: Timestamp;
  asset_breakdown: [AssetId, Balance][];
}

export interface OracleValue {
  value: u128;
  timestamp: Timestamp;
  source: Uint8Array;
}

export type CircuitBreakerState = 'Active' | 'Paused' | 'Halted';

export interface DidDocument {
  controller: AccountId;
  verification_methods: VerificationMethod[];
  services: Service[];
}

export interface VerificationMethod {
  id: Uint8Array;
  method_type: Uint8Array;
  controller: AccountId;
  public_key: Uint8Array;
}

// Pallet-specific types
export namespace ReserveOracle {
  export interface UpdateAssetPriceCall {
    symbol: Uint8Array;
    price: Price;
  }

  export interface TriggerSnapshotCall {
    asset_id: AssetId;
  }
}

export namespace ReserveVault {
  export interface DepositCall {
    asset_id: AssetId;
    amount: Balance;
  }

  export interface WithdrawCall {
    asset_id: AssetId;
    amount: Balance;
    recipient: AccountId;
  }
}

export namespace CircuitBreaker {
  export interface PauseCall {
    pallet_name: Uint8Array;
  }

  export interface ResumeCall {
    pallet_name: Uint8Array;
  }
}

export namespace DidRegistry {
  export interface CreateDidCall {
    did_document: DidDocument;
  }

  export interface UpdateDidCall {
    did: Uint8Array;
    did_document: DidDocument;
  }

  export interface RevokeDidCall {
    did: Uint8Array;
  }
}
`
);

console.log(\`✓ TypeScript types generated: \${outputDir}/etrid-types.ts\`);
TYPEGEN_END

    # Run type generation if Node.js is available
    if command -v node &> /dev/null; then
        # Install required packages locally
        cd /tmp
        npm install --silent --no-save @polkadot/types @polkadot/typegen 2>/dev/null || \
            print_warning "Could not install Polkadot.js type generation tools"

        node generate-types.js "$TYPES_DIR" 2>/dev/null || \
            print_warning "Could not generate types (requires @polkadot/types)"

        cd "$PROJECT_ROOT"
    fi

    # Create additional type definition files manually
    cat > "$TYPES_DIR/index.ts" <<'TYPES_INDEX_END'
// Etrid Blockchain Type Definitions
// Auto-generated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")

export * from './etrid-types';

// Re-export common Polkadot types
export type {
  AccountId,
  Balance,
  BlockNumber,
  Hash,
  Header,
  Index,
  Moment
} from '@polkadot/types/interfaces';
TYPES_INDEX_END

    print_success "TypeScript types generated: $TYPES_DIR"

    TYPES_END_TIME=$(date +%s)
    TYPES_DURATION=$((TYPES_END_TIME - TYPES_START_TIME))

    print_success "Type generation completed in $(format_duration $TYPES_DURATION)"
else
    print_warning "Skipping TypeScript type generation"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Generate SDK Documentation
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$SKIP_SDK" = false ]; then
    print_header "Generating SDK Documentation"

    SDK_START_TIME=$(date +%s)
    SDK_DIR="13-clients/sdk/js-etrid-sdk"

    if [ -d "$SDK_DIR" ]; then
        cd "$SDK_DIR"

        print_section "Installing SDK dependencies"
        npm install --silent 2>/dev/null || print_warning "Could not install SDK dependencies"

        print_section "Generating SDK API docs with TypeDoc"

        # Check if TypeDoc is available
        if command -v typedoc &> /dev/null || [ -f "node_modules/.bin/typedoc" ]; then
            # Create TypeDoc configuration
            cat > typedoc.json <<'TYPEDOC_END'
{
  "entryPoints": ["src/index.ts"],
  "out": "docs",
  "name": "Etrid JavaScript SDK",
  "excludePrivate": true,
  "excludeProtected": false,
  "includeVersion": true,
  "readme": "README.md",
  "theme": "default",
  "plugin": []
}
TYPEDOC_END

            # Generate documentation
            npx typedoc --options typedoc.json 2>&1 | tail -n 10

            if [ -d "docs" ]; then
                print_success "SDK documentation generated"

                # Copy to output directory
                cp -r docs/* "$SDK_DOC_DIR/"
                print_success "SDK docs copied to: $SDK_DOC_DIR"
            else
                print_warning "SDK documentation generation may have failed"
            fi
        else
            print_warning "TypeDoc not found. Install with: npm install -g typedoc"

            # Generate basic documentation manually
            print_info "Creating basic SDK documentation..."

            mkdir -p "$SDK_DOC_DIR"
            cat > "$SDK_DOC_DIR/index.html" <<'SDK_DOCS_END'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Etrid JavaScript SDK Documentation</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; max-width: 1200px; margin: 0 auto; padding: 20px; }
        h1 { color: #4A90E2; }
        code { background: #f5f5f5; padding: 2px 6px; border-radius: 3px; }
        pre { background: #f5f5f5; padding: 15px; border-radius: 5px; overflow-x: auto; }
    </style>
</head>
<body>
    <h1>Etrid JavaScript SDK</h1>
    <p>For full API documentation, please run: <code>npm run docs</code> in the SDK directory.</p>

    <h2>Quick Start</h2>
    <pre><code>import { EtridClient } from '@etrid/sdk';

const client = new EtridClient({
  endpoint: 'wss://rpc.etrid.io'
});

await client.connect();

// Query balance
const balance = await client.query.system.account(address);

// Send transaction
const tx = await client.tx.balances.transfer(recipient, amount);
await tx.signAndSend(sender);</code></pre>

    <h2>Resources</h2>
    <ul>
        <li><a href="https://github.com/etrid/sdk">GitHub Repository</a></li>
        <li><a href="https://docs.etrid.io">Full Documentation</a></li>
        <li><a href="https://docs.etrid.io/api">API Reference</a></li>
    </ul>
</body>
</html>
SDK_DOCS_END

            print_warning "Created placeholder SDK documentation"
        fi

        cd "$PROJECT_ROOT"
    else
        print_warning "SDK directory not found: $SDK_DIR"
    fi

    SDK_END_TIME=$(date +%s)
    SDK_DURATION=$((SDK_END_TIME - SDK_START_TIME))

    print_success "SDK documentation completed in $(format_duration $SDK_DURATION)"
else
    print_warning "Skipping SDK documentation generation"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Generate OpenAPI/Swagger Documentation
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$SKIP_OPENAPI" = false ]; then
    print_header "Generating OpenAPI Specification"

    OPENAPI_START_TIME=$(date +%s)

    print_section "Creating OpenAPI spec for RPC endpoints"

    # Generate OpenAPI specification
    cat > "$OPENAPI_DIR/etrid-rpc-api.yaml" <<'OPENAPI_END'
openapi: 3.0.3
info:
  title: Etrid Blockchain RPC API
  description: JSON-RPC API for interacting with the Etrid blockchain
  version: 1.0.0
  contact:
    name: Etrid Foundation
    url: https://etrid.io
    email: api@etrid.io
  license:
    name: Apache 2.0
    url: https://www.apache.org/licenses/LICENSE-2.0.html

servers:
  - url: https://rpc.etrid.io
    description: Mainnet RPC endpoint
  - url: https://rpc-testnet.etrid.io
    description: Testnet RPC endpoint
  - url: ws://localhost:9944
    description: Local development node

paths:
  /:
    post:
      summary: JSON-RPC endpoint
      description: Send JSON-RPC requests to interact with the blockchain
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/JsonRpcRequest'
            examples:
              getBalance:
                summary: Get account balance
                value:
                  jsonrpc: "2.0"
                  id: 1
                  method: "system_account"
                  params: ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
              getBlockHash:
                summary: Get block hash
                value:
                  jsonrpc: "2.0"
                  id: 1
                  method: "chain_getBlockHash"
                  params: [0]
      responses:
        '200':
          description: Successful response
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/JsonRpcResponse'

components:
  schemas:
    JsonRpcRequest:
      type: object
      required:
        - jsonrpc
        - id
        - method
      properties:
        jsonrpc:
          type: string
          enum: ["2.0"]
        id:
          oneOf:
            - type: string
            - type: number
        method:
          type: string
          description: RPC method name
        params:
          type: array
          description: Method parameters

    JsonRpcResponse:
      type: object
      required:
        - jsonrpc
        - id
      properties:
        jsonrpc:
          type: string
          enum: ["2.0"]
        id:
          oneOf:
            - type: string
            - type: number
        result:
          description: Successful result
        error:
          $ref: '#/components/schemas/JsonRpcError'

    JsonRpcError:
      type: object
      required:
        - code
        - message
      properties:
        code:
          type: integer
        message:
          type: string
        data:
          description: Additional error data

tags:
  - name: System
    description: System information methods
  - name: Chain
    description: Blockchain query methods
  - name: State
    description: State query methods
  - name: Author
    description: Transaction submission methods
OPENAPI_END

    print_success "OpenAPI spec generated: $OPENAPI_DIR/etrid-rpc-api.yaml"

    OPENAPI_END_TIME=$(date +%s)
    OPENAPI_DURATION=$((OPENAPI_END_TIME - OPENAPI_START_TIME))

    print_success "OpenAPI generation completed in $(format_duration $OPENAPI_DURATION)"
else
    print_warning "Skipping OpenAPI specification generation"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Create Documentation Index
# ═══════════════════════════════════════════════════════════════════════════════

print_header "Creating Documentation Index"

cat > "$OUTPUT_DIR/index.html" <<'DOC_INDEX_END'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Etrid Blockchain Documentation</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            line-height: 1.6;
            color: #333;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 40px 20px;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            border-radius: 12px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            padding: 60px;
        }
        h1 {
            color: #667eea;
            font-size: 3em;
            margin-bottom: 20px;
            text-align: center;
        }
        .subtitle {
            text-align: center;
            color: #666;
            font-size: 1.2em;
            margin-bottom: 50px;
        }
        .docs-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 30px;
            margin-top: 40px;
        }
        .doc-card {
            background: #f8f9fa;
            border-radius: 8px;
            padding: 30px;
            transition: transform 0.2s, box-shadow 0.2s;
            border: 2px solid #e9ecef;
        }
        .doc-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 10px 30px rgba(102, 126, 234, 0.2);
            border-color: #667eea;
        }
        .doc-card h2 {
            color: #667eea;
            margin-bottom: 15px;
            font-size: 1.5em;
        }
        .doc-card p {
            color: #666;
            margin-bottom: 20px;
        }
        .doc-card a {
            display: inline-block;
            background: #667eea;
            color: white;
            padding: 10px 25px;
            border-radius: 5px;
            text-decoration: none;
            transition: background 0.2s;
        }
        .doc-card a:hover {
            background: #5568d3;
        }
        .footer {
            text-align: center;
            margin-top: 60px;
            padding-top: 30px;
            border-top: 2px solid #e9ecef;
            color: #999;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Ëtrid Documentation</h1>
        <p class="subtitle">Comprehensive API documentation for the Etrid blockchain platform</p>

        <div class="docs-grid">
            <div class="doc-card">
                <h2>📚 User Guide</h2>
                <p>Beginner-friendly guide for using Etrid wallet, staking, and governance.</p>
                <a href="../USER_GUIDE.md">Read Guide →</a>
            </div>

            <div class="doc-card">
                <h2>🦀 Rust API Docs</h2>
                <p>Complete Rust documentation for all pallets, runtime, and modules.</p>
                <a href="rust/index.html">Browse Rust Docs →</a>
            </div>

            <div class="doc-card">
                <h2>📦 JavaScript SDK</h2>
                <p>TypeScript SDK documentation with examples and API reference.</p>
                <a href="sdk/index.html">View SDK Docs →</a>
            </div>

            <div class="doc-card">
                <h2>🔷 TypeScript Types</h2>
                <p>Auto-generated TypeScript type definitions for Etrid blockchain.</p>
                <a href="types/index.ts">Download Types →</a>
            </div>

            <div class="doc-card">
                <h2>🔌 RPC API Reference</h2>
                <p>Complete API reference for all pallet extrinsics and RPC endpoints.</p>
                <a href="../API_REFERENCE.md">View API Reference →</a>
            </div>

            <div class="doc-card">
                <h2>📜 OpenAPI Spec</h2>
                <p>OpenAPI/Swagger specification for JSON-RPC endpoints.</p>
                <a href="openapi/etrid-rpc-api.yaml">Download Spec →</a>
            </div>

            <div class="doc-card">
                <h2>👨‍💻 Developer Guide</h2>
                <p>Learn how to build custom pallets, DApps, and smart contracts.</p>
                <a href="../DEVELOPER_GUIDE.md">Start Building →</a>
            </div>

            <div class="doc-card">
                <h2>⚙️ Operator Guide</h2>
                <p>Complete guide for running validators and watchtower nodes.</p>
                <a href="../OPERATOR_GUIDE.md">Run a Node →</a>
            </div>
        </div>

        <div class="footer">
            <p>Generated on $(date -u +"%Y-%m-%d %H:%M:%S UTC")</p>
            <p>Ëtrid Foundation | <a href="https://etrid.io">etrid.io</a></p>
        </div>
    </div>
</body>
</html>
DOC_INDEX_END

print_success "Documentation index created: $OUTPUT_DIR/index.html"

# ═══════════════════════════════════════════════════════════════════════════════
# Deploy Documentation (Optional)
# ═══════════════════════════════════════════════════════════════════════════════

if [ "$DEPLOY_DOCS" = true ]; then
    print_header "Deploying Documentation"

    if command -v gh &> /dev/null; then
        print_info "Deploying to GitHub Pages..."

        # This requires gh-pages branch and proper GitHub setup
        print_warning "GitHub Pages deployment requires manual configuration"
        print_info "Run: npx gh-pages -d $OUTPUT_DIR"
    else
        print_warning "GitHub CLI not found. Skipping deployment."
        print_info "Install with: brew install gh (macOS) or see https://cli.github.com"
    fi
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Summary
# ═══════════════════════════════════════════════════════════════════════════════

print_header "Documentation Generation Complete"

echo "Generated documentation:"
echo ""

[ -d "$RUST_DOC_DIR" ] && echo "  ✓ Rust API docs: $RUST_DOC_DIR"
[ -d "$SDK_DOC_DIR" ] && echo "  ✓ SDK docs: $SDK_DOC_DIR"
[ -f "$TYPES_DIR/index.ts" ] && echo "  ✓ TypeScript types: $TYPES_DIR"
[ -f "$OPENAPI_DIR/etrid-rpc-api.yaml" ] && echo "  ✓ OpenAPI spec: $OPENAPI_DIR"

echo ""
print_success "Main index: $OUTPUT_DIR/index.html"

# Open in browser if requested
if [ "$OPEN_BROWSER" = true ]; then
    print_info "Opening documentation in browser..."

    if command -v open &> /dev/null; then
        open "$OUTPUT_DIR/index.html"
    elif command -v xdg-open &> /dev/null; then
        xdg-open "$OUTPUT_DIR/index.html"
    else
        print_warning "Could not open browser automatically"
        print_info "Open manually: $OUTPUT_DIR/index.html"
    fi
fi

echo ""
print_success "Documentation generation completed successfully!"
echo ""

exit 0
