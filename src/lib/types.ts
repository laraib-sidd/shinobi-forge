export type ModType = 'content' | 'dll' | 'hybrid' | 'unknown';
export type ModCategory = 'overhaul' | 'ai' | 'qol' | 'cosmetic' | 'unknown';
export type ConflictSeverity = 'warning' | 'error';

export interface InstalledMod {
  id: string;
  name: string;
  mod_type: ModType;
  category: ModCategory;
  enabled: boolean;
  installed_at: string;
  files: string[];
  dll_name?: string;
  source_archive: string;
}

export interface SaveBackup {
  id: string;
  label: string;
  created_at: string;
  path: string;
  size_bytes: number;
}

export interface Conflict {
  severity: ConflictSeverity;
  message: string;
  mod_ids: string[];
}

export interface GameInfo {
  path: string;
  mod_engine_installed: boolean;
}

export interface AppStatus {
  sekiro_path?: string;
  mod_engine_installed: boolean;
  active_mod_count: number;
  total_mod_count: number;
  save_backup_count: number;
  conflicts: Conflict[];
  first_run: boolean;
}
