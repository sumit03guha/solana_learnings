import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { AccountsAndSigners } from '../target/types/accounts_and_signers';

describe('accounts-and-signers', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .AccountsAndSigners as Program<AccountsAndSigners>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log('Your transaction signature', tx);
  });

  it('tests account signers', async () => {
    const tx = await program.methods
      .callFromSigner()
      .accounts({
        signer: program.provider.publicKey,
      })
      .rpc();

    console.log('Your transaction signature', tx);
    console.log('The signer is : ', program.provider.publicKey.toBase58());
  });
});
