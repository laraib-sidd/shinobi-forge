import { Button } from '@/components/ui/button';
import { useAppStore } from '../../stores/useAppStore';

interface Props { onDone: () => void; }

export default function StepReady({ onDone }: Props) {
  const { status } = useAppStore();
  return (
    <div className="space-y-4">
      <h2 className="text-lg font-semibold">Ready!</h2>
      <ul className="text-sm space-y-1 text-zinc-300">
        <li>Sekiro: <span className="text-green-400">{status?.sekiro_path}</span></li>
        <li>ModEngine: <span className="text-green-400">{status?.mod_engine_installed ? 'Installed' : 'Missing'}</span></li>
      </ul>
      <Button onClick={onDone} className="bg-accent hover:bg-red-700">
        Start Managing Mods
      </Button>
    </div>
  );
}
