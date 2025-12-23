
# --- LLM Collaboration Context Aliases ---
export LLM_CONTEXT_ROOT='/Users/macbook/Desktop/etrid'

# 1. Guarded Agents (Auto-logging & Recovery)
alias claude='sh "$LLM_CONTEXT_ROOT/universal_tools/guard/session_guard.sh" claude'
alias gemini='sh "$LLM_CONTEXT_ROOT/universal_tools/guard/session_guard.sh" gemini'
alias qwen='sh "$LLM_CONTEXT_ROOT/universal_tools/guard/session_guard.sh" qwen'
alias codex='sh "$LLM_CONTEXT_ROOT/universal_tools/guard/session_guard.sh" codex'

# 2. Orchestration Tools
alias fork="$LLM_CONTEXT_ROOT/universal_tools/orchestration/fork_terminal.sh"
alias mission="$LLM_CONTEXT_ROOT/universal_tools/mission_control.sh"
alias research="$LLM_CONTEXT_ROOT/universal_tools/research_topic.sh"
alias review="$LLM_CONTEXT_ROOT/universal_tools/review_code.sh"

# 3. Collaboration
alias brain="cat $LLM_CONTEXT_ROOT/universal_tools/collaboration/PROJECT_BRAIN.md"
alias note="$LLM_CONTEXT_ROOT/universal_tools/collaboration/update_brain.sh"

# 4. Recovery
alias recover="$LLM_CONTEXT_ROOT/universal_tools/guard/recover_session.sh"

echo '🤖 LLM Context Active: Mission Control ready (type "mission")'

