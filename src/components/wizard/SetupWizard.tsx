import { useState } from 'react';
import StepFindSekiro from './StepFindSekiro';
import StepCheckModEngine from './StepCheckModEngine';
import StepReady from './StepReady';
import { useAppStore } from '../../stores/useAppStore';

const STEPS = ['Find Sekiro', 'ModEngine', 'Ready'];

export default function SetupWizard() {
  const [step, setStep] = useState(0);
  const { loadAll } = useAppStore();

  const done = async () => {
    await loadAll();
  };

  return (
    <div className="fixed inset-0 bg-black/70 flex items-center justify-center z-50">
      <div className="bg-zinc-900 border border-zinc-700 rounded-lg p-8 w-[480px] space-y-6">
        <div className="flex gap-2 mb-2">
          {STEPS.map((_label, i) => (
            <div key={i} className="flex items-center gap-2">
              <div className={`w-6 h-6 rounded-full flex items-center justify-center text-xs font-bold ${
                i <= step ? 'bg-accent text-white' : 'bg-zinc-700 text-zinc-400'
              }`}>{i + 1}</div>
              {i < STEPS.length - 1 && <div className="w-8 h-px bg-zinc-700" />}
            </div>
          ))}
        </div>
        {step === 0 && <StepFindSekiro onNext={() => setStep(1)} />}
        {step === 1 && <StepCheckModEngine onNext={() => setStep(2)} />}
        {step === 2 && <StepReady onDone={done} />}
      </div>
    </div>
  );
}
