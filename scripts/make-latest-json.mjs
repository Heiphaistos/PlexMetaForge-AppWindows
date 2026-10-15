// Assemble latest.json (manifeste de mise a jour Tauri) apres un build signe.
//   node scripts/make-latest-json.mjs [dossier-nsis] [fichier-notes.md]
import { readFileSync, writeFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

const { version } = JSON.parse(readFileSync('package.json', 'utf8'));
const dir = process.argv[2] ?? 'src-tauri/target/release/bundle/nsis';
const notes = process.argv[3] ? readFileSync(process.argv[3], 'utf8').trim() : `PlexMetaForge ${version}`;
const exe = readdirSync(dir).find((f) => f.endsWith('-setup.exe') && f.includes(`_${version}_`));
if (!exe) throw new Error(`Installeur ${version} introuvable dans ${dir}`);

const manifest = {
  version,
  notes,
  pub_date: new Date().toISOString(),
  platforms: {
    'windows-x86_64': {
      signature: readFileSync(join(dir, `${exe}.sig`), 'utf8').trim(),
      url: `https://github.com/heiphaistos44-crypto/PlexMetaForge/releases/download/v${version}/${encodeURIComponent(exe)}`,
    },
  },
};
writeFileSync(join(dir, 'latest.json'), JSON.stringify(manifest, null, 2));
console.log(`ecrit ${join(dir, 'latest.json')}`);
