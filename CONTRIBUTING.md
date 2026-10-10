# Contributing

Talk is a Tauri v2 desktop client for Windows and Debian: a Rust backend around whisper.cpp, a React
19 frontend. This says how a change gets in, and what is particular to this repository. The shape of
the flow is the same one the other repositories here follow.

Install the hooks once per clone, before the first commit. Worktrees share the config file, so they
need nothing of their own:

    git config core.hooksPath .githooks

## Branches

Two long-lived branches, and the difference between them is one question: has this been published?

- `dev` is where work lands. It stays buildable, and it is what a topic branch is opened from and
  rebased onto.
- `main` is what is out there. Every commit on it has been released under a tag, and nothing
  reaches it except by fast-forward from `dev` at the moment of a release.

So `main` is always an exact prefix of `dev`: whatever `dev` holds beyond `main` is precisely what
is written and not yet published. `prefix.yml` says so out loud when it stops being true.

Every other branch is a topic branch, named for what it does:

    <type>/what-the-branch-does

The type is one of the nine below, whichever the branch mostly is. After the slash, two to five
words in lower case joined by dashes, saying what the branch does rather than which file it opens.
`release/0.8.0` is the one shape that departs from it: such a branch carries the version bump and
nothing else, so the version is the whole of the name.

No name carries a date, an author or an issue number. The commit that does the work names the issue
it closes.

**The history is linear and carries no merge commit.** A topic branch is rebased onto `dev` and
enters by a pull request merged with the rebase button. If that button refuses, the rebase was not
done, and the answer is to rebase rather than to merge.

## Pull requests

**Every batch enters by one**, including one written by whoever owns the repository: the build runs
on `pull_request`, so a batch folded in by hand is built only once it is already in `dev`.

**"Rebase and merge", and neither of the other two buttons.** A merge commit forks a history that
never forks. A squash collapses a branch into one commit and throws away the bodies, which is where
the reasoning for each step lives.

**`dev` reaches `main` by a fast-forward and NOT by that button.** All three buttons rewrite, and
`main` is already an ancestor of `dev`, so replaying `dev` onto it hands `main` a second copy of
every commit under a fresh hash. It was done once, on 0.8.0, and cost a reset of `main` and a tag
moved onto the commit `dev` already held. The request is still the right place for a release, for
the record and for the checks it runs; what merges it is:

    git push origin origin/dev:main

**An issue is closed from the commit that closes it**, on a line of its own at the foot of the body,
`Closes #12`. A closing keyword written in a request fires against the default branch, which is
`main`, and no ordinary request targets it.

## Commits

One logical change per commit, and a subject in conventional-commit form:

    <type>(<scope>)!: what the commit does

That form is worth more here than elsewhere, because the rebase button replays every subject
verbatim into the public history instead of collapsing them into the request's title.

| type | what it carries |
| --- | --- |
| `feat` | something the application did not do |
| `fix` | a defect corrected |
| `perf` | the same behaviour for less |
| `refactor` | no change of behaviour at all, dead code removed included |
| `docs` | the docs, the changelog, this file, comments on their own |
| `test` | the harness and what it runs over |
| `build` | the toolchain, the dependencies, the version bump |
| `ci` | `.github/workflows` |
| `chore` | whatever none of the others is |

The scope is optional and names a tree of code. The ones in use:

`transcription`, `recording`, `sound`, `models`, `audio`, `window`, `overlay`, `history`,
`dashboard`, `preferences`, `updater`, `server`, `installer`, `repo`.

The subject is imperative and starts on a verb, in lower case, and carries no full stop. The whole
line is 72 columns or fewer with the prefix counted in. A body is for the reason, when the reason is
not in the diff.

A `!` before the colon marks a change that breaks something that used to work, a settings file it
can no longer read included, and what breaks is written in the body.

`.githooks/commit-msg` refuses a subject or a branch name outside this form, and
`.github/workflows/commits.yml` runs that same file over every commit of a pull request. One rule,
one home.

## Changelog

`CHANGELOG.md` carries an `Unreleased` section, and a change somebody running the application would
notice is written into it **in the same commit that makes the change**. A refactor that changes
nothing visible gets no entry. Entries are written in the words of somebody who runs the thing, not
of somebody who builds it: why a change was made stays in the commit that made it.

## Verifying a change

The two feedback loops are not symmetric, and it is worth knowing which one a change is on.

    bun run build      # tsc and vite, the same pass the release runs
    bun run test       # the frontend suite, vitest on jsdom, seconds
    bun run test:ui    # the interface suite, Playwright on Chromium, a couple of minutes
    bun run test:ui:ci # the same suite the way the runner runs it
    bun run tauri:check # cargo check, MSVC environment loaded by scripts/vcenv.bat
    bun run test:rust  # cargo test, the same environment
    bun run tauri:check:linux # cargo check with the Vulkan SDK loaded
    bun run test:rust:linux  # cargo test with the same environment

