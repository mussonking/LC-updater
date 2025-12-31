import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { motion, AnimatePresence } from 'framer-motion';
import { RefreshCw, CheckCircle, Copy, ExternalLink, Info } from 'lucide-react';
import './Updater.css';

const Updater: React.FC = () => {
    const [installPath, setInstallPath] = useState('');
    const [step, setStep] = useState(1);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState('');

    useEffect(() => {
        const init = async () => {
            setIsLoading(true);
            try {
                // 1. Get path
                const path = await invoke<string>('get_install_path');
                setInstallPath(path);

                // 2. Trigger initial check/download
                await invoke('check_and_update', {
                    manifestUrl: 'https://dev.leclasseur.ca/static/extension/version.json'
                });
            } catch (err) {
                console.error("[Updater] Init error:", err);
                setError('Impossible de contacter le serveur de mise à jour.');
            } finally {
                setIsLoading(false);
            }
        };
        init();

        const interval = setInterval(async () => {
            try {
                await invoke('check_and_update', {
                    manifestUrl: 'https://dev.leclasseur.ca/static/extension/version.json'
                });
            } catch (err) {
                console.error("[Updater] Polling error:", err);
            }
        }, 10 * 60 * 1000);

        return () => clearInterval(interval);
    }, []);

    const copyPath = async () => {
        await writeText(installPath);
    };

    const openChrome = async () => {
        await invoke('open_chrome_extensions');
    };

    return (
        <div className="updater-container">
            <header className="updater-header">
                <img src="/tauri.svg" alt="LeClasseur" className="app-icon" />
                <h1>LeClasseur Extension</h1>
                <div className="version-badge">v3.2.0</div>
            </header>

            <AnimatePresence mode="wait">
                {step === 1 && (
                    <motion.div
                        key="step1"
                        initial={{ opacity: 0, x: 20 }}
                        animate={{ opacity: 1, x: 0 }}
                        exit={{ opacity: 0, x: -20 }}
                        className="step-card"
                    >
                        <div className="step-icon"><Info size={32} /></div>
                        <h2>Installation Initiale</h2>

                        {isLoading ? (
                            <div className="loading-state">
                                <RefreshCw size={24} className="spin" />
                                <p>Préparation des fichiers de l'extension...</p>
                            </div>
                        ) : error ? (
                            <div className="error-state">
                                <p style={{ color: '#ef4444' }}>{error}</p>
                                <button className="secondary-btn" onClick={() => window.location.reload()}>Réessayer</button>
                            </div>
                        ) : (
                            <>
                                <p>Pour commencer, nous devons copier les fichiers de l'extension sur votre ordinateur.</p>

                                <div className="path-box">
                                    <code>{installPath}</code>
                                    <button onClick={copyPath} title="Copier le chemin"><Copy size={16} /></button>
                                </div>

                                <button className="primary-btn" onClick={() => setStep(2)}>
                                    Continuer <ExternalLink size={18} />
                                </button>
                            </>
                        )}
                    </motion.div>
                )}

                {step === 2 && (
                    <motion.div
                        key="step2"
                        initial={{ opacity: 0, x: 20 }}
                        animate={{ opacity: 1, x: 0 }}
                        exit={{ opacity: 0, x: -20 }}
                        className="step-card"
                    >
                        <h2>Étape 1 : Mode Développeur</h2>
                        <p>Ouvrez Chrome et activez le mode développeur en haut à droite.</p>
                        <div className="tutorial-img-placeholder">
                            <img src="/tutorial_dev_mode.png" alt="Developer Mode Guide" />
                        </div>
                        <button className="secondary-btn" onClick={openChrome}>Ouvrir Chrome Extensions</button>
                        <button className="primary-btn" onClick={() => setStep(3)}>C'est fait !</button>
                    </motion.div>
                )}

                {step === 3 && (
                    <motion.div
                        key="step3"
                        initial={{ opacity: 0, x: 20 }}
                        animate={{ opacity: 1, x: 0 }}
                        exit={{ opacity: 0, x: -20 }}
                        className="step-card"
                    >
                        <h2>Étape 2 : Charger l'extension</h2>
                        <p>Cliquez sur "Charger l'extension non empaquetée" et collez le chemin copié précédemment.</p>
                        <div className="tutorial-img-placeholder">
                            <img src="/tutorial_load_unpacked.png" alt="Load Unpacked Guide" />
                        </div>
                        <button className="primary-btn" onClick={() => setStep(4)}>Terminer l'installation</button>
                    </motion.div>
                )}

                {step === 4 && (
                    <motion.div
                        key="ready"
                        initial={{ opacity: 0, scale: 0.9 }}
                        animate={{ opacity: 1, scale: 1 }}
                        className="status-card success"
                    >
                        <CheckCircle size={64} color="#10b981" />
                        <h2>Extension Connectée</h2>
                        <p>L'updater surveille maintenant les mises à jour en arrière-plan.</p>
                        <div className="polling-indicator">
                            <RefreshCw size={16} className="spin" /> Vérification toutes les 10 min
                        </div>
                    </motion.div>
                )}
            </AnimatePresence>

            <footer className="updater-footer">
                <p>© 2025 LeClasseur - Advanced Agentic Coding</p>
            </footer>
        </div>
    );
};

export default Updater;
