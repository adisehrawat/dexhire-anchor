import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'

import dexhireIDL from './dexhire.json'
import type { Dexhire } from './dexhire'

export {dexhireIDL,Dexhire}

export const DEXHIRE_PROGRAM_ID = new PublicKey(dexhireIDL.address)

export function getDexhireProgram(provider: AnchorProvider, address?: PublicKey): Program<Dexhire> {
    return new Program({ ...dexhireIDL, address: address ? address.toBase58() : dexhireIDL.address} as Dexhire, provider)
}

export function getDexhireProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
        return new PublicKey('53AaBgyUc8W2DbT2qEHYVdL2z7c9CGabF139WAeiTRKz')
    case 'testnet':
      return new PublicKey('53AaBgyUc8W2DbT2qEHYVdL2z7c9CGabF139WAeiTRKz')
    case 'mainnet-beta':
        return new PublicKey('53AaBgyUc8W2DbT2qEHYVdL2z7c9CGabF139WAeiTRKz')
    default:
      return DEXHIRE_PROGRAM_ID
  }
}