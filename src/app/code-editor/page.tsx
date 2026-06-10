'use client';

import { Suspense, useEffect, useState } from 'react';
import { useSearchParams } from 'next/navigation';
import CodeEditor from '@/components/CodeEditor';
import { readPluginCode, writePluginCode } from '@/lib/commands';

function CodeEditorInner() {
  const searchParams = useSearchParams();
  const pluginPath = searchParams.get('path') ?? '';

  const [code, setCode] = useState('');
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    if (!pluginPath) return;
    setLoading(true);
    readPluginCode(pluginPath)
      .then((content) => setCode(content))
      .catch((e) => setError(String(e)))
      .finally(() => setLoading(false));
  }, [pluginPath]);

  const handleSave = async () => {
    if (!pluginPath) return;
    setSaving(true);
    setError(null);
    try {
      await writePluginCode(pluginPath, code);
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (e) {
      setError(String(e));
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="p-6 flex flex-col h-full gap-4">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-xl font-bold text-plex-text">Éditeur de Code</h1>
          {pluginPath && (
            <p className="text-xs text-plex-muted mt-0.5 font-mono">{pluginPath}</p>
          )}
        </div>
        <div className="flex gap-2 items-center">
          {saved && <span className="text-xs text-green-400">Sauvegardé ✓</span>}
          {error && <span className="text-xs text-red-400">{error}</span>}
          <button
            onClick={handleSave}
            disabled={saving || !pluginPath}
            className="px-4 py-2 bg-plex-accent text-black font-semibold text-sm rounded hover:bg-yellow-400 disabled:opacity-50 transition-colors"
          >
            {saving ? 'Sauvegarde...' : 'Sauvegarder'}
          </button>
        </div>
      </div>

      {!pluginPath ? (
        <div className="flex-1 flex items-center justify-center text-plex-muted text-sm">
          Sélectionne un plugin depuis le tableau de bord pour éditer son code.
        </div>
      ) : loading ? (
        <div className="flex-1 flex items-center justify-center text-plex-muted text-sm">
          Chargement...
        </div>
      ) : (
        <div className="flex-1 min-h-0">
          <CodeEditor value={code} onChange={setCode} />
        </div>
      )}
    </div>
  );
}

export default function CodeEditorPage() {
  return (
    <Suspense
      fallback={
        <div className="flex items-center justify-center h-full text-plex-muted text-sm">
          Chargement...
        </div>
      }
    >
      <CodeEditorInner />
    </Suspense>
  );
}
