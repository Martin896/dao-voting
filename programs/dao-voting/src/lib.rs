use anchor_lang::prelude::*;

declare_id!("Dgva4jEprCw9jVQxpQSv6s4NerJV7ozBNXE9qrKFe6JU");

#[program]
pub mod dao_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.total_votes = 0;
        Ok(())
    }

    pub fn create_proposal(ctx: Context<CreateProposal>, proposal_text: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.text = proposal_text;
        proposal.vote_count = 0;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, proposal_id: u64) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.vote_count += 1;
        let state = &mut ctx.accounts.state;
        state.total_votes += 1;
        Ok(())
    }

    pub fn get_results(ctx: Context<GetResults>) -> Result<u64> {
        let proposal = &ctx.accounts.proposal;
        Ok(proposal.vote_count)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub state: Account<'info, State>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetResults<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
}

#[account]
pub struct State {
    pub total_votes: u64,
}

#[account]
pub struct Proposal {
    pub text: String,
    pub vote_count: u64,
}