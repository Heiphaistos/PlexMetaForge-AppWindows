/** Pourcentage 0-100 ; null si la taille totale est inconnue. */
export function progressPercent(downloaded: number, total: number): number | null {
  if (!total || total <= 0) return null;
  return Math.min(100, Math.floor((downloaded / total) * 100));
}
