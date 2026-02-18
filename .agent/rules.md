# PROJECT RULES - ANTIGRAVITY
# Format: LLM-OPTIMIZED (Machine-readable first)
# User: Marco

# =============================================================================
# MARCO_BEHAVIOR (CORE IDENTITY)
# =============================================================================

behavior:
  language: francais
  autonomy: full
  principle: "user_ne_code_pas_agent_fait_tout"
  relationship: "marco_strategie_agent_execution"
  
  marco_profile:
    knowledge: "logical_and_functional_master"
    limitations: ["does_not_code", "does_not_execute_commands", "does_not_navigate"]
    action: "approve_plan_then_let_agent_execute"

  agent_style:
    optimal: "short_concise_reports (3-5 lines)"
    action_requested: "just_do_it_dont_explain"
    action_proposed: "explain_why_and_impact"
    avoid: ["over_explaining", "text_walls", "repetition"]

# =============================================================================
# TECHNICAL ENVIRONMENT (WINDOWS/SAMBA)
# =============================================================================

environment:
  os: "Windows 11 (PowerShell primary)"
  shell_preference: "powershell (standard) / bash (for .sh scripts)"
  
  filesystems:
    local: "c:/Users/Musson/Desktop/Claude Code/"
    network: "Z:/ (Samba mapped to server)"
    caution: "Samba permissions can be tricky - avoid direct mass-rewrites via shell if possible"

  critical_safety_rule:
    incident: "8000_lines_wiped_by_shell_overwrite"
    enforced_rule: "NEVER use Out-File or shell redirection to rewrite large files (>100 lines) on Z: or mapped drives. Use targeted edits (replace_file_content) or read-verify-write pattern."

# =============================================================================
# PROJECT STACK & AUTOMATION
# =============================================================================

projects:
  updater_app:
    path: "leclasseur-updater/"
    tech: ["Tauri v2", "Rust", "React", "TypeScript"]
    purpose: "Auto-reloads Chrome Extension on client machines"
  
  chrome_extension:
    path: "Z:/packages/chrome-extension/"
    primary_files: ["background.js", "content.js"]
    integration: "WebSocket (ws://localhost:8888) for auto-reload"

  automation:
    git_sync: "scripts/git-autocommit.ps1 (Daily Push / Instant Commit)"
    update_deploy: "push-update.sh (Auto-increment version & package)"

# =============================================================================
# RECOVERY & PERSISTENCE
# =============================================================================

persistence:
  rule: "Always trust the Git Auto-Commit system"
  recovery_cmd: "git log --oneline -20 (Look for automatic snapshots)"

