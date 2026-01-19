## proofloops

Rust helpers for working in Lean 4 repos: verify, locate `sorry`s, build bounded prompt packs, and optionally call an OpenAI-compatible LLM.

This repo is **Rust-only**.

### What it does

- **Verify**: run `lake env lean` on a file or on an in-memory snippet.
- **Triage**: count errors/warnings, find nearest `sorry`, and emit a small “what to do next” JSON.
- **Prompt packs**: extract a bounded excerpt + imports + nearby decl headers.
- **Review packs**: build a bounded git-diff context with secret redaction.
- **MCP server**: expose the same operations via MCP (HTTP + optional stdio).

### Quickstart (CLI)

From the repo root:

```bash
cargo run --quiet -p proofloops-core --bin proofloops -- --help
```

Common commands (all output JSON):

```bash
# Verify + sorry scan (no LLM)
cargo run --quiet -p proofloops-core --bin proofloops -- triage-file \
  --repo /abs/path/to/lean-repo \
  --file Some/File.lean

# Build a bounded context pack around a declaration (no LLM)
cargo run --quiet -p proofloops-core --bin proofloops -- context-pack \
  --repo /abs/path/to/lean-repo \
  --file Some/File.lean \
  --decl some_theorem

# Suggest a proof for a lemma (LLM call)
cargo run --quiet -p proofloops-core --bin proofloops -- suggest \
  --repo /abs/path/to/lean-repo \
  --file Some/File.lean \
  --lemma some_theorem

# Patch first `sorry` in the lemma using a file and verify (in-memory; does not write)
cargo run --quiet -p proofloops-core --bin proofloops -- patch \
  --repo /abs/path/to/lean-repo \
  --file Some/File.lean \
  --lemma some_theorem \
  --replacement-file /tmp/replacement.lean
```

### Environment (LLM routing)

Prefer `PROOFLOOPS_*` env vars. Legacy `PROOFYLOOPS_*` are accepted in most places.

- **Provider order**: `PROOFLOOPS_PROVIDER_ORDER` (default `ollama,groq,openai,openrouter`)
- **Ollama**: `OLLAMA_MODEL` (+ optional `OLLAMA_HOST`)
- **Groq**: `GROQ_API_KEY`, `GROQ_MODEL`
- **OpenAI**: `OPENAI_API_KEY`, `OPENAI_MODEL` (+ optional `OPENAI_BASE_URL`)
- **OpenRouter**: `OPENROUTER_API_KEY`, `OPENROUTER_MODEL` (+ optional `OPENROUTER_BASE_URL`)

Verification behavior:
- **Auto-build**: `PROOFLOOPS_AUTO_BUILD=0` disables the “missing olean → lake build → retry” fallback (legacy: `PROOFYLOOPS_AUTO_BUILD`).

Environment loading (super-workspace convenience):
- reads `<repo_root>/.env` if present (does not override already-set vars)
- optionally searches one directory deep for a sibling `.env` if no API key is set yet  
  controls: `PROOFLOOPS_DOTENV_SEARCH=0`, `PROOFLOOPS_DOTENV_SEARCH_ROOT=/abs/path` (legacy `PROOFYLOOPS_*`)

### MCP server

```bash
cargo run --quiet -p proofloops-mcp --bin proofloops-mcp
```

Defaults:
- `PROOFLOOPS_MCP_ADDR=127.0.0.1:8087` (legacy: `PROOFYLOOPS_MCP_ADDR`)
- `PROOFLOOPS_MCP_TOOL_TIMEOUT_S=180` (legacy: `PROOFYLOOPS_MCP_TOOL_TIMEOUT_S`)

