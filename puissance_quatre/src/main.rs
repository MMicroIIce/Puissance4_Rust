
mod game;

use crate::game::player::Player;

fn main() {
    // Créez une instance de PlateauDeJeu avec une taille de grille de 7 colonnes x 6 lignes
    let grid = game::grid::Grid::new_grid();

    // Créez deux joueurs
    let joueur1 = game::player::LocalPlayer::new_player("Joueur 1".to_string(), 'X');
    let joueur2 = game::player::LocalPlayer::new_player("Joueur 2".to_string(), 'O');

    println!();
    println!("Le nom du joueur 1 est : {}", joueur1.get_name());
    println!("Le jeton du joueur 1 est : {}", joueur1.get_token());
    println!("Le nom du joueur 2 est : {}", joueur2.get_name());
    println!("Le jeton du joueur 2 est : {}", joueur2.get_token());
    println!();

    // Créez une instance de Partie en utilisant les joueurs et le plateau créés
    let mut gameplay = game::gameplay::Gameplay::new_gameplay(grid, &joueur1, &joueur2);

    // Commencez à jouer la partie
    gameplay.play();
}
