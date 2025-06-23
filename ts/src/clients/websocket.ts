import {
  WsConfig as WebSocketConfig,
  StringWsMessage,
  HashWsMessage,
  SetWsMessage,
  AdminWsMessage,
} from "../types";

// WebSocket type declaration for Node.js environments
declare global {
  interface WebSocket {
    readyState: number;
    OPEN: number;
    send(data: string): void;
    close(): void;
    onopen: ((event: Event) => void) | null;
    onmessage: ((event: MessageEvent) => void) | null;
    onerror: ((event: Event) => void) | null;
    onclose: ((event: Event) => void) | null;
  }

  var WebSocket: {
    new (url: string): WebSocket;
    OPEN: number;
  };
}

/**
 * WebSocket client for real-time operations
 */
export class WebSocketClient {
  private ws: WebSocket | null = null;
  private config: WebSocketConfig;
  private messageHandlers: Map<string, (data: any) => void> = new Map();

  constructor(config: WebSocketConfig) {
    this.config = config;
  }

  /**
   * Connect to WebSocket
   */
  connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        this.ws = new WebSocket(this.config.url);

        this.ws.onopen = (event: Event) => {
          if (this.config.onOpen) {
            this.config.onOpen(event);
          }
          resolve();
        };

        this.ws.onmessage = (event: MessageEvent) => {
          try {
            const message = JSON.parse(event.data);
            this.handleMessage(message);
          } catch (error) {
            console.error("Failed to parse WebSocket message:", error);
          }
        };

        this.ws.onerror = (error: Event) => {
          if (this.config.onError) {
            this.config.onError(error);
          }
          reject(error);
        };

        this.ws.onclose = (event: Event) => {
          if (this.config.onClose) {
            this.config.onClose(event);
          }
        };
      } catch (error) {
        reject(error);
      }
    });
  }

  /**
   * Handle incoming WebSocket messages
   */
  private handleMessage(message: any): void {
    if (this.config.onMessage) {
      this.config.onMessage(message);
    }

    // Call specific message handlers if registered
    if (message.type && this.messageHandlers.has(message.type)) {
      const handler = this.messageHandlers.get(message.type)!;
      handler(message);
    }
  }

  /**
   * Register a message handler for specific message types
   */
  onMessageType(type: string, handler: (data: any) => void): void {
    this.messageHandlers.set(type, handler);
  }

  /**
   * Remove a message handler
   */
  offMessageType(type: string): void {
    this.messageHandlers.delete(type);
  }

  /**
   * Send WebSocket message
   */
  sendMessage(message: StringWsMessage | HashWsMessage | SetWsMessage | AdminWsMessage): void {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      throw new Error("WebSocket is not connected");
    }

    this.ws.send(JSON.stringify(message));
  }

  /**
   * Send ping message
   */
  ping(): void {
    this.sendMessage({ type: "ping" });
  }

  /**
   * Close WebSocket connection
   */
  close(): void {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }

  /**
   * Check if WebSocket is connected
   */
  isConnected(): boolean {
    return this.ws !== null && this.ws.readyState === WebSocket.OPEN;
  }

  /**
   * Get WebSocket ready state
   */
  getReadyState(): number {
    return this.ws ? this.ws.readyState : -1;
  }
}
