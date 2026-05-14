import { Switch } from '@/components/ui/switch';
import { Button } from '@/components/ui/button';
import { Trash2 } from 'lucide-react';
import type { InstalledMod } from '../../lib/types';
import { useAppStore } from '../../stores/useAppStore';

const TYPE_COLOR: Record<string, string> = {
  content: 'bg-blue-900 text-blue-300',
  dll: 'bg-purple-900 text-purple-300',
  hybrid: 'bg-orange-900 text-orange-300',
  unknown: 'bg-zinc-700 text-zinc-300',
};

const CAT_COLOR: Record<string, string> = {
  overhaul: 'bg-red-900 text-red-300',
  ai: 'bg-green-900 text-green-300',
  qol: 'bg-cyan-900 text-cyan-300',
  cosmetic: 'bg-zinc-700 text-zinc-300',
  unknown: 'bg-zinc-700 text-zinc-300',
};

interface Props { mod: InstalledMod; }

export default function ModCard({ mod }: Props) {
  const { toggleMod, uninstallMod } = useAppStore();

  const confirmUninstall = () => {
    if (window.confirm(`Uninstall "${mod.name}"? This will remove all its files.`)) {
      uninstallMod(mod.id);
    }
  };

  return (
    <div className="bg-zinc-900 border border-zinc-800 rounded-lg px-4 py-3 flex items-center gap-4">
      <Switch
        checked={mod.enabled}
        onCheckedChange={enabled => toggleMod(mod.id, enabled)}
      />
      <div className="flex-1 min-w-0">
        <p className="font-medium text-zinc-100 truncate">{mod.name}</p>
        <div className="flex gap-2 mt-1">
          <span className={`text-xs px-2 py-0.5 rounded ${TYPE_COLOR[mod.mod_type]}`}>
            {mod.mod_type}
          </span>
          <span className={`text-xs px-2 py-0.5 rounded ${CAT_COLOR[mod.category]}`}>
            {mod.category}
          </span>
        </div>
      </div>
      <span className="text-xs text-zinc-500 shrink-0">
        {new Date(mod.installed_at).toLocaleDateString()}
      </span>
      <Button variant="ghost" size="icon" onClick={confirmUninstall} className="text-zinc-500 hover:text-red-400">
        <Trash2 size={16} />
      </Button>
    </div>
  );
}
