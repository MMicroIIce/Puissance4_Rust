/* 
 * game/player.rs
 * 
 * module du gameplay
 * 
 * La gestion d'erreur local dans ce module a été obtenue après recherches sur plusieurs sites (pour trouver une méthode fonctionnel dans notre cas), et à l'aide d'une IA.
 * 
 */

use std::io;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::game::grid::Grid;
use crate::game::player::{LocalPlayer, IAPlayer, Player};

// Déclaration d'une structure nommée Gameplay
pub struct Gameplay
{
    pub grid: Grid,
    pub player1: LocalPlayer,
    pub player2: LocalPlayer,
    pub ia: IAPlayer,
    pub current_player: CurrentPlayer,
}

// Déclaration d'une énumération nommée GameMod, correspondant au mode de jeu choisi par le joueur
pub enum GameMod
{
    LocalVsLocal,
    LocalVsIA,
}

// Déclaration d'une énumération nommée CurrentPlayer, indiquant si nous sommes au tour du joueur 1 ou au tour du joueur 2 lors d'une partie locale
#[derive(Copy, Clone, PartialEq)]
pub enum CurrentPlayer
{
    Player1,
    Player2,
}

// Implémentation de méthodes pour la structure Gameplay
impl Gameplay
{
    // Permet d'initialiser et instancier un nouveau gameplay
    pub fn new_gameplay(grid: Grid, player1: LocalPlayer, player2: LocalPlayer, ia: IAPlayer) -> Self
    {
        Gameplay
        {
            grid,
            player1,
            player2,
            ia,
            current_player: CurrentPlayer::Player1,
        }
    }

    // Permet de retourner le joueur qui possède le tour
    pub fn get_player(&self, player: CurrentPlayer) -> &LocalPlayer
    {
        match player
        {
            CurrentPlayer::Player1 => &self.player1,
            CurrentPlayer::Player2 => &self.player2,
        }
    }

    // Permet de détecter une victoire sur les lignes de la grille s'il y en a une
    fn check_line_victory(&self, player_token: char) -> bool
    {
        // Vérifie la séquence de pions dans toutes les lignes
        for ligne in &self.grid.grid
        {
            let mut count = 0;
            for cellule in ligne.iter()
            {
                if *cellule == player_token
                {
                    count += 1;
                    if count == 4
                    {
                        return true; // Victoire détectée
                    }
                }
                else
                {
                    count = 0;
                }
            }
        }
        false // Aucune victoire détectée
    }
    
    // Permet de détecter une victoire sur les colonnes de la grille s'il y en a une
    fn check_column_victory(&self, player_token: char) -> bool
    {
        let colonnes = self.grid.grid[0].len();
    
        // Vérifie la séquence de pions dans toutes les colonnes
        for colonne in 0..colonnes
        {
            let mut count = 0;
            for ligne in &self.grid.grid
            {
                let cellule = ligne[colonne];
                if cellule == player_token
                {
                    count += 1;
                    if count == 4
                    {
                        return true; // Victoire détectée
                    }
                }
                else
                {
                    count = 0;
                }
            }
        }
        false // Aucune victoire détectée
    }

    // Permet de détecter une victoire sur les diagonales de la grille s'il y en a une
    // Pour la fonction suivante, Mr. Jouault a approuvé l'utilisation des indices pour les itérations
    fn check_diagonal_victory(&self, player_token: char) -> bool
    {
        let lignes = self.grid.grid.len();
        let colonnes = self.grid.grid[0].len();

        // Vérification de haut en bas (de gauche à droite)
        for i in 0..lignes - 3
        {
            for j in 0..colonnes - 3
            {
                if self.grid.grid[i][j] == player_token && self.grid.grid[i + 1][j + 1] == player_token && self.grid.grid[i + 2][j + 2] == player_token && self.grid.grid[i + 3][j + 3] == player_token
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
                if self.grid.grid[i][j] == player_token && self.grid.grid[i + 1][j - 1] == player_token && self.grid.grid[i + 2][j - 2] == player_token && self.grid.grid[i + 3][j - 3] == player_token
                {
                    println!("Victoire détectée en diagonale (de droite à gauche) !");
                    return true; // Victoire détectée
                }
            }
        }
        false // Aucune victoire détectée
    }

