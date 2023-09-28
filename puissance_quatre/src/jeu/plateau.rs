// // Dans src/jeu/plateau.rs

// Déclaration d'une structure nommée PlateauDeJeu
pub struct PlateauDeJeu {
    grille: Vec<Vec<char>>,  // champ grille de type Vec<Vec<char>> donc un vecteur de vecteur de char
}

// Implémentation de méthodes pour la structure PlateauDeJeu
impl PlateauDeJeu {
    
    // Méthode statique nommée nouveau
    pub fn nv_plateau() -> Self 
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

    pub fn verifier_victoire_ligne(&self, ligne: usize, joueur: char) -> bool {
        let ligne_actuelle = &self.grille[ligne];
    
        // Vérifiez la séquence de jetons dans cette ligne
        let mut count = 0;
        for cellule in ligne_actuelle.iter() {
            if *cellule == joueur {
                count += 1;
                if count == 4 {
                    return true; // Victoire détectée dans cette ligne
                }
            } else {
                count = 0; // Réinitialisez le compteur si une cellule différente est rencontrée
            }
        }
    
        false // Aucune victoire détectée
    }

    pub fn ajouter_jeton(&mut self, colonne: usize, joueur: char) -> Result<(), String> 
    {
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
    
                // Après avoir ajouté le jeton, vérifiez s'il y a une victoire dans cette ligne
                if self.verifier_victoire_ligne(ligne, joueur) {
                    println!("c'est gagné !");
                    return Ok(());
                }
    
                // Si la colonne est pleine mais pas de victoire, renvoyez une erreur
                return Ok(()); // Ou vous pouvez renvoyer un autre résultat en fonction de vos besoins
            }
        }
    
        // Si la colonne est pleine, renvoyez une erreur
        Err(String::from("Colonne pleine"))
    }
}



