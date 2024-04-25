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

    pub fn update_user(ctx: Context<UpdateUser>, name: String, avatar: String) -> Result<()> {

        let user_account = &mut ctx.accounts.user_account;
        user_account.name = name;
        user_account.avatar = avatar;

        Ok(())
    }

    pub fn delete_user(ctx: Context<DeleteUser>, name: String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;

        msg!("Deleting a User account {} {}", user_account.name, name);
        Ok(())
    }

    pub fn crate_post(ctx: Context<CreatePost>, title: String, content: String) -> Result<()> {

        let post_account = &mut ctx.accounts.post_account;
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        post_account.id = user_account.last_post_id + 1;
        user_account.last_post_id = post_account.id;
        post_account.user = user_account.key();
        post_account.title = title;
        post_account.content = content;
        user_account.post_count = user_account.post_count + 1;
        post_account.authority = authority.key();
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
#[instruction(name: String, avatar: String)]
pub struct UpdateUser<'info> {
    #[account(
        mut,
        seeds = [USER_SEED, authority.key().as_ref()],
        bump,
        realloc = 8 + 32 + 8 + 8 + 4 + name.len() + 4 + avatar.len(),
        realloc::payer = authority,
        realloc::zero = true,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct DeleteUser<'info> {
    #[account(
        mut,
        seeds = [USER_SEED, authority.key().as_ref()],
        bump,
        close = authority,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
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


