export interface Plugin {
  name: string;
  path: string;
  enabled: boolean;
  has_code: boolean;
}

export interface MediaItem {
  id: number;
  title: string;
  year?: number;
  summary?: string;
  thumb?: string;
  media_type?: string;
  section_id?: number;
  duration?: number;
  rating?: number;
  tagline?: string;
  studio?: string;
  originally_available_at?: string;
}

export interface MetadataPayload {
  title: string;
  year?: number;
  plot?: string;
  poster_url?: string;
  fanart_url?: string;
  tmdb_id?: string;
  imdb_id?: string;
  tagline?: string;
  studio?: string;
  rating?: number;
  media_path: string;
}

export interface InjectionReport {
  nfo_written: boolean;
  poster_saved: boolean;
  fanart_saved: boolean;
  plex_api_refreshed: boolean;
  sqlite_updated: boolean;
  errors: string[];
}

export type MetadataStatus = 'complete' | 'incomplete' | 'missing';

export interface MediaNode {
  path: string;
  name: string;
  status: MetadataStatus;
  media_item?: MediaItem;
}

export type PluginTemplateId =
  | 'blank'
  | 'cinema'
  | 'series'
  | 'musique'
  | 'anime'
  | 'universal';

export interface PluginTemplateMeta {
  id: PluginTemplateId;
  label: string;
  icon: string;
  description: string;
  requires_tmdb: boolean;
  requires_lastfm: boolean;
}

export interface PluginConfig {
  name: string;
  template: PluginTemplateId;
  tmdb_api_key?: string;
  lastfm_api_key?: string;
}

export interface StorePlugin {
  id: string;
  name: string;
  author: string;
  description: string;
  category: string;
  subcategory: string;
  tags: string[];
  license: string;
  stars: string;
  github_url: string;
  zip_url: string;
  bundle_name: string;
  verified: boolean;
}

export interface SelectiveConfig {
  name: string;
  films: boolean;
  series: boolean;
  anime: boolean;
  music_artist: boolean;
  music_album: boolean;
  use_tmdb: boolean;
  use_anilist: boolean;
  use_lastfm: boolean;
  tmdb_key?: string;
  lastfm_key?: string;
}

export interface BatchInstallItem {
  bundle_name: string;
  ok: boolean;
  message: string;
}

export interface StoreCategory {
  name: string;
  subcategories: string[];
}

export interface InstallResult {
  bundle_path: string;
  bundle_name: string;
  already_existed: boolean;
}

export interface AppSettings {
  plex_url: string;
  plex_token: string;
  custom_plugins_dir?: string;
  custom_db_path?: string;
}

export interface ExportResult {
  zip_path: string;
  file_count: number;
  size_bytes: number;
}

export interface DatabaseStats {
  total_items: number;
  movies: number;
  shows: number;
  episodes: number;
  artists: number;
  albums: number;
  tracks: number;
  sections: number;
  items_with_thumb: number;
  items_without_summary: number;
}

export interface PlexSection {
  id: number;
  name: string;
  section_type: string;
  location?: string;
}

export interface BatchUpdateResult {
  updated: number;
  errors: string[];
}

export interface PlexPaths {
  plugins_dir: string;
  database_path: string;
  plugins_dir_exists: boolean;
  database_exists: boolean;
}
