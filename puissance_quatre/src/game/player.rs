/* Module player.rs
 * 
 * TODO : 
 * - ajouter online mais ne pas faire d'implémentation
 * - tout commenter
 * - faire de la gestion d'erreur
 * */

use rand::Rng;

// Déclaration d'un trait nommé JoueurTrait
pub trait Player
{
    fn get_name(&self) -> &str;
    fn get_token(&self) -> char;
}

// Déclaration d'une structure nommée IAPlayer
pub struct IAPlayer {
    pub name: String,
    pub token: char,
}

// Déclaration d'une structure nommée Player
#[derive(PartialEq)] // Permettra de comparer 2 instances de la structure LocalPlayer. Trouvé grâce à une IA.
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
        return LocalPlayer {name, token};
    }
}

impl IAPlayer {
    // Constructeur pour créer un nouveau joueur IA
    pub fn new_ia_player(name: String, token: char) -> Self 
    {
        IAPlayer {name, token}
    }

    // Fonction pour effectuer un coup au hasard
    pub fn make_random_move(&self) -> usize 
    {
        // Générez un nombre aléatoire entre 0 et 6 (ou la taille de votre grille - 1)
        let mut rng = rand::thread_rng();
        rng.gen_range(0..7) // Cela générera un nombre aléatoire entre 0 et 6 inclus
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

impl Player for IAPlayer {
    fn get_name(&self) -> &str 
    {
        &self.name
    }

    fn get_token(&self) -> char 
    {
        self.token
    }
}


