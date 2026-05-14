import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import type { InstalledMod, SaveBackup, Conflict, GameInfo, AppStatus } from './types';

export const api = {
  detectSekiroPath: () =>
    invoke<string | null>('detect_sekiro_path'),

  setSekiroPath: (path: string) =>
    invoke<GameInfo>('set_sekiro_path', { path }),

  getStatus: () =>
    invoke<AppStatus>('get_status'),

  scanInstalledMods: () =>
    invoke<InstalledMod[]>('scan_installed_mods'),

  installMod: (archivePath: string) =>
    invoke<InstalledMod>('install_mod', { archive_path: archivePath }),

  uninstallMod: (modId: string) =>
    invoke<void>('uninstall_mod', { mod_id: modId }),

  toggleMod: (modId: string, enabled: boolean) =>
    invoke<void>('toggle_mod', { mod_id: modId, enabled }),

  checkConflicts: () =>
    invoke<Conflict[]>('check_conflicts'),

  installModEngine: (archivePath: string) =>
    invoke<void>('install_mod_engine', { archive_path: archivePath }),

  backupSave: (label?: string) =>
    invoke<SaveBackup>('backup_save', { label }),

  restoreSave: (backupId: string) =>
    invoke<void>('restore_save', { backup_id: backupId }),

  importSave: (savePath: string) =>
    invoke<void>('import_save', { save_path: savePath }),

  listSaveBackups: () =>
    invoke<SaveBackup[]>('list_save_backups'),

  pickArchive: () =>
    open({ multiple: false, filters: [{ name: 'Mod Archives', extensions: ['zip', '7z', 'rar'] }] }),

  pickFolder: () =>
    open({ directory: true }),
};
