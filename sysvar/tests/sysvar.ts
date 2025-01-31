import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Sysvar } from '../target/types/sysvar';

describe('sysvar', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Sysvar as Program<Sysvar>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log('Your transaction signature', tx);
  });

  it('tests clock', async () => {
    const tx = await program.methods.clock().rpc();
    console.log('Your transaction signature', tx);
  });

  it('tests clock with chrono', async () => {
    const tx = await program.methods.clockWithChrono().rpc();
    console.log('Your transaction signature', tx);
  });
});
