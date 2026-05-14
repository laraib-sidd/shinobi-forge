import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { api } from '../../lib/tauri';
import { useAppStore } from '../../stores/useAppStore';

interface Props { onNext: () => void; }

export default function StepCheckModEngine({ onNext }: Props) {
  const { status, installModEngine } = useAppStore();
  const [error, setError] = useState('');

  const pickAndInstall = async () => {
    setError('');
    const file = await api.pickArchive();
    if (!file || typeof file !== 'string') return;
    try {
      await installModEngine(file);
      onNext();
    } catch (e) {
      setError(String(e));
    }
  };

  if (status?.mod_engine_installed) {
    return (
      <div className="space-y-4">
        <h2 className="text-lg font-semibold">Step 2 — ModEngine</h2>
        <p className="text-green-400 text-sm">ModEngine detected.</p>
        <Button onClick={onNext} className="bg-accent hover:bg-red-700">Continue</Button>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <h2 className="text-lg font-semibold">Step 2 — Install ModEngine</h2>
      <p className="text-zinc-400 text-sm">
        ModEngine was not found. Download it from Nexus Mods and select the zip here.
      </p>
      <Button onClick={pickAndInstall} className="bg-accent hover:bg-red-700">
        Select ModEngine ZIP
      </Button>
      {error && <p className="text-red-400 text-sm">{error}</p>}
    </div>
  );
}
