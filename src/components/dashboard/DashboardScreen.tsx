import StatusBar from '../layout/StatusBar';
import ConflictBanner from './ConflictBanner';
import ModCard from './ModCard';
import InstallButton from './InstallButton';
import { useAppStore } from '../../stores/useAppStore';

export default function DashboardScreen() {
  const { mods, conflicts, error } = useAppStore();

  return (
    <div>
      <StatusBar />
      <ConflictBanner conflicts={conflicts} />
      {error && (
        <div className="bg-red-900/40 border border-red-700 rounded-lg px-4 py-2 mb-4 text-red-300 text-sm">
          {error}
        </div>
      )}
      <InstallButton />
      {mods.length === 0 ? (
        <div className="text-center py-16 text-zinc-500">
          <p className="text-lg">No mods installed.</p>
          <p className="text-sm mt-1">Click Install Mod or drag an archive above.</p>
        </div>
      ) : (
        <div className="space-y-2">
          {mods.map(mod => <ModCard key={mod.id} mod={mod} />)}
        </div>
      )}
    </div>
  );
}
