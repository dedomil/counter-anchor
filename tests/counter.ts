import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { assert } from "chai";

describe("counter", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Counter as Program<Counter>;

  it("Is initialized!", async () => {
    let counter = anchor.web3.Keypair.generate();

    await program.methods
      .initialize()
      .accounts({
        counter: counter.publicKey,
        authority: provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counter])
      .rpc();

    let newCounter = await program.account.counter.fetch(counter.publicKey);

    assert.strictEqual(
      provider.publicKey.toBase58(),
      newCounter.authority.toBase58()
    );
  });
});
