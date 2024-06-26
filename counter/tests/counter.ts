import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import * as assert from "assert";

describe('counter', () => {
    
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.counter as Program<Counter>

  it('Initializes the counter', async () => {
      // Find the PDA for the data account.
      const [counterAccountPda, _] = web3.PublicKey.findProgramAddressSync(
          [Buffer.from('counter'), provider.wallet.publicKey.toBuffer()],
          program.programId
      );

      const tx = await program.methods
        .initialize(1)  // Pass a number directly
        .rpc()

      // Fetch the account to verify its state.
      const account = await program.account.counterAccount.fetch(counterAccountPda);
      console.log(account.counter)
      assert.ok(account.counter === 1);
  });

  it('Increments the counter', async () => {
      // Find the PDA for the data account.
      const [dataAccountPda, seed] = web3.PublicKey.findProgramAddressSync(
          [Buffer.from('counter'), provider.wallet.publicKey.toBuffer()],
          program.programId
      );

      // Call the increment function via RPC.
      await program.methods
        .increment()
        .rpc()

      // Fetch the account to verify its state.
      const account = await program.account.counterAccount.fetch(dataAccountPda);
      console.log(account.counter)
      assert.ok(account.counter === 2);
      console.log('Counter is incremented to', account.counter.toString());
  });
});
