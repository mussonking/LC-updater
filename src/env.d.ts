/// <reference types="vite/client" />

interface ImportMetaEnv {
    /** URL du manifest de version de l'extension Chrome */
    readonly VITE_MANIFEST_URL: string;
    /** URL de base du serveur */
    readonly VITE_BASE_URL: string;
}

interface ImportMeta {
    readonly env: ImportMetaEnv;
}
