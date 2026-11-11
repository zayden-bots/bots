# Zayden bots

The Zayden Discord bot binary and the background worker binary. They wire the
feature crates from `zayden-bots/modules` into the `bot-runtime` from
`zayden-bots/platform`.
Serenity for Discord, Postgres via `sqlx`. This repo has no SQL queries of its
own, so it has no `.sqlx/` cache.

## Layout

| Path                    | What it is                                                  |
| ----------------------- | ----------------------------------------------------------- |
| `bots/zayden/`          | the Zayden bot: a lib (`module_list`) and a bin             |
| `bots/llama/`           | LlamaBot: Zayden's module list plus llamad2                 |
| `bots/maria/`           | Maria: AI chat only for now                                 |
| `bots/worker/`          | the worker binary — scheduled jobs, no Discord gateway      |
| `docker/Dockerfile.bot` | release image for any bot binary in `bots/*`                |
| `radio.toml`            | radio station list the bot reads from the working directory |
| `docs/`                 | `build-and-toolchain.md`: toolchain and build profiles      |
| `.env.example`          | placeholder environment variables                           |
| `config.toml`           | gitignored symlink to `infra/config/zayden.toml`            |

The workspace members are `bots/*`. Everything else is a git dependency in the
root `Cargo.toml`, tracking the default branch of its repo:

| Repo       | Crates                                                       |
| ---------- | ------------------------------------------------------------ |
| `modules`  | one crate per feature (gambling, levels, music, palworld, …) |
| `platform` | `zayden-core`, `zayden-app`, `bot-runtime`                   |

`Cargo.lock` pins the commit. The migrations live in the `schema` repo and come
in through the other crates.

## Running locally

The bot reads `config.toml` and `radio.toml` from the working directory, and
`.env` for secrets.

```bash
../infra/scripts/checkout.sh      # symlinks config.toml to infra/config/zayden.toml
cp .env.example .env              # then fill in the values
cargo run -p zayden
```

## The bots

Each bot wires a module list into `bot-runtime`.

- **`zayden`** is a lib and a bin. The lib exports `module_list`, Zayden's
  module list before `finish()`, so another bin can extend it. The bin finishes
  the list and runs it in the Zayden slot. llamad2 is no longer in the default
  Zayden build.
