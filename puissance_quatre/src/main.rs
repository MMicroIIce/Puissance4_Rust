// // Utilisation du module
// mod jeu;

// fn main() {
//     // Créez une instance de PlateauDeJeu en utilisant la méthode nv_plateau()
//     let mut plateau = jeu::plateau::PlateauDeJeu::nv_plateau();

//     // Création de deux joueurs
//     let joueur1 = jeu::joueur::Joueur::nouveau("Joueur 1".to_string(), 'X');
//     let joueur2 = jeu::joueur::Joueur::nouveau("Joueur 2".to_string(), 'O');

//     // Variable pour suivre le joueur actuel (commencez par le joueur 1)
//     let mut joueur_actuel = &joueur1;

//     // Boucle de jeu
//     loop {
//         // Affiche le plateau avec le nom du joueur actuel
//         println!("Tour de {}", joueur_actuel.nom);
//         plateau.afficher_plateau();

//         // Ajoute le jeton du joueur actuel dans la colonne choisie
//         match plateau.ajouter_jeton(joueur_actuel.jeton) {
//             Ok(_) => {
//                 // L'ajout du jeton a réussi
//                 // // Vérifiez s'il y a un gagnant ou un match nul
//                 // if plateau.verifier_victoire(joueur_actuel.jeton) {
//                 //     println!("{} a gagné !", joueur_actuel.nom);
//                 //     break; // Sortez de la boucle si un joueur a gagné
//                 // } else if plateau.est_plein() {
//                 //     println!("Match nul !");
//                 //     break; // Sortez de la boucle si le plateau est plein (match nul)
//                 // }

//                 // Passez au joueur suivant
//                 joueur_actuel = if joueur_actuel == &joueur1 {
//                     &joueur2
//                 } else {
//                     &joueur1
//                 };
//             }
//             Err(err) => {
//                 // L'ajout du jeton a échoué, affichez un message d'erreur
//                 println!("Erreur : {}", err);
//             }
//         }
//     }
// }


mod jeu;

fn main() {
    // Créez une instance de PlateauDeJeu avec une taille de grille de 7 colonnes x 6 lignes
    let plateau = jeu::plateau::PlateauDeJeu::new_plateau();

    // Créez deux joueurs
    let joueur1 = jeu::joueur::Joueur::new_joueur("Joueur 1".to_string(), 'X');
    let joueur2 = jeu::joueur::Joueur::new_joueur("Joueur 2".to_string(), 'O');

    // Créez une instance de Partie en utilisant les joueurs et le plateau créés
    let mut partie = jeu::partie::Partie::new_partie(plateau, &joueur1, &joueur2);

    // Commencez à jouer la partie
    partie.jouer();
}
