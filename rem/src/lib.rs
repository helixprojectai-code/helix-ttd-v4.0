// The Risk Enforcement Module (REM) Stubs

#[no_mangle]
pub extern "C" fn helix_ledger_get_state(hash: *const u8) -> u8 {
    // Stub: In prod, this checks the immutable ledger
    // Return 0 for now
    0
}

#[no_mangle]
pub extern "C" fn helix_custodian_exists(vk_hash: *const u8) -> bool {
    // Stub: Checks if a custodian key is registered
    true
}
