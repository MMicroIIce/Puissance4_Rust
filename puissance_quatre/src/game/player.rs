pub struct Player {
    id: i8,
    piece_nb: i32,
}

pub fn player_create(id_val: i8, piece_nb_val: i32,) -> Player {
    
    let player = Player {
        id: id_val,
        piece_nb: piece_nb_val,
    };
    
    return player;
}

pub fn player_print(player: &Player) {
    println!("Joueur {} : {} pièce restante(s).", player.id, player.piece_nb);
}