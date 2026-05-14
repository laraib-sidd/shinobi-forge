import { Button } from '@/components/ui/button';
import { Plus } from 'lucide-react';
import { api } from '../../lib/tauri';
import { useAppStore } from '../../stores/useAppStore';

export default function InstallButton() {
  const { installMod } = useAppStore();

  const pick = async () => {
    const file = await api.pickArchive();
    if (file && typeof file === 'string') await installMod(file);
  };

  const onDrop = async (e: React.DragEvent) => {
    e.preventDefault();
    const file = e.dataTransfer.files[0];
    if (file) {
      const path = (file as File & { path?: string }).path ?? '';
      if (path) await installMod(path);
    }
  };

  return (
    <div
      onDrop={onDrop}
      onDragOver={e => e.preventDefault()}
      className="border-2 border-dashed border-zinc-700 rounded-lg p-4 flex items-center gap-4 mb-4 hover:border-zinc-500 transition-colors"
    >
      <Button onClick={pick} className="bg-accent hover:bg-red-700 shrink-0">
        <Plus size={16} className="mr-2" /> Install Mod
      </Button>
      <span className="text-sm text-zinc-500">or drag a .zip / .7z archive here</span>
    </div>
  );
}
