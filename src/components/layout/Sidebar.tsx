import { Home, Archive, Settings } from 'lucide-react';

type Screen = 'dashboard' | 'saves' | 'settings';

interface Props {
  active: Screen;
  onNavigate: (s: Screen) => void;
}

const items: { id: Screen; label: string; icon: typeof Home }[] = [
  { id: 'dashboard', label: 'Mods', icon: Home },
  { id: 'saves', label: 'Saves', icon: Archive },
  { id: 'settings', label: 'Settings', icon: Settings },
];

export default function Sidebar({ active, onNavigate }: Props) {
  return (
    <nav className="w-16 bg-zinc-900 border-r border-zinc-800 flex flex-col items-center py-4 gap-2">
      <div className="w-8 h-8 bg-accent rounded mb-4" title="Shinobi Forge" />
      {items.map(({ id, label, icon: Icon }) => (
        <button
          key={id}
          onClick={() => onNavigate(id)}
          title={label}
          className={`w-10 h-10 rounded flex items-center justify-center transition-colors ${
            active === id
              ? 'bg-accent text-white'
              : 'text-zinc-400 hover:bg-zinc-800 hover:text-zinc-100'
          }`}
        >
          <Icon size={18} />
        </button>
      ))}
    </nav>
  );
}
