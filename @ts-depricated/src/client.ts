import { DbxConfig, toSdkConfig } from "./config";
import { WebSocketConfig } from "./types/admin";
import {
  AdminClient,
  StringClient,
  SetClient,
  HashClient,
  CommonClient,
  WebSocketClient,
} from "./clients";

/**
 * DBX Modular Client - TypeScript SDK for DBX API
 * Provides modular access to different Redis operations
 */
export class DbxClient {
  public readonly admin: AdminClient;
  public readonly string: StringClient;
  public readonly set: SetClient;
  public readonly hash: HashClient;
  public readonly common: CommonClient;
  private config: DbxConfig;

  constructor(config: DbxConfig) {
    this.config = config;
    const sdkConfig = toSdkConfig(config);
    this.admin = new AdminClient(sdkConfig);
    this.string = new StringClient(sdkConfig);
    this.set = new SetClient(sdkConfig);
    this.hash = new HashClient(sdkConfig);
    this.common = new CommonClient(sdkConfig);
  }

  /**
   * Create WebSocket client for admin operations
   */
  createAdminWebSocket(wsConfig: Omit<WebSocketConfig, "url">): WebSocketClient {
    return new WebSocketClient({
      ...wsConfig,
      url: this.config.wsHostUrl + "/admin/ws",
    });
  }

  /**
   * Create WebSocket client for string operations
   */
  createStringWebSocket(wsConfig: Omit<WebSocketConfig, "url">): WebSocketClient {
    return new WebSocketClient({
      ...wsConfig,
      url: this.config.wsHostUrl + "/string/ws",
    });
  }

  /**
   * Create WebSocket client for hash operations
   */
  createHashWebSocket(wsConfig: Omit<WebSocketConfig, "url">): WebSocketClient {
    return new WebSocketClient({
      ...wsConfig,
      url: this.config.wsHostUrl + "/hash/ws",
    });
  }

  /**
   * Create WebSocket client for set operations
   */
  createSetWebSocket(wsConfig: Omit<WebSocketConfig, "url">): WebSocketClient {
    return new WebSocketClient({
      ...wsConfig,
      url: this.config.wsHostUrl + "/set/ws",
    });
  }

  /**
   * Create WebSocket client (legacy method)
   */
  createWebSocket(config: WebSocketConfig): WebSocketClient {
    return new WebSocketClient(config);
  }
}
