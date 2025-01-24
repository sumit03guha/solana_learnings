import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Day2 } from '../target/types/day_2';

describe('day_2', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day2 as Program<Day2>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(new anchor.BN(13), new anchor.BN(32), 'Hello, Solana!')
      .rpc();
    console.log('Your transaction signature', tx);
  });

  it('Passes an array', async () => {
    const tx = await program.methods
      .array([new anchor.BN(12), new anchor.BN(13), new anchor.BN(14)])
      .rpc();

    console.log('Your transaction signature', tx);
  });

  it('tests overlfow', async () => {
    const tx = await program.methods.thisWillOverflow(1).rpc();

    console.log('Your transaction signature', tx);
  });

  it('tests panic', async () => {
    const tx = await program.methods.thisWillPanic(1).rpc();

    console.log('Your transaction signature', tx);
  });

  it('tests calculator', async () => {
    const tx = await program.methods
      .calculator(new anchor.BN(12), '+', new anchor.BN(13))
      .rpc();

    console.log('Your transaction signature', tx);
  });
});
