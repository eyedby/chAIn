CONNECT.md
This repository is part of the chAIn Sovereign Protocol. It is archived via Protocol.Land. Follow these steps to fork, connect, or adapt the protocol for your own hardware.

Step 1: Forking the Pillar
To adapt chAIn to your own sovereign requirements, you do not need permission. Clone this repository locally. You are responsible for your own source management; treat your fork as a private-source, hardware-bound project.

Step 2: Configure Your Sovereignty
Navigate to the MANIFEST.toml in your cloned repository. Replace the existing DID and SNS_Handle with your own unique identifiers. This step anchors your specific instance of the protocol to your identity on the Solana blockchain.

Step 3: Implement the IPC Bridge
If you are building a tool to interact with the protocol, your software must communicate via the chAIn standard. The Agent exposes a local-only Unix Domain Socket at: /tmp/chain_protocol.sock. Ensure your binary or GUI is configured to interface exclusively through this pipe.

Step 4: Authenticate Local Access
The protocol authenticates all connections based on the hardware keypair residing on the local device. Ensure your fork retains the original key-signing logic; any attempt to bypass or spoof this authentication will trigger an EPERM (Operation Not Permitted) at the kernel level, effectively isolating the unauthorized process.

Step 5: Archival & Verification
Once your fork is operational, it is highly recommended to mirror your changes to Protocol.Land for permanence. Verify the integrity of any code you are forking by checking the Arweave transaction ID associated with the official chAIn pillars.

Step 6: Immutable Governance
chAIn is an autonomous protocol. We do not accept Pull Requests. If you wish to propose a change to the protocol's core logic, implement it in your own fork, test it against the kernel, and maintain it as an independent branch of the protocol.