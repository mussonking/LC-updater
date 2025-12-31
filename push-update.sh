#!/bin/bash
# Script pour pousser la mise à jour sur le serveur DEV

# 1. Zipper l'extension (exclure git et node_modules)
zip -r extension.zip . -x "*.git*" "node_modules/*"

# 2. Copier vers le dossier statique du serveur
cp extension.zip /votre/chemin/vers/Z/static/extension/extension.zip
cp version.json /votre/chemin/vers/Z/static/extension/version.json

echo "Update pushed successfully!"
