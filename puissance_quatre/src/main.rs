mod game;
use game::{game_create, game_main};

fn main() {
    println!("Puissance 4");
    //creer_fenetre();
    let game = game_create();
    game_main(&game);
}
