mod grid;
mod player;
use grid::{Grid, grid_create, grid_main};
use player::{Player, player_create, player_print};

const GRID_COLUMN_NB: i8 = 7;
const GRID_ROW_NB: i8 = 6;

trait Jeu {
    fn get_grid(&self) -> &Grid;
    fn get_player_1(&self) -> &Player;
    fn get_player_2(&self) -> &Player;
}
pub struct Game {
    grid: Grid,
    player_1: Player,
    player_2: Player,
}

impl Jeu for Game {
    fn get_grid(&self) -> &Grid {
        &self.grid
    }
    fn get_player_1(&self) -> &Player {
        &self.player_1
    }
    fn get_player_2(&self) -> &Player {
        &self.player_2
    }
}

pub fn game_create() -> Game {
    
    let game_grid = grid_create(GRID_COLUMN_NB, GRID_ROW_NB);
    let player_1 = player_create(1, ((GRID_COLUMN_NB * GRID_ROW_NB) / 2).into());
    let player_2 = player_create(2, ((GRID_COLUMN_NB * GRID_ROW_NB) / 2).into());
    
    let game = Game {
        grid: game_grid,
        player_1: player_1,
        player_2: player_2,
    };
    
    return game;
}

pub fn game_main(game: &Game) {
    grid_main(game.get_grid());
    player_print(game.get_player_1());
    player_print(game.get_player_2());
    game_tour(game);
}

fn game_tour(game: &Game) {

}