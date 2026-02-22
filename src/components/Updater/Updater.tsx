import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { motion, AnimatePresence } from 'framer-motion';
import { RefreshCw, CheckCircle, Copy, ExternalLink, Info, Power, ChevronLeft } from 'lucide-react';
import './Updater.css';

const Updater: React.FC = () => {
    const [installPath, setInstallPath] = useState('');
    const [step, setStep] = useState(1);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState('');

    // Modal State
    const [modalOpen, setModalOpen] = useState(false);
    const [modalContent, setModalContent] = useState({ title: '', message: '', image: '' });

    const showModal = (title: string, message: string, image: string = '') => {
        setModalContent({ title, message, image });
        setModalOpen(true);
    };

    const quitApp = async () => {
        await invoke('quit_app');
    };

    const prevStep = () => {
        if (step > 1) setStep(step - 1);
    };

    useEffect(() => {
        const init = async () => {
            setIsLoading(true);
            try {
                // 1. Get path
                const path = await invoke<string>('get_install_path');
                setInstallPath(path);

                // 2. Trigger initial check/download
                await invoke('check_and_update', {
                    manifestUrl: import.meta.env.VITE_MANIFEST_URL
                });
            } catch (err: any) {
                console.error("[Updater] Init error:", err);
                const msg = typeof err === 'string' ? err : (err.message || JSON.stringify(err));
                setError(`Erreur: ${msg}`);
            } finally {
                setIsLoading(false);
            }
        };
        init();

        const interval = setInterval(async () => {
            try {
                await invoke('check_and_update', {
                    manifestUrl: import.meta.env.VITE_MANIFEST_URL
                });
            } catch (err) {
                console.error("[Updater] Polling error:", err);
            }
        }, 10 * 60 * 1000);

        let unlistenFn: (() => void) | undefined;
        const setupListener = async () => {
            unlistenFn = await listen('trigger-manual-update', async () => {
                console.log("[Updater] Received trigger-manual-update");
                setIsLoading(true);
                try {
                    await invoke('check_and_update', {
                        manifestUrl: import.meta.env.VITE_MANIFEST_URL
                    });
                } catch (err) {
                    console.error("[Updater] Manual update error:", err);
                } finally {
                    setIsLoading(false);
                }
            });
        };
        setupListener();

        return () => {
            clearInterval(interval);
            if (unlistenFn) unlistenFn();
        };
    }, []);

    const copyPath = async () => {
        try {
            await writeText(installPath);
            showModal(
                "Chemin copié !",
                "1. Cliquez dans la barre d'adresse en HAUT de la fenêtre qui vient de s'ouvrir.\n2. Collez (Ctrl+V) le chemin et appuyez sur Entrée.\n3. Cliquez sur le bouton 'Sélectionner un dossier' en BAS.",
                "/tutorial_paste_path2.png"
            );
        } catch (err) {
            console.error("Copy failed", err);
            showModal("Erreur", "Impossible de copier le chemin.");
        }
    };

    const openChromeAndCopyLink = async () => {
        try {
            // 1. Copy URL to clipboard
            await writeText('chrome://extensions/');

            // 2. Open Chrome (Generic launch)
            await invoke('open_chrome_extensions');

            // 3. User feedback
            showModal("Lien copié !", "1. L'adresse 'chrome://extensions/' est copiée.\n2. Chrome va s'ouvrir.\n3. Collez (Ctrl+V) dans la barre d'adresse.");
        } catch (err: any) {
            console.error("Failed to open Chrome", err);
            showModal("Erreur", "Erreur lors de l'ouverture de Chrome : " + (err.message || err));
        }
    };

    return (
        <div className="updater-container">
            <header className="updater-header">
                {step > 1 && step < 4 && (
                    <button className="back-btn" onClick={prevStep} title="Retour">
                        <ChevronLeft size={24} />
                    </button>
                )}
                <button className="quit-btn" onClick={quitApp} title="Quitter l'application">
                    <Power size={20} />
                </button>
                <img src="/tauri.svg" alt="LeClasseur" className="app-icon" />
                <h1>LeClasseur Extension</h1>
                <div className="version-badge">v3.3.0</div>
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
                                <p>Les fichiers ont été installés sur votre ordinateur.</p>
                                <p>Cliquez sur <b>Continuer</b> pour configurer Chrome.</p>

                                <div className="path-box" style={{ opacity: 0.7 }}>
                                    <code>{installPath}</code>
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
                        <h2>Étape 1 : Ouvrir les Extensions</h2>
                        <ol className="instructions-list">
                            <li>Cliquez sur le bouton ci-dessous.</li>
                            <li><b>Collez (Ctrl+V)</b> le lien dans la barre d'adresse de Chrome.</li>
                            <li>Activez le <b>Mode développeur</b> (en haut à droite).</li>
                        </ol>

                        <div className="tutorial-img-placeholder">
                            <img src="/tutorial_dev_mode.png" alt="Guide Mode Développeur" />
                        </div>

                        <button className="secondary-btn" onClick={openChromeAndCopyLink}>
                            Ouvrir Chrome + Copier Lien
                        </button>

                        <div className="divider"></div>

                        <button className="primary-btn" onClick={() => setStep(3)}>
                            C'est fait, étape suivante
                        </button>
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
                        <ol className="instructions-list">
                            <li>Dans Chrome, cliquez sur <b>Charger l'extension non empaquetée</b>.</li>
                            <li>Sélectionnez le dossier de l'extension.</li>
                        </ol>

                        <div className="path-action-box">
                            <p>Chemin à sélectionner :</p>
                            <div className="path-box">
                                <code>{installPath}</code>
                                <button onClick={copyPath} title="Copier le chemin" className="copy-btn-large">
                                    <Copy size={20} /> COPIER CE CHEMIN
                                </button>
                            </div>
                        </div>

                        <div className="tutorial-img-placeholder contain-img">
                            <img src="/tutorial_load_unpacked.png" alt="Guide Load Unpacked" />
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

            <AnimatePresence>
                {modalOpen && (
                    <motion.div
                        className="modal-overlay"
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        exit={{ opacity: 0 }}
                        onClick={() => setModalOpen(false)}
                    >
                        <motion.div
                            className="modal-content"
                            initial={{ scale: 0.9, y: 20 }}
                            animate={{ scale: 1, y: 0 }}
                            exit={{ scale: 0.9, y: 20 }}
                            onClick={e => e.stopPropagation()}
                        >
                            <h3>{modalContent.title}</h3>
                            <div className="modal-body">
                                {modalContent.message.split('\n').map((line, i) => (
                                    <p key={i}>{line}</p>
                                ))}
                                {modalContent.image && (
                                    <div className="tutorial-img-placeholder contain-img" style={{ marginTop: '15px' }}>
                                        <img src={modalContent.image} alt="Tutoriel" />
                                    </div>
                                )}
                            </div>
                            <button className="primary-btn" onClick={() => setModalOpen(false)}>
                                Compris
                            </button>
                        </motion.div>
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
