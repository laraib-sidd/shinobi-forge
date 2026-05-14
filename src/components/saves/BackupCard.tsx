import { Button } from '@/components/ui/button';
import type { SaveBackup } from '../../lib/types';
import { useAppStore } from '../../stores/useAppStore';

interface Props { backup: SaveBackup; }

function formatBytes(bytes: number) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}

export default function BackupCard({ backup }: Props) {
  const { restoreSave } = useAppStore();

  const confirm = () => {
    if (window.confirm('This will overwrite your current save. Continue?')) {
      restoreSave(backup.id);
    }
  };

  return (
    <div className="bg-zinc-900 border border-zinc-800 rounded-lg px-4 py-3 flex items-center gap-4">
      <div className="flex-1 min-w-0">
        <p className="font-medium text-zinc-100 truncate">{backup.label}</p>
        <p className="text-xs text-zinc-500 mt-0.5">
          {new Date(backup.created_at).toLocaleString()} · {formatBytes(backup.size_bytes)}
        </p>
      </div>
      <Button variant="outline" size="sm" onClick={confirm}>Restore</Button>
    </div>
  );
}
