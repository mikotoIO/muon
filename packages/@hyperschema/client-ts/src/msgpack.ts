import { bytesToBase64, base64ToBytes } from "byte-base64";
import msgpack from "@msgpack/msgpack";

// base64-encoded messagepack, for use in query param
export function encode64(data: any): string {
  return bytesToBase64(msgpack.encode(data));
}

export function encode(data: any): Uint8Array {
  return msgpack.encode(data);
}

export function decode(data: Uint8Array): any {
  return msgpack.decode(data);
}
