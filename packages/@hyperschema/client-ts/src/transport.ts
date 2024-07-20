import { AxiosInstance } from "axios";
import { encode, decode } from "./msgpack";

export interface Transport {
  query(path: string, arg: any): Promise<any>;
  procedure(path: string, arg: any): Promise<any>;
}

export class AxiosTransport {
  constructor(private axios: AxiosInstance) {}

  // TODO: proper error handling for messagepack-serialized errors
  async query(path: string, arg: any): Promise<any> {
    const response = await this.axios.get(path, {
      params: { q: encode(arg) },
      headers: {
        "Content-Type": "application/msgpack",
      },
      responseType: "text",
    });
    return decode(response.data);
  }

  async procedure(path: string, arg: any): Promise<any> {
    const response = await this.axios.post(path, encode(arg), {
      headers: {
        "Content-Type": "application/msgpack",
      },
      responseType: "text",
    });
    return decode(response.data);
  }
}
