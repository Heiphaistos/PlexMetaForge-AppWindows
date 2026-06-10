'use client';

import { useState } from 'react';
import type { SelectiveConfig } from '@/lib/types';
import { createSelectivePlugin } from '@/lib/commands';

interface Props {
  onCreated: (path: string) => void;
}

const AGENTS = [
  { key: 'films',        label: '🎬 Films',              desc: 'Agent.Movies — TMDB', needsTmdb: true,   needsLastfm: false },
  { key: 'series',       label: '📺 Séries TV',          desc: 'Agent.TV_Shows — TMDB', needsTmdb: true, needsLastfm: false },
  { key: 'anime',        label: '⛩️ Anime / Manga',      desc: 'AniList (sans clé) — auto-détection dans Films+Séries', needsTmdb: false, needsLastfm: false },
  { key: 'music_artist', label: '🎵 Musique — Artiste',  desc: 'Agent.Artist — Last.fm', needsTmdb: false, needsLastfm: true },
  { key: 'music_album',  label: '🎵 Musique — Album',    desc: 'Agent.Album — Last.fm', needsTmdb: false, needsLastfm: true },
] as const;

type AgentKey = typeof AGENTS[number]['key'];

const DEFAULT: Omit<SelectiveConfig, 'name'> = {
  films: true, series: true, anime: true,
  music_artist: true, music_album: true,
  use_tmdb: true, use_anilist: true, use_lastfm: true,
  tmdb_key: '', lastfm_key: '',
};

