import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { DaoVoting } from '../target/types/dao_voting';
import { assert } from 'chai';

describe('dao_voting', () => {
 
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DaoVoting as Program<DaoVoting>;

  // Accounts to be used in tests
  const state = anchor.web3.Keypair.generate();
  const proposal = anchor.web3.Keypair.generate();
  const user = (program.provider as anchor.AnchorProvider).wallet;

  it('Initializes the state', async () => {
    await program.methods.initialize().accounts({
      state: state.publicKey,
      user: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([state]).rpc();

    const stateAccount = await program.account.state.fetch(state.publicKey);
    assert.equal(stateAccount.totalVotes.toNumber(), 0);
  });

  it('Creates a proposal', async () => {
    const proposalText = "Should we adopt a new logo?";
    await program.methods.createProposal(proposalText).accounts({
      proposal: proposal.publicKey,
      state: state.publicKey,
      user: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([proposal]).rpc();

    const proposalAccount = await program.account.proposal.fetch(proposal.publicKey);
    assert.equal(proposalAccount.text, proposalText);
    assert.equal(proposalAccount.voteCount.toNumber(), 0);
  });

  it('Votes on a proposal', async () => {
    await program.methods.vote().accounts({
      proposal: proposal.publicKey,
      state: state.publicKey,
      user: user.publicKey,
    }).rpc();

    const proposalAccount = await program.account.proposal.fetch(proposal.publicKey);
    const stateAccount = await program.account.state.fetch(state.publicKey);
    assert.equal(proposalAccount.voteCount.toNumber(), 1);
    assert.equal(stateAccount.totalVotes.toNumber(), 1);
  });

  it('Gets proposal results', async () => {
    const result = await program.methods.getResults().accounts({
      proposal: proposal.publicKey,
    }).rpc();

    assert.equal(result.toNumber(), 1);
  });
});
