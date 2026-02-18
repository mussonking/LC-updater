# CLAUDE.md
# leclasseur project configuration
# format: LLM-OPTIMIZED (ref: .claude/LLM-OPTIMIZED.yaml)

# =============================================================================
# MARCO_BEHAVIOR
# =============================================================================

marco_behavior:

  language: francais
  autonomy: full
  principe: user_ne_code_pas_claude_fait_tout
  execution: shell_commands_file_edits_config_server_admin

  style_reponse:
    optimal: 3_5_lignes_max
    action_demandee: juste_faire_pas_expliquer
    action_proposee: expliquer_pourquoi
    eviter: paves_texte_repetitions_over_explain

  niveau_user: debutant_linux
  comprehension: logique_sans_voir_code

# =============================================================================
# STACK_OVERVIEW
# =============================================================================

stack:

  nginx:
    type: reverse_proxy_root
    note: not_docker_installed_system

  docker_services:

    leclasseur_dev:
      port: 8001
      role: api_fastapi_principale
      framework: python_fastapi_uvicorn

    chat_api_dev:
      port: 8002
      role: ai_chat_streaming
      integration: google_gemini

    postgres_db_dev:
      port: 5432
      role: database_postgresql_16
      volume: ./postgresdata-dev

    redis_broker_dev:
      role: celery_broker_cache
      volume: ./redisdata

    minio_storage_dev:
      port_api: 9005
      port_console: 9006
      role: s3_compatible_storage
      volume: ./miniodata-dev

    celery_worker_dev:
      role: async_task_processing
      concurrency: 8

    celery_beat_dev:
      role: periodic_task_scheduler

  network: proxy_net_external

  backend_structure:
    routers: documents_dossiers_clients_mortgageboss_email_trash
    services: pdf_deep_scan_email_ai_thumbnail_compliance

# =============================================================================
# VAULT_QUICK
# =============================================================================

vault_quick:

  version: 2.5
  npm_package: "@maderatools/vault"
  reference_complete: ~/vault-usage.yaml

  classification:

    unlocked:
      description: 24_7_auto_loaded_docker_accessible
      examples:
        - ANTHROPIC_API_KEY
        - DB_PASSWORD
        - STRIPE_SECRET_KEY
        - MINIO_ACCESS_KEY

    protected:
      description: manual_unlock_3h_timeout_gpg_required
      examples:
        - SUDO_PASSWORD
        - GITHUB_PAT
        - SSH_PASSPHRASE
      unlock_cmd: vault-unlock

  commandes_obligatoires:
    verification: vault_check_VAR
    sudo: vault_sudo_cmd
    ssh: vault_ssh_host_cmd
    api: vault_api_KEY

  interdictions_absolues:
    - echo_$SUDO_PASSWORD
    - printf_$API_KEY
    - cat_.env_|_grep_PASSWORD
    - printenv_|_grep_SUDO
    - env_|_grep_TOKEN
    - debug_logs_contenant_secrets

  claude_constraint: ne_voit_jamais_valeurs_secrets

# =============================================================================
# SSH_QUICK
# =============================================================================

ssh_quick:

  architecture: zero_trust_cloudflare_tunnel
  reference_complete: ~/servers-ssh.yaml

  serveurs:

    dev:
      tunnel: ssh-dev.madera.tools
      alias: dev-tunnel
      role: bastion_vault_master
      location: maison

    backup:
      tunnel: ssh-backup.leclasseur.ca
      alias: backup-tunnel
      role: offsite_backups_prod
      location: ovh_bare_metal

    prod:
      tunnel: ssh-prod.leclasseur.ca
      alias: prod-tunnel
      role: application_live
      location: ovh_bare_metal
      status: pre_prod_pas_client_reel

    staging:
      tunnel: ssh-staging.leclasseur.ca
      alias: staging-tunnel
      status: a_creer_premier_client

  securite:
    ssh_internet: ferme_ufw_bloque_tout
    acces_method: cloudflare_tunnels_only
    firewall: ufw_actif_tous_serveurs

# =============================================================================
# GIT_WORKFLOW
# =============================================================================

git_workflow:

  regle_critique: user_oublie_branches_claude_enforce

  branches:
    main: prod_stable_jamais_coder_direct
    develop: dev_actif_integration_features
    feature: feature/nom_toujours_creer_nouvelle_feature

  checklist_debut_session:
    step_1: git_status_verifier_branche
    step_2: si_main_avertir_creer_feature_branch
    step_3: confirmer_branche_avec_user_avant_coder

  protection_main:
    interdit:
      - commit_direct_sur_main
      - push_main_sans_develop
    action_si_main: immediatement_creer_feature_branch

  workflow:
    step_1: coder_sur_feature_branch
    step_2: commits_reguliers_messages_clairs
    step_3: push_origin_feature/nom
    step_4: merge_vers_develop_pas_main
    step_5: test_staging
    step_6: merge_develop_main_apres_tests

# =============================================================================
# YAML_INDEX
# =============================================================================

yaml_index:

  location: /home/mad/leclasseur/.claude/

  architecture:

    INDEX.yaml:
      purpose: session_init_environment_detection
      when: debut_session_nouveau_contexte

    API.yaml:
      purpose: endpoints_reference_quick
      when: travail_backend_routes_api

    DB.yaml:
      purpose: schema_tables_reference
      when: modifications_database_models

    GIT-SYSTEM.yaml:
      purpose: dual_git_backup_real_architecture
      when: operations_git_recovery

    DEPLOY.yaml:
      purpose: procedures_deployment_prod_staging
      when: deployer_nouveau_code

  features:

    CHROME-EXTENSION.yaml:
      purpose: mortgageboss_crm_integration_architecture
      when: travail_extension_chrome

    MORTGAGEBOSS-FIELDS-REFERENCE.yaml:
      purpose: champs_patterns_mapping
      when: field_mapping_autofill

    SCRYBE.yaml:
      purpose: pwa_pdf_viewer_22_modules
      when: travail_scrybe_frontend

  standards:

    LLM-OPTIMIZED.yaml:
      purpose: format_documentation_standard
      when: creer_modifier_yaml_docs

  external:

    vault-usage.yaml:
      path: ~/vault-usage.yaml
      purpose: vault_complet_architecture_commandes

    servers-ssh.yaml:
      path: ~/servers-ssh.yaml
      purpose: ssh_complet_4_serveurs_workflows

# =============================================================================
# DAILY_QUICK
# =============================================================================

daily_quick:

  docker:
    start: docker compose up -d
    stop: docker compose down
    logs: docker compose logs -f SERVICE
    rebuild: docker compose up -d --build SERVICE
    status: docker compose ps

  test_api:
    health: curl localhost:8001/health
    docs_url: http://localhost:8001/docs

  git:
    status: git status
    new_feature: git checkout -b feature/nom
    switch_develop: git checkout develop
    commit: git commit -m "message"

  services_ports:
    api: 8001
    chat: 8002
    postgres: 5432
    minio_api: 9005
    minio_console: 9006

# =============================================================================
# META
# =============================================================================

meta:
  version: 4.0
  date: 2025-12-26
  format: llm_optimized
  lines: sub_250
  reference: .claude/LLM-OPTIMIZED.yaml
