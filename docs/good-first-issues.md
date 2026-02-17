# 🌟 Good First Issues (Contracts)

Here are 10 issues specifically for our Soroban Smart Contracts. These are great for developers looking to learn Rust and the Soroban SDK.

## 🟢 Easy / Good First Issue

### 1. `docs: Add Rustdoc to Liquidity Pool`
**Context**: The `contracts/liquidity_pool/src/lib.rs` file lacks documentation.
**Task**: Add `///` comments to public functions (`initialize`, `deposit`, `swap`, `withdraw`) explaining parameters and return values.

### 2. `test: Add Zero-Amount Deposit Test`
**Context**: `contracts/liquidity_pool/src/test.rs` doesn't explicitly test depositing 0 tokens.
**Task**: Write a test case that attempts to call `deposit` with 0 amounts and assert the expected error or behavior.

### 3. `refactor: Use 'const' for Error Codes`
**Context**: `contracts/liquidity_pool/src/lib.rs` uses an enum with integer values.
**Task**: Refactor the `Error` enum to use explicit `#[repr(u32)]` if not already presenting clearly, or add helper methods to convert errors to readable strings for debugging.

### 4. `test: Verify Event Emission in Token Contract`
**Context**: `contracts/token` emits events on transfer.
**Task**: Add a test case in `contracts/token/src/test.rs` that specifically asserts that the correct topics and data are published during a `transfer`.

---

## 🟡 Medium / Feature

### 5. `feat: Implement 'approve' in Liquidity Pool`
**Context**: LP shares should follow CAP-46.
**Task**: Implement the `approve` and `allowance` functions in `contracts/liquidity_pool` to allow spending shares.

### 6. `feat: Implement 'transfer_from' in Liquidity Pool`
**Context**: Required for full CAP-46 compliance.
**Task**: Implement `transfer_from` to allow spending authorized LP shares.

### 7. `feat: Add 'burn' to Liquidity Pool`
**Context**: Users can withdraw, but a dedicated `burn` function that mirrors the token interface is missing.
**Task**: Implement `burn(from, amount)` which calls `withdraw` internally or shares logic.

### 8. `feat: Add Pausable Functionality`
**Context**: Smart contracts often need an emergency stop.
**Task**: 
1. Add a `paused` boolean state to `LiquidityPool`. 
2. Add `set_paused(bool)` (admin only).
3. Ensure `deposit` and `swap` revert when paused.

---

## 🔴 Hard / Advanced

### 9. `feat: Admin Fee Control`
**Context**: The 0.3% fee is hardcoded.
**Task**: Implement dynamic fee setting (store fee in ledger, add admin setter, read in swap).

### 10. `test: Fuzz Testing for Swap Formula`
**Context**: The CPMM formula in `swap` is critical.
**Task**: Create a property-based test (using `proptest` or a loop with random inputs) to verify that `k` (invariant) never decreases after a swap (ignoring fees).
