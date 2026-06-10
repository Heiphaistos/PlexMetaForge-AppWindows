'use client';

import { useEffect, useState, useCallback } from 'react';
import type { InstallResult, StorePlugin } from '@/lib/types';
import { getStoreCatalog, installStorePlugin, getInstalledPluginIds } from '@/lib/commands';

const CATEGORIES = [
  'Tous', 'Métadonnées', 'Sous-titres', 'Scanner',
  'Musique', 'Audiobooks', 'Comics', 'IPTV', 'Podcast', 'Sync', 'Utilitaires',
];

export default function StorePage() {
  const [plugins, setPlugins] = useState<StorePlugin[]>([]);
  const [installed, setInstalled] = useState<Set<string>>(new Set());
  const [category, setCategory] = useState('Tous');
  const [search, setSearch] = useState('');
  const [loading, setLoading] = useState(true);
  const [installing, setInstalling] = useState<string | null>(null);
  const [results, setResults] = useState<Record<string, { ok: boolean; msg: string }>>({});

  const loadData = useCallback(async () => {
    setLoading(true);
    try {
      const [catalog, ids] = await Promise.all([getStoreCatalog(), getInstalledPluginIds()]);
      setPlugins(catalog);
      setInstalled(new Set(ids));
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { loadData(); }, [loadData]);

  const handleInstall = async (plugin: StorePlugin) => {
    setInstalling(plugin.id);
    setResults((r) => ({ ...r, [plugin.id]: { ok: false, msg: '' } }));
    try {
      const res: InstallResult = await installStorePlugin(plugin.zip_url, plugin.bundle_name);
      const msg = res.already_existed
        ? `✓ Mis à jour : ${res.bundle_name}`
        : `✓ Installé : ${res.bundle_name}`;
      setResults((r) => ({ ...r, [plugin.id]: { ok: true, msg } }));
      // Refresh liste installés
      getInstalledPluginIds().then((ids) => setInstalled(new Set(ids)));
    } catch (e) {
      setResults((r) => ({ ...r, [plugin.id]: { ok: false, msg: String(e) } }));
    } finally {
      setInstalling(null);
    }
  };

  const filtered = plugins.filter((p) => {
    const matchCat = category === 'Tous' || p.category === category;
    const q = search.toLowerCase();
    const matchSearch =
      !q ||
      p.name.toLowerCase().includes(q) ||
      p.description.toLowerCase().includes(q) ||
      p.tags.some((t) => t.toLowerCase().includes(q));
    return matchCat && matchSearch;
  });

  // Normalise l'ID pour comparer avec les bundles installés
  const isInstalled = (plugin: StorePlugin) => {
    const norm = (s: string) => s.toLowerCase().replace(/[-_.bundle\s]/g, '');
    return [...installed].some((id) => norm(id).includes(norm(plugin.id)) || norm(plugin.bundle_name).includes(norm(id)));
  };

  return (
    <div className="p-6 space-y-5">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-xl font-bold text-plex-text">Catalogue de plugins</h1>
          <p className="text-xs text-plex-muted mt-0.5">
            {plugins.length} plugins open-source vérifiés — installation 1-clic depuis GitHub
          </p>
        </div>
        <button onClick={loadData}
          className="text-xs px-3 py-1.5 rounded bg-plex-surface border border-plex-border text-plex-muted hover:text-plex-text transition-colors">
          ↻ Actualiser
        </button>
      </div>

      {/* Search + filtre catégorie */}
      <div className="space-y-2">
        <input
          type="text"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Rechercher un plugin…"
          className="w-full bg-plex-surface border border-plex-border rounded px-3 py-2 text-sm text-plex-text placeholder-plex-muted focus:outline-none focus:border-plex-accent"
        />
        <div className="flex gap-1 flex-wrap">
          {CATEGORIES.map((c) => (
            <button key={c} onClick={() => setCategory(c)}
              className={`text-xs px-3 py-1 rounded transition-colors ${
                category === c
                  ? 'bg-plex-accent text-black font-semibold'
                  : 'bg-plex-surface border border-plex-border text-plex-muted hover:text-plex-text'
              }`}>
              {c}
            </button>
          ))}
        </div>
      </div>

      {loading ? (
        <div className="text-plex-muted text-sm text-center py-16">Chargement…</div>
      ) : filtered.length === 0 ? (
        <div className="text-plex-muted text-sm text-center py-16">Aucun plugin trouvé.</div>
      ) : (
        <div className="grid grid-cols-1 gap-3">
          {filtered.map((plugin) => {
            const inst = isInstalled(plugin);
            const res = results[plugin.id];
            const busy = installing === plugin.id;

            return (
              <div key={plugin.id}
                className="bg-plex-surface border border-plex-border rounded p-4 space-y-3">
                {/* Top row */}
                <div className="flex items-start gap-3">
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 flex-wrap">
                      <span className="text-sm font-bold text-plex-text">{plugin.name}</span>
                      {plugin.verified && (
                        <span className="text-xs px-1.5 py-0.5 rounded bg-green-900/30 text-green-400 border border-green-800/40">
                          ✓ Vérifié
                        </span>
                      )}
                      {inst && (
                        <span className="text-xs px-1.5 py-0.5 rounded bg-blue-900/30 text-blue-400 border border-blue-800/40">
                          Installé
                        </span>
                      )}
                      <span className="text-xs px-1.5 py-0.5 rounded bg-plex-bg border border-plex-border text-plex-muted">
                        {plugin.category}
                      </span>
                    </div>
                    <div className="text-xs text-plex-muted mt-0.5">
                      par <span className="text-plex-text">{plugin.author}</span>
                      {' · '}⭐ {plugin.stars}
                      {' · '}{plugin.license}
                    </div>
                  </div>

                  {/* Actions */}
                  <div className="flex gap-2 flex-shrink-0">
                    <a href={plugin.github_url} target="_blank" rel="noreferrer"
                      className="text-xs px-2 py-1.5 rounded bg-plex-bg border border-plex-border text-plex-muted hover:text-plex-text transition-colors">
                      GitHub
                    </a>
                    <button
                      onClick={() => handleInstall(plugin)}
                      disabled={busy || installing !== null}
                      className={`text-xs px-3 py-1.5 rounded font-semibold transition-colors disabled:opacity-50 ${
                        inst
                          ? 'bg-blue-900/40 text-blue-400 hover:bg-blue-900/60'
                          : 'bg-plex-accent text-black hover:bg-yellow-400'
                      }`}>
                      {busy ? '⬇ Installation…' : inst ? '↻ Mettre à jour' : '⬇ Installer'}
                    </button>
                  </div>
                </div>

                {/* Description */}
                <p className="text-xs text-plex-muted leading-relaxed">{plugin.description}</p>

                {/* Tags */}
                <div className="flex gap-1 flex-wrap">
                  {plugin.tags.map((tag) => (
                    <span key={tag}
                      className="text-xs px-2 py-0.5 rounded-full bg-plex-bg border border-plex-border text-plex-muted">
                      {tag}
                    </span>
                  ))}
                </div>

                {/* Résultat install */}
                {res && res.msg && (
                  <div className={`text-xs rounded p-2 ${
                    res.ok
                      ? 'text-green-400 bg-green-900/20 border border-green-800/40'
                      : 'text-red-400 bg-red-900/20 border border-red-800/40'
                  }`}>
                    {res.msg}
                  </div>
                )}

                {/* Bundle name */}
                <div className="text-xs font-mono text-plex-muted opacity-60">
                  → {plugin.bundle_name}
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
