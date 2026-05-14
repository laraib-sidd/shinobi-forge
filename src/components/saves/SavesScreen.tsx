import { Button } from '@/components/ui/button';
import BackupCard from './BackupCard';
import { api } from '../../lib/tauri';
import { useAppStore } from '../../stores/useAppStore';

export default function SavesScreen() {
  const { backups, backupSave, importSave, error } = useAppStore();

  const doBackup = async () => {
    const label = window.prompt('Backup label (leave blank for timestamp):', '') ?? undefined;
    await backupSave(label || undefined);
  };

  const doImport = async () => {
    const file = await api.pickArchive();
    if (file && typeof file === 'string') await importSave(file);
  };

  return (
    <div>
      <h1 className="text-xl font-semibold mb-4">Save Files</h1>
      <div className="flex gap-3 mb-6">
        <Button onClick={doBackup} className="bg-accent hover:bg-red-700">
          Backup Current Save
        </Button>
        <Button variant="outline" onClick={doImport}>
          Import NG+ Save
        </Button>
      </div>
      {error && (
        <div className="bg-red-900/40 border border-red-700 rounded-lg px-4 py-2 mb-4 text-red-300 text-sm">
          {error}
        </div>
      )}
      {backups.length === 0 ? (
        <p className="text-zinc-500 text-sm">No backups yet. Click Backup to create one.</p>
      ) : (
        <div className="space-y-2">
          {[...backups].reverse().map(b => <BackupCard key={b.id} backup={b} />)}
        </div>
      )}
    </div>
  );
}
