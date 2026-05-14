import { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { api } from '../../lib/tauri';
import { useAppStore } from '../../stores/useAppStore';

interface Props { onNext: () => void; }

export default function StepFindSekiro({ onNext }: Props) {
  const [path, setPath] = useState('');
  const [checking, setChecking] = useState(true);
  const [error, setError] = useState('');
  const { setSekiroPath } = useAppStore();

  useEffect(() => {
    api.detectSekiroPath().then(p => {
      if (p) setPath(p);
      setChecking(false);
    });
  }, []);

  const browse = async () => {
    const selected = await api.pickFolder();
    if (selected && typeof selected === 'string') setPath(selected);
  };

  const confirm = async () => {
    setError('');
    try {
      await setSekiroPath(path);
      onNext();
    } catch {
      setError('sekiro.exe not found at that path. Try again.');
    }
  };

  return (
    <div className="space-y-4">
      <h2 className="text-lg font-semibold">Step 1 — Find Sekiro</h2>
      {checking ? (
        <p className="text-zinc-400 text-sm">Auto-detecting...</p>
      ) : path ? (
        <p className="text-green-400 text-sm">Found: {path}</p>
      ) : (
        <p className="text-zinc-400 text-sm">Not found automatically. Browse manually.</p>
      )}
      <div className="flex gap-2">
        <input
          value={path}
          onChange={e => setPath(e.target.value)}
          placeholder="C:\...\Sekiro"
          className="flex-1 bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm text-zinc-100"
        />
        <Button variant="outline" onClick={browse}>Browse...</Button>
      </div>
      {error && <p className="text-red-400 text-sm">{error}</p>}
      <Button onClick={confirm} disabled={!path} className="bg-accent hover:bg-red-700">
        Confirm Path
      </Button>
    </div>
  );
}
