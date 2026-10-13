# Quantum Random Seed Oracle

A randomness oracle prototype: a Qiskit circuit produces a random seed, a Calimero peer-to-peer app turns it into an HMAC-SHA256 hash chain, and Calimero's ICP context proxy is used to move seeds and processing requests through on-chain proposals.

**Stack:** Rust · Calimero SDK · Internet Computer (dfx, Candid) · Python · Qiskit · React + TypeScript (scaffold)    **Context:** ICP x Calimero hackathon, 2025

## Overview

Smart contracts can't generate good randomness on their own: every node has to compute the same result, so anything "random" computed on-chain is predictable. The idea here was to take the seed from outside (a quantum circuit), then derive a sequence of values from it inside a Calimero context, where several nodes share the same state, and anchor the requests on the Internet Computer through Calimero's ICP proxy canister.

```
Qiskit generator ──seed──▶ Calimero node(s)                ◀──proposals──▶ ICP: Calimero context-config
 (Python, Aer)              RandomChain app (Rust → WASM)                  + context-proxy canisters
                            seed → HMAC chain of values
```

## How it works

### 1. Seed generation: `src/backend/quantum_seed/quantum_seed_generator.py`

Builds five 20-qubit circuits, puts every qubit into superposition with a Hadamard gate, measures, and concatenates the bitstrings into a 100-bit number (printed as decimal, binary and hex).

Two caveats, both visible in the code:

- It runs on **`AerSimulator`**, Qiskit's classical simulator, so the bits come from the simulator's pseudo-random sampler, not from quantum hardware. Pointing it at a real IBM Quantum backend is the step that would make the "quantum" part real.
- Each circuit is created with a 20-bit classical register and then `measure_all()` adds a second one. The counts key therefore holds 20 measured bits followed by 20 unused zeros, and after truncation to 100 characters only 60 of the 100 bits are random. Measuring into the existing register (`qc.measure(range(20), range(20))`) or building the circuit without classical bits would fix it.

### 2. RandomChain Calimero app: `src/app/random/src/lib.rs`

A Calimero application (Rust compiled to `wasm32-unknown-unknown`) whose state is replicated across the nodes of a context:

| Method | What it does |
|---|---|
| `set_seed(quantum_seed: Vec<u8>)` | Stores the seed (rejects empty input or anything over 1 KiB) |
| `process_value(node)` | Derives the next value and appends it with the node name and a sequence number; emits a `ValueProcessed` event |
| `get_all_values()` / `get_last_value()` | Read the chain |

The derivation is a hash chain: `value[n] = HMAC-SHA256(key = value[n-1], msg = n as u64 big-endian)`, with `value[0]` keyed by the seed. Each value is 32 bytes. The chain is deterministic, so anyone with the seed can recompute and check every value; equally, the values are only as unpredictable as the seed is secret.

### 3. ICP proxy contract: `src/backend/proxy_contract/proxy_contract.rs`

A second Calimero app that uses the SDK's external-proposal API. `create_new_proposal` accepts `SetQuantumSeed` or `ProcessValue` requests and turns them into proposals for an external function call (`random_chain.set_seed` / `random_chain.process_value`) that goes through Calimero's context proxy canister on ICP; `approve_proposal` approves one. It also keeps seeds and processed values per proposal ID. This file has no `Cargo.toml` of its own in this repo.

### 4. ICP devnet tooling: `tools/`

- `context-config/` and `context-proxy/`: prebuilt Calimero ICP canisters (`.wasm` + `.did`) and the mock external canister, from Calimero's [icp-devnet](https://github.com/calimero-network/icp-devnet).
- `deploy_devnet.sh`: a merged version of Calimero's fresh/addon devnet deploy scripts (identities, ledger, cycles, context contract, proxy code). See [`tools/tools.md`](tools/tools.md).
- `init_and_run_nodes.sh` / `setup_calimero_icp.sh`: initialise and run three `merod` nodes, install an app on node1, create an ICP-backed context, generate identities for node2 and node3, invite them and join. The resulting IDs (and, in the setup script, node keys) go to a local `.env`, which is gitignored.
- `upnodes.py` / `setup.py`: `pexpect` scripts that start `merod`, find the context and identity IDs, and run `set_seed` → `process_value` → `get_last_value` end to end.

### 5. Frontend: `src/frontend/`

A Vite + React 18 + TypeScript + Tailwind 4 scaffold wired to `@calimero-network/calimero-client`: node setup page, Calimero login page, and a placeholder home page. It does not yet display random values.

## Build & run

Prerequisites: Rust with the `wasm32-unknown-unknown` target, Python 3, `dfx` (the Makefile pins 0.24.3), Calimero's `merod` / `meroctl`, Node.js + pnpm for the frontend.

```bash
# Seed generator (creates ./venv, installs qiskit + qiskit-aer, runs the script)
./cli_script.sh

# Build the RandomChain app to src/app/random/res/quantum_rng.wasm
./src/app/random/build.sh

# Local ICP devnet with the Calimero canisters
./tools/deploy_devnet.sh

# Calimero nodes (node1..node3), app install, ICP-backed context
./tools/init_and_run_nodes.sh

# Frontend scaffold
cd src/frontend && pnpm install && pnpm dev
```

Calling the app directly with `meroctl`:

```bash
meroctl --node-name node1 call --as <IDENTITY> <CONTEXT_ID> set_seed --args '{"quantum_seed": [10,20,30,40]}'
meroctl --node-name node1 call --as <IDENTITY> <CONTEXT_ID> process_value --args '{"node": "node1"}'
meroctl --node-name node1 call --as <IDENTITY> <CONTEXT_ID> get_all_values
```

The node scripts install a hard-coded WASM path (`./res/blockchain.wasm` in `init_and_run_nodes.sh`, a `hello_app.wasm` in `setup_calimero_icp.sh`); point it at `src/app/random/res/quantum_rng.wasm` to run the oracle app.

Note: the `Makefile` comes from an earlier PoW-mining direction of the project. It calls `./icp-devnet/deploy_devnet_*.sh` and a `mining_contract` canister; `icp-devnet` is recorded as a submodule without a `.gitmodules` entry, so a fresh clone gets an empty directory. Use `tools/deploy_devnet.sh` instead.

## Project structure

```
src/
  app/random/              RandomChain Calimero app (the oracle logic) + built .wasm
  app/mining_pool/         Earlier experiment: PoW mining pool as a Calimero app
  backend/quantum_seed/    Qiskit seed generator
  backend/proxy_contract/  Calimero app that proposes calls via the ICP context proxy
  logic/                   Calimero proposal/chat demo app (ExternalFunctionCall proposals)
  frontend/                React + Calimero client scaffold
tools/                     ICP devnet deploy, node setup scripts, Calimero canisters
exp/                       Learning experiments: hello-icp, key-value stores, macros, mining pool
docs/                      Notes on Calimero, merod/meroctl, consensus and the PoW-on-ICP idea
```

## Credits

The ICP devnet scripts and the context-config / context-proxy canisters are from [Calimero Network](https://github.com/calimero-network/icp-devnet). Several files in `docs/` are adapted from Calimero's documentation.
