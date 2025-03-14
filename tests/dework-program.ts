import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DeworkProgram } from "../target/types/dework_program";
import { assert } from "chai";

describe("dework_program", () => {
  // Set up provider and program
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.DeworkProgram as Program<DeworkProgram>;

  it("Creates a job", async () => {
    // Generate a new Keypair for the job account
    const jobAccount = anchor.web3.Keypair.generate();
    // Job details
    const title = "Build a Web3 Dashboard";
    const description = "Create a Solana-based analytics dashboard";
    const budget = new anchor.BN(1000000000); // 1 SOL (in lamports)
    // Send the transaction
    await program.methods
      .createNewJob(title, description, budget)
      .accounts({
        job: jobAccount.publicKey,
        client: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId, // Add this line
      })
      .signers([jobAccount])
      .rpc();
    // Fetch the created job account
    const job = await program.account.job.fetch(jobAccount.publicKey);
    // Assertions
    assert.strictEqual(job.title, title);
    assert.strictEqual(job.description, description);
    assert.strictEqual(job.budget.toString(), budget.toString());
    assert.strictEqual(job.client.toBase58(), provider.wallet.publicKey.toBase58());
    assert.isTrue(job.isOpen);
  });
});