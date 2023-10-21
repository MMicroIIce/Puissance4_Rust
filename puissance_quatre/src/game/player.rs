// src/jeu/player.rs

use std::time::Duration;
use rand::Rng;

// TODO : faire 3 implémentations de traits : LOCAL, ONLINE, IA
// TODO : pour IA tu fais un truc rapide qui place un pion au pif, ne rien faire pour le ONLINE
// TODO : permettre au joueur de choisir son nom

// Déclaration d'un trait nommé JoueurTrait
pub trait Player
{
    fn get_name(&self) -> &str;
    fn get_token(&self) -> char;
    fn get_timer(&self) -> Duration;
}

// Déclaration d'une structure nommée IAPlayer
pub struct IAPlayer {
    pub name: String,
    pub token: char,
    pub timer: Duration,
}

// Déclaration d'une structure nommée Player
#[derive(PartialEq)] // Permettra de comparer 2 instances de la structure Player. Trouvé grâce à une IA (ChatGPT).
pub struct LocalPlayer 
{
    pub name: String,
    pub token: char,
    pub timer: Duration,
}

// Implémentation de méthodes pour la structure Player
impl LocalPlayer 
{
    // Constructeur pour créer un nouveau joueur
    pub fn new_player(name: String, token: char) -> Self 
    {
        let timer = Duration::new(0, 0);
        return LocalPlayer {name, token, timer};
    }
}

impl IAPlayer {
    // Constructeur pour créer un nouveau joueur IA
    pub fn new_ia_player(name: String, token: char) -> Self 
    {
        let timer = Duration::new(0, 0);
        IAPlayer { name, token, timer }
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

    fn get_timer(&self) -> Duration 
    {
        self.timer
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

    fn get_timer(&self) -> Duration 
    {
        self.timer
    }
}


