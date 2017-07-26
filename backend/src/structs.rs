use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use wordlist::Wordlist;

pub type PlayerRef = Uuid;
pub type TeamRef = Uuid;

pub struct Game {
    pub board: Board,
    pub turn: Turn,
}

impl Game {
    pub fn new(wordlist: &Wordlist) -> Game {
        let board = Board::new(wordlist);
        let turn = Turn::new(board.teams[0].id);
        Game {
            board,
            turn,
        }
    }

    pub fn choose_tile(&mut self, player: PlayerRef, tile: Uuid) {
        if self.turn.tile.is_some() {
            println!("choose_tile: Tile already set");
            return;
        }
        if let Some(team) = self.board.get_team(self.turn.team) {
            if team.leader == Some(player) {
                println!("Choosing tile {}", tile);
                self.turn.tile = Some(tile);
            } else {
                println!("choose_tile: Not correct leader");
            }
        }
    }

    pub fn submit_hint(&mut self, player: PlayerRef, hint: String) {
        if let Some(tile_id) = self.turn.tile {
            if let Some(tile) = self.board.get_tile(tile_id) {
                if tile.word == hint {
                    println!("submit_hint: Hint same as word. Not allowed!!!");
                    return;
                }
            }
            if let Some(team) = self.board.get_team(self.turn.team) {
                if team.has_player_not_leader(player) {
                    if self.turn.hints.contains_key(&player) {
                        return;
                    }
                    println!("Player {} added hint {}", player, hint);
                    self.turn.hints.insert(player, hint);
                } else if self.turn.spyhint.is_none() {
                    self.turn.spyhint = Some(hint);
                }
            }
        } else {
            println!("submit_hint: Tile not set");
            return;
        }
    }

    pub fn submit_answer(&mut self, player: PlayerRef, answer: String) {
        if let Some(team) = self.board.get_team(self.turn.team) {
            if team.leader != Some(player) {
                println!("submit_answer: Not correct leader");
                return;
            }
        }

        if let Some(tile_id) = self.turn.tile {
            let next_team_id = self.board.get_next_team_id(self.turn.team);
            if let Some(tile) = self.board.get_tile(tile_id) {
                if tile.word == answer {
                    println!("Correct answer! New turn!");
                    tile.state = Some(self.turn.team);
                } else {
                    println!("Wrong answer. Changing turn");
                    tile.hints.extend(self.turn.hints.values().cloned());
                }
                self.turn = Turn::new(next_team_id);
            } else {
                println!("submit_answer: Invalid tile set");
            }
        } else {
            println!("submit_answer: Tile not set");
            return;
        }
    }
}

#[derive(Serialize)]
pub struct ClientState {
    pub board: Board,
    pub turn: Turn,
}

#[derive(Serialize, Clone)]
pub struct Board {
    tiles: Vec<Tile>,
    teams: Vec<Team>,
}

impl Board {
    pub fn new(wordlist: &Wordlist) -> Board {
        let team1 = Team::new(String::from("red"));
        let team2 = Team::new(String::from("blue"));
        let mut tiles = Vec::new();
        for y in 0..5 {
            for x in 0..5 {
                let tile = Tile::new(wordlist.get_random_word(), [x, y]);
                tiles.push(tile);
            }
        }

        Board {
            tiles,
            teams: vec![team1, team2],
        }
    }

    pub fn add_player(&mut self, player: PlayerRef) {
        // let mut smallest_team_size = 999999999;
        // let mut smallest_team = None;
        // for team in &mut self.teams {
        //     if team.team_size() < smallest_team_size {
        //         smallest_team_size = team.team_size();
        //         smallest_team = Some(team);
        //     }
        // }
        // smallest_team.unwrap().add_player(player);
        self.teams[0].add_player(player);
    }

    pub fn remove_player(&mut self, player: PlayerRef) -> bool {
        let mut must_restart = false;
        for team in &mut self.teams {
            must_restart = must_restart || team.remove_player(player);
        }
        must_restart
    }

    pub fn get_team(&self, team_id: TeamRef) -> Option<&Team> {
        for team in &self.teams {
            if team.id == team_id {
                return Some(team);
            }
        }
        None
    }

    pub fn get_next_team_id(&self, team_id: TeamRef) -> TeamRef {
        let current_i = self.teams.iter().position(|team| team.id == team_id).unwrap_or(0);
        let next_i = (current_i + 1) % self.teams.len();
        self.teams[next_i].id.clone()
    }

    pub fn get_tile(&mut self, tile_id: Uuid) -> Option<&mut Tile> {
        for tile in &mut self.tiles {
            if tile.id == tile_id {
                return Some(tile);
            }
        }
        None
    }
}

#[derive(Serialize, Clone)]
pub struct Tile {
    id: Uuid,
    word: String,
    position: [u64; 2],
    state: Option<TeamRef>,
    hints: Vec<String>,
}

impl Tile {
    pub fn new(word: String, position: [u64; 2]) -> Tile {
        println!("Creating tile with word '{}'", word);
        Tile {
            id: Uuid::new_v4(),
            word,
            position,
            state: None,
            hints: Vec::new(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Team {
    id: TeamRef,
    color: String,
    players: HashSet<PlayerRef>,
    leader: Option<PlayerRef>,
}

impl Team {
    pub fn new(color: String) -> Team {
        Team {
            id: Uuid::new_v4(),
            color,
            players: HashSet::new(),
            leader: None,
        }
    }

    pub fn team_size(&self) -> usize {
        self.players.len() + if self.leader.is_some() { 1 } else { 0 }
    }

    pub fn has_player_not_leader(&self, player: PlayerRef) -> bool {
        self.players.contains(&player)
    }

    pub fn add_player(&mut self, player: PlayerRef) {
        if self.leader.is_none() {
            println!("Adding player {} as leader in team {}", player, self.color);
            self.leader = Some(player);
        } else {
            println!("Adding player {} to team {}", player, self.color);
            self.players.insert(player);
        }
    }

    pub fn remove_player(&mut self, player: PlayerRef) -> bool {
        if let Some(leader) = self.leader {
            if player == leader {
                self.leader = None;
                println!("Removing leader {} from team {}", player, self.color);
                return true;
            }
        }
        if self.players.remove(&player) {
            println!("Removing player {} from team {}", player, self.color)
        }
        false
    }
}

#[derive(Serialize, Clone)]
pub struct Turn {
    team: TeamRef,
    tile: Option<Uuid>,
    hints: HashMap<PlayerRef, String>,
    spyhint: Option<String>,
    answer: Option<String>,
}

impl Turn {
    pub fn new(team: TeamRef) -> Turn {
        Turn {
            team,
            tile: None,
            hints: HashMap::new(),
            spyhint: None,
            answer: None,
        }
    }
}
