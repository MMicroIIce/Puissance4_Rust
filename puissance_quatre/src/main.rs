
//TODO : uniformiser les commentaires dans tous les modules
mod game;

use crate::game::player::Player;

fn main() {
    // Créez une instance de PlateauDeJeu avec une taille de grille de 7 colonnes x 6 lignes
    let grid = game::grid::Grid::new_grid();

    // Créez deux joueurs
    let player1 = game::player::LocalPlayer::new_player("Joueur 1".to_string(), 'X');
    let player2 = game::player::LocalPlayer::new_player("Joueur 2".to_string(), 'O');

    println!();
    println!("Le nom du joueur 1 est : {}", player1.get_name());
    println!("Le jeton du joueur 1 est : {}", player1.get_token());
    println!("Le timer du joueur 1 est initialisé à : {:?}", player1.get_timer());
    println!("Le nom du joueur 2 est : {}", player2.get_name());
    println!("Le jeton du joueur 2 est : {}", player2.get_token());
    println!("Le timer du joueur 2 est initialisé à : {:?}", player2.get_timer());
    println!();

    // Créez une instance de Partie en utilisant les joueurs et le plateau créés
    let mut gameplay = game::gameplay::Gameplay::new_gameplay(grid, &player1, &player2);

    // Commencez à jouer la partie
    gameplay.play();
}
