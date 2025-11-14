# rust-kzg-node

High-performance KZG Cryptographic Library for Node.js, powered by Rust.

**rust-kzg-node** provides fast, reliable, and cross-platform bindings for KZG (Kate, Zaverucha, Goldberg) polynomial
commitments, essential for Ethereum's Data Availability Sampling (DAS) and EIP-4844/EIP-7594 implementation. The core
logic is built on the robust and battle-tested rust-kzg library, ensuring native speeds in your Node.js application.

## ✨ Features

- Native Performance: Core cryptographic operations are executed in Rust, bypassing JavaScript overhead.
- Parallel Batching: Utilizes the Rayon crate to execute batch commitment and proof computations concurrently across
  available CPU cores.
- Full API Coverage: Supports EIP-4844 (blob_to_commitment) and EIP-7594 (compute_cell_proofs).
- Cross-Platform Binaries: Includes pre-compiled binaries for all major platforms (Windows, macOS, Linux, and their ARM
  variants).

## 📦 Installation

This package is distributed via npm and includes pre-built native binaries (.node files).

```
npm install rust-kzg-node
# or
yarn add rust-kzg-node
```

## 🛠️ Usage

### 1. Loading the Trusted Setup

The library must be initialized once by loading the necessary G1 and G2 parameters from the trusted setup. These bytes
must be prepared outside of this library.

```
import { KzgWrapper } from 'rust-kzg-node';

// NOTE: Replace these with your actual trusted setup bytes (Uint8Array)
const G1_MONOMIAL_BYTES = new Uint8Array(...);
const G1_LAGRANGE_BYTES = new Uint8Array(...);
const G2_MONOMIAL_BYTES = new Uint8Array(...);

// Initialize the KZG instance
const kzgInstance = KzgWrapper.loadKzg(
  G1_MONOMIAL_BYTES,
  G1_LAGRANGE_BYTES,
  G2_MONOMIAL_BYTES
);

```

### 2. Computing a KZG Commitment

The primary function for EIP-4844. Takes a single blob (32 * 4096 bytes) and returns the KZG commitment.

```
// A single blob array (must be 131072 bytes long)
const blob = new Uint8Array(131072).fill(1); 

try {
    const commitment = kzgInstance.blobToCommitment(blob);
    console.log(`Commitment: ${commitment}`); 
    // Example output: 0x93361414e5a973a9437b084e62217c45... (48 bytes + 0x prefix)
} catch (error) {
    console.error("Error computing commitment:", error.message);
}
```

### 3. Parallel Batch Processing

For maximum performance, use the batch methods. All inputs will be processed concurrently using Rust's Rayon library.

```
const blobs = [
    new Uint8Array(131072).fill(1),
    new Uint8Array(131072).fill(2),
    // ... many more blobs
];

// Computes commitments for all blobs in parallel
const commitments = kzgInstance.blobToCommitmentBatch(blobs);
console.log(`Computed ${commitments.length} commitments in parallel.`);

// Computes cell proofs for all blobs in parallel (EIP-7594)
// Returns Array<Array<string>>
const allProofs = kzgInstance.computeCellProofsBatch(blobs);
console.log(`Computed ${allProofs.length} sets of proofs in parallel.`);
```
