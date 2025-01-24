import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day3 } from "../target/types/day_3";

describe("day_3", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day3 as Program<Day3>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(123).rpc();
    console.log("Your transaction signature", tx);
  });
});
