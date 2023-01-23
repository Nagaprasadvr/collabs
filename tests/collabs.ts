import * as anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
import { Program } from "@project-serum/anchor";
import { Collabs } from "../target/types/collabs";

describe("collabs", () => {
  const bonkMint = new anchor.web3.PublicKey(
    "3Mt4tLjCYCEYDaZmksSBxVQCaHuhPvHLeqhc3C8ojQfM"
  );
  const leaderTokenAccount = new anchor.web3.PublicKey(
    "2fYuysfh3nqFK6Nv5rxuPWhw3FD6fzJggEfAHRzydtBg"
  );
  const gitRepoXpPoolPubkey = new anchor.web3.PublicKey(
    "GvQHuc89r2MLSf15BtmxeDb3YhCfivJsTFHk7JRSitFP"
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
  // it("Can create Git Repo xpPool account!", async () => {
  //   // Add your test here.

  //   const [gitRepoPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("git_repo_xp_pool"), leader.publicKey.toBuffer()],
  //     program.programId
  //   );
  //   console.log("leader:", leader.publicKey.toBase58());
  //   const [BonkEscrowpda, Bonkbump] =
  //     anchor.web3.PublicKey.findProgramAddressSync(
  //       [Buffer.from("total_bonk_stake"), leader.publicKey.toBuffer()],
  //       program.programId
  //     );
  //   const gitRepoName: string = "TheDARKFURY";
  //   const stakeAmount = new anchor.BN(1);

  //   try {
  //     const tx = await program.methods
  //       .createGitRepoXpPoolWithStake(gitRepoName, stakeAmount)
  //       .accounts({
  //         gitRepoXpPoolAccount: gitRepoPda,
  //         bonkEscrowTokenAcc: BonkEscrowpda,
  //         bonkMint: bonkMint,
  //         tokenProgram: spl.TOKEN_PROGRAM_ID,
  //         leaderTokenAcc: leaderTokenAccount,
  //         leader: leader.publicKey,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([leader.payer])
  //       .rpc();
  //     console.log("Your transaction signature", tx);
  //   } catch (e) {
  //     console.error(e);
  //   }
  // });
  // it("can fetch xp pool!", async () => {
  //   // Add your test here.
  //   const gitRepoXpPool = await program.account.gitRepoXpPoolAccount.fetch(
  //     gitRepoXpPoolPubkey
  //   );
  //   console.log(gitRepoXpPool);
  // });

  // it("can create Contributor Account!", async () => {
  //   // Add your test here.
  //   console.log("leader:", leader.publicKey.toBase58());
  //   const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("contributor"), contributorPubkey.toBuffer()],
  //     program.programId
  //   );
  //   const contributorName = "soham";
  //   const tx = await program.methods
  //     .createContributor(contributorName)
  //     .accounts({
  //       contributorAccount: pda,
  //       contributor: contributorPubkey,
  //       gitRepoXpPoolAccount: gitRepoXpPoolPubkey,
  //       leader: leader.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     })
  //     .signers([leader.payer])
  //     .rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("can transfer Xps", async () => {
    const tx = await program.methods
      .transferXpToContributor(new anchor.BN(20))
      .accounts({
        contributorAccount: contributorAccountPubkey,
        leader: leader.publicKey,
        gitRepoXpPoolAccount: gitRepoXpPoolPubkey,
      })
      .signers([leader.payer])
      .rpc();
    console.log("Your transaction signature", tx);
  });
  it("can fetch contributor xp!", async () => {
    // Add your test here.

    const [contributorPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("contributor"), contributorPubkey.toBuffer()],
      program.programId
    );

    const contributor = await program.account.contributorAccount.fetch(
      contributorPda
    );
    console.log("xp:", contributor.xp.toNumber());
    console.log(
      "gitRepoXpPool pubkey:",
      contributor.gitRepoXpPoolPubkey.toBase58()
    );
  });
});
