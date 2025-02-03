import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { AccountsAndSigners } from '../target/types/accounts_and_signers';
import { assert } from 'chai';

describe('accounts-and-signers', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .AccountsAndSigners as Program<AccountsAndSigners>;

  let myKeyPair1 = anchor.web3.Keypair.generate();
  let myKeyPair2 = anchor.web3.Keypair.generate();

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

  it('tests account signers', async () => {
    const tx = await program.methods
      .callFromMultipleSigners()
      .accounts({
        signer: program.provider.publicKey,
        signer2: myKeyPair1.publicKey,
        signer3: myKeyPair2.publicKey,
      })
      .signers([myKeyPair1, myKeyPair2])
      .rpc();

    console.log('Your transaction signature', tx);
    console.log('The signer is : ', program.provider.publicKey.toBase58());
    console.log('The signer2 is : ', myKeyPair1.publicKey.toBase58());
    console.log('The signer3 is : ', myKeyPair2.publicKey.toBase58());
  });

  it('tests only owner function', async () => {
    const tx = await program.methods
      .onlyOwnerFunction()
      .accounts({
        signer: program.provider.publicKey,
      })
      .rpc();
    console.log('Your transaction signature', tx);

    try {
      await program.methods
        .onlyOwnerFunction()
        .accounts({
          signer: myKeyPair1.publicKey,
        })
        .signers([myKeyPair1])
        .rpc();
    } catch (_err) {
      let err: anchor.AnchorError = _err;
      const errMsg = 'Only the owner can call this function';
      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log('Error number: ', err.error.errorCode.number);
      console.log('Error code: ', err.error.errorCode.code);
    }
  });
});
