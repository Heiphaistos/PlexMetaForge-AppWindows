'use client';

import { useEffect, useState } from 'react';
import { getSettings } from '@/lib/commands';
import { checkForUpdate } from '@/lib/updater';

/** Vérification silencieuse au démarrage (si le réglage est actif) + bandeau de progression. */
export default function UpdateBoot() {
  const [percent, setPercent] = useState<number | null | undefined>(undefined);

  useEffect(() => {
    if (process.env.NODE_ENV === 'development' || !('__TAURI_INTERNALS__' in window)) return;
    getSettings()
      .then((s) => (s.auto_update_check ? checkForUpdate(true, setPercent) : false))
      .catch(() => {})
      .finally(() => setPercent(undefined));
  }, []);

  if (percent === undefined) return null;
  return (
    <div className="fixed bottom-3 right-3 bg-plex-surface border border-plex-border rounded px-4 py-2 text-sm text-plex-text shadow">
      {percent === null ? 'Téléchargement de la mise à jour…' : `Téléchargement de la mise à jour… ${percent} %`}
    </div>
  );
}
