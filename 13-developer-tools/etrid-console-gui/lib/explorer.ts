import { getExplorerBaseUrl as getExplorerBaseUrlFromConfig } from '@/lib/runtime-config'

function normalizeBaseUrl(url: string): string {
  return url.replace(/\/+$/, '')
}

export function getExplorerBaseUrl(): string {
  return normalizeBaseUrl(getExplorerBaseUrlFromConfig())
}

export function explorerHome(): string {
  return getExplorerBaseUrl()
}

export function explorerAccount(address: string): string {
  return `${getExplorerBaseUrl()}/account/${address}`
}

export function explorerExtrinsic(hash: string): string {
  return `${getExplorerBaseUrl()}/extrinsic/${hash}`
}

export function explorerTx(hash: string): string {
  return `${getExplorerBaseUrl()}/tx/${hash}`
}

export function explorerBlock(id: string | number): string {
  return `${getExplorerBaseUrl()}/block/${id}`
}
