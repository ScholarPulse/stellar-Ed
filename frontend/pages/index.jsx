import { useState } from "react";
import { isConnected, getAddress, signTransaction } from "@stellar/freighter-api";
import { Contract, SorobanRpc, TransactionBuilder, Networks, BASE_FEE } from "@stellar/stellar-sdk";

const RPC = "https://soroban-testnet.stellar.org";
const CONTRACT_ID = process.env.NEXT_PUBLIC_CONTRACT_ID || "";

export default function Dashboard() {
  const [wallet, setWallet] = useState(null);
  const [stats, setStats] = useState(null);
  const [status, setStatus] = useState("");

  async function connect() {
    if (!(await isConnected())) return alert("Install Freighter wallet");
    const { address } = await getAddress();
    setWallet(address);
    await fetchStats(address);
  }

  async function fetchStats(address) {
    const server = new SorobanRpc.Server(RPC);
    const contract = new Contract(CONTRACT_ID);
    const account = await server.getAccount(address);
    const tx = new TransactionBuilder(account, { fee: BASE_FEE, networkPassphrase: Networks.TESTNET })
      .addOperation(contract.call("get_stats", ...[]))
      .setTimeout(30)
      .build();
    const sim = await server.simulateTransaction(tx);
    if (SorobanRpc.Api.isSimulationSuccess(sim)) {
      const val = sim.result.retval.value();
      setStats({ points: Number(val[0].value()), modules: Number(val[1].value()), dripRate: Number(val[2].value()) });
    }
  }

  return (
    <main style={{ fontFamily: "sans-serif", maxWidth: 640, margin: "60px auto", padding: "0 16px" }}>
      <h1>🎓 ScholarPulse</h1>

      {!wallet ? (
        <button onClick={connect}>Connect Freighter</button>
      ) : (
        <>
          <p style={{ wordBreak: "break-all" }}>
            <strong>Wallet:</strong> {wallet}
          </p>
          {stats ? (
            <div style={{ border: "1px solid #ddd", borderRadius: 8, padding: 16, marginTop: 16 }}>
              <h2>Your Stats</h2>
              <p>🏅 Points: <strong>{stats.points}</strong></p>
              <p>📚 Modules Completed: <strong>{stats.modules}</strong></p>
              <p>💧 Drip Rate: <strong>{(stats.dripRate / 1e6).toFixed(6)} USDC/sec</strong></p>
            </div>
          ) : (
            <p>Loading stats…</p>
          )}
          {status && <p style={{ color: "green" }}>{status}</p>}
        </>
      )}
    </main>
  );
}
