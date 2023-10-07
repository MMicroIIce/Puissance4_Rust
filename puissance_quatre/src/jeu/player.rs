// Dans src/jeu/joueur.rs

// Déclaration d'un trait nommé JoueurTrait
pub trait PlayerTrait 
{
    fn get_name(&self) -> &str;
    fn get_token(&self) -> char;
}

// Déclaration d'une structure nommée Player
#[derive(PartialEq)] // Permettra de comparer 2 instances de la structure Player. Trouvé grâce à une IA (ChatGPT).
pub struct Player 
{
    pub name: String,
    pub token: char,
}

// Implémentation de méthodes pour la structure Player
impl Player 
{
    // Constructeur pour créer un nouveau joueur
    pub fn new_player(name: String, token: char) -> Self 
    {
        Player {name, token}
    }
}

// Implémentation du trait PlayerTrait pour la structure Player
impl PlayerTrait for Player 
{
    fn get_name(&self) -> &str 
    {
        &self.name
    }

    fn get_token(&self) -> char 
    {
        self.token
    }
}
