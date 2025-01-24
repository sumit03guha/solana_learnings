import * as anchor from '@coral-xyz/anchor';
import { Program, AnchorError } from '@coral-xyz/anchor';
import { Day4 } from '../target/types/day_4';
import { assert } from 'chai';

describe('day_4', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day4 as Program<Day4>;

  it('tests number greater than 10', async () => {
    // Add your test here.
    try {
      const tx = await program.methods.limitRange(71).rpc();
      console.log('Your transaction signature', tx);
    } catch (_err) {
      console.error('raw Error', _err);
      const err: AnchorError = _err;
      const errMsg = 'number is not less than 10';

      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log('Error code : ', err.error.errorCode);
    }
  });

  it('tests number less than 3', async () => {
    try {
      const tx = await program.methods.limitRange(1).rpc();
      console.log('Your transaction signature', tx);
    } catch (_err) {
      console.error('raw Error', _err);
      const err: AnchorError = _err;
      const errMsg = 'number is not greater than 3';

      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log('Error code : ', err.error.errorCode);
    }
  });

  it('tests always reverts', async () => {
    try {
      const tx = await program.methods.thisFunctionAlwaysReverts().rpc();
      console.log('Your transaction signature', tx);
    } catch (_err) {
      console.error('raw Error', _err);
      const err: AnchorError = _err;

      assert.strictEqual(err.error.errorMessage, 'This always reverts');
      console.log('Error code : ', err.error.errorCode);
    }
  });
});
