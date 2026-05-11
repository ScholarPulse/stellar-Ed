# ScholarPulse — Wave Program Contribution Plan

## What Is the Wave Program?

The Wave Program is ScholarPulse's open-source sprint cycle. Maintainers post scoped issues every month. Contributors pick them up, ship the work, and earn on-chain reputation points — the same points the platform awards students. Contributors are builders; the Wave is their classroom.

Each sprint runs for 30 days (roughly 1 Stellar Wave cycle). At the end, merged contributions are tallied, top contributors receive a share of the monthly bonus pool via Drips, and new issues open for the next wave.

---

## How a Wave Works

1. **Wave Opens** — Maintainers label and publish scoped issues tagged `wave-N`.
2. **Claim** — A contributor comments `claim` on an issue. Maintainers assign it within 24 hours.
3. **Build** — Contributor opens a draft PR linked to the issue.
4. **Review** — At least one maintainer reviews. Two approvals required for contract changes.
5. **Merge & Score** — On merge, the contributor's Wave score is updated. Points vary by issue type (see below).
6. **Wave Closes** — Admin calls `close_wave` on-chain. The reward service distributes the bonus pool to the top 10% of contributors.

---

## Types of Work We Post

### 1. Bug Fixes (`type: bug`)
Broken behavior in the contract, reward service, or frontend.

**Examples:**
- `close_wave` does not validate that the wave is still active before closing.
- `get_stats` panics when called for an address that has never interacted with the contract.
- Freighter wallet disconnect does not clear local state in the dashboard.

**Points:** 10–30 depending on severity.
**What we expect:** A failing test that reproduces the bug, the fix, and the test passing.

---

### 2. New Features (`type: feature`)
Additive work that extends the platform's capabilities.

**Examples:**
- **Instructor dashboard** — a `/instructor` page where instructors can submit `complete_module` transactions for their students.
- **Wave leaderboard** — a `/wave/[id]` page that reads on-chain events and ranks students by points earned during that wave.
- **Sponsor deposit flow** — a UI for sponsors to deposit USDC into the Drips pool with a chosen stream duration.
- **Multi-token drip** — extend the contract and reward service to support XLM streams alongside USDC.

**Points:** 40–100 depending on scope.
**What we expect:** Feature-complete implementation, unit tests, and a short demo GIF in the PR description.

---

### 3. Documentation (`type: docs`)
Clear writing that helps contributors and users understand the system.

**Examples:**
- Expand the contract's inline `///` doc comments so `stellar contract doc` generates useful output.
- Write a `CONTRIBUTING.md` that explains the claim → build → review → merge flow.
- Add a `docs/architecture.md` with a sequence diagram showing the full event flow: instructor calls `complete_module` → Soroban event → reward service → Drips stream update.
- Translate the README into Spanish or Portuguese (Stellar's largest non-English communities).

**Points:** 5–20.
**What we expect:** Accurate, concise prose. No padding. PRs that only fix typos score 2 points.

---

### 4. Testing (`type: test`)
Coverage for contract logic, the reward service, and the frontend.

**Examples:**
- Rust unit tests for `open_wave` / `close_wave` state transitions using `soroban-sdk testutils`.
- Test that a non-instructor address calling `complete_module` panics with `"not instructor"`.
- Jest tests for `distributeWaveBonus` mocking the Drips SDK client.
- Playwright end-to-end test: connect Freighter (mock) → read stats → display drip rate.

**Points:** 15–40.
**What we expect:** Tests must run in CI (`cargo test` / `npm test`) with no additional setup.

---

### 5. DevEx & Tooling (`type: tooling`)
Scripts and config that make the development loop faster.

**Examples:**
- A `scripts/seed.sh` that deploys the contract to testnet, initializes it, adds a test instructor, and completes two modules — giving new contributors a running environment in one command.
- GitHub Actions workflow: build the Wasm, run Rust tests, run JS tests on every PR.
- A `Makefile` with targets: `make build`, `make test`, `make deploy-testnet`.

**Points:** 10–25.
**What we expect:** Works on Linux and macOS. Documented in the PR.

---

## Scoring Table

| Type       | Min pts | Max pts |
|------------|---------|---------|
| Bug fix    | 10      | 30      |
| Feature    | 40      | 100     |
| Docs       | 5       | 20      |
| Testing    | 15      | 40      |
| Tooling    | 10      | 25      |

Points feed directly into the on-chain leaderboard via the same `complete_module` mechanism students use. Contributors and students compete on the same scoreboard.

---

## Rules

- One active claim per contributor at a time.
- Unclaimed issues are released after 7 days of inactivity.
- Contract changes require two maintainer approvals and must not break existing tests.
- All contributions must pass CI before review begins.
- Plagiarism or AI-generated code submitted without review disqualifies the contributor from the current wave.

---

## Getting Started

```bash
git clone <repo> && cd stellar-Ed
cp .env.example .env
npm install
npm run contract:build
stellar keys generate dev --network testnet --fund
npm run contract:deploy
```

Browse open issues tagged `wave-current` and comment `claim` on one that fits your skills.
