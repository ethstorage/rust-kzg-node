import test from 'ava'
import { loadTrustedSetup } from "./utils/core"
import { encodeOpBlobs } from "./utils/blobs"
import { KzgWrapper } from '../index'

const testData = new TextEncoder().encode("this is a test")

test('KzgWrapper loads and exposes methods', (t) => {
  t.truthy(KzgWrapper)
  t.true(typeof KzgWrapper.loadKzg === 'function')
})

test('KzgWrapper can be initialized with trusted setup', (t) => {
  const ts = loadTrustedSetup()
  const kzg = KzgWrapper.loadKzg(
    ts.g1Monomial,
    ts.g1Lagrange,
    ts.g2Monomial
  )
  t.truthy(kzg)
})

test('KzgWrapper can encode data to blobs', (t) => {
  const blobs = encodeOpBlobs(testData)
  t.true(blobs.length > 0)
  t.is(blobs[0].length, 4096 * 32)
})

test('KzgWrapper can compute blob commitment', (t) => {
  const ts = loadTrustedSetup()
  const kzg = KzgWrapper.loadKzg(
    ts.g1Monomial,
    ts.g1Lagrange,
    ts.g2Monomial
  )
  const blobs = encodeOpBlobs(testData)
  const commitment = kzg.blobToCommitment(blobs[0])
  t.true(typeof commitment === 'string')
  t.true(commitment.startsWith('0x'))
  t.is(commitment.length, 98)
})

test('KzgWrapper can compute batch blob commitments', (t) => {
  const ts = loadTrustedSetup()
  const kzg = KzgWrapper.loadKzg(
    ts.g1Monomial,
    ts.g1Lagrange,
    ts.g2Monomial
  )
  const blobs = encodeOpBlobs(testData)
  const commitments = kzg.blobToCommitmentBatch(blobs)
  t.is(commitments.length, blobs.length)
  commitments.forEach(commitment => {
    t.true(commitment.startsWith('0x'))
    t.is(commitment.length, 98)
  })
})

test('KzgWrapper can compute cell proofs', (t) => {
  const ts = loadTrustedSetup()
  const kzg = KzgWrapper.loadKzg(
    ts.g1Monomial,
    ts.g1Lagrange,
    ts.g2Monomial
  )
  const blobs = encodeOpBlobs(testData)
  const proofs = kzg.computeCellProofs(blobs[0])
  t.true(proofs.length > 0)
  proofs.forEach(proof => {
    t.true(proof.startsWith('0x'))
    t.is(proof.length, 98)
  })
})

test('KzgWrapper can compute batch cell proofs', (t) => {
  const ts = loadTrustedSetup()
  const kzg = KzgWrapper.loadKzg(
    ts.g1Monomial,
    ts.g1Lagrange,
    ts.g2Monomial
  )
  const blobs = encodeOpBlobs(testData)
  const proofsBatch = kzg.computeCellProofsBatch(blobs)
  t.is(proofsBatch.length, blobs.length)
  proofsBatch.forEach(proofs => {
    t.true(proofs.length > 0)
    proofs.forEach(proof => {
      t.true(proof.startsWith('0x'))
      t.is(proof.length, 98)
    })
  })
})
