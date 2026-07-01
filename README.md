**chAIn OS — Official Specification**  
**Version:** 1.0  
**SNS Core Reference:** `xn--9-4lb.sol`  
**Protocol:** Consent Stack

### chAIn: Sovereign Infrastructure Protocol
The device obeys its last signed instruction until a new instruction is cryptographically enforced.

1. The Directive
chAIn removes the requirement for the user to "manage" an operating system. The system does not "update" unless authorized by the Conductor; it does not "track" unless permitted by the registry. It exists in a state of permanent adherence to its current local policy.

2. Zero-Hoop Governance
The user is not required to jump through hoops, navigate menus, or fight update cycles.

The "Setup" is a singular event: Once chAIn is initiated, the device is locked to the SNS jurisdiction (xn--9-4lb.sol).

The "Maintenance" is non-existent: The kernel-level eBPF filter automatically drops unauthorized packets or process-injection attempts. The system requires no further input from the user to maintain its security posture.

3. The "Forever" Parameter
The device acts with temporal indifference. Whether it is one hour or ten years after the last signed instruction, the device will continue to execute that command with absolute precision.

No "End of Support" dates.

No "Forced Deprecation" of features.

No "UI Overhauls" that hide your controls.

4. Interpretation Only
The GUI is not a control panel for the user to "do work." It is a Transparent Interpreter. It exists solely to translate the device's internal state—what it is currently doing and why—into plain, secure text. The user is a witness to the device's adherence, not a babysitter.

chAIn OS is a decentralized sovereign execution environment that replaces ambiguous data processing with immutable, protocol-enforced logic.

By decoupling the execution engine from the application layer, chAIn ensures all operations strictly follow the **Consent Stack** — a rigid, user-defined framework for permissioning, access control, procedural intent, **and dispute resolution**.

### 2. Architecture

#### 2.1 Consent Stack (Protocol Layer)
The immutable foundation of the system. It defines invitation-only criteria, detour protocols, and user self-governance mechanisms — including dispute resolution. All actions and conflicts are governed and audited against this layer.

#### 2.2 chAIn Core (Execution Engine)
The minimal kernel anchored at `xn--9-4lb.sol`. It executes only pre-defined immutable paths and performs no interpretive or RAG-style processing.

#### 2.3 Utility Layer (Applets)
Independent applets residing in separate repositories. Each applet:
- Resolves exclusively via the chAIn SNS pointer.
- Operates strictly within the boundaries set by the Consent Stack.
- Includes a transparent `README.html` manifest for auditability.

### 3. Core Principles

- **Immutable Pathing**: Data is processed only along pre-approved paths. Non-conforming input is rejected.
- **Sovereign Resolution**: All core references use decentralized SNS naming.
- **Zero-Dependency**: Applets are built with minimal footprints and local, human-readable documentation.
- **Strict Gatekeeping**: No utility or process may interact with the core without explicit authorization via the Invite protocol.

### 4. Governance & Dispute Resolution

The Consent Stack enables complete user self-governance, which includes:
- Deterministic weighted voting based on Token-2022 ledger snapshots.
- **Dispute Resolution**: All disputes, conflicts, and appeals are resolved through the same transparent, on-chain governance process using snapshot-based weighted voting. Resolutions are final and immutable.

### 5. Security Model

chAIn enforces a **Firewall of Intent**. By cleanly separating governance (including dispute resolution) from execution, the system significantly reduces its attack surface. Any unauthorized or non-compliant action is rejected at the engine level. The default response to violations is silence.

---

**Permanence & Provenance**

This specification and official releases are permanently archived on Arweave for immutability and timestamped proof of existence.

Anyone claiming ownership or prior creation of this work must provide an earlier verifiable Arweave (or equivalent permanent) timestamp.

**This document serves as the canonical source of truth** for all entities resolving to `xn--9-4lb.sol`.

---
