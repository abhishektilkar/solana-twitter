use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserAccount {
    pub authority: Pubkey,
    pub name: String,
    pub avatar: String,
    pub last_post_id: u8,
    pub post_count: u8
}


#[account]
#[derive(Default)]
pub struct PostAccount {
    pub id: u8,
    pub user: Pubkey,
    pub content: String,
    pub title: String,
    pub authority: Pubkey

}