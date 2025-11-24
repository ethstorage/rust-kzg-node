use hex::encode;
use napi::{bindgen_prelude::*, Error, Status};
use napi_derive::napi;
use rayon::prelude::*;

// KZG library imports
use kzg::{
  eip_4844::{blob_to_kzg_commitment_raw, load_trusted_setup_rust},
  eth::eip_7594::{compute_cells_and_kzg_proofs_raw, CellsKzgProofs},
  eth::{BYTES_PER_BLOB, BYTES_PER_PROOF},
  G1,
};
use rust_kzg_blst::eip_7594::BlstBackend;
use rust_kzg_blst::types::kzg_settings::FsKZGSettings;

/// KZG wrapper class, holding the Trusted Setup parameters for computation.
/// The internal settings are thread-safe and used across parallel operations.
#[napi]
pub struct KzgWrapper {
  settings: FsKZGSettings,
}

#[napi]
impl KzgWrapper {
  /// Loads the KZG Trusted Setup from G1/G2 monomial and Lagrange byte arrays.
  /// Returns: KzgWrapper instance or a specific error message on failure.
  #[napi(factory)]
  pub fn load_kzg(
    g1_monomial_bytes: Uint8Array,
    g1_lagrange_bytes: Uint8Array,
    g2_monomial_bytes: Uint8Array,
  ) -> Result<Self> {
    let settings = load_trusted_setup_rust(
      g1_monomial_bytes.as_ref(),
      g1_lagrange_bytes.as_ref(),
      g2_monomial_bytes.as_ref(),
    )
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to load trusted setup: {:?}", e),
      )
    })?;

    Ok(Self { settings })
  }

  /// Converts a single blob into a KZG commitment.
  /// - `blob_bytes`: The blob byte array.
  /// Returns: Commitment as a "0x..." prefixed hex string.
  #[napi]
  pub fn blob_to_commitment(&self, blob_bytes: Uint8Array) -> Result<String> {
    self.process_single_commitment(&blob_bytes)
  }

  /// **Batch** and **concurrently** converts multiple blobs to KZG commitments using Rayon.
  /// - `blobs_bytes`: A vector of blob byte arrays.
  /// Returns: A vector of commitment strings.
  #[napi]
  pub fn blob_to_commitment_batch(&self, blobs_bytes: Vec<Uint8Array>) -> Result<Vec<String>> {
    blobs_bytes
      .par_iter()
      .map(|blob_bytes| self.process_single_commitment(blob_bytes))
      .collect()
  }

  /// Computes the KZG proofs for all cells of a single blob.
  /// - `blob_bytes`: The blob byte array.
  /// Returns: An array of proof strings (each prefixed with "0x").
  #[napi]
  pub fn compute_cell_proofs(&self, blob_bytes: Uint8Array) -> Result<Vec<String>> {
    self.process_single_blob_proof(&blob_bytes)
  }

  /// **Batch** and **concurrently** computes cell KZG proofs for multiple blobs using Rayon.
  /// - `blobs_bytes`: A vector of blob byte arrays.
  /// Returns: A 2D array of proof strings.
  #[napi]
  pub fn compute_cell_proofs_batch(
    &self,
    blobs_bytes: Vec<Uint8Array>,
  ) -> Result<Vec<Vec<String>>> {
    blobs_bytes
      .par_iter()
      .map(|blob_bytes| self.process_single_blob_proof(blob_bytes))
      .collect()
  }

  // ------------------ Internal Reusable Logic ------------------

  /// Internal: Processes a single blob to commitment (reused by single and batch methods).
  fn process_single_commitment(&self, blob_bytes: &Uint8Array) -> Result<String> {
    let blob_array = Self::parse_blob_array(blob_bytes)?;

    let commitment = blob_to_kzg_commitment_raw(blob_array, &self.settings).map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to convert blob to commitment: {:?}", e),
      )
    })?;

    let commitment_bytes: [u8; 48] = G1::to_bytes(&commitment);
    Ok(format!("0x{}", encode(commitment_bytes)))
  }

  /// Internal: Processes a single blob to compute Cell proofs (reused by single and batch methods).
  fn process_single_blob_proof(&self, blob_bytes: &Uint8Array) -> Result<Vec<String>> {
    let blob_array = Self::parse_blob_array(blob_bytes)?;

    let (_, proofs): CellsKzgProofs =
      compute_cells_and_kzg_proofs_raw::<BlstBackend>(blob_array, &self.settings).map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("Failed to compute cell proofs: {}", e),
        )
      })?;

    let proof_strings = proofs
      .into_iter()
      // Re-added explicit type hint using BYTES_PER_PROOF for clarity and to remove the "unused import" warning.
      .map(|proof_bytes: [u8; BYTES_PER_PROOF]| format!("0x{}", encode(proof_bytes)))
      .collect();

    Ok(proof_strings)
  }

  /// Universal: Converts Uint8Array to a fixed-size blob array, validating length.
  fn parse_blob_array(blob_bytes: &Uint8Array) -> Result<[u8; BYTES_PER_BLOB]> {
    let slice = blob_bytes.as_ref();
    if slice.len() != BYTES_PER_BLOB {
      return Err(Error::new(
        Status::InvalidArg,
        format!(
          "Invalid blob length: expected {} bytes, got {} bytes",
          BYTES_PER_BLOB,
          slice.len()
        ),
      ));
    }
    let mut blob_array = [0u8; BYTES_PER_BLOB];
    blob_array.copy_from_slice(slice);
    Ok(blob_array)
  }
}
