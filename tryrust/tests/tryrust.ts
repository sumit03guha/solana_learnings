import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Tryrust } from '../target/types/tryrust';

describe('tryrust', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Tryrust as Program<Tryrust>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log('Your transaction signature', tx);
  });

  it('checks age', async () => {
    const tx = await program.methods.ageChecker(18).rpc();
    console.log('Your transaction signature', tx);
  });

  it('checks age with match', async () => {
    const tx = await program.methods.checkAgeWithMatch(17).rpc();
    console.log('Your transaction signature', tx);
  });

  it('tests hashmap', async () => {
    const tx = await program.methods.tryingHashmap('Alex', 24).rpc();
    console.log('Your transaction signature', tx);
  });
});
