import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Recursyon } from "../target/types/recursyon";

describe("recursyon", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Recursyon as Program<Recursyon>;

  it("test recursyion", async () => {
    // Add your test here.
    const tx = await program.methods
      .execute(4)
      .accounts({
        myself: program.programId,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
