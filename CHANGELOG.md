# Journal des modifications

## 1.5.0

- Mise à jour automatique (Tauri updater) : signature minisign vérifiée, installation silencieuse, relance.
- Vérification silencieuse au démarrage, désactivable (Paramètres, « Vérifier les mises à jour automatiquement »).
- Bouton « Rechercher une mise à jour » dans les paramètres, avec dialogue (version, notes) et progression.
- Version portable : pas de mise à jour automatique, l'application renvoie vers la page des versions.
- Workflow `release.yml` (tag `v*`) : installeur, signature et `latest.json` joints à la release.
