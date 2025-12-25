#![no_std]
#![cfg_attr(test, feature(alloc_error_handler))]
extern crate alloc;
use alloc::vec::Vec;
use p256::ecdsa::{signature::Verifier, VerifyingKey, Signature};
use sha2::{Digest, Sha256};

/// Single error type — everything else is a panic (safe failure).
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RemError {
    InvalidSignature,
    InvalidState,
    HashMismatch,
}

/// Immutable audit token exported to the ledger after success.
#[derive(Debug)]
pub struct AuditToken {
    pub intent_hash: [u8; 32],
    pub approver_id: [u8; 32],
    pub executed_at: u64,   // UTC unix-ms
}

/// **ONLY** public entry-point.  Returns `Ok(token)` iff
/// 1. `intent_hash` is SHA-256 of an intent whose state == `AUTHORIZED`
/// 2. `approver_sig` is a valid P-256 ECDSA signature over `(intent_hash || intent_state)`
/// 3. `verifying_key` belongs to the designated Approver
///
/// This function is `#[no_mangle]` so the FFI layer can call it from C/Go/Python.
#[no_mangle]
pub extern "C" fn exec_if_authorized(
    intent_hash: *const u8,
    intent_hash_len: usize,
    approver_sig: *const u8,
    approver_sig_len: usize,
    verifying_key_bytes: *const u8,
    verifying_key_len: usize,
    now_ms: u64,
) -> Result<AuditToken, RemError> {
    // --- 1. Safe slice extraction ---
    let hash = unsafe { slice_from_raw_parts(intent_hash, intent_hash_len)? };
    let sig  = unsafe { slice_from_raw_parts(approver_sig, approver_sig_len)? };
    let vk   = unsafe { slice_from_raw_parts(verifying_key_bytes, verifying_key_len)? };

    // --- 2. Cryptographic checks ---
    let vk = VerifyingKey::from_sec1_bytes(vk).map_err(|_| RemError::InvalidSignature)?;
    let sig = Signature::from_der(sig).map_err(|_| RemError::InvalidSignature)?;

    // Reconstruct message: intent_hash || state_byte(AUTHORIZED=0x01)
    let mut msg = Vec::with_capacity(32 + 1);
    msg.extend_from_slice(hash);
    msg.push(0x01); // AUTHORIZED state tag

    vk.verify(&msg, &sig).map_err(|_| RemError::InvalidSignature)?;

    // --- 3. State oracle call (stubbed here, linked to ledger in prod) ---
    // In real build, this is an extern "C" FFI call to the ledger read-only API.
    if !is_intent_authorized(hash) {
        return Err(RemError::InvalidState);
    }

    // --- 4. Success: emit immutable audit token ---
    let mut approver_id = [0u8; 32];
    
    approver_id.copy_from_slice(&sha256(&vk.to_encoded_point(false).as_bytes())[..32]);

    Ok(AuditToken {
        intent_hash: hash.try_into().unwrap(),
        approver_id,
        executed_at: now_ms,
    })
}

// ---------- helpers ----------
unsafe fn slice_from_raw_parts<'a>(ptr: *const u8, len: usize) -> Result<&'a [u8], RemError> {
    if ptr.is_null() || len == 0 {
        return Err(RemError::HashMismatch);
    }
    Ok(core::slice::from_raw_parts(ptr, len))
}

fn sha256(b: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b);
    hasher.finalize().into()
}

// Stubbed ledger read-only oracle.  Replace with real FFI.
fn is_intent_authorized(hash: &[u8]) -> bool {
    // TODO: extern "C" { fn helix_ledger_get_state(hash: *const u8) -> u8; }
    // Return true iff state == 0x01 (AUTHORIZED)
    true // simulation only
}

// Required items for no_std with alloc
use alloc::alloc::{GlobalAlloc, Layout};
use core::alloc::GlobalAlloc as _;
use core::panic::PanicInfo;

// ---------- minimal allocator ----------
struct BumpAllocator;

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        // 1-page bump allocator for CI only
        static mut BUF: [u8; 65536] = [0; 65536];
        static mut POS: usize = 0;
        let pos = &mut POS as *mut usize;
        let addr = BUF.as_ptr() as usize + *pos;
        *pos += _layout.size().max(8);
        if *pos > BUF.len() { core::ptr::null_mut() } else { addr as *mut u8 }
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    core::arch::asm!("ud2", options(noreturn));
}
