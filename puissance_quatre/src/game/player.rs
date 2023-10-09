// src/jeu/player.rs

// TODO : faire 3 implémentations de traits : LOCAL, ONLINE, IA
// TODO : pour IA tu fais un truc rapide qui place un pion au pif

// Déclaration d'un trait nommé JoueurTrait
pub trait Player
{
    fn get_name(&self) -> &str;
    fn get_token(&self) -> char;
}

// Déclaration d'une structure nommée Player
#[derive(PartialEq)] // Permettra de comparer 2 instances de la structure Player. Trouvé grâce à une IA (ChatGPT).
pub struct LocalPlayer 
{
    pub name: String,
    pub token: char,
}

// Implémentation de méthodes pour la structure Player
impl LocalPlayer 
{
    // Constructeur pour créer un nouveau joueur
    pub fn new_player(name: String, token: char) -> Self 
    {
        LocalPlayer {name, token}
    }
}

// Implémentation du trait PlayerTrait pour la structure Player
impl Player for LocalPlayer 
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
