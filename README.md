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
