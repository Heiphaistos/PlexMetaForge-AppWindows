'use client';

import { useState, useEffect } from 'react';
import type { InjectionReport, MediaItem, MetadataPayload } from '@/lib/types';
import { injectMetadata } from '@/lib/commands';

interface Props {
  selected: MediaItem | null;
}

type FormData = Omit<MetadataPayload, 'year' | 'rating'> & { year: string; rating: string };

const EMPTY: FormData = {
  title: '',
  year: '',
  plot: '',
  poster_url: '',
  fanart_url: '',
  tmdb_id: '',
  imdb_id: '',
  tagline: '',
  studio: '',
  rating: '',
  media_path: '',
};

export default function MetadataForm({ selected }: Props) {
  const [form, setForm] = useState<FormData>(EMPTY);
  const [injecting, setInjecting] = useState(false);
  const [report, setReport] = useState<InjectionReport | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!selected) return;
    setReport(null);
    setError(null);
    setForm({
      title: selected.title,
      year: selected.year ? String(selected.year) : '',
      plot: selected.summary ?? '',
      poster_url: selected.thumb ?? '',
      fanart_url: '',
      tmdb_id: '',
      imdb_id: '',
      tagline: '',
      studio: '',
      rating: '',
      media_path: '',
    });
  }, [selected]);

  const set = <K extends keyof FormData>(key: K, value: FormData[K]) =>
    setForm((prev) => ({ ...prev, [key]: value }));

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!form.media_path.trim()) {
      setError('Le chemin du média est requis.');
      return;
    }
    setInjecting(true);
    setReport(null);
    setError(null);

    const payload: MetadataPayload = {
      ...form,
      year: form.year ? Number(form.year) : undefined,
      rating: form.rating ? Number(form.rating) : undefined,
    };

    try {
      const r = await injectMetadata(payload);
      setReport(r);
    } catch (e) {
      setError(String(e));
    } finally {
      setInjecting(false);
    }
  };

  return (
    <form
      onSubmit={handleSubmit}
      className="bg-plex-surface border border-plex-border rounded p-4 space-y-4"
    >
      <h2 className="text-sm font-semibold text-plex-muted uppercase tracking-wide">
        Métadonnées
      </h2>

      <div className="grid grid-cols-2 gap-3">
        <Field label="Titre" value={form.title} onChange={(v) => set('title', v)} />
        <Field
          label="Année"
          type="number"
          value={form.year}
          onChange={(v) => set('year', v)}
        />
      </div>

      <Field
        label="Synopsis"
        value={form.plot ?? ''}
        onChange={(v) => set('plot', v)}
        multiline
      />

      <div className="grid grid-cols-2 gap-3">
        <Field
          label="TMDB ID"
          value={form.tmdb_id ?? ''}
          onChange={(v) => set('tmdb_id', v)}
          placeholder="ex: 550"
        />
        <Field
          label="IMDB ID"
          value={form.imdb_id ?? ''}
          onChange={(v) => set('imdb_id', v)}
          placeholder="ex: tt0137523"
        />
      </div>

      <div className="grid grid-cols-2 gap-3">
        <Field
          label="URL Poster"
          value={form.poster_url ?? ''}
          onChange={(v) => set('poster_url', v)}
        />
        <Field
          label="URL Fanart"
          value={form.fanart_url ?? ''}
          onChange={(v) => set('fanart_url', v)}
        />
      </div>

      <Field
        label="Tagline"
        value={form.tagline ?? ''}
        onChange={(v) => set('tagline', v)}
        placeholder="ex: Rien n'est impossible."
      />

      <div className="grid grid-cols-2 gap-3">
        <Field
          label="Studio"
          value={form.studio ?? ''}
          onChange={(v) => set('studio', v)}
        />
        <Field
          label="Note (0-10)"
          type="number"
          value={form.rating ?? ''}
          onChange={(v) => set('rating', v)}
          placeholder="ex: 7.5"
        />
      </div>

      <Field
        label="Chemin du dossier média *"
        value={form.media_path}
        onChange={(v) => set('media_path', v)}
        placeholder={`C:\\Films\\MonFilm`}
      />

      {/* Injection report */}
      {report && <InjectionResult report={report} />}

      {error && (
        <div className="text-sm text-red-400 bg-red-900/20 border border-red-800/40 rounded p-2">
          {error}
        </div>
      )}

      <button
        type="submit"
        disabled={injecting}
        className="w-full py-2 bg-plex-accent text-black font-semibold text-sm rounded hover:bg-yellow-400 disabled:opacity-50 transition-colors"
      >
        {injecting ? 'Injection en cours…' : 'Injecter les métadonnées'}
      </button>
    </form>
  );
}

function InjectionResult({ report }: { report: InjectionReport }) {
  const steps: { label: string; ok: boolean }[] = [
    { label: 'NFO écrit', ok: report.nfo_written },
    { label: 'Poster sauvegardé', ok: report.poster_saved },
    { label: 'Fanart sauvegardé', ok: report.fanart_saved },
    { label: 'API Plex rafraîchie', ok: report.plex_api_refreshed },
    { label: 'SQLite mis à jour', ok: report.sqlite_updated },
  ];

  return (
    <div className="bg-plex-bg border border-plex-border rounded p-3 space-y-2">
      <div className="text-xs font-semibold text-plex-muted uppercase tracking-wide">
        Rapport d'injection
      </div>
      <ul className="space-y-1">
        {steps.map(({ label, ok }) => (
          <li key={label} className="flex items-center gap-2 text-sm">
            <span className={ok ? 'text-green-400' : 'text-plex-muted'}>
              {ok ? '✓' : '–'}
            </span>
            <span className={ok ? 'text-plex-text' : 'text-plex-muted'}>{label}</span>
          </li>
        ))}
      </ul>
      {report.errors.length > 0 && (
        <ul className="space-y-1 pt-1 border-t border-plex-border">
          {report.errors.map((err, i) => (
            <li key={i} className="text-xs text-orange-400">
              ⚠ {err}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}

interface FieldProps {
  label: string;
  value: string;
  onChange: (v: string) => void;
  type?: string;
  multiline?: boolean;
  placeholder?: string;
}

function Field({ label, value, onChange, type = 'text', multiline, placeholder }: FieldProps) {
  const base =
    'w-full bg-plex-bg border border-plex-border rounded px-3 py-2 text-sm text-plex-text placeholder-plex-muted focus:outline-none focus:border-plex-accent';
  return (
    <div className="space-y-1">
      <label className="text-xs text-plex-muted">{label}</label>
      {multiline ? (
        <textarea
          value={value}
          onChange={(e) => onChange(e.target.value)}
          placeholder={placeholder}
          rows={3}
          className={`${base} resize-none`}
        />
      ) : (
        <input
          type={type}
          value={value}
          onChange={(e) => onChange(e.target.value)}
          placeholder={placeholder}
          className={base}
        />
      )}
    </div>
  );
}
