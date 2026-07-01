export interface KeystoreConfig {
  keystorePath: string;
  storePassword: string;
  keyAlias: string;
  keyPassword: string;
}

export interface AppSettings {
  bundletoolJarPath: string;
  defaultOutputDir: string;
  lastKeystoreConfig: KeystoreConfig;
  lastDeviceTemplate: string;
  language: string;
}

export interface EnvStatus {
  javaOk: boolean;
  javaVersion: string;
  bundletoolOk: boolean;
  bundletoolPath: string;
}

export interface AppFileInfo {
  packageName: string;
  versionName: string;
  versionCode: string;
  minSdkVersion: string;
  targetSdkVersion: string;
  permissions: string[];
}

export interface KeystoreAliasInfo {
  name: string;
  expires: string;
}

export interface ConvertResult {
  success: boolean;
  apkPath: string;
  outputDir: string;
  apkFiles: string[];
}

export interface ConvertProgress {
  line: string;
  done: boolean;
  success: boolean;
  apkPath?: string;
  error?: string;
}

export interface KeystoreEntry {
  name: string;
  config: KeystoreConfig;
}

export interface DeviceTemplate {
  key: string;
  name: string;
  description: string;
  deviceSpec: string | null;
}
