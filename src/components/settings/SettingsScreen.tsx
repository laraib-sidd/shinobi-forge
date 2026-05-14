import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { api } from '../../lib/tauri';
import { useAppStore } from '../../stores/useAppStore';

export default function SettingsScreen() {
  const { status, setSekiroPath, installModEngine } = useAppStore();
  const [path, setPath] = useState(status?.sekiro_path ?? '');
  const [saved, setSaved] = useState(false);

  const browsePath = async () => {
    const dir = await api.pickFolder();
    if (dir && typeof dir === 'string') setPath(dir);
  };

  const autoDetect = async () => {
    const detected = await api.detectSekiroPath();
    if (detected) setPath(detected);
  };

  const savePath = async () => {
    await setSekiroPath(path);
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  };

  const pickModEngine = async () => {
    const file = await api.pickArchive();
    if (file && typeof file === 'string') await installModEngine(file);
  };

  return (
    <div className="max-w-lg space-y-8">
      <h1 className="text-xl font-semibold">Settings</h1>

      <section className="space-y-3">
        <h2 className="text-sm font-semibold text-zinc-400 uppercase tracking-wider">
          Sekiro Install Path
        </h2>
        <div className="flex gap-2">
          <input
            value={path}
            onChange={e => setPath(e.target.value)}
            className="flex-1 bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm text-zinc-100"
          />
          <Button variant="outline" onClick={browsePath}>Browse</Button>
          <Button variant="outline" onClick={autoDetect}>Auto-detect</Button>
        </div>
        <Button onClick={savePath} className="bg-accent hover:bg-red-700">
          {saved ? 'Saved!' : 'Save Path'}
        </Button>
      </section>

      <section className="space-y-3">
        <h2 className="text-sm font-semibold text-zinc-400 uppercase tracking-wider">ModEngine</h2>
        <p className="text-sm text-zinc-400">
          Status:{' '}
          {status?.mod_engine_installed
            ? <span className="text-green-400">Installed</span>
            : <span className="text-red-400">Not detected</span>}
        </p>
        {!status?.mod_engine_installed && (
          <Button variant="outline" onClick={pickModEngine}>Install from ZIP</Button>
        )}
      </section>

      <section className="space-y-2">
        <h2 className="text-sm font-semibold text-zinc-400 uppercase tracking-wider">About</h2>
        <p className="text-sm text-zinc-500">Shinobi Forge v0.1.0</p>
      </section>
    </div>
  );
}
