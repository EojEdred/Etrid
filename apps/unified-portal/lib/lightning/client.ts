/**
 * Lightning-Bloc Client
 *
 * Connects to the Etrid Lightning Network backend.
 * Communicates with the Cross-PBC Router for multi-chain payments.
 */

import type {
  Channel,
  Payment,
  NetworkStats,
  CrossPBCRoute,
  OpenChannelParams,
  SendPaymentParams,
  FindRouteParams
} from "./types"

export class LightningClient {
  private baseUrl: string
  private wsUrl: string

  constructor() {
    // Configure based on environment
    this.baseUrl = process.env.NEXT_PUBLIC_LIGHTNING_API_URL || "http://localhost:9944"
    this.wsUrl = process.env.NEXT_PUBLIC_LIGHTNING_WS_URL || "ws://localhost:9944"
  }

  /**
   * Get all Lightning channels for the current user
   */
  async getChannels(): Promise<Channel[]> {
    const response = await this.request("/lightning/channels")
    return response.channels
  }

  /**
   * Get payment history
   */
  async getPayments(): Promise<Payment[]> {
    const response = await this.request("/lightning/payments")
    return response.payments
  }

  /**
   * Get network statistics
   */
  async getNetworkStats(): Promise<NetworkStats> {
    const response = await this.request("/lightning/stats")
    return response.stats
  }

  /**
   * Find optimal cross-chain route
   */
  async findCrossPBCRoute(params: FindRouteParams): Promise<CrossPBCRoute> {
    const response = await this.request("/lightning/route", {
      method: "POST",
      body: JSON.stringify({
        source_chain: params.sourceChain,
        dest_chain: params.destChain,
        source_address: params.sourceAddress,
        dest_address: params.destAddress,
        amount: params.amount,
      }),
    })
    return response.route
  }

  /**
   * Send cross-chain Lightning payment
   */
  async sendPayment(params: SendPaymentParams) {
    const response = await this.request("/lightning/send", {
      method: "POST",
      body: JSON.stringify({
        route: params.route,
        source_address: params.sourceAddress,
        dest_address: params.destAddress,
      }),
    })
    return response
  }

  /**
   * Open new Lightning channel
   */
  async openChannel(params: OpenChannelParams) {
    const response = await this.request("/lightning/channels/open", {
      method: "POST",
      body: JSON.stringify({
        chain: params.chain,
        counterparty: params.counterparty,
        capacity: params.capacity,
      }),
    })
    return response
  }

  /**
   * Close Lightning channel
   */
  async closeChannel(channelId: string) {
    const response = await this.request(`/lightning/channels/${channelId}/close`, {
      method: "POST",
    })
    return response
  }

  /**
   * Get exchange rate between two chains
   */
  async getExchangeRate(fromChain: string, toChain: string) {
    const response = await this.request(
      `/lightning/rates?from=${fromChain}&to=${toChain}`
    )
    return response.rate
  }

  /**
   * Subscribe to Lightning network events
   */
  subscribeToEvents(callback: (event: any) => void) {
    const ws = new WebSocket(this.wsUrl)

    ws.onmessage = (event) => {
      const data = JSON.parse(event.data)
      callback(data)
    }

    ws.onerror = (error) => {
      console.error("WebSocket error:", error)
    }

    return () => ws.close()
  }

  /**
   * Internal request helper
   */
  private async request(endpoint: string, options: RequestInit = {}) {
    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      ...options,
      headers: {
        "Content-Type": "application/json",
        ...options.headers,
      },
    })

    if (!response.ok) {
      const error = await response.json().catch(() => ({ message: "Unknown error" }))
      throw new Error(error.message || `HTTP ${response.status}`)
    }

    return response.json()
  }
}
