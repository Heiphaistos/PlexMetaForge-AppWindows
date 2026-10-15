import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { ask, message } from '@tauri-apps/plugin-dialog';
import { open } from '@tauri-apps/plugin-shell';
import { isInstalledBuild } from './commands';
import { progressPercent } from './updateProgress';

const RELEASES = 'https://github.com/heiphaistos44-crypto/PlexMetaForge/releases/latest';
const TITLE = 'Mise à jour';
let running = false;

/**
 * @param silent true = rien d'affiché s'il n'y a rien / canal injoignable (démarrage).
 * @param onProgress pourcentage de téléchargement (null = inconnu).
 * @returns true si une mise à jour a été installée (relance en cours).
 */
export async function checkForUpdate(
  silent = true,
  onProgress?: (percent: number | null) => void,
): Promise<boolean> {
  // En dev le binaire n'est pas installé ; hors Tauri (navigateur) il n'y a rien à remplacer.
  if (process.env.NODE_ENV === 'development' || !('__TAURI_INTERNALS__' in window) || running) {
    return false;
  }
  running = true;
  try {
    // Copie portable : l'updater lancerait l'installeur NSIS au lieu de remplacer le fichier.
    if (!(await isInstalledBuild())) {
      if (!silent) {
        const go = await ask(
          'Cette copie est la version portable : elle ne peut pas se mettre à jour toute seule.\n\n' +
            'Télécharge la dernière version portable depuis la page des versions et remplace ce fichier.',
          { title: TITLE, kind: 'info', okLabel: 'Ouvrir la page', cancelLabel: 'Fermer' },
        );
        if (go) await open(RELEASES);
      }
      return false;
    }

    const update = await check();
    if (!update) {
      if (!silent) await message('PlexMetaForge est à jour.', { title: TITLE });
      return false;
    }
    const ok = await ask(
      `PlexMetaForge ${update.version} est disponible (version actuelle : ${update.currentVersion}).` +
        (update.body ? `\n\nNouveautés :\n${update.body}` : '') +
        "\n\nInstaller maintenant ? L'application redémarrera.",
      { title: 'Nouvelle version disponible', kind: 'info', okLabel: 'Installer', cancelLabel: 'Plus tard' },
    );
    if (!ok) return false;

    let downloaded = 0;
    let total = 0;
    onProgress?.(0);
    await update.downloadAndInstall((ev) => {
      if (ev.event === 'Started') total = ev.data.contentLength ?? 0;
      else if (ev.event === 'Progress') {
        downloaded += ev.data.chunkLength;
        onProgress?.(progressPercent(downloaded, total));
      }
    });
    await relaunch();
    return true;
  } catch (e) {
    // Canal injoignable, signature refusée… : on garde la version actuelle.
    console.error('[maj] échec', e);
    if (!silent) await message(`Vérification impossible : ${String(e)}`, { title: TITLE, kind: 'error' });
    return false;
  } finally {
    running = false;
  }
}
