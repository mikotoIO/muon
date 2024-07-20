export interface Transport {
  query(path: string, arg: any): Promise<any>;
  procedure(path: string, arg: any): Promise<any>;
}
