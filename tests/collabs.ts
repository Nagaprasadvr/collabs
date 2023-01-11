import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Collabs } from "../target/types/collabs";

describe("collabs", () => {
  const xpPoolPubkey = new anchor.web3.PublicKey(
    "DpTHkAfu4fcRcnyRGC4dvupG1YyejEsFNtCGL4MTxZxJ"
  );
  const contributorPubkey = new anchor.web3.PublicKey(
    "AsM97N16ejpKcVJTwEWtnLsDMz7jFPGr6SU1vzJD9xZt"
  );
  const contributorAccountPubkey = new anchor.web3.PublicKey(
    "7yaJNidZTMjh8mUeZHJpiwN1pe9gSKbAtfhHqy3LDcJt"
  );
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Collabs as Program<Collabs>;
  const leader = anchor.AnchorProvider.env().wallet as anchor.Wallet;
  // it("Can create xpPool account!", async () => {
  //   // Add your test here.
  //   console.log("leader:", leader.publicKey.toBase58());
  //   const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("xp_pool"), leader.publicKey.toBuffer()],
  //     program.programId
  //   );
  //   const tx = await program.methods
  //     .createXpPool()
  //     .accounts({
  //       xpPoolAccount: pda,
  //       leader: leader.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     })
  //     .signers([leader.payer])
  //     .rpc();
  //   console.log("Your transaction signature", tx);
  // });
  //   it("can fetch xp pool!", async () => {
  //     // Add your test here.
  //     const xpPool = await program.account.xpPoolAccount.fetch(xpPoolPubkey);
  //     console.log(xpPool);
  //   });

  // it("can create Contributor Account!", async () => {
  //   // Add your test here.
  //   console.log("leader:", leader.publicKey.toBase58());
  //   const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("contributor"), contributorPubkey.toBuffer()],
  //     program.programId
  //   );
  //   const tx = await program.methods
  //     .createContributor()
  //     .accounts({
  //       contributorAccount: pda,
  //       contributor: contributorPubkey,
  //       xpPoolAccount: xpPoolPubkey,
  //       leader: leader.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     })
  //     .signers([leader.payer])
  //     .rpc();
  //   console.log("Your transaction signature", tx);
  // });
  it("can fetch contributor pubkey!", async () => {
    // Add your test here.
    const contributor = await program.account.contributorAccount.fetch(
      contributorAccountPubkey
    );
    console.log("xp:", contributor.xp.toNumber());
    console.log("xpPool pubkey:", contributor.xpPoolPubkey.toBase58());
  });

  it("can transfer Xps", async () => {
    const tx = await program.methods
      .transferXpToContributor(new anchor.BN(200))
      .accounts({
        contributorAccount: contributorAccountPubkey,
        leader: leader.publicKey,
        xpPoolAccount: xpPoolPubkey,
      })
      .signers([leader.payer])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
