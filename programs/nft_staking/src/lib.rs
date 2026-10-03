pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2PksHHeD27bNHzaxbKSECGCGKHoDjPbvjMzXnCMvRL3g");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn init_config(ctx: Context<InitConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32, points_to_rewards_multiplier:u32) -> Result<()> {
        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, points_to_rewards_multiplier, &ctx.bumps)
    }

    pub fn init_user(ctx: Context<InitUser>, ) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }
    pub fn unstake(ctx: Context<UnStake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<ClaimRewards>, points_to_claim: Option<u32>) -> Result<()> {
        ctx.accounts.claim_rewards(points_to_claim)
    }
}
