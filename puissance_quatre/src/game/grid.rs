/* 
 * game/grid.rs
 * 
 * module du plateau de jeu
 * 
 */

use std::io;
use std::io::Write;

// Déclaration d'une structure nommée Grid
pub struct Grid 
{
    pub grid: Vec<Vec<char>>,
}


// Implémentation de méthodes pour la structure Grid
impl Grid
{
    // Permet d'initialiser et instancier un nouveau plateau de jeu
    pub fn new_grid() -> Self
    {   
        let grid = vec![vec![' '; 7]; 6];
        Grid {grid}
    }

    // Permet d'afficher le plateau de jeu dans la console
    pub fn display_grid(&self)
    {
        println!("Tableau : ");
        println!("-----------------------------");
        for ligne in &self.grid
        {
            for cellule in ligne
            {
                print!("|");
                print!(" {} ", cellule);
            }
            println!("|");
            println!("-----------------------------");
        }
    }
  
    // Permet de demander à un joueur dans quelle colonne il souhaite placer un jeton
    fn ask_column(&self) -> usize
    {
        loop
        {
            println!("Dans quelle colonne souhaitez-vous placer votre jeton (0-{}):", self.grid[0].len() - 1);
            
            io::stdout().flush().ok(); // Pour s'assurer que le message s'affiche sans retard, trouvé à l'aide d'une IA
            let mut input = String::new();
            if let Err(_) = io::stdin().read_line(&mut input)
            {
                println!("Erreur lors de la lecture de l'entrée");
                continue;
            }

            // La ligne suivante a été obtenue à l'aide d'une IA
            match input.trim().parse::<usize>()
            {
                Ok(colonne) if colonne < self.grid[0].len() => 
                {
                    return colonne;
                }
                _ =>
                {
                    println!("Colonne invalide. Veuillez choisir une colonne valide.");
                    continue;
                }
            }
        }
    }

    // Permet de vérifier si le plateau de jeu est plein ou s'il reste une ou plusieurs cases vides
    pub fn check_full(&self) -> bool
    {
        for ligne in &self.grid
        {
            for cellule in ligne
            {
                if *cellule == ' '
                {
                    return false;
                }
            }
        }
        true
    }

    // Permet d'ajouter un jeton sur le plateau de jeu
    pub fn add_token(&mut self, player_token: char, nb: usize) -> Result<(), String>
    {
        let mut colonne = nb;

        //Si ce n'est pas l'IA
        if player_token != 'W'
        {
            colonne = self.ask_column();
        }

        if colonne >= self.grid[0].len()
        {
            return Err(String::from("Colonne invalide"));
        }

        // Cherche la 1ère case vide de la colonne
        for ligne in (0..self.grid.len()).rev()
        {
            if self.grid[ligne][colonne] == ' '
            {
                self.grid[ligne][colonne] = player_token;
                return Ok(());
            }
        }
    
        Err(String::from("Colonne pleine"))
    }

    // Permet d'enlever tous les jetons du plateau de jeu
    pub fn empty_grid(&mut self) 
    {
        for ligne in &mut self.grid
        {
            for cellule in ligne
            {
                *cellule = ' ';
            }
        }
    }
}



