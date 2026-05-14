import { useEffect, useState } from 'react';
import { useAppStore } from './stores/useAppStore';
import Sidebar from './components/layout/Sidebar';
import DashboardScreen from './components/dashboard/DashboardScreen';
import SavesScreen from './components/saves/SavesScreen';
import SettingsScreen from './components/settings/SettingsScreen';
import SetupWizard from './components/wizard/SetupWizard';

type Screen = 'dashboard' | 'saves' | 'settings';

export default function App() {
  const { status, loadAll } = useAppStore();
  const [screen, setScreen] = useState<Screen>('dashboard');

  useEffect(() => { loadAll(); }, [loadAll]);

  const showWizard = status !== null && status.first_run;

  return (
    <div className="flex h-screen bg-zinc-950 text-zinc-100 overflow-hidden">
      <Sidebar active={screen} onNavigate={setScreen} />
      <main className="flex-1 overflow-y-auto p-6">
        {screen === 'dashboard' && <DashboardScreen />}
        {screen === 'saves' && <SavesScreen />}
        {screen === 'settings' && <SettingsScreen />}
      </main>
      {showWizard && <SetupWizard />}
    </div>
  );
}
