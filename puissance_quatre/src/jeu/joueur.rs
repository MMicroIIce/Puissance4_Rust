// // Dans src/jeu/plateau.rs

// Déclaration d'une structure nommée Joueur
#[derive(PartialEq)]
pub struct Joueur {
    pub nom: String,
    pub jeton: char,
}

// Implémentation de méthodes pour la structure Joueur
impl Joueur {
    // Constructeur pour créer un nouveau joueur
    pub fn new_joueur(nom: String, jeton: char) -> Self {
        Joueur {nom, jeton}
    }

    // Ajoutez d'autres méthodes liées aux joueurs ici
}