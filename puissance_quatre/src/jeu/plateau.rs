// // Dans src/jeu/plateau.rs

use std::io;

// Déclaration d'une structure nommée PlateauDeJeu
pub struct PlateauDeJeu {
    pub grille: Vec<Vec<char>>,  // champ grille de type Vec<Vec<char>> donc un vecteur de vecteur de char
}

// Implémentation de méthodes pour la structure PlateauDeJeu
impl PlateauDeJeu {
    
    // Méthode statique nommée nouveau
    pub fn new_plateau() -> Self 
    { //Self indique qu'il retournera une instance de PlateauDeJeu
        
        // Initialisation d'un plateau vide avec 6 lignes et 7 colonnes
        let grille = vec![vec![' '; 7]; 6];  // Crée une grille 2D remplie de caractères ' '
        PlateauDeJeu { grille }  // Crée une nouvelle instance de PlateauDeJeu avec la grille initialisée
    
    }

    // Méthode pour afficher le plateau de jeu dans la console
    pub fn afficher_plateau(&self) 
    {
        for ligne in &self.grille 
        {
            for cellule in ligne 
            {
                print!("{} ", cellule);
            }
            println!(); // Passage à la ligne pour afficher la prochaine ligne du plateau
        }
    }
  
    // Fonction pour demander à l'utilisateur dans quelle colonne placer le jeton
    fn demander_colonne(&self) -> usize {
        loop {
            println!("Dans quelle colonne souhaitez-vous placer votre jeton (0-{}):", self.grille[0].len() - 1);

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Échec de la lecture de l'entrée");

            // Convertir l'entrée de l'utilisateur en un nombre
            match input.trim().parse::<usize>() {
                Ok(colonne) if colonne < self.grille[0].len() => {
                    return colonne;
                }
                _ => {
                    println!("Colonne invalide. Veuillez choisir une colonne valide.");
                    continue;
                }
            }
        }
    }

    // Méthode publique pour vérifier si le plateau est plein
    pub fn est_plein(&self) -> bool {
        for ligne in &self.grille {
            for cellule in ligne {
                if *cellule == ' ' {
                    return false; // Il y a au moins une case vide
                }
            }
        }
        true // Le plateau est plein
    }

    pub fn ajouter_jeton(&mut self, joueur: char) -> Result<(), String> 
    {
        // Demander au joueur dans quelle colonne placer le jeton
        let colonne = self.demander_colonne();

        // Vérifiez que la colonne est valide
        if colonne >= self.grille[0].len() 
        {
            return Err(String::from("Colonne invalide"));
        }
    
        // Parcourez la colonne de bas en haut pour trouver la première case vide
        for ligne in (0..self.grille.len()).rev() 
        {
            if self.grille[ligne][colonne] == ' ' 
            {
                self.grille[ligne][colonne] = joueur;
    
                // // Après avoir ajouté le jeton, vérifiez s'il y a une victoire dans cette ligne
                // if Partie::verifier_victoire_ligne(&self.grille, ligne, joueur) {
                //     println!("c'est gagné pour une ligne !");
                //     partie.etat = EtatPartie::Gagnee; // Mettez à jour l'état de la partie en cas de victoire
                //     return Ok(());
                // }
                // else if Partie::verifier_victoire_colonne(&self.grille, colonne, joueur) {
                //     println!("c'est gagné pour une colonne !");
                //     partie.etat = EtatPartie::Gagnee; // Mettez à jour l'état de la partie en cas de victoire
                //     return Ok(());
                // }
                // else if Partie::verifier_victoire_diagonale(&self.grille, joueur) {
                //     println!("c'est gagné pour une diagonale !");
                //     partie.etat = EtatPartie::Gagnee; // Mettez à jour l'état de la partie en cas de victoire
                //     return Ok(());
                // }
    
                // Si la colonne est pleine mais pas de victoire, renvoyez une erreur
                return Ok(()); // Ou vous pouvez renvoyer un autre résultat en fonction de vos besoins
            }
        }
    
        // Si la colonne est pleine, renvoyez une erreur
        Err(String::from("Colonne pleine"))
    }
}



