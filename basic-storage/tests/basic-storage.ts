import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { BasicStorage } from '../target/types/basic_storage';

describe('basic-storage', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  console.log('Provider set to:', anchor.AnchorProvider.env());

  const program = anchor.workspace.BasicStorage as Program<BasicStorage>;

  it('Is initialized!', async () => {
    // Add your test here.
    const seeds = [];
    const [storage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );
    console.log('Storage account address:', storage.toBase58());

    const tx = await program.methods
      .initialize()
      .accounts({ storage: storage })
      .rpc();
    console.log('Your transaction signature', tx);
  });
});
