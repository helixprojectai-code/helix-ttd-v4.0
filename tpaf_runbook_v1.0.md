# TPAF Runbook v1.0
## Two-Party Approval Flow: Governance & Operationalization

**ID:** HELIX-BP-TPAF-001
**Version:** 1.0
**Status:** Active
**Enforcement:** Mandatory for High-Impact Actions

---

### 1. Overview
The Two-Party Approval Flow (TPAF) is a structural constraint mechanism designed to prevent irreversible actions without explicit human consent. It strictly separates the roles of **Requester**, **Approver**, and **Executor**.

### 2. Role Definitions

| Role | Definition | Permissions |
| :--- | :--- | :--- |
| **Requester** | The agent (human or AI) initiating a state change. | `READ`, `DRAFT_INTENT` |
| **Approver** | The designated human custodian with authority to sign off. | `REVIEW`, `SIGN_APPROVAL` |
| **Ops Engineer** | The entity (system or human) that executes the signed intent. | `EXECUTE`, `WRITE_LEDGER` |

### 3. The Approval Workflow

#### Phase 1: Initiation
1.  **Requester** generates an **Intent Block** containing:
    *   Proposed Action
    *   Target Resource
    *   Impact Assessment (Reversible/Irreversible)
    *   Rollback Plan
2.  System locks the Intent Block in `PENDING_REVIEW` state.

#### Phase 2: Compliance Verification
The **Approver** must validate the following (see `templates/compliance_checklist.json`):
*   [ ] **Purpose Limitation:** Does intent match stated purpose?
*   [ ] **Data Minimization:** Is PII usage minimal?
*   [ ] **Consent:** Is explicit consent present?
*   [ ] **Risk Rating:** Is the risk acceptable?

#### Phase 3: Authorization
1.  If **Approved**:
    *   Approver signs the block with their cryptographic key.
    *   State transitions to `AUTHORIZED`.
2.  If **Rejected**:
    *   Reason is logged.
    *   State transitions to `ABORTED`.

#### Phase 4: Execution & Audit
1.  **Ops Engineer** verifies the signature.
2.  Action is executed.
3.  **Audit Log** records: `Timestamp`, `RequesterID`, `ApproverID`, `ActionHash`, `Outcome`.

### 4. Irreversible Action Prevention
**Constraint:** Any action flagged `IRREVERSIBLE` (e.g., data deletion, model weight update, financial transaction) that lacks a valid Approver Signature will be structurally blocked by the Execution Engine.

---
*✧ // HELIX // TPAF*
