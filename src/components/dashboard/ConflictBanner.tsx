import { X } from 'lucide-react';
import { useState } from 'react';
import type { Conflict } from '../../lib/types';

interface Props { conflicts: Conflict[]; }

export default function ConflictBanner({ conflicts }: Props) {
  const [dismissed, setDismissed] = useState(false);
  if (!conflicts.length || dismissed) return null;

  return (
    <div className="bg-amber-900/50 border border-amber-600 rounded-lg px-4 py-3 mb-4 flex items-start gap-3">
      <div className="flex-1 text-sm text-amber-200 space-y-1">
        {conflicts.map((c, i) => <p key={i}>{c.message}</p>)}
      </div>
      <button onClick={() => setDismissed(true)} className="text-amber-400 hover:text-amber-200 shrink-0">
        <X size={16} />
      </button>
    </div>
  );
}
