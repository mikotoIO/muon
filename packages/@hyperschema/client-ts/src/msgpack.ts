import { bytesToBase64, base64ToBytes } from "byte-base64";
import msgpack from "@msgpack/msgpack";

// base64-encoded messagepack
export function encode(data: any): string {
  return bytesToBase64(msgpack.encode(data));
}

export function decode(data: string): any {
  return msgpack.decode(base64ToBytes(data));
}
