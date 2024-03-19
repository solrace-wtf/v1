use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// Global program parameters
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Params {
    pub admin: Pubkey, // Public key of the admin multisig (can pause and update these params)
    pub paused: bool, // Game global pausing - can new games be started
    pub fee_collector: Pubkey, // Public key of the fee collector
    pub fee: u16, // Game ticket fee in bps (0-100_00)
    pub next_game_id: u32, // Next game id to be used - reset to 0 after reaching u32::MAX to save space
    pub min_pot: u64, // Minimum pot size for a game to start
    pub max_pot: u64, // Max pot size for a game
    pub min_grid: u16, // Minimum size of the game grid
    pub max_grid: u16, // Max size of the game grid
    pub min_speed: u8, // Minimum speed of the bike in blocs per second
    pub max_speed: u8, // Max speed of the bike in blocs per second
    pub min_trail: u16, // Minimum trail length at the start of the game
    pub max_trail: u16, // Max trail length at the start of the game
    pub min_trail_cooldown: u8, // Min seconds before a trail increases +base_trail_length
    pub max_trail_cooldown: u8, // Max seconds before a trail increases +base_trail_length
    pub max_disconnects: u32, // Max number of disconnects before kick
    pub disconnect_max_sec: u32, // Max time in seconds a player can be disconnected before kick
}

// Metrics object
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Metrics {
    pub total_pot: u64, // Sum of all finished games' pots
    pub open_pot: u64, // Sum of all ongoing games' pots
    pub earningsLeaderboard: Vec<Pubkey>, // Top 10 players by earnings (only players with more than 42 games)
    pub winrateLeaderboard: Vec<Pubkey>, // Top 10 players by winrate (wins/games, only players with more than 42 games)
    pub earnings: u64, // Sum of all players' earnings
}

// Player object
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Player {
    // global attributes
    pub wins: u32, // Number of games won
    pub losses: u32, // Number of games lost
    pub earnings: i128, // Sum of earnings in lamports (no support for SPL tokens)
    pub name: String, // Player defined name, defaults to (public key)[:8]
    pub title: String, // Player defined title, can only be set every 42 games (wins+losses) % 42 == 0
    pub picture: String, // Player defined URI pointing to a arweave/ipfs
    // in-game attributes
    pub game_id: Pubkey, // Game id of the game the player is currently in
    pub position: u64, // Grid position of the player in-game (empty for stateless, webrtc only games)
    pub creator_of: u32, // Game id of the current game created if any (a new game cannot be started by a player if his previous game is still pending)
}

// Game status enum
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum GStatus {
    Pending, // Lobby is on, waiting for players to join
    Cancelled, // Game was cancelled by creator before starting
    Ongoing, // Game is currently being played or pot is still unclaimed
    Finished, // Game has finished for more than 1 hour and a user withdrew or pot has been claimed by winner
}

// Game parameters
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GParams {
    pub max_players: u8, // Max number of players in the game
    pub min_players: u8, // Minimum number of players in the game
    pub split_between: u8, // How the pot is split among winners
    pub speed: u8, // Speed of the bike in blocs per second
    pub base_trail_length: u8, // Length of the trail left by the bike at the start of the game
    pub trail_cooldown: u8 // Seconds before a trail increases +base_trail_length
    pub grid_size: u16, // Size of the game grid (grid_size x grid_size)
    pub ticket_price: u64, // Price of a ticket to join the game
}

// Game object
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Game {
    pub id: u32, // Unique game identifier (incremental)
    pub creator: Pubkey, // The player who created the game
    pub params: GParams, // Parameters for the game
    pub players: Vec<GPlayer>, // Current players in the game
    pub pot: u64, // Current pot size (sum of all player tickets)
    pub status: GStatus, // Current status of the game
}