export default function SelectiveBuilder({ onCreated }: Props) {
  const [name, setName] = useState('MonAgentUniversel');
  const [cfg, setCfg] = useState(DEFAULT);
  const [creating, setCreating] = useState(false);
  const [success, setSuccess] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const toggle = (key: AgentKey) =>
    setCfg((c) => ({ ...c, [key]: !c[key] }));

  const needsTmdb = AGENTS.some((a) => a.needsTmdb && cfg[a.key as AgentKey]);
  const needsLastfm = AGENTS.some((a) => a.needsLastfm && cfg[a.key as AgentKey]);
  const anySelected = AGENTS.some((a) => cfg[a.key as AgentKey]);

  const agentCount = AGENTS.filter((a) => cfg[a.key as AgentKey]).length;

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim() || !anySelected) return;
    setCreating(true); setError(null); setSuccess(null);
    try {
      const path = await createSelectivePlugin({
        name: name.trim(),
        ...cfg,
        use_tmdb: cfg.use_tmdb && needsTmdb,
        use_anilist: cfg.anime,
        use_lastfm: cfg.use_lastfm && needsLastfm,
        tmdb_key: cfg.tmdb_key?.trim() || undefined,
        lastfm_key: cfg.lastfm_key?.trim() || undefined,
      });
      setSuccess(path);
      onCreated(path);
    } catch (e) {
      setError(String(e));
    } finally {
      setCreating(false);
    }
  };

  return (
    <form onSubmit={handleCreate} className="space-y-5">
      <div className="space-y-1">
        <label className="text-xs text-plex-muted">Nom du plugin *</label>
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="MonAgentUniversel"
          className="w-full bg-plex-bg border border-plex-border rounded px-3 py-2 text-sm text-plex-text placeholder-plex-muted focus:outline-none focus:border-plex-accent"
        />
      </div>

      {/* Sélection des agents */}
      <div className="space-y-2">
        <label className="text-xs text-plex-muted font-semibold uppercase tracking-wide">
          Agents à inclure
        </label>
        <div className="space-y-1.5">
          {AGENTS.map((agent) => {
            const checked = cfg[agent.key as AgentKey] as boolean;
            return (
              <label
                key={agent.key}
                className={`flex items-start gap-3 p-3 rounded border cursor-pointer transition-colors ${
                  checked
                    ? 'bg-plex-accent/10 border-plex-accent/40'
                    : 'bg-plex-bg border-plex-border hover:border-plex-muted'
                }`}
              >
                <input
                  type="checkbox"
                  checked={checked}
                  onChange={() => toggle(agent.key as AgentKey)}
                  className="mt-0.5 accent-yellow-400"
                />
                <div>
                  <div className="text-sm font-medium text-plex-text">{agent.label}</div>
                  <div className="text-xs text-plex-muted">{agent.desc}</div>
                </div>
              </label>
            );
          })}
        </div>
      </div>

      {/* Clés API conditionnelles */}
      {needsTmdb && (
        <div className="space-y-1">
          <label className="text-xs text-blue-400">
            Clé TMDB{' '}
            <a href="https://www.themoviedb.org/settings/api" target="_blank" rel="noreferrer"
              className="underline opacity-70">(obtenir)</a>
          </label>
          <input
            type="text"
            value={cfg.tmdb_key ?? ''}
            onChange={(e) => setCfg((c) => ({ ...c, tmdb_key: e.target.value }))}
            placeholder="Optionnelle — injectable plus tard dans le code"
            className="w-full bg-plex-bg border border-blue-800/40 rounded px-3 py-2 text-sm text-plex-text placeholder-plex-muted focus:outline-none focus:border-blue-500 font-mono"
          />
        </div>
      )}
      {needsLastfm && (
        <div className="space-y-1">
          <label className="text-xs text-purple-400">
            Clé Last.fm{' '}
            <a href="https://www.last.fm/api/account/create" target="_blank" rel="noreferrer"
              className="underline opacity-70">(obtenir)</a>
          </label>
          <input
            type="text"
            value={cfg.lastfm_key ?? ''}
            onChange={(e) => setCfg((c) => ({ ...c, lastfm_key: e.target.value }))}
            placeholder="Optionnelle — injectable plus tard dans le code"
            className="w-full bg-plex-bg border border-purple-800/40 rounded px-3 py-2 text-sm text-plex-text placeholder-plex-muted focus:outline-none focus:border-purple-500 font-mono"
          />
        </div>
      )}

      {/* Aperçu */}
      {anySelected && (
        <div className="bg-plex-bg border border-plex-border rounded p-3 text-xs space-y-1">
          <div className="font-semibold text-plex-text">Aperçu — {agentCount} classe{agentCount > 1 ? 's' : ''} générée{agentCount > 1 ? 's' : ''} :</div>
          <div className="font-mono text-plex-muted space-y-0.5">
            {cfg.films && <div className="text-green-400">class {name.replace(' ','_')}Movies(Agent.Movies)</div>}
            {cfg.series && <div className="text-blue-400">class {name.replace(' ','_')}Shows(Agent.TV_Shows)</div>}
            {cfg.music_artist && <div className="text-purple-400">class {name.replace(' ','_')}Artist(Agent.Artist)</div>}
            {cfg.music_album && <div className="text-purple-400">class {name.replace(' ','_')}Album(Agent.Album)</div>}
            {cfg.anime && <div className="text-orange-400">  → Détection anime automatique dans Films+Séries</div>}
          </div>
          <div className="text-plex-muted mt-1">
            Sources : {[cfg.films||cfg.series ? 'TMDB' : '', cfg.anime ? 'AniList' : '', cfg.music_artist||cfg.music_album ? 'Last.fm' : ''].filter(Boolean).join(' + ')}
          </div>
        </div>
      )}

      {!anySelected && (
        <div className="text-sm text-orange-400 text-center">Sélectionne au moins un agent.</div>
      )}

      {error && <div className="text-sm text-red-400 bg-red-900/20 border border-red-800/40 rounded p-2">{error}</div>}
      {success && <div className="text-xs text-green-400 bg-green-900/20 border border-green-800/40 rounded p-2 font-mono break-all">✓ {success}</div>}

      <button
        type="submit"
        disabled={creating || !name.trim() || !anySelected}
        className="w-full py-2.5 bg-plex-accent text-black font-bold text-sm rounded hover:bg-yellow-400 disabled:opacity-50 transition-colors"
      >
        {creating ? 'Génération…' : `Générer "${name || '…'}" (${agentCount} agent${agentCount > 1 ? 's' : ''})`}
      </button>
    </form>
  );
}
