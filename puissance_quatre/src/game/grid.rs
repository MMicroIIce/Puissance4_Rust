// src/game/grid.rs

use std::io;

// Déclaration d'une structure nommée Grid
pub struct Grid 
{
    pub grid: Vec<Vec<char>>,  // champ grid de type Vec<Vec<char>> donc un vecteur de vecteur de char
}


// Implémentation de méthodes pour la structure Grid
impl Grid
{
    
    // Méthode statique nommée nouveau
    pub fn new_grid() -> Self 
    {
        
        // Initialisation d'un plateau vide avec 6 lignes et 7 colonnes
        let grid = vec![vec![' '; 7]; 6];  // Crée une grid 2D remplie de caractères ' '
        Grid {grid}  // Crée une nouvelle instance de Grid avec la grid initialisée
    
    }

    // Méthode pour afficher le plateau de jeu dans la console
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
            println!("-----------------------------"); // Nouvelle ligne après chaque ligne de la grid
        }
    }
  
    // Fonction pour demander à l'utilisateur dans quelle colonne placer le jeton
    fn ask_column(&self) -> usize 
    {
        loop 
        {
            println!("Dans quelle colonne souhaitez-vous placer votre jeton (0-{}):", self.grid[0].len() - 1);

            let mut input = String::new();

            // Pour lire la ligne entrée par l'utilisateur, obtenu grâce à une IA
            io::stdin()
                .read_line(&mut input)
                .expect("Échec de la lecture de l'entrée");

            // Convertir l'entrée de l'utilisateur en un nombre, obtenu grâce à une IA
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

    // Méthode publique pour vérifier si le plateau est plein
    pub fn ask_full(&self) -> bool 
    {
        for ligne in &self.grid
        {
            for cellule in ligne 
            {
                if *cellule == ' ' 
                {
                    return false; // Il y a au moins une case vide
                }
            }
        }
        true // Le plateau est plein
    }

    pub fn add_token(&mut self, player: char) -> Result<(), String> 
    {
        // Demander au joueur dans quelle colonne placer le jeton
        let colonne = self.ask_column();

        // Vérifiez que la colonne est valide
        if colonne >= self.grid[0].len() 
        {
            return Err(String::from("Colonne invalide"));
        }
    
        // Parcourez la colonne de bas en haut pour trouver la première case vide
        for ligne in (0..self.grid.len()).rev() 
        {
            if self.grid[ligne][colonne] == ' ' 
            {
                self.grid[ligne][colonne] = player;
    
                return Ok(()); // Ou vous pouvez renvoyer un autre résultat en fonction de vos besoins
            }
        }
    
        // Si la colonne est pleine, renvoyez une erreur
        Err(String::from("Colonne pleine"))
    }

    // Méthode publique pour vider le plateau et remettre des cases vides
    pub fn empty_grid(&mut self) 
    {
        for ligne in &mut self.grid 
        {
            for cellule in ligne 
            {
                *cellule = ' '; // Réinitialisez chaque cellule avec un espace
            }
        }
    }
}



