type StorageKey =
  | 'etrid_onboarding_completed'
  | 'etrid_primearc_ws_endpoints'
  | 'etrid_explorer_base_url'

function safeGet(key: StorageKey): string | null {
  if (typeof window === 'undefined') return null
  try {
    return window.localStorage.getItem(key)
  } catch {
    return null
  }
}

function safeSet(key: StorageKey, value: string): void {
  if (typeof window === 'undefined') return
  try {
    window.localStorage.setItem(key, value)
  } catch {
    // Ignore storage write failures (restricted environments).
  }
}

function unique(strings: string[]): string[] {
  const seen = new Set<string>()
  const out: string[] = []
  for (const value of strings) {
    const trimmed = value.trim()
    if (!trimmed) continue
    if (seen.has(trimmed)) continue
    seen.add(trimmed)
    out.push(trimmed)
  }
  return out
}

export function getPrimearcWsEndpoints(): string[] {
  const fromStorage = safeGet('etrid_primearc_ws_endpoints')
    ?.split(',')
    .map((s) => s.trim())
    .filter(Boolean)

  const fromEnv = (process.env.NEXT_PUBLIC_PRIMEARC_WS_ENDPOINTS || '')
    .split(',')
    .map((s) => s.trim())
    .filter(Boolean)

  const defaults = [
    process.env.NEXT_PUBLIC_PRIMEARC_WS_ENDPOINT,
    'wss://rpc.etrid.org',
  ].filter((v): v is string => Boolean(v))

  return unique([...(fromStorage ?? []), ...fromEnv, ...defaults])
}

export function setPrimearcWsEndpoints(endpoints: string[]): void {
  safeSet('etrid_primearc_ws_endpoints', unique(endpoints).join(','))
}

export function getExplorerBaseUrl(): string {
  const fromStorage = safeGet('etrid_explorer_base_url')
  if (fromStorage && fromStorage.trim()) return fromStorage.trim()
  return process.env.NEXT_PUBLIC_EXPLORER_BASE_URL || 'https://explorer.etrid.org'
}

export function setExplorerBaseUrl(url: string): void {
  safeSet('etrid_explorer_base_url', url.trim())
}