    // Fonction pour vérifier la victoire
    fn check_victory(&self, player_token: char) -> bool
    {
        // Vous pouvez réutiliser vos fonctions de vérification de victoire ici
        Gameplay::check_line_victory(self, player_token) || Gameplay::check_column_victory(self, player_token) || Gameplay::check_column_victory(self, player_token) || Gameplay::check_diagonal_victory(self, player_token)
    }

    // Permet au joueur de choisir un mode de jeu, en entrant son choix dans le terminal
    fn choose_mod() -> GameMod
    {
        println!("Choisissez un mode de jeu :");
        println!("Ecrivez 'local' pour une partie Local player vs Local player");
        println!("Ecrivez 'ia' pour une partie Local player vs IA player");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée.");
        let choice = input.trim().to_lowercase();
        match choice.as_str()
        {
            "local" => GameMod::LocalVsLocal,
            "ia" => GameMod::LocalVsIA,
            _ =>
            {
                println!("Choix de mode invalide, choisissez à nouveau.");
                Self::choose_mod()
            }
        }
    }

    // Permet de jouer une partie
    pub fn play(&mut self)
    {
        loop
        {
            self.current_player = CurrentPlayer::Player1;
            let mut _first_time = true;
            let mut tokens_places_player1 = 0;
            let game_mod = Self::choose_mod(); // le joueur choisit le mode de jeu

            // Utilisation d'un mutex pour sortir du thread séparé dans le cas où la partie est terminée avant la fin du minuteur
            let gameplay_finish = Arc::new(Mutex::new(false));
            let gameplay_finish_clone = Arc::clone(&gameplay_finish);

            // Minuterie lancé dans un thread séparé, utilisant un Mutex pour suivre et contrôler le délai d'une partie.
            let time_limit = Arc::new(Mutex::new(false));
            let time_limit_clone = Arc::clone(&time_limit);

            let time_limit_thread_handle = thread::spawn(move || 
            {
                let mut count = 0;
                
                while count < 5 // Pour définir le temps limite d'une partie, ici 120 secondes
                {
                    thread::sleep(Duration::from_secs(1));
                    count += 1;
                    let gameplay_finish = match gameplay_finish_clone.lock()
                    {
                        Ok(guard) => guard,
                        Err(err) =>
                        {
                            println!("Erreur lors de la récupération du verrou : {}", err);
                            std::process::exit(1); // on quitte le programme
                        }
                    };
                    if *gameplay_finish
                    {
                        return; // la partie est terminée, on sort de la fonction
                    }
                }

                let mut time_limit = match time_limit_clone.lock()
                {
                    Ok(guard) => guard,
                    Err(err) =>
                    {
                        println!("Erreur lors de la récupération du verrou : {}", err);
                        std::process::exit(1); // on quitte le programme
                    }
                };
                *time_limit = true;
            }); 
            
            while (!Gameplay::check_victory(&self, self.get_player(self.current_player).get_token()) || !Gameplay::check_victory(&self, self.ia.get_token())) && !*time_limit.lock().unwrap()
            {
                println!("Tour de {}", self.get_player(self.current_player).name);
                self.grid.display_grid();
                match self.grid.add_token(self.get_player(self.current_player).token, 0)
                {
                    // L'ajout du token a réussi
                    Ok(_) =>
                    {
                        if self.grid.check_full()
                        {
                            println!("Partie terminée. Match nul !");
                            break;
                        }
                        if self.get_player(self.current_player) == &self.player1
                        {
                            tokens_places_player1 += 1;
                        }
                        if tokens_places_player1 >= 4
                        {
                            if Gameplay::check_victory(&self, self.get_player(self.current_player).get_token())
                            {
                                self.grid.display_grid();
                                println!("Partie terminée. {} a gagné !", self.get_player(self.current_player).name);
                                if let Ok(mut gameplay_finish) = gameplay_finish.lock()
                                {
                                    *gameplay_finish = true; // Pour sortir du minuteur
                                    break;
                                }
                                else
                                {
                                    println!("Erreur lors du verrouillage pour terminer la partie");
                                    break;
                                }
                            }
                        }
                        match game_mod
                        {
                            // Si le mode de jeu est LocalVsLocal, on change le joueur actuel
                            GameMod::LocalVsLocal =>
                            {
                                if self.get_player(self.current_player) == &self.player1
                                {
                                    self.current_player = CurrentPlayer::Player2;
                                    _first_time = false;
                                }
                                else
                                {
                                    self.current_player = CurrentPlayer::Player1
                                };
                            }
                            // Si le mode de jeu est LocalVsIA, le joueur actuel reste le joueur 1 et l'IA place un pion
                            GameMod::LocalVsIA =>
                            {
                                match self.grid.add_token(self.ia.get_token(), self.ia.make_random_move())
                                {
                                    // L'ajout du token a réussi
                                    Ok(_) =>
                                    {
                                        if self.grid.check_full()
                                        {
                                            println!("Partie terminée. Match nul !");
                                            if let Ok(mut gameplay_finish) = gameplay_finish.lock()
                                            {
                                                *gameplay_finish = true; // Pour sortir du minuteur
                                                break;
                                            }
                                            else
                                            {
                                                println!("Erreur lors du verrouillage pour terminer la partie");
                                                break;
                                            }
                                        }
                                        if tokens_places_player1 >= 4
                                        {
                                            if Gameplay::check_victory(&self, self.ia.get_token())
                                            {
                                                self.grid.display_grid();
                                                println!("Partie terminée. IA a gagné !");
                                                if let Ok(mut gameplay_finish) = gameplay_finish.lock()
                                                {
                                                    *gameplay_finish = true; // Pour sortir du minuteur
                                                    break;
                                                }
                                                else
                                                {
                                                    println!("Erreur lors du verrouillage pour terminer la partie");
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    // L'ajout du token a échoué
                                    Err(err) =>
                                    {
                                        println!("Erreur pour l'ajout du token : {}", err);
                                    }
                                }
                            }
                        }
                    }
                    // L'ajout du token a échoué
                    Err(err) =>
                    {
                        println!("Erreur pour l'ajout du token : {}", err);
                    }
                }
            }

            if let Ok(guard) = time_limit.lock()
            {
                if let Ok(gameplay_finish) = gameplay_finish.lock()
                {
                    if !*gameplay_finish && *guard
                    {
                        println!("Le temps limite a été atteint, la partie est terminée. Match nul !");
                    }
                }
                else 
                {
                    println!("Erreur lors de la récupération du verrou pour le gameplay_finish");
                    std::process::exit(1); // quitte le programme
                }
            }
            else
            {
                println!("Erreur lors de la récupération du verrou pour le temps limite");
                std::process::exit(1); // quitte le programme
            }
            

            // Demandez si les joueurs veulent rejouer
            println!("Voulez-vous rejouer ? (oui/non)");
            let mut input = String::new();
            while input.trim().to_lowercase() != "oui" && input.trim().to_lowercase() != "non"
            {
                input.clear();
                io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée.");
            }
            if input.trim().to_lowercase() == "non"
            {
                break;
            }
            self.grid.empty_grid(); //On vide la grille

            if let Err(err) = time_limit_thread_handle.join() // On attend la fin du thread gérant le minuteur
            {
                eprintln!("Le thread de minuterie a échoué : {:?}", err);
            }
        }
    }
}
