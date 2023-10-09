// Dans src/jeu/joueur.rs

// Déclaration d'un trait nommé JoueurTrait
pub trait JoueurTrait 
{
    fn get_name(&self) -> &str;
    fn get_token(&self) -> char;
}

// Déclaration d'une structure nommée Player
#[derive(PartialEq)] // Permettra de comparer 2 instances de la structure Player. Trouvé grâce à une IA (ChatGPT).
pub struct Joueur 
{
    pub nom: String,
    pub jeton: char,
}

// Implémentation de méthodes pour la structure Player
impl Joueur 
{
    // Constructeur pour créer un nouveau joueur
    pub fn new_joueur(nom: String, jeton: char) -> Self 
    {
        Joueur {nom, jeton}
    }
}

// Implémentation du trait PlayerTrait pour la structure Player
impl JoueurTrait for Joueur 
{
    fn get_name(&self) -> &str 
    {
        &self.nom
    }

    fn get_token(&self) -> char 
    {
        self.jeton
    }
}
