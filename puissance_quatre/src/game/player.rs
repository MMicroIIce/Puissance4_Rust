/// Cette structure permet de créer un joueur avec deux caractéristique : son identifiant et son nombre de jetons disponibles
pub struct Player {
    id: i8,
    piece_nb: i32,
}

/// Cette fonction permet de créer un élément de type `Player` et de le renvoyer
/// 
/// # Arguments
///
/// * `id_val` - Id du joueur créé
/// * `piece_nb_val` - Nombre de jeton disponible pour le joueur en début de partie
pub fn player_create(id_val: i8, piece_nb_val: i32,) -> Player {
    
    let player = Player {
        id: id_val,
        piece_nb: piece_nb_val,
    };
    
    return player;
}

/// Cette fonction permet d'écrire sur le terminal de commande les informations du joueur mis en paramètre
/// 
/// # Arguments
///
/// * `player` - Le joueur dont on souhaite écrire les informations
pub fn player_print(player: &Player) {
    println!("Joueur {} : {} pièce restante(s).", player.id, player.piece_nb);
}