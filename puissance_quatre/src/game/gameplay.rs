use std::io;

use crate::game::grid::Grid;
use crate::game::player::LocalPlayer;

pub struct Gameplay<'a> 
{
    pub grid: Grid,
    pub player1: &'a LocalPlayer,
    pub player2: &'a LocalPlayer,
    pub current_player: &'a LocalPlayer, // Utilisez une référence mutable pour suivre le player actuel
}

impl<'a> Gameplay<'a> 
{
    pub fn new_gameplay(grid: Grid, player1: &'a LocalPlayer, player2: &'a LocalPlayer) -> Self 
    {
        Gameplay 
        {
            grid,
            player1,
            player2,
            current_player: player1, // Initialisez le player actuel avec player1
        }
    }

    pub fn check_line_victory(grid: &Vec<Vec<char>>, player: &'a LocalPlayer) -> bool 
    {
        // Vérifiez la séquence de tokens dans toutes les lignes
        for ligne in grid 
        {
            // TODO : comprendre le .iter()
            let mut count = 0;
            for cellule in ligne.iter() 
            {
                if *cellule == player.token
                {
                    count += 1;
                    if count == 4 
                    {
                        return true; // Victoire détectée dans cette ligne
                    }
                } 
                else 
                {
                    count = 0; // Réinitialisez le compteur si une cellule différente est rencontrée
                }
            }
        }
    
        false // Aucune victoire détectée dans toutes les lignes
    }
    
    pub fn check_column_victory(grid: &Vec<Vec<char>>, player: &'a LocalPlayer) -> bool 
    {
        let colonnes = grid[0].len();
    
        // Parcourez toutes les colonnes pour vérifier s'il y a 4 tokens consécutifs du player spécifié
        for colonne in 0..colonnes 
        {
            let mut count = 0;
            for ligne in grid 
            {
                let cellule = ligne[colonne];
                if cellule == player.token 
                {
                    count += 1;
                    if count == 4 
                    {
                        return true; // Victoire détectée dans cette colonne
                    }
                } 
                else 
                {
                    count = 0; // Réinitialisez le compteur si une cellule différente est rencontrée
                }
            }
        }
    
        false // Aucune victoire détectée dans toutes les colonnes
    }

    // TODO : faire attention aux dépassements du tableau, essayer sans indice ou mettre une gestion d'erreur à la fin si besoin.
    // TODO : rajouter un com qui dit que Jouault a vu cette partie et est en accord avec celle ci
    pub fn check_diagonal_victory(grid: &Vec<Vec<char>>, player: &'a LocalPlayer) -> bool 
    {
        let lignes = grid.len();
        let colonnes = grid[0].len();

        // Vérification de haut en bas (de gauche à droite)
        for i in 0..lignes - 3 
        {
            for j in 0..colonnes - 3 
            {
                if grid[i][j] == player.token
                    && grid[i + 1][j + 1] == player.token
                    && grid[i + 2][j + 2] == player.token
                    && grid[i + 3][j + 3] == player.token
                {
                    println!("Victoire détectée en diagonale (de gauche à droite) !");
                    return true; // Victoire détectée
                }
            }
        }

        // Vérification de haut en bas (de droite à gauche)
        for i in 0..lignes - 3 
        {
            for j in 3..colonnes 
            {
                if grid[i][j] == player.token
                    && grid[i + 1][j - 1] == player.token
                    && grid[i + 2][j - 2] == player.token
                    && grid[i + 3][j - 3] == player.token
                {
                    println!("Victoire détectée en diagonale (de droite à gauche) !");
                    return true; // Victoire détectée
                }
            }
        }

        false
    }

    // Fonction pour vérifier la victoire
    fn check_victory(grid: &Vec<Vec<char>>, player: &'a LocalPlayer) -> bool 
    {
        // Vous pouvez réutiliser vos fonctions de vérification de victoire ici
        Gameplay::check_line_victory(grid, player)
            || Gameplay::check_column_victory(grid, player)
            || Gameplay::check_column_victory(grid, player)
            || Gameplay::check_diagonal_victory(grid, player)
    }

    // Fonction principale pour jouer la partie
    pub fn play(&mut self) 
    {
        self.current_player = self.player1;

        let mut tokens_places_player1 = 0;

        loop 
        {
            while !Gameplay::check_victory(&self.grid.grid, self.current_player) 
            {
                println!("Tour de {}", self.current_player.name);
                self.grid.display_grid();

                match self.grid.add_token(self.current_player.token) 
                {
                    // L'ajout du token a réussi
                    Ok(_) => 
                    {
                        if self.grid.ask_full() 
                        {
                            println!("Partie terminée. Match nul !");
                            break;
                        }

                        if self.current_player == self.player1 
                        {
                            tokens_places_player1 += 1;
                        }
                        if tokens_places_player1 >= 4 
                        {
                            if Gameplay::check_victory(&self.grid.grid, self.current_player) 
                            {
                                self.grid.display_grid();
                                println!("Partie terminée. {} a gagné !", self.current_player.name);
                                break;
                            }
                        }

                        self.current_player = if self.current_player == self.player1 
                        {
                            self.player2
                        } else 
                        {
                            self.player1
                        };
                    }
                    // L'ajout du token a échoué, affichez un message d'erreur
                    Err(err) => 
                    {
                        println!("Erreur : {}", err);
                    }
                }
            }

            // Demandez si les players veulent rejouer
            println!("Voulez-vous rejouer ? (oui/non)");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée.");

            if input.trim().to_lowercase() != "oui" 
            {
                break; // Sortez de la boucle infinie si la réponse n'est pas "oui"
            }

            self.grid.empty_grid();
            tokens_places_player1 = 0;
        }
    }
}
