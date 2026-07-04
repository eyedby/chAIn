# chAIn: Sovereign Infrastructure
**Autonomous. Local-First. Immutable.**

`chAIn` is not a service; it is a kernel-level protocol that enforces device sovereignty through eBPF packet-filtering and ledger-based integrity verification.

## 🚀 The Sovereign Launch
To transition your device into a sovereign node, run the following sequence:

1. **Initialize:** `chain config --init --sns-handle "yournode.sol"`
2. **Seal:** `chain seal --manifest ./manifest.toml`
3. **Launch:** `chain seal --launch --rpc "https://api.mainnet-beta.solana.com"`

## 🍕 The Applet Engine
Manage your device’s security footprint with the "Pizza Counter" routine. Toggle applets in your `manifest.toml`:
- `5g-scrape-block = true`
- `sovereign-mesh = false`

## 🛡 Security Policy
See [SECURITY.md](./SECURITY.md) for the technical breakdown of our eBPF "Dead-Bolt" and biometric-bound cryptographic triggers.

## ⚡ Energy Decentralization
`chAIn` eliminates background telemetry, reducing the "trash-energy" consumption of your device. View your reclaimed wattage via: `chain audit --efficiency`

---

## ª aiOut: Sovereign Digital Infrastructure
aiOut is a local-first, privacy-preserving infrastructure layer that puts you back in charge of your own digital destiny. We reject the "corporate-managed" model in favor of a sovereign, code-enforced **Consent Stack**.

### The Philosophy
In the current web, you are a guest in someone else’s server room. We’ve built a system where **you are the infrastructure.**

* **Local-First:** All data and execution logic stay on your hardware.
* **Authentication:** Every interaction is authenticated by your device’s Secure Enclave.
* **Protocol-Driven:** No central servers, no backdoors, no "phony" metrics. Just immutable code anchored to the permaweb.

### How It Works
1. **The Protection Circuit (NIC Level):** aiOut uses eBPF/XDP to filter network traffic at the kernel level. Before a packet touches your browser or OS, the NIC drops unauthorized requests from scrapers and trackers.
2. **The Identity Hub (SNS + DID):** Each device is a sovereign entity. We link your hardware to a unique SNS resolution. Interactions are cryptographically signed proofs, not tracking cookies.
3. **The "Protect My Identity" Action:** Your device performs a WebAuthn ceremony, cryptographically signing your intent and updating local filtering rules—zero third-party telemetry.

![chAIn Architecture](https://github.com/user-attachments/assets/b505e9c3-0ce5-44b8-9efa-5bd261ab0a4a)

---

## Quick Start
1. **Clone:** `git clone https://github.com/eyedby/aiout.git`
2. **Configure:** Update `manifest.toml` with your specific SNS handle and DID.
3. **Deploy:** Pin your configuration to the Protocol.Land/Arweave network.
4. **Activate:** Run the local agent to initialize the kernel-level XDP filter. 

![Local Agent Interface](https://github.com/user-attachments/assets/33697345-b75d-475c-a218-f51e768fec07)

---

## Why Open Source?
Trust is verified by math, not promised by a company.
* **Verified:** Inspect code in `/kernel` and `/agent`.
* **Immutable:** Anchored to the Arweave permaweb.
* **Independent:** Functions with or without the "main" web.

## Roadmap
- [ ] Stabilize XDP-based scraper-blocking kernel module.
- [ ] Finalize biometric-to-SNS binding logic.
- [ ] Release the "Bells and Whistles" UI for non-CLI interactions.

**License:** MIT | **Contribute:** Git/Protocol.Land
