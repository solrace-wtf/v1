# Concept

Solrace is a game built on the Solana blockchain, inspired by the iconic Tron game. Players control motorcycles that leave behind growing trails, similar to the mechanics of the classic game Snake. The objective is to outmaneuver opponents, causing them to crash into a trail.

## Key Features

- **Gameplay**: Players join a lobby and stake an entry ticket, denominated in $SOL, to participate. The game blends the multiplayer snake concept with modern PvP elements found in games like slither.io.
- **Elimination Mechanics**: A player is eliminated if they collide with another player's trail. The game is set on a square grid (default size is 256x256), with motorcycles moving at fixed speeds and capable of turning at 90-degree angles.
- **Performance and Technology**: Solrace runs at 60 FPS and is WebGL-based, ensuring a smooth gaming experience across compatible web browsers.
- **Cheat Resistance and Decentralization**: The game operates without a central server. Players are interconnected through WebRTC, enabling real-time position tracking and consensus mechanisms for cheat detection and lag management. If at least 66% of players agree on a cheat or disconnection, the offending player is eliminated from the game.
- **Client-Side Operations**:
  - Players can create games with custom rules and submit them to the blockchain along with their entry ticket.
  - Game lobbies are fetched from the Solana blockchain, with the game starting automatically once the lobby is full, or manually if a sufficient number of players consent.
  - A consensus mechanism among players, facilitated by WebRTC, confirms positions and detects cheating or disconnections, requiring agreement from 66% of participants for enforcement.

## Winning and Rewards

- The game's winners share the total pot accumulated from entry tickets.
- A unique consensus mechanism for eliminations and winner determination ensures fairness, with the last player standing—or the last players, depending on game rules—claiming victory.
- Rewards are claimed by submitting a "claim_win" transaction on the blockchain, accompanied by signatures from eliminated players as proof of consensus.

## Gameplay Mechanics

- **Enrollment and Withdrawal**: Players must stake $SOL to join a game and can opt out before the game starts, retrieving their stake.
- **Game Start**: The game commences shortly after the necessary blockchain transactions are confirmed, ensuring all participants are ready.
- **In-Game Consensus**: Continuous consensus among players is required to validate positions and actions within the game, with specific thresholds set for detecting disconnections and cheating.

## Blockchain Integration

- **Game Creation and Management**: Games are managed through smart contracts on the Solana blockchain, providing transparency and security for transactions and game outcomes.
- **Economic Model**: Entry fees, pot distribution, and reward claims are handled through the blockchain, with mechanisms in place to ensure fairness and prevent abuse.

# Data Model

The provided Rust code defines the data structures for players, game metrics, game parameters, game statuses, game parameters specific to each game, and the game entity itself. This model supports the game's decentralized architecture, including tracking wins, losses, earnings, player contributions to the game pot, and game outcomes.

# Data model
```rust
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Metrics {
    pub total_pot: u64, // Sum of all finished games' pots
    pub open_pot: u64, // Sum of all ongoing games' pots
    pub earningsLeaderboard: Vec<Pubkey>, // Top 10 players by earnings (only players with more than 42 games)
    pub winrateLeaderboard: Vec<Pubkey>, // Top 10 players by winrate (wins/games, only players with more than 42 games)
    pub earnings: u64, // Sum of all players' earnings
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Params {
    pub admin: Pubkey, // Public key of the admin multisig (can pause and update these params)
    pub paused: bool, // Game global pausing - can new games be started
    pub next_game_id: u32, // Next game id to be used - reset to 0 after reaching u32::MAX to save space
    pub fee: u16, // Game ticket fee in bps (0-100_00)
    pub fee_collector: Pubkey, // Public key of the fee collector
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum GStatus {
    Pending, // Lobby is on, waiting for players to join
    Cancelled, // Game was cancelled by creator before starting
    Ongoing, // Game is currently being played or pot is still unclaimed
    Finished, // Game has finished for more than 10 minutes and a user withdrew or pot has been claimed by winner
}

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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Game {
    pub id: u32, // Unique game identifier (incremental)
    pub creator: Pubkey, // The player who created the game
    pub params: GParams, // Parameters for the game
    pub players: Vec<GPlayer>, // Current players in the game
    pub pot: u64, // Current pot size (sum of all player tickets)
    pub status: GStatus, // Current status of the game
}
```