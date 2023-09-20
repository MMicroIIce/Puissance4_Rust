# Puissance4_Rust


Répartition modulaire

    +-----------------------------------------------------+
    |                    Puissance 4                      |
    +-----------------------------------------------------+
            |                   |                    |
    +-------v--------+  +-------v--------+  +--------v--------+
    |   Interface   |  |    Logique    |  |    Timer/Thread  |
    |   Graphique   |  |    du Jeu    |  |     de Timer     |
    +---------------+  +---------------+  +------------------+


Interface Graphique : Cette partie du projet gère l'interface utilisateur et la visualisation du jeu. Vous pouvez utiliser une bibliothèque Rust telle que gtk-rs ou druid pour créer une interface graphique conviviale pour le jeu. Cette interface permettra aux joueurs d'interagir avec le jeu, de sélectionner des colonnes pour placer leurs jetons, et d'afficher le plateau de jeu et le timer.

Logique du Jeu : La logique du jeu gère le fonctionnement interne du Puissance 4. Elle comprend la création du plateau de jeu, la gestion des tours des joueurs, la vérification des victoires et la gestion des erreurs. Cette partie du projet contient également les règles du jeu, telles que les conditions de victoire.

Timer/Thread de Timer : Cette composante gère le timer du jeu, qui impose une limite de temps aux joueurs pour prendre leur décision. Vous pouvez utiliser un thread dédié pour gérer le timer, en utilisant Mutex pour garantir un accès sécurisé au compteur de temps. Lorsque le timer atteint zéro, le joueur peut perdre automatiquement.