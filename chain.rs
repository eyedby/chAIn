// src/chain.rs
pub fn toggle_applet(applet: &str, state: bool) -> Result<(), String> {
    // 1. Read manifest.toml
    let mut manifest = read_manifest()?;
    
    // 2. Update the "topping" (Applet state)
    manifest.applets.insert(applet.to_string(), state);
    
    // 3. Save and trigger the Re-Seal routine
    save_manifest(&manifest)?;
    seal_and_anchor()?;
    
    Ok(())
}

fn seal_and_anchor() -> Result<(), String> {
    // This executes the eBPF filter update and signs the manifest
    let hash = generate_manifest_hash();
    let signature = secure_enclave::sign_hash(hash)?;
    solana::submit_anchor(signature, hash)
}
