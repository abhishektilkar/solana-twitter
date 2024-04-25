use anchor_lang::prelude::*;

pub mod state;
pub mod constant;

use crate::state::*;
use crate::constant::*;

declare_id!("48gdDQPFDoo8krKhFzetsxLHqo7SZ1Jab1ybmLTLrwm3");

#[program]
mod blog_app {
    use super::*;
    pub fn init_user(ctx: Context<InitUser>, name: String, avatar: String) -> Result<()> {

        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        user_account.authority = authority.key();
        user_account.name = name;
        user_account.avatar = avatar;
        // user_account.last_post_id = 1;
        user_account.post_count = 0;
        Ok(())
    }    

    
}

#[derive(Accounts)]
#[instruction(name: String, avatar: String)]
pub struct InitUser<'info> {
    #[account(
        init,
        seeds = [USER_SEED, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 4 + name.len() + 4 + avatar.len() + 8 + 8,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut
    )]
    pub authority: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
#[instruction(title: String, content: String)]
pub struct CreatePost<'info> {
    #[account(
        init,
        seeds = [POST_SEED, authority.key().as_ref(), &[user_account.last_post_id as u8].as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 32 + 4 + title.len() + 4 + content.len() + 1,
    )]
    pub post_account: Account<'info, PostAccount>,
    
    #[account(
        mut,
        seeds = [USER_SEED, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut
    )]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}


