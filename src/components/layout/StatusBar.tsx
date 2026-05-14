import { Badge } from '@/components/ui/badge';
import { useAppStore } from '../../stores/useAppStore';

export default function StatusBar() {
  const { status } = useAppStore();
  if (!status) return null;

  return (
    <div className="flex items-center gap-4 py-2 px-1 border-b border-zinc-800 mb-4 text-sm text-zinc-400">
      <span className="truncate max-w-xs" title={status.sekiro_path}>
        {status.sekiro_path ?? 'No path set'}
      </span>
      <Badge variant={status.mod_engine_installed ? 'default' : 'destructive'} className="shrink-0">
        {status.mod_engine_installed ? 'ModEngine OK' : 'ModEngine Missing'}
      </Badge>
      <span className="ml-auto shrink-0">
        {status.active_mod_count}/{status.total_mod_count} mods active
      </span>
    </div>
  );
}
