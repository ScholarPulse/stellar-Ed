import { DripsClient } from "@drips-network/sdk";
import { SorobanRpc, Contract, Networks } from "@stellar/stellar-sdk";

const RPC_URL = process.env.SOROBAN_RPC_URL || "https://soroban-testnet.stellar.org";
const CONTRACT_ID = process.env.CONTRACT_ID;
const ADMIN_SECRET = process.env.ADMIN_SECRET;

const server = new SorobanRpc.Server(RPC_URL);

/** Increase a student's Drip stream when they pass a module. */
export async function handleModulePass(studentAddress, pointsAwarded, newDripRate) {
  const drips = new DripsClient({ signer: ADMIN_SECRET });

  // newDripRate comes from the contract event (micro-USDC/sec * 1e-6 = USDC/sec)
  const amountPerSecond = newDripRate / 1_000_000;

  await drips.updateStream({
    receiver: studentAddress,
    token: "USDC",
    newAmountPerSecond: amountPerSecond,
  });

  console.log(`[RewardService] ${studentAddress} → ${amountPerSecond} USDC/sec`);
}

/** Distribute wave bonus pool pro-rata to top students. */
export async function distributeWaveBonus(topStudents, poolAmount) {
  const drips = new DripsClient({ signer: ADMIN_SECRET });
  const share = poolAmount / topStudents.length;

  await Promise.all(
    topStudents.map((addr) =>
      drips.give({ receiver: addr, token: "USDC", amount: share })
    )
  );

  console.log(`[RewardService] Wave bonus distributed: ${share} USDC × ${topStudents.length}`);
}

/** Poll Soroban events and dispatch to handlers. */
export async function startEventListener() {
  let cursor = "now";

  setInterval(async () => {
    const events = await server.getEvents({
      startLedger: cursor,
      filters: [{ contractIds: [CONTRACT_ID], topics: [["mod_pass"], ["wave_end"]] }],
    });

    for (const ev of events.events ?? []) {
      const topic = ev.topic[0].value();
      const [addr, data] = [ev.topic[1].value(), ev.value.value()];

      if (topic === "mod_pass") await handleModulePass(addr, data[0], data[1]);
      if (topic === "wave_end") await distributeWaveBonus(data, /* pool */ 0);
    }

    if (events.latestLedger) cursor = events.latestLedger;
  }, 5000);
}
