import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Dexhire } from "../target/types/dexhire";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { assert } from "chai";

describe("dexhire", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = anchor.AnchorProvider.env().connection;
  const user = (provider.wallet as anchor.Wallet).payer
  const program = anchor.workspace.dexhire as Program<Dexhire>;

  //   test variables
  const free_name = "Aditya";
  const free_email = "sehrawataditya22@gmail.com";
  const free_name_update = "Aditya Sehrawat";
  const free_email_update = "adiseh2004@gmail.com";
  const free_bio = "I thrive at the intersection of creativity and technology, solving complex problems and building impactful solutions. Proficient in Rust, Java, and C, I specialize in blockchain and Web3 development, with a strong foundation in data structures, algorithms, and compiler design. Passionate about decentralized systems, smart contracts, and high-performance applications, I’m particularly drawn to Solana and its potential. A lifelong learner and problem-solver, I’m driven by curiosity and a desire to create meaningful change through innovation.";
  const skills = [
      { name: "Rust" },
      { name: "Solana" },
    ];
    const country =  "India";
    const linkedin = "https://www.linkedin.com/in/adityasehrawattt/";
    
  let freelancerPda: anchor.web3.PublicKey;

  before(async () => {
    [freelancerPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('freelancer'), user.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Create freelancer profile", async () => {
    const balance = await provider.connection.getBalance(user.publicKey)
    if (balance < anchor.web3.LAMPORTS_PER_SOL) {
      const sig = await provider.connection.requestAirdrop(user.publicKey, anchor.web3.LAMPORTS_PER_SOL)
      await provider.connection.confirmTransaction(sig)
    }
    // Add your test here.
    const tx = await program.methods.createFreelanceProfile(free_name,free_email)
    .accountsStrict({
        freelanceprofile: freelancerPda,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([user])
    .rpc();
    console.log("Your transaction signature", tx);
    const freelancerProfile = await program.account.freelancerProfile.fetch(freelancerPda);

    assert.equal(freelancerProfile.name,free_name);
    assert.equal(freelancerProfile.email,free_email);
    assert.equal(freelancerProfile.avatar,"Ad");
    assert.equal(freelancerProfile.authority.toBase58(),user.publicKey.toBase58());
    console.log(`Freelancer name: ${freelancerProfile.name}`);
    console.log(`Freelancer avatar: ${freelancerProfile.avatar}`);
    console.log(`Freelancer email: ${freelancerProfile.email}`);
    console.log(`Freelancer wallet address: ${freelancerProfile.authority.toBase58()}`);
    console.log("Freelancer profile created successfully");
  });

  it("update freelancer profile", async () => {
    const balance = await provider.connection.getBalance(user.publicKey)
    if (balance < anchor.web3.LAMPORTS_PER_SOL) {
      const sig = await provider.connection.requestAirdrop(user.publicKey, anchor.web3.LAMPORTS_PER_SOL)
      await provider.connection.confirmTransaction(sig)
    }
    // Add your test here.
    const tx = await program.methods.updateFreelanceProfile(free_name_update,free_email_update,free_bio,skills,country,linkedin)
    .accountsStrict({
        freelanceprofile: freelancerPda,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([user])
    .rpc();
    console.log("Your transaction signature", tx);
    const freelancerProfile = await program.account.freelancerProfile.fetch(freelancerPda);

    assert.equal(freelancerProfile.name,free_name_update);
    assert.equal(freelancerProfile.email,free_email_update);
    assert.equal(freelancerProfile.avatar,"AS");
    assert.equal(freelancerProfile.bio,free_bio);
    assert.equal(freelancerProfile.country,country);
    assert.equal(freelancerProfile.linkedin,linkedin);
    assert.equal(freelancerProfile.linkedin,linkedin);
    assert.equal(freelancerProfile.skills.length, 2);
    assert.equal(freelancerProfile.skills[0].name, "Rust");
    assert.equal(freelancerProfile.skills[1].name, "Solana");
    assert.equal(freelancerProfile.authority.toBase58(),user.publicKey.toBase58());
    console.log(`Freelancer name: ${freelancerProfile.name}`);
    console.log(`Freelancer avatar: ${freelancerProfile.avatar}`);
    console.log(`Freelancer email: ${freelancerProfile.email}`);
    console.log(`Freelancer bio: ${freelancerProfile.bio}`);
    console.log(`Freelancer country: ${freelancerProfile.country}`);
    console.log(`Freelancer linkedin: ${freelancerProfile.linkedin}`);
    console.log(`Freelancer wallet address: ${freelancerProfile.authority.toBase58()}`);
    console.log(`Freelancer skills: ${freelancerProfile.skills[0].name} , ${freelancerProfile.skills[1].name}`);
    console.log("Freelancer profile updates successfully");
  });

  it('Delete freelancer profile', async () => {
      const balance = await provider.connection.getBalance(user.publicKey)
      if (balance < anchor.web3.LAMPORTS_PER_SOL) {
        const sig = await provider.connection.requestAirdrop(user.publicKey, anchor.web3.LAMPORTS_PER_SOL)
        await provider.connection.confirmTransaction(sig)
      }
      const tx = await program.methods
        .deleteFreelanceProfile()
        .accountsStrict({
          freelanceprofile: freelancerPda,
          owner: user.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([user])
        .rpc()
      console.log('Your transaction signature', tx)
      console.log('Freelancer Profile deleted successfully!')
    })
});
