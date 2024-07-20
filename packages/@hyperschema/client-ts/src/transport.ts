import { AxiosInstance } from "axios";
import { encode64, encode, decode } from "./msgpack";

export interface Transport {
  query(path: string, arg: any): Promise<any>;
  procedure(path: string, arg: any): Promise<any>;
}

// we pass around msgpack, encoded as base64.
// Content-Type: text/plain is used, not application/messagepack.
export class AxiosTransport {
  constructor(private axios: AxiosInstance) {}

  // TODO: proper error handling for messagepack-serialized errors
  async query(path: string, arg: any): Promise<any> {
    const response = await this.axios.get(path, {
      params: { q: encode64(arg) },
      responseType: "arraybuffer",
    });
    return decode(new Uint8Array(response.data));
  }

  async procedure(path: string, arg: any): Promise<any> {
    const response = await this.axios.post(path, encode(arg), {
      headers: {
        "Content-Type": "application/msgpack",
      },
      responseType: "arraybuffer",
    });
    return decode(new Uint8Array(response.data));
  }
}
