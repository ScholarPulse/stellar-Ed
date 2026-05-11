#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Map, Symbol, Vec,
};

// ── Storage Keys ──────────────────────────────────────────────────────────────
const STATS: Symbol = symbol_short!("STATS");
const WAVE: Symbol = symbol_short!("WAVE");
const ADMIN: Symbol = symbol_short!("ADMIN");

// ── Types ─────────────────────────────────────────────────────────────────────
#[contracttype]
#[derive(Clone)]
pub struct StudentStats {
    pub points: u32,
    pub modules_completed: u32,
    pub drip_rate: u64, // micro-USDC per second
}

#[contracttype]
#[derive(Clone)]
pub struct Wave {
    pub id: u32,
    pub title: soroban_sdk::String,
    pub end_ledger: u32,
    pub pool_amount: i128, // stroops / smallest unit
    pub active: bool,
}

// ── Contract ──────────────────────────────────────────────────────────────────
#[contract]
pub struct ScholarPulseContract;

#[contractimpl]
impl ScholarPulseContract {
    // ── Admin ──────────────────────────────────────────────────────────────
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        env.storage().instance().set(&ADMIN, &admin);
    }

    // ── Module Completion ──────────────────────────────────────────────────
    /// Instructor verifies a student passed a module and awards points.
    pub fn complete_module(
        env: Env,
        instructor: Address,
        student: Address,
        points_awarded: u32,
    ) {
        instructor.require_auth();
        Self::require_instructor(&env, &instructor);

        let mut stats = Self::get_stats_internal(&env, &student);
        stats.points += points_awarded;
        stats.modules_completed += 1;
        // Drip rate scales linearly: 1 point = 10 micro-USDC/sec
        stats.drip_rate = (stats.points as u64) * 10;

        let key = (STATS, student.clone());
        env.storage().instance().set(&key, &stats);

        env.events().publish(
            (symbol_short!("mod_pass"), student),
            (points_awarded, stats.drip_rate),
        );
    }

    // ── Wave (Competition) ─────────────────────────────────────────────────
    /// Admin opens a new monthly Knowledge Sprint wave.
    pub fn open_wave(
        env: Env,
        admin: Address,
        wave_id: u32,
        title: soroban_sdk::String,
        duration_ledgers: u32,
        pool_amount: i128,
    ) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let wave = Wave {
            id: wave_id,
            title,
            end_ledger: env.ledger().sequence() + duration_ledgers,
            pool_amount,
            active: true,
        };
        let key = (WAVE, wave_id);
        env.storage().instance().set(&key, &wave);

        env.events()
            .publish((symbol_short!("wave_open"), wave_id), pool_amount);
    }

    /// Close a wave and emit top-earner event for the reward service to act on.
    pub fn close_wave(env: Env, admin: Address, wave_id: u32, top_students: Vec<Address>) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let key = (WAVE, wave_id);
        let mut wave: Wave = env.storage().instance().get(&key).expect("wave not found");
        wave.active = false;
        env.storage().instance().set(&key, &wave);

        env.events()
            .publish((symbol_short!("wave_end"), wave_id), top_students);
    }

    // ── Views ──────────────────────────────────────────────────────────────
    pub fn get_stats(env: Env, student: Address) -> StudentStats {
        Self::get_stats_internal(&env, &student)
    }

    pub fn get_wave(env: Env, wave_id: u32) -> Wave {
        let key = (WAVE, wave_id);
        env.storage().instance().get(&key).expect("wave not found")
    }

    // ── Instructor Registry ────────────────────────────────────────────────
    pub fn add_instructor(env: Env, admin: Address, instructor: Address) {
        admin.require_auth();
        Self::require_admin(&env, &admin);
        let key = (symbol_short!("INSTR"), instructor);
        env.storage().instance().set(&key, &true);
    }

    // ── Helpers ────────────────────────────────────────────────────────────
    fn get_stats_internal(env: &Env, student: &Address) -> StudentStats {
        let key = (STATS, student.clone());
        env.storage().instance().get(&key).unwrap_or(StudentStats {
            points: 0,
            modules_completed: 0,
            drip_rate: 0,
        })
    }

    fn require_admin(env: &Env, caller: &Address) {
        let admin: Address = env.storage().instance().get(&ADMIN).expect("no admin");
        if *caller != admin {
            panic!("not admin");
        }
    }

    fn require_instructor(env: &Env, caller: &Address) {
        let key = (symbol_short!("INSTR"), caller.clone());
        if !env.storage().instance().get::<_, bool>(&key).unwrap_or(false) {
            panic!("not instructor");
        }
    }
}
