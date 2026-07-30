# Monospace Telegram Bot (Rust port)

A Rust port of the original Go implementation (also ported to Python),
using [teloxide](https://github.com/teloxide/teloxide) — the most widely
used and actively developed Telegram bot framework for Rust — and
[`rusqlite`](https://github.com/rusqlite/rusqlite) (bundled SQLite, no
system dependency) for statistics storage.

Converts any text you send — plus captions on photos, videos, voice
notes, and other media — into Telegram monospace formatting, chunked by
Word / Sentence / Paragraph / Full mode, with smart splitting for
Telegram's message length limits and a SQLite-backed usage-statistics
page.

## ⚡ One-click deploy: the one file you need to edit

**Before deploying, open [`deploy_config.rs`](./deploy_config.rs) at the
repository root and replace the placeholder `BOT_TOKEN` with a real token
from [@BotFather](https://t.me/BotFather).** Every other file is ready to
go as-is — nothing else needs to change for a Railway deployment.
`deploy_config.rs` is a plain Rust file with three `pub const`s and
extensive comments explaining each one; environment variables (if you set
them instead, e.g. in Railway's dashboard) always take priority over the
constants in that file, so you can use either approach.

**Security note:** if you fill in a real token and this repository is or
might become public, use the Railway environment variable instead of
editing `deploy_config.rs` — a token committed to a public repo is a
public token.

## Project structure

A Cargo workspace with one library crate per feature area, plus a thin
binary crate at the root that wires everything together — the most
idiomatic structure for a Rust project of this size, giving each area a
compiler-enforced boundary (a crate cannot accidentally depend on another
crate that doesn't list it as a dependency).

```
monospace-telegram-bot-rust/
├── Cargo.toml                  workspace manifest + root binary crate
├── deploy_config.rs            ← EDIT THIS: your BOT_TOKEN (see above)
├── Dockerfile
├── .github/workflows/ci.yml    CI: fmt, clippy, build, test, Docker build
├── src/
│   └── main.rs                 entry point — builds the dptree schema, starts the webhook listener
│
├── crates/bot_core/             domain types shared by every other crate
│   └── src/
│       ├── lib.rs
│       ├── config.rs            loads BOT_TOKEN / PORT / RAILWAY_PUBLIC_DOMAIN (env, with deploy_config.rs fallback)
│       ├── mode.rs               Mode enum + DEFAULT_MODE
│       ├── mode_label.rs         Mode -> button label
│       ├── mode_parse.rs         button label -> Mode
│       └── limits.rs             Telegram message/caption length limits
│   └── tests/mode_tests.rs
│
├── crates/rendering/             pure text-transformation logic — no Telegram dependency
│   └── src/
│       ├── lib.rs
│       ├── render.rs              render(): text -> monospaced text, by mode
│       ├── render_units.rs        wraps each split unit in its own code span
│       ├── wrap_code.rs           wraps a string in a Telegram code span
│       ├── split_surrounding_space.rs
│       ├── split_words.rs
│       ├── split_sentences.rs
│       ├── split_paragraphs.rs
│       ├── closing_mark.rs
│       ├── split_for_telegram.rs  splits long output into multiple messages
│       ├── best_split_point.rs
│       ├── last_word_break.rs
│       ├── last_sentence_break.rs
│       └── last_index_after.rs
│   └── tests/render_tests.rs
│
├── crates/telegram_ui/            bot-facing layer — menus, dptree handlers, media, webhook
│   └── src/
│       ├── lib.rs
│       ├── new_bot.rs
│       ├── webhook_url.rs
│       ├── store.rs                per-user mode storage (Arc<Mutex<HashMap>>)
│       ├── welcome_text.rs
│       ├── main_menu.rs
│       ├── settings_menu.rs
│       ├── mode_handler.rs
│       ├── register_main_menu_handlers.rs
│       ├── register_settings_handlers.rs
│       ├── register_content_handlers.rs
│       ├── send_rendered.rs
│       ├── handle_media.rs
│       └── resend_media.rs
│
└── crates/stats/                  SQLite-backed usage statistics
    └── src/
        ├── lib.rs
        ├── db.rs                    Arc<Mutex<rusqlite::Connection>>, points at /data/stats.db
        ├── db_schema.rs
        ├── track_message.rs
        ├── track_middleware.rs      dptree map_async step, runs before every branch
        ├── stats.rs                 Stats struct
        ├── load_stats.rs            includes a Feb-29 leap-year fix (see Porting notes)
        ├── format_stats.rs
        ├── stats_menu.rs
        └── register_stats_handlers.rs
    └── tests/stats_tests.rs
```

## Requirements

- Rust 1.85+ (teloxide 0.17's minimum supported version)
- A Telegram bot token from [@BotFather](https://t.me/BotFather)
- A C compiler (`build-essential` on Debian/Ubuntu) — needed to compile
  `rusqlite`'s bundled SQLite from source; already installed in the
  Dockerfile and CI workflow

## ⚠️ Verification status — please read before relying on this in production

This port was written in a sandboxed environment **without a Rust
compiler or internet access to run `cargo build`**. Every API used here
(teloxide's dptree dispatch model, `rusqlite`, `chrono`) was checked
against current, real documentation and official example code found via
web search — not written from memory alone — but none of it has been
compiled or run against a live Telegram bot. This is a meaningfully
different confidence level than the accompanying Go and Python ports,
where at least the pure-logic portions were executed directly.

What this means in practice:
- The **pure-logic crates** (`rendering`, and the non-Telegram parts of
  `stats`/`bot_core`) are ordinary Rust with no exotic dependencies —
  low risk, and covered by the unit tests in `tests/`, which CI runs.
- The **dptree-based dispatch schema** (`main.rs`, the `register_*.rs`
  files in `telegram_ui`, and `stats/register_stats_handlers.rs`) is the
  highest-risk area. dptree's dependency injection is checked at
  **runtime**, not compile time — a missing or mistyped dependency in
  `dptree::deps![...]` produces a panic only when a matching update
  arrives, not a compile error (this is a documented, known sharp edge
  of dptree itself, not specific to this port). **Run `cargo build`,
  `cargo clippy`, and a real end-to-end test against a Telegram test bot
  before deploying this to a bot real users depend on.**
- The CI workflow (see below) will catch compile errors and clippy
  lints automatically on the first push — treat a green CI run as the
  first real compilation check this code has received.

## Porting notes (Go/Python → Rust)

- **Strings are UTF-8 bytes, not code points.** Go's `[]rune` and
  Python's `str` both index by Unicode code point; Rust's `&str` indexes
  by byte. Every place the original logic does arithmetic on a
  character *count* or *position* (splitting, length limits) is
  reimplemented here using `.chars()` / `Vec<char>` rather than raw byte
  indices, so behavior matches exactly — including with multi-byte
  content like emoji. This was audited file-by-file; see the inline
  comments in `crates/rendering/src/`.
- **Concurrency**: teloxide's `Dispatcher` runs handlers as concurrent
  tokio tasks (unlike Python's single-threaded event loop). Both the
  per-user mode store (`telegram_ui::Store`) and the SQLite connection
  (`stats::Db`) are wrapped in `Arc<Mutex<_>>` to make this safe — the
  Rust type system would refuse to compile a version that shared these
  without synchronization (`rusqlite::Connection` is `Send` but not
  `Sync`).
- **Webhook serving**: teloxide's `webhooks::axum(...)` builds an update
  listener backed by axum internally — no separate web framework needed,
  the same way the Go version's `telebot.Webhook` poller and the Python
  port's `Application.run_webhook(...)` are self-contained.
- **Dispatch model**: teloxide uses `dptree`, a branch-based
  chain-of-responsibility dispatcher, rather than Go/Python's
  register-a-handler-per-exact-button-text model. The practical behavior
  is the same (button handlers are tried before the generic text/media
  catch-all), but the mechanism is structurally different — see
  `crates/telegram_ui/src/register_*.rs`.
- **Feb 29 leap-year fix**: `crates/stats/src/load_stats.rs` includes a
  `minus_one_year` helper mirroring Go's `time.AddDate(-1, 0, 0)`
  rollover behavior (Feb 29 → Mar 1 in a non-leap target year) — the
  same fix made in the Python port, since a naive `chrono` date
  subtraction would otherwise fail once a year.

## Configuration

| Variable                 | Description                                                              |
| ------------------------- | -------------------------------------------------------------------------- |
| `BOT_TOKEN`               | Your Telegram bot token. Falls back to `deploy_config.rs` if unset.       |
| `PORT`                    | Port the webhook server listens on. Falls back to `deploy_config.rs` (default `8080`) if unset. |
| `RAILWAY_PUBLIC_DOMAIN`   | Public domain Telegram sends webhook updates to (no scheme). Falls back to `deploy_config.rs` if unset. |

## Running locally

```bash
# Either export environment variables:
export BOT_TOKEN=your-telegram-bot-token
export RAILWAY_PUBLIC_DOMAIN=your-public-domain   # e.g. via a tunnel like ngrok

# ...or just edit deploy_config.rs and skip the exports above.

cargo run --release
```

On first run this creates `/data/stats.db` (WAL mode) — make sure `/data`
exists and is writable, or change `DB_PATH` in
`crates/stats/src/db.rs`.

## Deploying on Railway

1. Edit `deploy_config.rs` with your bot token (or plan to set
   `BOT_TOKEN` as a Railway environment variable instead).
2. Push this repository to GitHub.
3. Create a new project on Railway and connect your repository. Railway
   will detect the `Dockerfile` and build from it automatically.
4. If you didn't edit `deploy_config.rs`, add a `BOT_TOKEN` environment
   variable in the service settings. `RAILWAY_PUBLIC_DOMAIN` and `PORT`
   are provided by Railway automatically.
5. **Add a Volume** mounted at `/data` so `stats.db` (and your usage
   statistics) survive across deploys.
6. Deploy.

## Building with Docker

```bash
docker build -t monospace-bot-rust .
docker run -e BOT_TOKEN=your-telegram-bot-token \
  -e RAILWAY_PUBLIC_DOMAIN=your-public-domain \
  -v monospace-bot-data:/data \
  -p 8080:8080 monospace-bot-rust
```

## CI/CD

`.github/workflows/ci.yml` runs on every push and pull request to `main`:

1. `cargo fmt --check` — formatting.
2. `cargo clippy --workspace --all-targets -- -D warnings` — lints,
   treated as errors.
3. `cargo build --workspace --release` — compiles every crate.
4. `cargo test --workspace --release` — runs the unit tests in
   `crates/*/tests/`.
5. A Docker build of the full deploy image (not pushed anywhere — just
   validates the `Dockerfile` builds cleanly).

There is deliberately no auto-deploy step: Railway's own GitHub
integration (connect the repo in the Railway dashboard) handles
deployment on every push to `main` once you've done that one-time setup,
so CI here focuses purely on catching build/lint/test failures before
they reach that point.

## Notes

- No `Cargo.lock` is included — this environment had no Rust toolchain
  available to generate one. The first `cargo build` (locally or in CI)
  will create it; for a binary crate like this one, commit the resulting
  `Cargo.lock` so builds are reproducible.
- Modes are stored in memory per user and reset if the process restarts.
- Statistics are stored in SQLite on disk and survive process restarts;
  only a fresh deploy without a mounted volume resets them.
- Stickers and video notes are re-sent as-is (no caption support in the
  Telegram Bot API for those types).
