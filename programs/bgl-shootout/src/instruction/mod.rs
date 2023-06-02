use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

mod end_game;
mod play_round;
mod start_game;
pub use end_game::*;
pub use play_round::*;
pub use start_game::*;

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum BglShootoutInstruction {
    /// Create the game account.
    #[account(0, writable, name="game_pda", desc = "The PDA of the game")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    StartGame(StartGameArgs),

    /// Close the game account.
    #[account(0, writable, name="game_pda", desc = "The PDA of the game")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    EndGame(EndGameArgs),

    /// Close the game account.
    #[account(0, writable, name="game_pda", desc = "The PDA of the game")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    PlayRound(PlayRoundArgs),
}
