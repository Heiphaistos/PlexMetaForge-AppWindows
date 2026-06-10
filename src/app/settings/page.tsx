'use client';

import { useEffect, useState } from 'react';
import type { AppSettings } from '@/lib/types';
import { getSettings, saveSettings, testPlexConnection } from '@/lib/commands';

export default function SettingsPage() {
  const [form, setForm] = useState<AppSettings>({
    plex_url: 'http://localhost:32400',
    plex_token: '',
    custom_plugins_dir: '',
    custom_db_path: '',
  });
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [testing, setTesting] = useState(false);
  const [saved, setSaved] = useState(false);
  const [testResult, setTestResult] = useState<{ ok: boolean; msg: string } | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    getSettings()
      .then((s) => setForm({ ...s, custom_plugins_dir: s.custom_plugins_dir ?? '', custom_db_path: s.custom_db_path ?? '' }))
      .catch(() => {})
      .finally(() => setLoading(false));
  }, []);

  const set = <K extends keyof AppSettings>(k: K, v: AppSettings[K]) =>
    setForm((p) => ({ ...p, [k]: v }));

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault();
    setSaving(true); setError(null); setSaved(false);
    try {
      await saveSettings({
        ...form,
        custom_plugins_dir: form.custom_plugins_dir?.trim() || undefined,
        custom_db_path: form.custom_db_path?.trim() || undefined,
      } as AppSettings);
      setSaved(true);
      setTimeout(() => setSaved(false), 3000);
    } catch (e) { setError(String(e)); }
    finally { setSaving(false); }
  };

  const handleTest = async () => {
    // Sauvegarde d'abord, puis teste
    setSaving(true);
    try {
      await saveSettings({
        ...form,
        custom_plugins_dir: form.custom_plugins_dir?.trim() || undefined,
        custom_db_path: form.custom_db_path?.trim() || undefined,
      } as AppSettings);
    } catch { /* ignore */ }
    setSaving(false);

    setTesting(true); setTestResult(null);
    try {
      const msg = await testPlexConnection();
      setTestResult({ ok: true, msg: `✓ Connecté : ${msg}` });
    } catch (e) {
      setTestResult({ ok: false, msg: String(e) });
    } finally { setTesting(false); }
  };

  if (loading) return (
    <div className="flex items-center justify-center h-full text-plex-muted text-sm">Chargement…</div>
  );

  return (
    <div className="p-6 max-w-xl space-y-6">
      <h1 className="text-xl font-bold text-plex-text">⚙ Paramètres</h1>

      <form onSubmit={handleSave} className="space-y-5">

        {/* Connexion Plex */}
        <section className="bg-plex-surface border border-plex-border rounded p-4 space-y-4">
          <h2 className="text-sm font-semibold text-plex-muted uppercase tracking-wide">
            Connexion Plex Media Server
          </h2>

          <Field
            label="URL du serveur Plex"
            value={form.plex_url}
            onChange={(v) => set('plex_url', v)}
            placeholder="http://localhost:32400"
            hint="Laisse localhost:32400 si Plex est sur ce PC. Sinon mets l'IP de ta machine."
          />

          <div className="space-y-1">
            <label className="text-xs text-plex-muted">
              Token Plex (X-Plex-Token){' '}
              <span className="text-plex-muted opacity-60">— comment l'obtenir ci-dessous</span>
            </label>
            <input
              type="password"
              value={form.plex_token}
              onChange={(e) => set('plex_token', e.target.value)}
              placeholder="Colle ton token ici…"
              className="w-full bg-plex-bg border border-plex-border rounded px-3 py-2 text-sm font-mono text-plex-text placeholder-plex-muted focus:outline-none focus:border-plex-accent"
            />
            <div className="bg-plex-bg border border-plex-border rounded p-3 text-xs text-plex-muted space-y-1.5 mt-1">
              <div className="font-semibold text-plex-text">Comment trouver ton token :</div>
              <div>1. Ouvre Plex Web : <code className="text-plex-accent">http://localhost:32400/web</code></div>
              <div>2. Clique sur n'importe quel film → icône <strong>⋮</strong> → <strong>Voir info XML</strong></div>
              <div>3. Dans l'URL du navigateur, copie la valeur après <code className="text-plex-accent">X-Plex-Token=</code></div>
              <div className="pt-1 border-t border-plex-border">Ou via PowerShell (copie dans ton terminal) :</div>
              <code className="block bg-plex-surface p-2 rounded text-xs break-all select-all text-green-400">
                {'(Get-Content "$env:LOCALAPPDATA\\Plex Media Server\\Preferences.xml") -match \'PlexOnlineToken="([^"]+)"\''}
              </code>
            </div>
          </div>

          {/* Test connexion */}
          <button
            type="button"
            onClick={handleTest}
            disabled={testing || saving || !form.plex_token.trim()}
            className="w-full py-2 bg-plex-border text-plex-text text-sm rounded hover:bg-plex-accent hover:text-black disabled:opacity-50 transition-colors font-medium"
          >
            {testing ? 'Test en cours…' : '🔌 Tester la connexion Plex'}
          </button>

          {testResult && (
            <div className={`text-sm rounded p-3 border ${
              testResult.ok
                ? 'text-green-400 bg-green-900/20 border-green-800/40'
                : 'text-red-400 bg-red-900/20 border-red-800/40'
            }`}>
              {testResult.msg}
            </div>
          )}
        </section>

        {/* Chemins personnalisés */}
        <section className="bg-plex-surface border border-plex-border rounded p-4 space-y-4">
          <h2 className="text-sm font-semibold text-plex-muted uppercase tracking-wide">
            Chemins (optionnel — auto-détectés si vides)
          </h2>

          <Field
            label="Dossier Plug-ins (personnalisé)"
            value={form.custom_plugins_dir ?? ''}
            onChange={(v) => set('custom_plugins_dir', v)}
            placeholder="%LOCALAPPDATA%\Plex Media Server\Plug-ins\"
            hint="Laisse vide pour utiliser le dossier Plex standard."
          />

          <Field
            label="Chemin base de données SQLite (personnalisé)"
            value={form.custom_db_path ?? ''}
            onChange={(v) => set('custom_db_path', v)}
            placeholder="%LOCALAPPDATA%\Plex Media Server\Plug-in Support\Databases\*.db"
            hint="Laisse vide pour utiliser la DB Plex standard."
          />
        </section>

        {error && (
          <div className="text-sm text-red-400 bg-red-900/20 border border-red-800/40 rounded p-3">
            {error}
          </div>
        )}

        <button
          type="submit"
          disabled={saving}
          className="w-full py-2.5 bg-plex-accent text-black font-bold text-sm rounded hover:bg-yellow-400 disabled:opacity-50 transition-colors"
        >
          {saving ? 'Sauvegarde…' : saved ? '✓ Sauvegardé' : 'Sauvegarder les paramètres'}
        </button>
      </form>

      <div className="text-xs text-plex-muted border-t border-plex-border pt-4 space-y-1">
        <div>Les paramètres sont enregistrés dans :</div>
        <code className="text-plex-accent">%APPDATA%\PlexMetaForge\settings.json</code>
      </div>
    </div>
  );
}

function Field({
  label, value, onChange, placeholder, hint,
}: {
  label: string;
  value: string;
  onChange: (v: string) => void;
  placeholder?: string;
  hint?: string;
}) {
  return (
    <div className="space-y-1">
      <label className="text-xs text-plex-muted">{label}</label>
      <input
        type="text"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        className="w-full bg-plex-bg border border-plex-border rounded px-3 py-2 text-sm text-plex-text placeholder-plex-muted focus:outline-none focus:border-plex-accent"
      />
      {hint && <p className="text-xs text-plex-muted opacity-70">{hint}</p>}
    </div>
  );
}
