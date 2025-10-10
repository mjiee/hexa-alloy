import { call } from "@/utils/call";

export enum ConfigKey {
  ForgeClient = "forge_client",
}

export interface ForgeClientConfig {
  enable: boolean;
  host: string;
}

export interface SystemConfig<T> {
  key: string;
  value: T;
}

export interface GetConfigRequest {
  key: string;
}

export async function saveConfig<T>(req: SystemConfig<T>) {
  return call<SystemConfig<T>, undefined>("save_config", req);
}

export async function getConfig<T>(req: GetConfigRequest) {
  return call<GetConfigRequest, SystemConfig<T>>("get_config", req);
}
