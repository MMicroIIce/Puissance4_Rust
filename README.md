# Puissance4_Rust

Commandes utiles :
    - Build le projet : cargo build
    - Build + run le projet : cargo run
    - Générer la doc : cargo doc
    - Regénérer uniquement la doc du projet : cargo doc --package puissance_quatre

Répartition modulaire

    +---------------------------------------------------------+
    |                       Puissance 4                       |
    +---------------------------------------------------------+
            |                   |                    |
    +-------v-------+  +--------v-------+  +---------v--------+
    |   Interface   |  |    Logique     |  |    Timer/Thread  |
    |   Graphique   |  |    du Jeu      |  |     de Timer     |
    +---------------+  +----------------+  +------------------+


Interface Graphique : Cette partie du projet gère l'interface utilisateur et la visualisation du jeu. Vous pouvez utiliser une bibliothèque Rust telle que gtk-rs ou druid pour créer une interface graphique conviviale pour le jeu. Cette interface permettra aux joueurs d'interagir avec le jeu, de sélectionner des colonnes pour placer leurs jetons, et d'afficher le plateau de jeu et le timer.

Logique du Jeu : La logique du jeu gère le fonctionnement interne du Puissance 4. Elle comprend la création du plateau de jeu, la gestion des tours des joueurs, la vérification des victoires et la gestion des erreurs. Cette partie du projet contient également les règles du jeu, telles que les conditions de victoire.

Timer/Thread de Timer : Cette composante gère le timer du jeu, qui impose une limite de temps aux joueurs pour prendre leur décision. Vous pouvez utiliser un thread dédié pour gérer le timer, en utilisant Mutex pour garantir un accès sécurisé au compteur de temps. Lorsque le timer atteint zéro, le joueur peut perdre automatiquement.

Quand on allume le jeu :
    - Affiche directement le plateau de jeu

Plateau de jeu :
    - 7 cases de large, 6 cases de haut
    - Ne pas oublier le gravité ==> le joueur ne selectionne pas une case mais une colonne
    - Timer de chaque joueur
    - Bouton démarrer/recommencer
    - Bouton quitter

Déroulé d'une partie :
    - affichage du plateau
    - affichage du timer
    - tirage au sort de la couleur qui commence
    A chaque tour :
        - démarrer le chrono du joueur courant
        - le joueur clique sur une colonne
            - si colonne pleine ==> message erreur "Colonne pleine choisir une autre colonne"
            - sinon ajouter le jeton du joueur dans la première case libre en partant du bas
        - arreter le chrono du joueur courant
        - check si un puissance 4 à été fait
            - partir du pion mis pour lire les cases autour selon un ordre (haut + bas, haut droit + bas gauche, gauche + droite, haut gauche + bas droit)
                - si le pion est de la même couleur continuer
                - sinon arreter la lecture
                    - additionner les lignes selon la description (haut + bas, haut droit + bas gauche, gauche + droite, haut gauche + bas droit)
                    - si 4 ou plus
                        - déclarer victoire du joueur courant
                    - sinon
                        - passer au tour suivant
        - si le chrono du joueur courant tombe à zero
            - l'autre joueur à gagné
        - si 42 tours ont étés faits
            - match nul
    A la fin de la partie :
        - Afficher temps restant pour chacun
        - Si joueur gagnant
            - Afficher joueur gagnant
                - si victoire puissance 4
                    - mettre en évidence le puissance 4
                - si victoire au temps
                    - écrire message "Victoire au temps"
        - Si match nul
            - Afficher message "Match nul"
