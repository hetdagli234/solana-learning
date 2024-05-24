import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SystemProgram } from "@solana/web3.js";
import { expect } from "chai";
import { CreateAccountCpi } from "../target/types/create_account_cpi";
import { CreatePdaExample } from "../target/types/create_pda_example";

describe("create_account_cpi", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CreatePdaExample as Program<CreatePdaExample>;

  it("Creates a PDA account", async () => {
    const payer = anchor.web3.Keypair.generate();
    const signature = await provider.connection.requestAirdrop(payer.publicKey, 1_000_000_000);
    await provider.connection.confirmTransaction(signature, 'confirmed');

    const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("my_seed")],
      program.programId
    );
    

    await program.methods
      .createAccountCpi()
      .accounts({
        payer: payer.publicKey,
        pdaAccount: pda,
        systemProgram: SystemProgram.programId,
      })
      .signers([payer])
      .rpc();

    const pdaAccount = await provider.connection.getAccountInfo(pda);
    expect(pdaAccount).to.not.be.null;
    expect(pdaAccount!.owner).to.eql(program.programId);
    expect(pdaAccount!.data.length).to.equal(8);
  });
});
