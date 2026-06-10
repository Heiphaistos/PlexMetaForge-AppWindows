import { invoke } from '@tauri-apps/api/core';
import type {
  AppSettings,
  BatchUpdateResult,
  DatabaseStats,
  ExportResult,
  InjectionReport,
  MediaItem,
  MetadataPayload,
  Plugin,
  PlexPaths,
  PlexSection,
  PluginConfig,
  PluginTemplateMeta,
} from './types';

// Settings
export const getSettings = (): Promise<AppSettings> =>
  invoke('get_settings');

export const saveSettings = (newSettings: AppSettings): Promise<void> =>
  invoke('save_settings', { newSettings });

export const testPlexConnection = (): Promise<string> =>
  invoke('test_plex_connection');

// Config
export const getPlexPaths = (): Promise<PlexPaths> =>
  invoke('get_plex_paths');

// Scanner
export const listPlugins = (): Promise<Plugin[]> =>
  invoke('list_plugins');

export const togglePlugin = (path: string, enable: boolean): Promise<string> =>
  invoke('toggle_plugin', { path, enable });

export const deletePlugin = (path: string): Promise<void> =>
  invoke('delete_plugin', { path });

// Generator
export const createPlugin = (name: string): Promise<string> =>
  invoke('create_plugin', { name });

export const readPluginCode = (path: string): Promise<string> =>
  invoke('read_plugin_code', { path });

export const writePluginCode = (path: string, content: string): Promise<void> =>
  invoke('write_plugin_code', { path, content });

// Database
export const searchPlexDb = (query: string): Promise<MediaItem[]> =>
  invoke('search_plex_db', { query });

// Export
export const exportPlugin = (path: string, destDir?: string): Promise<ExportResult> =>
  invoke('export_plugin', { path, destDir });

export const exportAllPlugins = (destDir?: string): Promise<ExportResult> =>
  invoke('export_all_plugins', { destDir });

export const getExportDir = (): Promise<string> =>
  invoke('get_export_dir');

// Database
export const getDbStats = (): Promise<DatabaseStats> =>
  invoke('get_db_stats');

export const getDbSections = (): Promise<PlexSection[]> =>
  invoke('get_db_sections');

export const getItemsBySection = (sectionId: number, limit?: number): Promise<MediaItem[]> =>
  invoke('get_items_by_section', { sectionId, limit });

export const getIncompleteItems = (limit?: number): Promise<MediaItem[]> =>
  invoke('get_incomplete_items', { limit });

export const dbBatchClearLocks = (): Promise<BatchUpdateResult> =>
  invoke('db_batch_clear_locks');

// Generator — templates
export const getPluginTemplates = (): Promise<PluginTemplateMeta[]> =>
  invoke('get_plugin_templates');

export const createPluginFromTemplate = (config: PluginConfig): Promise<string> =>
  invoke('create_plugin_from_template', { config });

// Module C — Moteur Hybride
export const injectMetadata = (payload: MetadataPayload): Promise<InjectionReport> =>
  invoke('inject_metadata', { payload });
