/* 
 * game/player.rs
 * 
 * module du joueur
 * 
 */

use rand::Rng;

// Déclaration d'un trait nommé JoueurTrait
pub trait Player
{
    fn get_name(&self) -> &str;
    fn get_token(&self) -> char;
}

// Déclaration d'une structure nommée Player
#[derive(PartialEq)] // Permettra de comparer 2 instances de la structure LocalPlayer.
pub struct LocalPlayer
{
    pub name: String,
    pub token: char,
}

// Déclaration d'une structure nommée IAPlayer
pub struct IAPlayer
{
    pub name: String,
    pub token: char,
}

// Implémentation de méthodes pour la structure LocalPlayer
impl LocalPlayer
{
    // Permet d'initialiser et d'instancier un nouveau joueur local
    pub fn new_local_player(name: String, token: char) -> Self
    {
        LocalPlayer {name, token}
    }
}

// Implémentation de méthodes pour la structure IAPlayer
impl IAPlayer
{
    // Permet d'initialiser et d'instancier un nouveau joueur IA
    pub fn new_ia_player(name: String, token: char) -> Self
    {
        IAPlayer {name, token}
    }

    // Permet de générer un nombre aléatoire entre 0 et 6 correspondant au nombre d'une colonne
    pub fn make_random_move(&self) -> usize
    {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..7)
    }
}

// Implémentation du trait Player pour la structure LocalPlayer
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

// Implémentation du trait Player pour la structure IAPlayer
impl Player for IAPlayer
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