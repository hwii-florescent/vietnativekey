# vietnativekey: future plan

Ideas agreed but not built. Nothing here is implemented unless it says so.

## 1. Sentence-level language decision (local model)

Goal: decide per word, using the whole sentence, whether it should be Vietnamese or English
(e.g. `thí í` -> `this is`, or the reverse), fast enough that typing never feels it.

Today (built): rules only. Last-3-word votes, run rewrite for two ambiguous words in a row, and a
left-Option flip key. See `core/src/engine/mod.rs` (`note_word_for_context`,
`try_ambiguous_run_rewrite`, `flip_last_ambiguous`) and `core/tests/ambiguous_context_test.rs`.

### Why a model must stay out of the keystroke path
- Real typing gaps are about 80-150 ms. The macOS event-tap handler blocks each key until it returns,
  and macOS disables a tap that stays slow. A 30-130 ms model call per key is not acceptable.
- Decisions are only needed at word boundaries, and only when a sentence contains an ambiguous word.

### Architecture to aim for
1. Rules decide instantly on every word (as now). Typing never waits.
2. A model reads the sentence in the background, only when it has an ambiguous word.
3. It corrects only when the user pauses (about 300-400 ms) and only if the screen still matches what
   the engine remembers (same check as the flip key). Never rewrite blindly.
4. On Enter in chat apps, wait for the model with a hard limit (about 100 ms); on timeout, send as the
   rules decided.

### Candidate models (all unverified; claims are from the projects' own pages)
- **jevos** (`feder-cr/jev`): single CPU binary, claims 28-130 ms, GGUF available.
- **Von** (`wfzyx/von`): 395M params, claims under 15 ms, but archived (points to `LiquidAI/d1-omni-600M`).
- **Decima-small**: CPU-sized; part of its training data is non-commercial, check the license.
- smalljev / OpenJev-1.7B / JevK5: 1.7-2.5B, look GPU-oriented.
- TypeSafe Jev itself is hosted and closed-weight: rejected (network latency, text leaves the machine).

### Open risks
- Vietnamese quality is unknown; these are English-centric. Biggest risk.
- Late rewrites of earlier words flicker (delete and retype everything after the word).
- Shipping and notarizing a model file; CPU, memory and battery cost.

### Cheaper option to try first
A tiny in-process tagger (a few MB, microseconds): per word, English or Vietnamese from its letters
plus the previous words' tags. Train on Vietnamese and English text plus simulated Telex typing.
No sidecar process and no pause rule needed.

### Steps, in order
1. Add opt-in, off-by-default logging of ambiguous decisions and user flips (word forms only, never
   full text) to build a real test set.
2. Measure how often the current rules are wrong on it.
3. Try the tiny tagger; run jevos on the same set for comparison.
4. Add the background and pause-correction path only if a model clearly beats the tagger on the
   user's real Vietnamese.

## 2. CrossOver / Wine games

Findings so far:
- CrossOver's generated launchers in `~/Applications` (`Steam.app`, game `.app` wrappers) have **no
  `CFBundleIdentifier`** and run `run.sh`. CrossOver itself is `com.codeweavers.CrossOver`.
- Wine processes run from `~/Applications/CrossOver.app/Contents/SharedSupport/CrossOver/...` and carry
  an environment variable `CX_LAUNCHER_BUNDLE_NAME` (e.g. `Steam`).
- Not yet known: what a game window's `NSRunningApplication` looks like (bundle ID, name, executable).
  The app now logs `activate: name=... bundleId=... exe=...` on every app switch to find out.

Built: an app that comes to the front with no bundle ID gets the key `noid:<name>` and starts with
Vietnamese **off** (fail closed). Toggling it on there is remembered.

To do once we see a game in the log:
- If game windows report `com.codeweavers.CrossOver`: just set that app to the **Game** profile.
- If they report no ID or a shared Wine name: distinguish games by process arguments or
  `CX_LAUNCHER_BUNDLE_NAME`, and let the user mark which ones are games.
- Check that Option/modifier taps and the toggle hotkey reach the game window under Wine.
