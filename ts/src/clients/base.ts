import { ApiResponse, DbxConfig } from "../types";
import axios, { AxiosRequestConfig, AxiosResponse } from "axios";
import { API_BASE_URL } from "../config";

/**
 * Base client class with common HTTP request functionality
 */
export abstract class BaseClient {
  protected baseUrl: string;
  protected timeout: number;
  protected headers: Record<string, string>;

  constructor(config: DbxConfig) {
    this.baseUrl = (config.baseUrl || API_BASE_URL).replace(/\/$/, ""); // Remove trailing slash
    this.timeout = config.timeout || 10000;
    this.headers = {
      "Content-Type": "application/json",
      ...config.headers,
    };
  }

  /**
   * Make HTTP request with axios
   */
  protected async makeRequest<T>(url: string, options: AxiosRequestConfig = {}): Promise<T> {
    try {
      const fullUrl = url.startsWith("http") ? url : `${this.baseUrl}${url}`;
      const config: AxiosRequestConfig = {
        timeout: this.timeout,
        headers: {
          ...this.headers,
          ...options.headers,
        },
        ...options,
      };

      const response: AxiosResponse<T> = await axios(fullUrl, config);
      return response.data;
    } catch (error) {
      if (axios.isAxiosError(error)) {
        const errorMessage =
          error.response?.data?.error ||
          `HTTP ${error.response?.status}: ${error.response?.statusText}` ||
          error.message;
        throw new Error(errorMessage);
      }
      throw new Error("Request failed");
    }
  }

  /**
   * Helper method to handle API responses
   */
  protected handleApiResponse<T>(response: ApiResponse<T>): T {
    if (!response.success || !response.data) {
      throw new Error("API request failed");
    }
    return response.data;
  }
}
