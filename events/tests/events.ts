import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Events } from '../target/types/events';

describe('events', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Events as Program<Events>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log('Your transaction signature', tx);
  });

  it('tests for events', async () => {
    const eventListener = program.addEventListener('myEvent', (event, slot) => {
      console.log('Event received', event, slot);
    });

    const tx = await program.methods.thisEmitsEvents(12, 'Hello World').rpc();
    console.log('Your transaction signature', tx);

    await new Promise((resolve) => setTimeout(resolve, 1000));

    program.removeEventListener(eventListener);
  });
});