`build` and `test` are what the `build` workflow runs, and `test:ui` is what the `interface`
workflow runs, on Windows. The Linux scripts load the Vulkan SDK before Cargo runs; use them for
Linux Rust checks and tests. **The Rust side is not compiled in CI**: `whisper-rs` builds
whisper.cpp from its build script, so even `cargo check` pays a compile measured in tens of minutes.
A change to `src-tauri/` is checked locally with `tauri:check` and read carefully, or it is answered
only by the release build.

`test:ui` runs the real frontend in a browser with the native side faked, so it answers whether the
pages still lay out, move and behave, and it fails on a console error, a command nobody mocked, a
cut-off label, a control out of reach and a serious accessibility violation. It also walks the
Appearance page: every shipped preset must pass the contrast check, and saving, removing and undoing
a theme, the gradient editor from the keyboard and the window buttons on either side are exercised.
Run it after any change to a page, the sidebar, the theme or the styles. A change that moves pixels on purpose regenerates the
baselines, and the new images are reviewed in the diff like code:

    bun run test:ui:update

**Before pushing, run `bunx tsc -p e2e` and `bun run test:ui:ci`.** The first is the type check the
`interface` workflow runs ahead of the suite. The second is the suite as the runner sees it: one
bundle instead of the development server, four workers, and every frame painted by the processor,
the runner having no graphics card. A page that is merely slow on a desk is a page that times out
there, and what is slow there is a defect of the application rather than of the runner: nothing
that moves without end may cost a frame it does not need. A red check is found, never rerun.

The baselines are the ones Windows draws, which is the machine CI runs on; they are not regenerated
anywhere else. The first run needs the browser, `bun run playwright install chromium-headless-shell`.
Playwright needs a real Node, which `node-win-x64` provides inside `node_modules`, so nothing has
to be installed on the machine. The native mock lives in `e2e/` and nothing under `src/` may import
it.

Call cargo through those scripts and not directly: `cargo test` on its own inherits whatever
environment the shell has, and without the MSVC one loaded the native build fails in ways that read
like a Rust error.

## Versions and releasing

A released version is three numbers, and after them either `-alpha`, or `-beta`, or nothing at
all. Nothing follows the word, a counter least of all. The one exception is `-dev` below, which
never reaches a tag, and the counter test installers add after it.

Between two releases `dev` carries the next version with `-dev` after it, `0.10.0-dev` once `0.9.0`
is out, opened by a `build/open-next-dev-version` branch right after the release. It names the version
being written towards and is never tagged: the `release/<version>` branch is what takes the suffix
off. A `-dev` version is also what lets a test installer be told apart from the release.
`bun run tauri:test-installer` builds one stamped `0.10.0-dev.1`, then `.2` and so on, so each one
installs as an upgrade over the last instead of offering a repair. The stamp goes through `--config`
and the counter lives under `.git`, so nothing in the tree moves and nothing has to be put back.

The version lives in four files and they move together:

- `src-tauri/tauri.conf.json`, which is what the installer and the updater compare against
- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/Cargo.lock`, in the `talk` entry

Nothing derives the tag from them: a human types the tag, and `release.yml` refuses it when the tag
and `tauri.conf.json` disagree rather than publishing an installer named after one and built from
the other.

The path, in order:

1. A branch of its own, `release/<version>`, carrying the bump and the changelog rename and nothing
   else. It enters `dev` by a pull request like every other batch.
2. Once `dev` is green, open the request from `dev` to `main` with its own template:
   `gh pr create --base main --head dev --template release.md`. It is not there to be pressed.
3. `git push origin origin/dev:main`, and the request closes itself as merged.
4. `git tag v<version> origin/main` and push the tag. **The tag is what publishes**: `release.yml`
   runs on tags and on nothing else, builds the NSIS installer on Windows, and uploads it with its
   signature and the manifest the updater reads. The release page takes the changelog section named
   after the tag, and a tag whose section is missing or empty is refused before the build.
5. Open the next version on `dev` straight away, `build/open-next-dev-version`, carrying the four files
   moved to `<next>-dev` and nothing else.

An installed copy compares the releases page against its own `tauri.conf.json`, so a release proves
nothing about the updater on its own: it is the version after it that shows whether an installed
copy finds and takes it.

## Encoding and text

UTF-8 without BOM everywhere, accents included, and LF line endings in the repository and the
working tree alike. On Windows, `Set-Content` and `Out-File -Encoding utf8` both write a BOM: use
`[System.IO.File]::WriteAllText` with `UTF8Encoding($false)`.

Prose uses plain ASCII punctuation and no dash between two spaces. English in code, comments,
commits and documentation.
