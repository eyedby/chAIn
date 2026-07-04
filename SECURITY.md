# SECURITY.md: Sovereign Threat Modeling & Kernel Enforcement

This document defines the automated, zero-trust security architecture of the `chAIn` and `aiOut` protocols.

## 1. The Kernel Dead-Bolt (eBPF/XDP Enforcement)
Security vulnerabilities inside the `chAIn` runtime environment are mathematically and programmatically isolated at the Network Interface Card (NIC) layer before they can execute within user space:

*   **Ingress Dropping:** Our native eBPF probe monitors all traffic entering `/tmp/chain_protocol.sock` and system interface hooks.
*   **Tamper Isolation:** Any non-authenticated binary attempting to spoof the hardware key pair triggers an instant `EPERM` (Operation Not Permitted) error, forcing an atomic panic state that isolates the process.

## 2. Hardware Enclave & Biometric Triggers
Cryptographic signing keys do not live in plain text or standard operating system storage layers:
*   **WebAuthn Ceremonies:** Key state modifications, profile configuration adjustments, or token-minting executions require a physical hardware enclave or cryptographic biometric assertion check.
*   **No Third-Party Telemetry:** Security posture monitoring and reclaimed wattage audits (`chain audit --efficiency`) are calculated strictly locally on your host hardware. No crash data or system logs are ever transmitted to an external server.

## 3. Vulnerability Philosophy & Disclosure
Because `chAIn` is an autonomous infrastructure protocol designed for isolated hardware deployment, we do not utilize standard human security disclosure programs:

*   **No Central Reporting:** There is no centralized security team, corporate desk, or email inbox to handle vulnerability reports. 
*   **The Sovereign Rule:** If you discover a vector or exploit path within the code, your primary recourse is to apply the sovereign fork rules outlined in [CONNECT.md](./CONNECT.md). Implement the patch inside your own private, hardware-bound branch.
*   **Immutable Verification:** To guarantee the integrity of your code base against malicious modifications, always cross-reference your repository state with the immutable transaction hash anchored via Protocol.Land.
