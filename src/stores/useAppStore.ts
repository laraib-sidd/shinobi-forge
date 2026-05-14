import { create } from 'zustand';
import { api } from '../lib/tauri';
import type { AppStatus, InstalledMod, SaveBackup, Conflict } from '../lib/types';

interface AppStore {
  status: AppStatus | null;
  mods: InstalledMod[];
  backups: SaveBackup[];
  conflicts: Conflict[];
  loading: boolean;
  error: string | null;

  loadAll: () => Promise<void>;
  installMod: (archivePath: string) => Promise<void>;
  uninstallMod: (modId: string) => Promise<void>;
  toggleMod: (modId: string, enabled: boolean) => Promise<void>;
  backupSave: (label?: string) => Promise<void>;
  restoreSave: (backupId: string) => Promise<void>;
  importSave: (savePath: string) => Promise<void>;
  setSekiroPath: (path: string) => Promise<void>;
  installModEngine: (archivePath: string) => Promise<void>;
  clearError: () => void;
}

export const useAppStore = create<AppStore>((set, _get) => ({
  status: null,
  mods: [],
  backups: [],
  conflicts: [],
  loading: false,
  error: null,

  clearError: () => set({ error: null }),

  loadAll: async () => {
    set({ loading: true, error: null });
    try {
      const [status, mods, backups, conflicts] = await Promise.all([
        api.getStatus(),
        api.scanInstalledMods(),
        api.listSaveBackups(),
        api.checkConflicts(),
      ]);
      set({ status, mods, backups, conflicts, loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  installMod: async (archivePath) => {
    set({ loading: true, error: null });
    try {
      const mod = await api.installMod(archivePath);
      const conflicts = await api.checkConflicts();
      set(s => ({ mods: [...s.mods, mod], conflicts, loading: false }));
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  uninstallMod: async (modId) => {
    set({ loading: true, error: null });
    try {
      await api.uninstallMod(modId);
      const conflicts = await api.checkConflicts();
      set(s => ({ mods: s.mods.filter(m => m.id !== modId), conflicts, loading: false }));
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  toggleMod: async (modId, enabled) => {
    set({ error: null });
    try {
      await api.toggleMod(modId, enabled);
      const conflicts = await api.checkConflicts();
      set(s => ({
        mods: s.mods.map(m => m.id === modId ? { ...m, enabled } : m),
        conflicts,
      }));
    } catch (e) {
      set({ error: String(e) });
    }
  },

  backupSave: async (label) => {
    set({ loading: true, error: null });
    try {
      const backup = await api.backupSave(label);
      set(s => ({ backups: [...s.backups, backup], loading: false }));
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  restoreSave: async (backupId) => {
    set({ loading: true, error: null });
    try {
      await api.restoreSave(backupId);
      set({ loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  importSave: async (savePath) => {
    set({ loading: true, error: null });
    try {
      await api.importSave(savePath);
      set({ loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  setSekiroPath: async (path) => {
    set({ loading: true, error: null });
    try {
      const info = await api.setSekiroPath(path);
      set(s => ({
        status: s.status
          ? { ...s.status, sekiro_path: info.path, mod_engine_installed: info.mod_engine_installed, first_run: false }
          : null,
        loading: false,
      }));
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  installModEngine: async (archivePath) => {
    set({ loading: true, error: null });
    try {
      await api.installModEngine(archivePath);
      set(s => ({
        status: s.status ? { ...s.status, mod_engine_installed: true } : null,
        loading: false,
      }));
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },
}));