- **`llama`** takes `zayden::module_list` and adds llamad2. It boots in the
  Zayden slot, so it runs as a custom bot (its application id is not the
  slot's default).
- **`maria`** boots in the Maria slot and links only the `ai` module: AI chat
  and nothing else for now. Its persona is in `bots/maria/src/persona.rs`. It
  has no voice client.
- **`worker`** is described below.

`llama` and `maria` run with `cargo run -p llama` and `cargo run -p maria`.

## The worker

`bots/zayden` keeps the gateway, the commands and the per-bot jobs.
`bots/worker` boots with `bot_runtime::worker::boot()`: no gateway connection
and no `DISCORD_TOKEN`. It runs the migrations, then the scheduled jobs that
used to run only in the Zayden bot: youtube, patreon, marathon, deadlock,
destiny2, gambling (stamina), ticket (gc), jellyfin, watch and hosting, plus the
daily guild retention purge.

Broadcasts go through per-channel Discord webhooks owned by each bot. When a
channel has no healthy webhook, and for DMs, the worker queues a task that the
bot serving that guild sends with its own token.

It needs `DATABASE_URL`, `config.toml` and the `CUSTOM_BOT_KEYS` /
`CUSTOM_BOT_ACTIVE_KEY` keyring (it seals and opens webhook tokens, and fails to
start without it).

```bash
cargo run -p worker
```

Both binaries run migrations, so deploy the bot and the worker together; see
`../infra/README.md`.

`DATABASE_URL` in `.env` points at the live database. Point it at a throwaway
one for local work:

```bash
docker run --rm -d --name zayden-prepare -p 55432:5432 -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=zayden_prepare postgres:18-alpine
```

## Commands

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo fmt --all
```

`--all-targets` defaults to _off_ and silently skips tests, benches and
examples. Always pass it.

Clippy never codegens or links, so it reports green while the build is broken.
Only a real build exercises linking:

```bash
cargo build --workspace --all-targets
```

`bacon.toml` defines these as watch jobs:

```bash
bacon            # clippy, the default job
bacon test       # cargo test
bacon fmt        # cargo fmt --all --check
bacon build      # link check
bacon nextest    # needs `cargo install cargo-nextest`
```

Keep the flags identical across this README, `bacon.toml` and CI. A local
command weaker than CI is how drift starts.

## Adding a bot binary

1. Create `bots/<name>/` with its own `Cargo.toml` (`[lints] workspace = true`,
   dependencies as `foo.workspace = true`) and a `[[bin]]` named `<name>`.
   `bots/*` is already a workspace member glob.
2. Add an entry to the `images` job matrix in `.github/workflows/ci.yml`:
   `name`, `bin`, `dockerfile: docker/Dockerfile.bot`, `image`, and
   `needs_ytdlp` if it plays audio.

`docker/Dockerfile.bot` takes `--build-arg BIN=<bin name>` (default `zayden`)
and installs the binary as `/usr/local/bin/bot`, which is the image entrypoint.

## Docker

```bash
docker build -f docker/Dockerfile.bot -t zayden .
docker build -f docker/Dockerfile.bot --build-arg BIN=worker -t worker .
```

The Dockerfile is a cargo-chef build: dependencies are cooked into a cached
layer, then the binary is compiled and copied into a `debian:trixie-slim`
runtime with `ffmpeg`, `yt-dlp` and `deno` that runs as a non-root `zayden`
user. It builds on **stable** with `SQLX_OFFLINE=true`; `.dockerignore` keeps
`rust-toolchain.toml` and `.cargo/` out of the context, so anything in the
manifests must parse on stable cargo. The git dependencies are fetched from
GitHub during the build. `config.toml`, `.env` and `docs/` are not in the build
context.

## Conventions

- Lints are declared once in the root `Cargo.toml` under `[workspace.lints]`;
  every crate opts in with `[lints] workspace = true`.
- Dependency versions are pinned once in `[workspace.dependencies]`; member
  crates say `foo.workspace = true` and may _add_ features on top, but never
  restate a version. Unused workspace entries cost nothing — cargo only
  resolves a dependency once a member opts in.
- `unwrap`/`expect`/`panic`/`todo`/`dbg!` are **denied** outside tests;
  `clippy.toml` re-permits them under `#[cfg(test)]`.
- Tests live in `tests/`, not in inline `#[cfg(test)] mod tests` blocks.
- Commands are registered and routed manually through Serenity traits. `poise`
  is not used.
- `rustfmt.toml` uses unstable options, so `rust-toolchain.toml` pins nightly
  for local work. Never write `cargo +nightly` — the prefix is redundant.
- Lint tables in the root `Cargo.toml` and the files `rust-toolchain.toml`,
  `rustfmt.toml`, `clippy.toml` and `.cargo/config.toml` are shared with the
  other Rust repos; `../infra/scripts/sync-standards.sh` copies the files in
  (`--check` reports drift).

## Toolchain

Nightly for local development, stable for release images. See
`docs/build-and-toolchain.md` for the two mechanisms that keep the split
working (`Cargo.toml` must stay stable-parseable; `.dockerignore` must keep
excluding `.cargo/`), how the build profiles are tuned, and why the cranelift
codegen backend is not used.

## Cross-repo dependencies

Never give two entries for the same org repo different `branch`/`tag`/`rev`
values: cargo would treat them as different crates and types would stop
matching. CI enforces it (step "One ref per org repository" in the `deny` job).

To take an upstream change, push it first, then here:

```bash
cargo update zayden-app accounts schema     # one crate per org repo moves that whole repo
cargo build --workspace --all-targets
```

Move all three org repos in the same update. The modules' queries were checked
against the `schema` commit that `modules` pins, while the migrations the bot
applies at startup come from the `schema` commit pinned here; updating
`modules` alone can ship queries for a column this binary never creates.

Commit `Cargo.lock`. To work across repos without pushing, run
`../infra/scripts/dev-link.sh on` (`off` removes it, `status` reports). Never
commit a `Cargo.lock` written while the link is on: run `dev-link.sh off` and
`cargo update <crate>` (or `git restore Cargo.lock`) first.

## CI

`.github/workflows/ci.yml` runs these jobs in parallel: `rustfmt`, workspace
`clippy`, workspace `test`, and `cargo-deny` (plus the one-ref check above).
`clippy` and `test` build on **stable** with `--locked`
(`RUSTUP_TOOLCHAIN=stable` outranks `rust-toolchain.toml`, and `RUSTFLAGS=""`
clears the nightly-only flags in `.cargo/config.toml`); `rustfmt` runs on
nightly. `SQLX_OFFLINE=true` is set for every job.

The `images` job builds a release image per matrix entry (`zayden`, `worker`,
`maria` and `llama`) from `docker/Dockerfile.bot` once `rustfmt`, `clippy` and
`cargo-deny` pass, and pushes it to GHCR as `ghcr.io/<owner>/<image>` for every
event except pull requests. Entries with `needs_ytdlp: true` (`zayden`,
`llama`) get the latest yt-dlp release as `YTDLP_VERSION`; `worker` and `maria`
set it to `false`.
