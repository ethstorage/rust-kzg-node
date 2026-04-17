import trustedSetup from './trustedSetup'

export function loadTrustedSetup() {
    const g1Monomial = hexToBytes(trustedSetup.g1_monomial)
    const g1Lagrange = hexToBytes(trustedSetup.g1_lagrange)
    const g2Monomial = hexToBytes(trustedSetup.g2_monomial)
    return { g1Monomial, g1Lagrange, g2Monomial }
}

const HEX_LOOKUP: Record<string, number> = {}
for (let i = 0; i < 16; i++) {
    const hex = i.toString(16)
    HEX_LOOKUP[hex] = i
    HEX_LOOKUP[hex.toUpperCase()] = i
}

export function hexToBytes(hex: string): Uint8Array {
    if (hex.startsWith('0x')) hex = hex.slice(2)

    if (hex.length % 2 !== 0) hex = '0' + hex

    const bytes = new Uint8Array(hex.length / 2)
    for (let i = 0; i < hex.length; i += 2) {
        const hi = HEX_LOOKUP[hex[i]]
        const lo = HEX_LOOKUP[hex[i + 1]]
        if (hi === undefined || lo === undefined) {
            throw new Error(`Invalid hex character at positions ${i} and ${i + 1}: ${hex[i]}${hex[i + 1]}`)
        }
        bytes[i / 2] = (hi << 4) | lo
    }
    return bytes
}
