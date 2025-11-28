import type { WebSocketMessage } from '../types';

export type WebSocketStatus = 'connecting' | 'connected' | 'disconnected' | 'error';

export interface WebSocketConfig {
  url: string;
  reconnectInterval?: number;
  maxReconnectAttempts?: number;
  heartbeatInterval?: number;
}

export type MessageHandler = (message: WebSocketMessage) => void;
export type StatusHandler = (status: WebSocketStatus) => void;

export class WatchtowerWebSocket {
  private ws: WebSocket | null = null;
  private config: Required<WebSocketConfig>;
  private reconnectAttempts = 0;
  private reconnectTimer: NodeJS.Timeout | null = null;
  private heartbeatTimer: NodeJS.Timeout | null = null;
  private messageHandlers: Set<MessageHandler> = new Set();
  private statusHandlers: Set<StatusHandler> = new Set();
  private currentStatus: WebSocketStatus = 'disconnected';

  constructor(config: WebSocketConfig) {
    this.config = {
      url: config.url,
      reconnectInterval: config.reconnectInterval || 5000,
      maxReconnectAttempts: config.maxReconnectAttempts || 10,
      heartbeatInterval: config.heartbeatInterval || 30000,
    };
  }

  /**
   * Connect to the WebSocket server
   */
  connect(): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      console.log('WebSocket already connected');
      return;
    }

    this.setStatus('connecting');

    try {
      this.ws = new WebSocket(this.config.url);

      this.ws.onopen = this.handleOpen.bind(this);
      this.ws.onmessage = this.handleMessage.bind(this);
      this.ws.onerror = this.handleError.bind(this);
      this.ws.onclose = this.handleClose.bind(this);
    } catch (error) {
      console.error('Failed to create WebSocket:', error);
      this.setStatus('error');
      this.scheduleReconnect();
    }
  }

  /**
   * Disconnect from the WebSocket server
   */
  disconnect(): void {
    this.clearTimers();

    if (this.ws) {
      this.ws.onclose = null; // Prevent reconnection
      this.ws.close();
      this.ws = null;
    }

    this.setStatus('disconnected');
  }

  /**
   * Send a message to the server
   */
  send(message: WebSocketMessage): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
    } else {
      console.warn('WebSocket not connected. Message not sent:', message);
    }
  }

  /**
   * Subscribe to incoming messages
   */
  onMessage(handler: MessageHandler): () => void {
    this.messageHandlers.add(handler);

    // Return unsubscribe function
    return () => {
      this.messageHandlers.delete(handler);
    };
  }

  /**
   * Subscribe to status changes
   */
  onStatusChange(handler: StatusHandler): () => void {
    this.statusHandlers.add(handler);

    // Immediately call with current status
    handler(this.currentStatus);

    // Return unsubscribe function
    return () => {
      this.statusHandlers.delete(handler);
    };
  }

  /**
   * Get current connection status
   */
  getStatus(): WebSocketStatus {
    return this.currentStatus;
  }

  private handleOpen(): void {
    console.log('WebSocket connected');
    this.reconnectAttempts = 0;
    this.setStatus('connected');
    this.startHeartbeat();
  }

  private handleMessage(event: MessageEvent): void {
    try {
      const message: WebSocketMessage = JSON.parse(event.data);

      // Notify all message handlers
      this.messageHandlers.forEach(handler => {
        try {
          handler(message);
        } catch (error) {
          console.error('Error in message handler:', error);
        }
      });
    } catch (error) {
      console.error('Failed to parse WebSocket message:', error);
    }
  }

  private handleError(event: Event): void {
    console.error('WebSocket error:', event);
    this.setStatus('error');
  }

  private handleClose(event: CloseEvent): void {
    console.log('WebSocket closed:', event.code, event.reason);
    this.clearTimers();
    this.setStatus('disconnected');

    // Attempt to reconnect if not a normal closure
    if (event.code !== 1000) {
      this.scheduleReconnect();
    }
  }

  private setStatus(status: WebSocketStatus): void {
    this.currentStatus = status;

    // Notify all status handlers
    this.statusHandlers.forEach(handler => {
      try {
        handler(status);
      } catch (error) {
        console.error('Error in status handler:', error);
      }
    });
  }

  private scheduleReconnect(): void {
    if (this.reconnectAttempts >= this.config.maxReconnectAttempts) {
      console.error('Max reconnection attempts reached');
      this.setStatus('error');
      return;
    }

    if (this.reconnectTimer) {
      return; // Already scheduled
    }

    this.reconnectAttempts++;
    const delay = this.config.reconnectInterval * Math.min(this.reconnectAttempts, 5);

    console.log(`Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts}/${this.config.maxReconnectAttempts})`);

    this.reconnectTimer = setTimeout(() => {
      this.reconnectTimer = null;
      this.connect();
    }, delay);
  }

  private startHeartbeat(): void {
    this.heartbeatTimer = setInterval(() => {
      if (this.ws?.readyState === WebSocket.OPEN) {
        this.send({
          type: 'channel_update',
          payload: { ping: true },
          timestamp: new Date(),
        });
      }
    }, this.config.heartbeatInterval);
  }

  private clearTimers(): void {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }

    if (this.heartbeatTimer) {
      clearInterval(this.heartbeatTimer);
      this.heartbeatTimer = null;
    }
  }
}

/**
 * Create a WebSocket connection with default configuration
 */
export function createWatchtowerWebSocket(url?: string): WatchtowerWebSocket {
  // Get URL from config or use default
  const wsUrl = url || (typeof window !== 'undefined' && localStorage.getItem('watchtower-config')
    ? JSON.parse(localStorage.getItem('watchtower-config')!).wsEndpoint
    : 'ws://localhost:9944');

  return new WatchtowerWebSocket({
    url: wsUrl,
    reconnectInterval: 5000,
    maxReconnectAttempts: 10,
    heartbeatInterval: 30000,
  });
}

// Singleton instance for use across the application
let wsInstance: WatchtowerWebSocket | null = null;

/**
 * Get or create the singleton WebSocket instance
 */
export function getWatchtowerWebSocket(): WatchtowerWebSocket {
  if (!wsInstance) {
    wsInstance = createWatchtowerWebSocket();
  }
  return wsInstance;
}

/**
 * Reset the singleton WebSocket instance
 */
export function resetWatchtowerWebSocket(): void {
  if (wsInstance) {
    wsInstance.disconnect();
    wsInstance = null;
  }
}
