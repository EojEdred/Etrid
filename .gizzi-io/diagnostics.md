# Gizzi Diagnostics

- ok: false
- exitCode: 101
- checked: 2025-11-06T18:09:50.384Z

## Findings (14)
### 1. **E0432**
- location: `07-transactions/lightning-bloc/src/auto_discovery.rs:21:5`
- message:

```
error[E0432]: unresolved import `crate::network_graph`
```

### 2. **E0432**
- location: `07-transactions/lightning-bloc/src/invoice.rs:250:5`
- message:

```
error[E0432]: unresolved import `hex`
```

### 3. **E0433**
- location: `07-transactions/lightning-bloc/src/auto_discovery.rs:103:29`
- message:

```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `tracing`
```

### 4. **E0433**
- location: `07-transactions/lightning-bloc/src/auto_discovery.rs:109:29`
- message:

```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `tracing`
```

### 5. **E0433**
- location: `07-transactions/lightning-bloc/src/auto_discovery.rs:140:9`
- message:

```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `tracing`
```

### 6. **E0433**
- location: `07-transactions/lightning-bloc/src/auto_discovery.rs:195:9`
- message:

```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `tracing`
```

### 7. **E0433**
- location: `07-transactions/lightning-bloc/src/auto_discovery.rs:207:9`
- message:

```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `tracing`
```

### 8. **E0433**
- location: `07-transactions/lightning-bloc/src/auto_discovery.rs:220:9`
- message:

```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `tracing`
```

### 9. **E0433**
- location: `07-transactions/lightning-bloc/src/auto_discovery.rs:232:9`
- message:

```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `tracing`
```

### 10. **E0433**
- location: `07-transactions/lightning-bloc/src/auto_discovery.rs:292:13`
- message:

```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `tracing`
```

### 11. **E0433**
- location: `07-transactions/lightning-bloc/src/auto_discovery.rs:119:13`
- message:

```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `tokio`
```

### 12. **E0599**
- location: `07-transactions/lightning-bloc/src/auto_discovery.rs:149:21`
- message:

```
error[E0599]: no method named `add_pbc_graph` found for struct `CrossPBCRouter` in the current scope
```

### 13. **E0599**
- location: `07-transactions/lightning-bloc/src/auto_discovery.rs:290:20`
- message:

```
error[E0599]: no method named `add_pbc_graph` found for mutable reference `&mut CrossPBCRouter` in the current scope
```

### 14.
- location: `unknown`
- message:

```
error: could not compile `etrid-lightning-bloc` (lib) due to 13 previous errors; 13 warnings emitted
```
