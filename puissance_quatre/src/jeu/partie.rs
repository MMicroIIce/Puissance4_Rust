use crate::jeu::plateau::PlateauDeJeu;
use crate::jeu::joueur::Joueur;

pub struct Partie<'a> {
    pub plateau: PlateauDeJeu,
    pub joueur1: &'a Joueur,
    pub joueur2: &'a Joueur,
    pub joueur_actuel: &'a Joueur, // Utilisez une référence mutable pour suivre le joueur actuel
}


impl<'a> Partie<'a> {
    pub fn new_partie(plateau: PlateauDeJeu, joueur1: &'a Joueur, joueur2: &'a Joueur) -> Self {
        Partie {
            plateau,
            joueur1,
            joueur2,
            joueur_actuel: joueur1, // Initialisez le joueur actuel avec joueur1
        }
    }

    pub fn verifier_victoire_ligne(grille: &Vec<Vec<char>>, joueur: char) -> bool {
        // Vérifiez la séquence de jetons dans toutes les lignes
        for ligne in grille {
            let mut count = 0;
            for cellule in ligne.iter() {
                if *cellule == joueur {
                    count += 1;
                    if count == 4 {
                        return true; // Victoire détectée dans cette ligne
                    }
                } else {
                    count = 0; // Réinitialisez le compteur si une cellule différente est rencontrée
                }
            }
        }
    
        false // Aucune victoire détectée dans toutes les lignes
    }
    

    pub fn verifier_victoire_colonne(grille: &Vec<Vec<char>>, joueur: char) -> bool {
        let colonnes = grille[0].len();
    
        // Parcourez toutes les colonnes pour vérifier s'il y a 4 jetons consécutifs du joueur spécifié
        for colonne in 0..colonnes {
            let mut count = 0;
            for ligne in grille {
                let cellule = ligne[colonne];
                if cellule == joueur {
                    count += 1;
                    if count == 4 {
                        return true; // Victoire détectée dans cette colonne
                    }
                } else {
                    count = 0; // Réinitialisez le compteur si une cellule différente est rencontrée
                }
            }
        }
    
        false // Aucune victoire détectée dans toutes les colonnes
    }

    pub fn verifier_victoire_diagonale(grille: &Vec<Vec<char>>, joueur: char) -> bool {
        let lignes = grille.len();
        let colonnes = grille[0].len();

        // Vérification de haut en bas (de gauche à droite)
        for i in 0..lignes - 3 {
            for j in 0..colonnes - 3 {
                if grille[i][j] == joueur
                    && grille[i + 1][j + 1] == joueur
                    && grille[i + 2][j + 2] == joueur
                    && grille[i + 3][j + 3] == joueur
                {
                    println!("Victoire détectée en diagonale (de gauche à droite) !");
                    return true; // Victoire détectée
                }
            }
        }

        // Vérification de haut en bas (de droite à gauche)
        for i in 0..lignes - 3 {
            for j in 3..colonnes {
                if grille[i][j] == joueur
                    && grille[i + 1][j - 1] == joueur
                    && grille[i + 2][j - 2] == joueur
                    && grille[i + 3][j - 3] == joueur
                {
                    println!("Victoire détectée en diagonale (de droite à gauche) !");
                    return true; // Victoire détectée
                }
            }
        }

        false
    }

    // Fonction pour vérifier la victoire
    fn verifier_victoire(grille: &Vec<Vec<char>>, joueur: char) -> bool {
        // Vous pouvez réutiliser vos fonctions de vérification de victoire ici
        Partie::verifier_victoire_ligne(grille, joueur)
            || Partie::verifier_victoire_colonne(grille, joueur)
            || Partie::verifier_victoire_diagonale(grille, joueur)
    }

    // Fonction principale pour jouer la partie
    pub fn jouer(&mut self) {

        self.joueur_actuel = self.joueur1;

        while !Partie::verifier_victoire(&self.plateau.grille, self.joueur_actuel.jeton)
        {
            println!("Tour de {}", self.joueur_actuel.nom);
            self.plateau.afficher_plateau();

            match self.plateau.ajouter_jeton(self.joueur_actuel.jeton) {
                Ok(_) => {
                    // L'ajout du jeton a réussi
                    if self.plateau.est_plein()
                    {
                        println!("Partie terminée. Match nul !");
                        break;
                    }
                    if Partie::verifier_victoire(&self.plateau.grille, self.joueur_actuel.jeton) {
                        println!("Partie terminée. {} a gagné !", self.joueur_actuel.nom);
                        break; // Sortez de la boucle si la partie est terminée
                    }

                    self.joueur_actuel = if self.joueur_actuel == self.joueur1 {
                        self.joueur2
                    } else {
                        self.joueur1
                    };

                }
                Err(err) => {
                    // L'ajout du jeton a échoué, affichez un message d'erreur
                    println!("Erreur : {}", err);
                }
            }
        }
    }
}