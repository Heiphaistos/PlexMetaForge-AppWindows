'use client';

import { useState } from 'react';
import Link from 'next/link';
import MetadataForm from '@/components/MetadataForm';
import type { MediaItem } from '@/lib/types';
import { searchPlexDb } from '@/lib/commands';

export default function EditorPage() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<MediaItem[]>([]);
  const [selected, setSelected] = useState<MediaItem | null>(null);
  const [searching, setSearching] = useState(false);
  const [searchError, setSearchError] = useState<string | null>(null);

  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!query.trim()) return;
    setSearching(true); setSearchError(null);
    try { setResults(await searchPlexDb(query.trim())); }
    catch (e) { setSearchError(String(e)); }
    finally { setSearching(false); }
  };

  return (
    <div className="p-6 space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-bold text-plex-text">Éditeur de Métadonnées</h1>
        <Link href="/settings"
          className="text-xs px-3 py-1.5 rounded bg-plex-surface border border-plex-border text-plex-muted hover:text-plex-text transition-colors">
          ⚙ Configurer Plex
        </Link>
      </div>

      {/* Search */}
      <div className="bg-plex-surface border border-plex-border rounded p-4 space-y-3">
        <h2 className="text-sm font-semibold text-plex-muted uppercase tracking-wide">
          Rechercher dans la base Plex
        </h2>
        <form onSubmit={handleSearch} className="flex gap-2">
          <input type="text" value={query} onChange={(e) => setQuery(e.target.value)}
            placeholder="Titre du média…"
            className="flex-1 bg-plex-bg border border-plex-border rounded px-3 py-2 text-sm text-plex-text placeholder-plex-muted focus:outline-none focus:border-plex-accent" />
          <button type="submit" disabled={searching || !query.trim()}
            className="px-4 py-2 bg-plex-accent text-black font-semibold text-sm rounded hover:bg-yellow-400 disabled:opacity-50 transition-colors">
            {searching ? '…' : 'Chercher'}
          </button>
        </form>
        {searchError && <div className="text-sm text-red-400">{searchError}</div>}
        {results.length > 0 && (
          <ul className="space-y-1 max-h-48 overflow-y-auto">
            {results.map((item) => (
              <li key={item.id} onClick={() => setSelected(item)}
                className={`px-3 py-2 rounded text-sm cursor-pointer transition-colors ${
                  selected?.id === item.id
                    ? 'bg-plex-accent/20 text-plex-accent border border-plex-accent/40'
                    : 'hover:bg-plex-border text-plex-text'}`}>
                {item.title}
                {item.year && <span className="ml-2 text-plex-muted">({item.year})</span>}
                {item.media_type && <span className="ml-2 text-xs text-plex-muted">{item.media_type}</span>}
              </li>
            ))}
          </ul>
        )}
      </div>

      <MetadataForm selected={selected} />
    </div>
  );
}
