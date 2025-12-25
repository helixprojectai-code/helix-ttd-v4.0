rem/spec/REM.k   (K-framework stub)
requires "rust.k"
module REM
imports RUST-SYNTAX
// Top-level claim: exec_if_authorized can return Success(_) only when
// 1. signature verifies
// 2. ledger state == AUTHORIZED
// 3. approver key is listed in Custodian registry
rule <k>
<ledger>
<custodian>
requires verify_p256(HASH +Bytes b"\x01", SIG, VK)
rule <k>
requires notBool verify_p256(...)
rule <k>
<ledger>
endmodule
