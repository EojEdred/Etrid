# Ëtrid Protocol SDKs

> Software Development Kits for building on Ëtrid Protocol

## Status

🔨 **In Active Development** - Basic implementations available, full features coming soon

## Available SDKs

### 1. rust-etrid-sdk
**Status**: ✅ Basic implementation available
**Location**: `13-clients/sdk/rust-etrid-sdk/`
**Use Cases**: Rust applications, node extensions, backend services

### 2. js-etrid-sdk
**Status**: ✅ Basic implementation available
**Location**: `13-clients/sdk/js-etrid-sdk/`
**Use Cases**: Web applications, Node.js backends, browser extensions

### 3. python-etrid-sdk
**Status**: ✅ Basic implementation available
**Location**: `13-clients/sdk/python-etrid-sdk/`
**Use Cases**: Python applications, data science, automation scripts

### 4. swift-etrid-sdk
**Status**: 📋 Planned
**Location**: `13-clients/sdk/swift-etrid-sdk/`
**Use Cases**: iOS apps, macOS apps, Swift backends

## Quick Start

Each SDK directory contains:
- `README.md` - Detailed documentation
- `examples/` - Usage examples
- `src/` - Source code
- `tests/` - Test suite

## Why Implement SDKs Now?

While the CLIs (etrust, etrcpp, pyE) provide complete command-line access, SDKs enable:

1. **Programmatic Integration** - Embed Ëtrid functionality in applications
2. **Custom Business Logic** - Build complex workflows beyond CLI capabilities
3. **Performance** - Direct library calls vs spawning CLI processes
4. **Type Safety** - Native language type checking
5. **Developer Experience** - IDE autocomplete, documentation, debugging

## Relationship to CLIs

```
CLIs (etrust, etrcpp, pyE)
    ↓ use
SDKs (libraries)
    ↓ communicate with
Ëtrid Blockchain (RPC/WebSocket)
```

The SDKs provide the underlying libraries that power the CLIs, and can be used directly by developers building applications.

---

**Next Steps**: Implement each SDK with basic RPC client, account management, and transaction signing.
