/* Module player.rs
 * 
 * TODO : 
 * - tout commenter
 * - faire de la gestion d'erreur
 * - Simplifier, corriger, donner du sens (surtout d'un aspect modulable)
 * */

mod game;

use crate::game::player::Player;

fn main() {
    // Créez une instance de PlateauDeJeu avec une taille de grille de 7 colonnes x 6 lignes
    let grid = game::grid::Grid::new_grid();

    // Créez deux joueurs
    let player1 = game::player::LocalPlayer::new_local_player("Joueur 1".to_string(), 'X');
    let player2 = game::player::LocalPlayer::new_local_player("Joueur 2".to_string(), 'O');
    let ia = game::player::IAPlayer::new_ia_player("IA".to_string(), 'W');

    println!();
    println!("Le nom du joueur 1 est : {}", player1.get_name());
    println!("Le jeton du joueur 1 est : {}", player1.get_token());
    println!("Le nom du joueur 2 est : {}", player2.get_name());
    println!("Le jeton du joueur 2 est : {}", player2.get_token());
    println!("Le nom de l'IA est : {}", ia.get_name());
    println!("Le jeton de l'IA est : {}", ia.get_token());
    println!();

    // Créez une instance de Partie en utilisant les joueurs et le plateau créés
    let mut gameplay = game::gameplay::Gameplay::new_gameplay(grid, player1, player2, ia);

    // Commencez à jouer la partie
    gameplay.play();
}
