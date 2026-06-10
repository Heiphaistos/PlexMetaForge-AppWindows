import type { Metadata } from 'next';
import Link from 'next/link';
import '@/styles/globals.css';

export const metadata: Metadata = {
  title: 'PlexMetaForge',
  description: 'Plex plugin manager & metadata injector',
};

const NAV_LINKS = [
  { href: '/dashboard',  label: 'Plugins',          icon: '🔌' },
  { href: '/editor',     label: 'Métadonnées',       icon: '🎬' },
  { href: '/database',   label: 'Base de données',   icon: '🗄️' },
  { href: '/code-editor',label: 'Code',              icon: '📝' },
  { href: '/settings',   label: 'Paramètres',        icon: '⚙️' },
];

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="fr">
      <body className="flex h-screen overflow-hidden">
        {/* Sidebar */}
        <aside className="w-52 flex-shrink-0 bg-plex-surface border-r border-plex-border flex flex-col">
          {/* Logo */}
          <div
            data-tauri-drag-region
            className="h-12 flex items-center px-4 border-b border-plex-border"
          >
            <span className="text-plex-accent font-bold text-lg tracking-tight">
              PlexMeta<span className="text-plex-text">Forge</span>
            </span>
          </div>

          {/* Nav */}
          <nav className="flex-1 p-3 space-y-1">
            {NAV_LINKS.map(({ href, label, icon }) => (
              <Link
                key={href}
                href={href}
                className="flex items-center gap-2 px-3 py-2 rounded text-sm text-plex-text hover:bg-plex-border transition-colors"
              >
                <span>{icon}</span>
                <span>{label}</span>
              </Link>
            ))}
          </nav>

          {/* Version */}
          <div className="px-4 py-3 text-xs text-plex-muted border-t border-plex-border">
            v1.0.0
          </div>
        </aside>

        {/* Main content */}
        <main className="flex-1 overflow-auto bg-plex-bg">
          {children}
        </main>
      </body>
    </html>
  );
}
