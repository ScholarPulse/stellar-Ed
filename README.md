# 🎓 ScholarPulse: Decentralized Education & Rewards

ScholarPulse is a "Proof-of-Learning" platform built on **Soroban** and **Drips**. Students earn real-time USDC streams as they complete verified learning modules, and compete in monthly **Knowledge Sprint Waves** for bonus rewards.

---

## 🏗 Project Structure

```
stellar-Ed/
├── contracts/scholar_pulse/   # Soroban smart contract (Rust)
│   ├── Cargo.toml
│   └── src/lib.rs
├── services/
│   └── reward_service.js      # Drips stream manager (Node.js)
├── frontend/
│   ├── pages/
│   │   ├── _app.jsx
│   │   └── index.jsx          # Dashboard + Freighter wallet
│   └── next.config.js
├── .soroban/config.yml        # Network config
├── .env.example
├── Cargo.toml                 # Workspace root
└── package.json
```

---

## 🛠 Tech Stack

| Layer | Technology |
|---|---|
| Smart Contract | Rust / Soroban (Stellar) |
| Reward Engine | Drips Network SDK |
| Frontend | Next.js + Freighter Wallet |
| RPC | Soroban Testnet / Mainnet |

---

## 🚀 Setup

### Prerequisites
- [Rust + `wasm32` target](https://www.rust-lang.org/tools/install): `rustup target add wasm32-unknown-unknown`
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-cli): `cargo install stellar-cli`
- Node.js ≥ 18
- [Freighter browser wallet](https://www.freighter.app/)

### 1. Clone & configure

```bash
git clone <repo>
cd stellar-Ed
cp .env.example .env
# Fill in CONTRACT_ID and ADMIN_SECRET after deploying
```

### 2. Build & deploy the contract

```bash
npm run contract:build
# Fund a testnet account first:
stellar keys generate admin --network testnet --fund
npm run contract:deploy
# Copy the printed contract ID into .env
```

### 3. Initialize the contract

```bash
stellar contract invoke --id $CONTRACT_ID --network testnet \
  -- initialize --admin $(stellar keys address admin)
```

### 4. Add an instructor

```bash
stellar contract invoke --id $CONTRACT_ID --network testnet \
  -- add_instructor \
  --admin $(stellar keys address admin) \
  --instructor <INSTRUCTOR_ADDRESS>
```

### 5. Start the reward service

```bash
npm install
npm run service
```

### 6. Run the frontend

```bash
cd frontend
npm install
npm run dev
# Open http://localhost:3000
```

---

## 🏆 Competition Model (Knowledge Sprint Wave)

1. **Admin opens a Wave** — sets a pool amount and duration in ledgers.
2. **Students complete modules** — instructors call `complete_module`; points accumulate on-chain.
3. **Wave closes** — admin calls `close_wave` with the top-student list; the reward service distributes the bonus pool via Drips.

### Key contract functions

| Function | Caller | Description |
|---|---|---|
| `initialize(admin)` | deployer | Set contract admin |
| `add_instructor(admin, instructor)` | admin | Grant instructor role |
| `complete_module(instructor, student, points)` | instructor | Award points + update drip rate |
| `open_wave(admin, id, title, duration, pool)` | admin | Start a Knowledge Sprint |
| `close_wave(admin, id, top_students)` | admin | End wave, emit payout event |
| `get_stats(student)` | anyone | Read points, modules, drip rate |
| `get_wave(wave_id)` | anyone | Read wave details |

---

## 💧 Study Drip Formula

```
drip_rate (micro-USDC/sec) = total_points × 10
```

A student with 100 points earns `0.001 USDC/sec` → `~86.4 USDC/day` at full engagement.
Sponsors deposit into the Drips pool; the contract never holds funds directly.

---

## 🌍 Why Stellar/Soroban?

- **Low fees** — distribute $1 to 1,000 students for pennies.
- **Real-time** — Drips streams update per-second, not per-semester.
- **Verifiable** — all milestones are on-chain; no trust in a central authority.
