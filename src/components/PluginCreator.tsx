'use client';

import { useState, useEffect } from 'react';
import type { PluginConfig, PluginTemplateId, PluginTemplateMeta } from '@/lib/types';
import { getPluginTemplates, createPluginFromTemplate } from '@/lib/commands';
import SelectiveBuilder from './SelectiveBuilder';

interface Props {
  onCreated: (path: string) => void;
}

type Tab = 'template' | 'selective';

export default function PluginCreator({ onCreated }: Props) {
  const [activeTab, setActiveTab] = useState<Tab>('template');
  const [templates, setTemplates] = useState<PluginTemplateMeta[]>([]);
  const [selected, setSelected] = useState<PluginTemplateId>('universal');
  const [name, setName] = useState('');
  const [tmdbKey, setTmdbKey] = useState('');
  const [lastfmKey, setLastfmKey] = useState('');
  const [creating, setCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  useEffect(() => {
    getPluginTemplates().then(setTemplates).catch(() => {});
  }, []);

  const current = templates.find((t) => t.id === selected);

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) { setError('Nom requis.'); return; }
    setCreating(true);
    setError(null);
    setSuccess(null);

    const config: PluginConfig = {
      name: name.trim(),
      template: selected,
      tmdb_api_key: tmdbKey.trim() || undefined,
      lastfm_api_key: lastfmKey.trim() || undefined,
    };

    try {
      const path = await createPluginFromTemplate(config);
      setSuccess(path);
      setName('');
      onCreated(path);
    } catch (e) {
      setError(String(e));
    } finally {
      setCreating(false);
    }
  };

  return (
    <div className="space-y-5">
      {/* Tabs */}
      <div className="flex gap-1 bg-plex-bg border border-plex-border rounded p-1">
        {([
          { id: 'template',  icon: '📄', label: 'Depuis un modèle' },
          { id: 'selective', icon: '🔧', label: 'Plugin personnalisé' },
        ] as { id: Tab; icon: string; label: string }[]).map((t) => (
          <button
            key={t.id}
            type="button"
            onClick={() => setActiveTab(t.id)}
            className={`flex-1 text-xs py-2 rounded font-semibold transition-colors ${
              activeTab === t.id
                ? 'bg-plex-accent text-black'
                : 'text-plex-muted hover:text-plex-text'
            }`}
          >
            {t.icon} {t.label}
          </button>
        ))}
      </div>

      {activeTab === 'selective' ? (
        <SelectiveBuilder onCreated={onCreated} />
      ) : (
      <>
      <h2 className="text-sm font-semibold text-plex-muted uppercase tracking-wide">
        Créer un plugin depuis un modèle
      </h2>

      {/* Template grid */}
      <div className="grid grid-cols-3 gap-2">
        {templates.map((t) => (
          <button
            key={t.id}
            type="button"
            onClick={() => setSelected(t.id)}
            className={`text-left p-3 rounded border transition-colors ${
              selected === t.id
                ? 'border-plex-accent bg-plex-accent/10 text-plex-text'
                : 'border-plex-border bg-plex-bg text-plex-muted hover:border-plex-muted'
            }`}
          >
            <div className="text-xl mb-1">{t.icon}</div>
            <div className="text-xs font-semibold">{t.label}</div>
          </button>
        ))}
      </div>

      {/* Description du template sélectionné */}
      {current && (
        <div className="bg-plex-bg border border-plex-border rounded p-3 text-xs text-plex-muted space-y-1">
          <div className="text-plex-text font-medium">
            {current.icon} {current.label}
          </div>
          <p>{current.description}</p>
          <div className="flex gap-3 pt-1">
            {current.requires_tmdb && (
              <span className="px-2 py-0.5 rounded bg-blue-900/30 text-blue-400 border border-blue-800/40">
                Clé TMDB requise
              </span>
            )}
            {current.requires_lastfm && (
              <span className="px-2 py-0.5 rounded bg-purple-900/30 text-purple-400 border border-purple-800/40">
                Clé Last.fm requise
              </span>
            )}
            {!current.requires_tmdb && !current.requires_lastfm && (
              <span className="px-2 py-0.5 rounded bg-green-900/30 text-green-400 border border-green-800/40">
                Aucune clé requise
              </span>
            )}
          </div>
        </div>
      )}

      {/* Formulaire */}
      <form onSubmit={handleCreate} className="space-y-3">
        <div className="space-y-1">
          <label className="text-xs text-plex-muted">Nom du plugin *</label>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="MonAgent, CinemaFR, AnimePerso…"
            className="w-full bg-plex-bg border border-plex-border rounded px-3 py-2 text-sm text-plex-text placeholder-plex-muted focus:outline-none focus:border-plex-accent"
          />
        </div>

        {current?.requires_tmdb && (
          <div className="space-y-1">
            <label className="text-xs text-blue-400">
              Clé API TMDB{' '}
              <a
                href="https://www.themoviedb.org/settings/api"
                target="_blank"
                rel="noreferrer"
                className="underline opacity-70 hover:opacity-100"
              >
                (obtenir)
              </a>
            </label>
            <input
              type="text"
              value={tmdbKey}
              onChange={(e) => setTmdbKey(e.target.value)}
              placeholder="Optionnelle — peut être ajoutée plus tard"
              className="w-full bg-plex-bg border border-blue-800/40 rounded px-3 py-2 text-sm text-plex-text placeholder-plex-muted focus:outline-none focus:border-blue-500 font-mono"
            />
          </div>
        )}

        {current?.requires_lastfm && (
          <div className="space-y-1">
            <label className="text-xs text-purple-400">
              Clé API Last.fm{' '}
              <a
                href="https://www.last.fm/api/account/create"
                target="_blank"
                rel="noreferrer"
                className="underline opacity-70 hover:opacity-100"
              >
                (obtenir)
              </a>
            </label>
            <input
              type="text"
              value={lastfmKey}
              onChange={(e) => setLastfmKey(e.target.value)}
              placeholder="Optionnelle — peut être ajoutée plus tard"
              className="w-full bg-plex-bg border border-purple-800/40 rounded px-3 py-2 text-sm text-plex-text placeholder-plex-muted focus:outline-none focus:border-purple-500 font-mono"
            />
          </div>
        )}

        {error && (
          <div className="text-sm text-red-400 bg-red-900/20 border border-red-800/40 rounded p-2">
            {error}
          </div>
        )}

        {success && (
          <div className="text-xs text-green-400 bg-green-900/20 border border-green-800/40 rounded p-2 font-mono break-all">
            ✓ Créé : {success}
          </div>
        )}

        <button
          type="submit"
          disabled={creating || !name.trim()}
          className="w-full py-2 bg-plex-accent text-black font-bold text-sm rounded hover:bg-yellow-400 disabled:opacity-50 transition-colors"
        >
          {creating ? 'Génération en cours…' : `Générer "${name || '…'}" (${current?.label ?? ''})`}
        </button>
      </form>

      {/* Fichiers qui seront générés */}
      {current && (
        <div className="text-xs text-plex-muted border-t border-plex-border pt-3 space-y-1">
          <div className="font-semibold text-plex-muted">Fichiers générés :</div>
          <div className="font-mono space-y-0.5 ml-2">
            <div>{name || 'MonPlugin'}.bundle/</div>
            <div className="ml-3">Contents/</div>
            <div className="ml-6">Info.plist</div>
            <div className="ml-6">DefaultPrefs.json</div>
            <div className="ml-6">Code/</div>
            <div className="ml-9">__init__.py</div>
            <div className="ml-6">Resources/</div>
            <div className="ml-9">README.txt</div>
            {current.id === 'universal' && (
              <>
                <div className="ml-3 text-plex-accent">→ 4 classes d'agents</div>
                <div className="ml-3 text-plex-accent">→ Films + Séries + Anime + Musique</div>
              </>
            )}
          </div>
        </div>
      )}
      </>
      )}
    </div>
  );
}
