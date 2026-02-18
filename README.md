# LeClasseur Updater

Application de mise à jour automatique pour l'extension Chrome LeClasseur.  
Construit avec **Tauri v2 + React + TypeScript**.

## Features

- 📦 Installe et met à jour l'extension Chrome automatiquement
- 🔄 Vérifie les nouvelles versions toutes les 10 minutes
- 🖥️ Fonctionne en arrière-plan (system tray)
- 🚀 Auto-update du programme lui-même via `tauri-plugin-updater`
- 🍎 Supporte **Windows** et **macOS** (Intel + Apple Silicon)

## Configuration

Copier `.env.example` vers `.env` et ajuster les URLs :
```bash
cp .env.example .env
```

| Variable | Description |
|---|---|
| `VITE_MANIFEST_URL` | URL du manifest de version de l'extension Chrome |
| `VITE_BASE_URL` | URL de base du serveur |

## Développement

```bash
npm install
npm run tauri dev
```

## Build local

```powershell
# 1. Set la clé de signature
$env:TAURI_SIGNING_PRIVATE_KEY = Get-Content "$env:USERPROFILE\.tauri\leclasseur.key" -Raw

# 2. Build
npm run tauri build
```

## Release (CI/CD)

Le projet utilise **GitHub Actions** pour builder automatiquement sur Windows + macOS.

### Setup initial (une seule fois)

1. Ajouter le secret `TAURI_SIGNING_PRIVATE_KEY` dans **Settings → Secrets → Actions** :
   ```
   Valeur = contenu de ~/.tauri/leclasseur.key
   ```

2. (Optionnel) Pour le déploiement automatique sur le serveur, ajouter aussi :
   - `DEPLOY_SSH_KEY` — Clé SSH privée
   - `DEPLOY_HOST` — `user@hostname`
   - `DEPLOY_PATH` — `/path/to/static/app-update`

### Publier une release

```bash
# 1. Bump la version dans src-tauri/tauri.conf.json
# 2. Commit et tag
git add -A
git commit -m "release: v1.1.0"
git tag v1.1.0
git push && git push --tags
```

GitHub Actions va automatiquement :
- ✅ Builder pour Windows (`.exe`) et macOS (`.app` universal)
- ✅ Signer les artefacts
- ✅ Créer une GitHub Release avec les fichiers
- ✅ Générer `update.json` pour l'auto-updater

## Clés de signature

| Fichier | Usage |
|---|---|
| `~/.tauri/leclasseur.key` | Clé **privée** — ⚠️ NE JAMAIS PARTAGER |
| `~/.tauri/leclasseur.key.pub` | Clé publique (dans `tauri.conf.json`) |

> **⚠️** Si tu perds la clé privée, tu ne pourras plus publier de mises à jour pour les clients existants.
