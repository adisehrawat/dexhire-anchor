import * as anchor from '@coral-xyz/anchor'
import { Program, BN } from '@coral-xyz/anchor'
import { Dexhire } from '../target/types/dexhire'
import { expect } from 'chai'
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js'

describe('dexhire-solana', () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.Dexhire as Program<Dexhire>

  /* ------------------------------------------------------------------
     Helper keypairs
  ------------------------------------------------------------------ */
  const owner = Keypair.generate() // client owner
  const freelancerOwner = Keypair.generate() // freelancer owner
  const projectName = 'test_project'
  const price = new BN(1 * LAMPORTS_PER_SOL)

  /* ------------------------------------------------------------------
     PDAs
  ------------------------------------------------------------------ */
  let clientProfilePda: PublicKey
  let freelancerProfilePda: PublicKey
  let projectPda: PublicKey
  let vaultPda: PublicKey
  let proposalPda: PublicKey

//   before(async () => {
//     console.log('🪂 Airdropping SOL...')
//     // airdrop both users
//     await provider.connection.requestAirdrop(owner.publicKey, 10 * LAMPORTS_PER_SOL)
//     await provider.connection.requestAirdrop(freelancerOwner.publicKey, 10 * LAMPORTS_PER_SOL)
//     await new Promise((resolve) => setTimeout(resolve, 1000)) // wait for airdrop
//     console.log('✅ Airdrop complete')
//   })

  const logTx = (tx: string, label: string) =>
    console.log(`✅ ${label}: https://solscan.io/tx/${tx}?cluster=devnet`)
  /* ------------------------------------------------------------------
     Profiles
  ------------------------------------------------------------------ */
  it('Create client profile', async () => {
    ;[clientProfilePda] = PublicKey.findProgramAddressSync(
      [Buffer.from('client'), owner.publicKey.toBuffer()],
      program.programId,
    )
    console.log('🏗️  Creating client profile...')
    const tx = await program.methods
    .createClientProfile('Alice Client', 'alice@client.com')
    .accounts({ owner: owner.publicKey })
    .signers([owner])
    .rpc()
  logTx(tx, 'Create client profile')

    const profile = await program.account.clientProfile.fetch(clientProfilePda)
    console.log(`✅ Freelancer profile created: ${profile.name} with PDA ${clientProfilePda}`)
    expect(profile.name).eq('Alice Client')
  })

  it('Create freelancer profile', async () => {
    ;[freelancerProfilePda] = PublicKey.findProgramAddressSync(
      [Buffer.from('freelancer'), freelancerOwner.publicKey.toBuffer()],
      program.programId,
    )
    console.log('🏗️  Creating freelancer profile...')
    const tx = await program.methods
    .createFreelancerProfile('Bob Freelancer', 'bob@freelancer.com')
    .accounts({ owner: freelancerOwner.publicKey })
    .signers([freelancerOwner])
    .rpc()
  logTx(tx, 'Create freelancer profile')

    const profile = await program.account.freelancerProfile.fetch(freelancerProfilePda)
    console.log(`✅ Freelancer profile created: ${profile.name} with PDA ${freelancerProfilePda}`)
    expect(profile.name).eq('Bob Freelancer')
  })

  /* ------------------------------------------------------------------
     Project lifecycle
  ------------------------------------------------------------------ */
  it('Create project', async () => {
    ;[projectPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('project'), Buffer.from(projectName), clientProfilePda.toBuffer(), owner.publicKey.toBuffer()],
      program.programId,
    )
    ;[vaultPda] = PublicKey.findProgramAddressSync([Buffer.from('vault'), projectPda.toBuffer()], program.programId)

    console.log('🏗️  Creating project...')
    const tx = await program.methods
      .createProject(projectName, 'Build a dApp', price, new BN(Date.now() / 1000 + 86400))
      .accountsStrict({
        project: projectPda,
        owner: owner.publicKey,
        client: clientProfilePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc()
    logTx(tx, 'Create project')

    const project = await program.account.project.fetch(projectPda)
    console.log(`✅ Project created: ${project.name} with PDA ${projectPda}`)
    console.log(`Project price: ${project.price.toNumber()}`)
    const valuePubkey = await provider.connection.getAccountInfo(vaultPda)
    console.log(`Value pubkey: ${valuePubkey}`)
    const projectBalance = await provider.connection.getBalance(vaultPda)
    console.log(`Project balance: ${projectBalance}`)
    expect(project.name).eq(projectName)
    expect(project.price.eq(price)).true
  })

  it('Fund project vault', async () => {
    console.log('💰 Funding project vault...')
    const tx = await program.methods
      .fundProject(new BN(2 * LAMPORTS_PER_SOL))
      .accountsStrict({
        client: owner.publicKey,
        project: projectPda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc()
    logTx(tx, 'Fund project vault')

    const balance = await provider.connection.getBalance(vaultPda)
    console.log(`Project balance: ${balance}`)
    expect(balance).gte(2 * LAMPORTS_PER_SOL)
  })

  it('Submit proposal', async () => {
    ;[proposalPda] = PublicKey.findProgramAddressSync(
      [Buffer.from('proposal'), projectPda.toBuffer(), freelancerProfilePda.toBuffer()],
      program.programId,
    )
    console.log('🏗️  Submitting proposal...')
    const tx = await program.methods
      .submitProposal(projectName, 'I can do it!')
      .accountsStrict({
        proposal: proposalPda,
        freelancerSigner: freelancerOwner.publicKey,
        freelancer: freelancerProfilePda,
        project: projectPda,
        client: clientProfilePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([freelancerOwner])
      .rpc()

    const proposal = await program.account.proposal.fetch(proposalPda)
    console.log(`✅ Proposal submitted: ${proposal.msg} with PDA ${proposalPda}`)
    expect(proposal.msg).eq('I can do it!')
  })

  it('Client accepts proposal', async () => {
    console.log('👀 Client accepting proposal...')
    const tx = await program.methods
      .respondToProposal(true)
      .accountsStrict({
        proposal: proposalPda,
        clientSigner: owner.publicKey,
        project: projectPda,
        freelancer: freelancerProfilePda,
        clientProfile: clientProfilePda,
      })
      .signers([owner])
      .rpc()
    logTx(tx, 'Client accepts proposal')

    // reload proposal
    const proposal = await program.account.proposal.fetch(proposalPda)
    console.log(`✅ Proposal accepted: ${proposal.status.accepted}`)
    expect(proposal.status).to.have.property('accepted')
  })

  it('Freelancer submits work', async () => {
    console.log('🏗️  Submitting work...')
    const tx = await program.methods
      .submitWork('https://github.com/user/repo')
      .accountsStrict({
        project: projectPda,
        freelancer: freelancerOwner.publicKey,
      })
      .signers([freelancerOwner])
      .rpc()

    const project = await program.account.project.fetch(projectPda)
    console.log(`✅ Work submitted: ${project.githubLink}`)
    expect(project.githubLink).eq('https://github.com/user/repo')
  })

  it('Client approves work & pays', async () => {
    const freelancerBefore = await provider.connection.getBalance(freelancerOwner.publicKey)
    console.log('👀 Client approving work...')
    const tx = await program.methods
      .approveWorkAndPay()
      .accountsStrict({
        project: projectPda,
        proposal: proposalPda,
        freelancer: freelancerOwner.publicKey,
        client: owner.publicKey,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc()
    logTx(tx, 'Freelancer submits work')

    const freelancerAfter = await provider.connection.getBalance(freelancerOwner.publicKey)
    console.log(`✅ Work approved & paid: ${freelancerAfter - freelancerBefore}`)
    expect(freelancerAfter - freelancerBefore).eq(price.toNumber())
  })

  /* ------------------------------------------------------------------
     Optional: update / delete profile tests
  ------------------------------------------------------------------ */
  it('Update client profile', async () => {
    console.log('🏗️  Updating client profile...')
    const tx = await program.methods
      .updateClientProfile('Alice Updated', 'new@mail.com', 'bio', 'US', 'linkedin.com', owner.publicKey)
      .accounts({ clientProfile: clientProfilePda, owner: owner.publicKey })
      .signers([owner])
      .rpc()
    logTx(tx, 'Update client profile')

    const profile = await program.account.clientProfile.fetch(clientProfilePda)
    console.log(`✅ Client profile updated: ${profile.name}`)
    expect(profile.name).eq('Alice Updated')
  })

  it('Delete client profile', async () => {
    console.log('🏗️  Deleting client profile...')
    const tx = await program.methods
      .deleteClientProfile()
      .accounts({ clientProfile: clientProfilePda, owner: owner.publicKey })
      .signers([owner])
      .rpc()
    logTx(tx, 'Delete client profile')
    console.log('✅ Client profile deleted')
    try {
      await program.account.clientProfile.fetch(clientProfilePda)
      expect.fail('Account should be closed')
    } catch {}
  })
})
