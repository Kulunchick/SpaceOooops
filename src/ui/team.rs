//! Team setup data: solo or team play, player names, and which player holds which Role (ADR-0002:
//! one screen, no networking). Plus the Go/No-Go votes. Pure data; the screens live in `team_setup.rs`
//! and `go_no_go.rs`.
use spaceooops::sim::Role;

pub const MIN_TEAM: usize = 2;
pub const MAX_TEAM: usize = 4;

#[derive(Clone, Debug, PartialEq)]
pub struct Team {
    pub solo: bool,
    /// Entered names, one per player; blank means "Player N".
    pub players: Vec<String>,
    /// Index into `players` of the owner of each Role, in `Role::ALL` order.
    pub owner: [usize; 4],
}

impl Default for Team {
    fn default() -> Self {
        Team { solo: true, players: vec![String::new()], owner: [0; 4] }
    }
}

pub fn role_index(role: Role) -> usize {
    Role::ALL.iter().position(|r| *r == role).unwrap_or(0)
}

impl Team {
    /// Solo play: one player holds every Role.
    pub fn set_solo(&mut self) {
        self.solo = true;
        self.players.truncate(1);
        if self.players.is_empty() {
            self.players.push(String::new());
        }
        self.owner = [0; 4];
    }

    /// Team play with `n` players (clamped to 2..=4); Roles are dealt out round-robin.
    pub fn set_team(&mut self, n: usize) {
        let n = n.clamp(MIN_TEAM, MAX_TEAM);
        self.solo = false;
        self.players.resize(n, String::new());
        if !self.is_complete() || self.owner.iter().any(|o| *o >= n) {
            for (i, owner) in self.owner.iter_mut().enumerate() {
                *owner = i % n;
            }
        }
    }

    /// Name shown for player `index`.
    pub fn player_name(&self, index: usize) -> String {
        let typed = self.players.get(index).map(|n| n.trim()).unwrap_or("");
        if typed.is_empty() { format!("Player {}", index + 1) } else { typed.to_string() }
    }

    /// Name of the player who holds `role`.
    pub fn owner_name(&self, role: Role) -> String {
        self.player_name(self.owner[role_index(role)])
    }

    /// Every player holds at least one Role.
    pub fn is_complete(&self) -> bool {
        (0..self.players.len()).all(|p| self.owner.contains(&p))
    }
}

/// A Role's vote at Launch Review: `None` = not yet voted, `Some(true)` = Go, `Some(false)` = No-Go.
pub type Votes = [Option<bool>; 4];
