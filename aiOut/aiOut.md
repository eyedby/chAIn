aiOut: Sovereign Digital Infrastructure
aiOut is a local-first, privacy-preserving infrastructure layer that puts you back in charge of your own digital destiny. We reject the "corporate-managed" model—where software companies force intrusive updates and track your interactions—in favor of a sovereign, code-enforced "Consent Stack."

The Philosophy
In the current web, you are a guest in someone else’s server room. We’ve built a system where you are the infrastructure.

Local-First: All data and execution logic stay on your hardware.

Biometric-Gated: Every interaction is authenticated by your device’s Secure Enclave, ensuring only you can authorize actions.

Protocol-Driven: No central servers, no backdoors, no "phony" metrics. Just immutable code anchored to the permaweb.

How It Works
1. The Protection Circuit (NIC Level)
aiOut uses eBPF/XDP to filter network traffic at the kernel level. Before a packet ever touches your browser or OS, the NIC drops unauthorized requests from scrapers and trackers. You control the filter; the network obeys the code.

2. The Identity Hub (SNS + DID)
Each device is a sovereign entity. We link your hardware to a unique SNS resolution. When you interact with the digital world, you present a cryptographically signed proof from your device, not a tracking cookie.

3. The "Protect My Identity" Action
The core of aiOut is a one-touch biometric handshake. When you trigger the Protect My Identity action, your device performs a WebAuthn ceremony, cryptographically signing your intent and updating your local filtering rules—all without a single line of telemetry being sent to a third party.

Quick Start
Clone the Repository:

Bash
git clone https://github.com/eyedby/aiout.git
cd aiout
Configure:
Update your manifest.toml with your specific SNS handle and DID.

Deploy:
Use your local agent to pin your configuration to the Protocol.Land/Arweave network.

Activate:
Run the local agent to initialize the kernel-level XDP filter.

Why Open Source?
We believe that trust should be verified by math, not promised by a company. The aiout stack is 100% open source because the code that governs your identity and privacy should be transparent, forkable, and immune to corporate interference.

Verified: Inspect the code in the /kernel and /agent folders.

Immutable: Anchored to the Arweave permaweb.

Independent: Works with or without the "main" web.

Roadmap
[ ] Stabilize XDP-based scraper-blocking kernel module.

[ ] Finalize biometric-to-SNS binding logic.

[ ] Release the "Bells and Whistles" UI for non-CLI interactions.

License: MIT | Contribute via Git/Protocol.Land