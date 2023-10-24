/* Module gameplay.rs
 * 
 * TODO : 
 * - ajouter le timer
 * - faire une macro pour la durée du Timer par joueur.
 * - tout commenter
 * - faire de la gestion d'erreur
 * - Simplifier, corriger, donner du sens (surtout d'un aspect modulable)
 * */

use std::io;

use crate::game::grid::Grid;
use crate::game::player::{LocalPlayer, IAPlayer, Player};
use crate::game::timer::{Timer, run_timer};

use std::time::Instant;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread;

pub enum GameMod {
    LocalVsLocal,
    LocalVsIA,
}

pub struct Gameplay
{
    pub grid: Grid,
    pub player1: LocalPlayer,
    pub player2: LocalPlayer,
    pub ia: IAPlayer,
    pub current_player: CurrentPlayer,
    pub timer_player1: Arc<Mutex<Timer>>,
    pub timer_player2: Arc<Mutex<Timer>>,
}

#[derive(Copy, Clone, PartialEq)]
pub enum CurrentPlayer
{
    Player1,
    Player2,
}

impl Gameplay
{
    pub fn new_gameplay(grid: Grid, player1: LocalPlayer, player2: LocalPlayer, ia: IAPlayer, timer_player1: Timer, timer_player2: Timer) -> Self 
    {
        Gameplay 
        {
            grid,
            player1,
            player2,
            ia,
            current_player: CurrentPlayer::Player1, // Initialisez le player actuel avec player1
            timer_player1: Arc::new(Mutex::new(timer_player1)),
            timer_player2: Arc::new(Mutex::new(timer_player2)),
        }
    }

    pub fn get_player(&self, player: CurrentPlayer) -> &LocalPlayer
    {
        match player {
            CurrentPlayer::Player1 => &self.player1,
            CurrentPlayer::Player2 => &self.player2,
        }

    }

    pub fn get_player_mut(&mut self, player: CurrentPlayer) -> &mut LocalPlayer
    {
        match player {
            CurrentPlayer::Player1 => &mut self.player1,
            CurrentPlayer::Player2 => &mut self.player2,
        }

    }

    fn check_line_victory(&self, player_token: char) -> bool 
    {
        // Vérifiez la séquence de tokens dans toutes les lignes
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
    
    fn check_column_victory(&self, player_token: char) -> bool 
    {
        let colonnes = self.grid.grid[0].len();
    
        // Parcourez toutes les colonnes pour vérifier s'il y a 4 tokens consécutifs du player spécifié
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
                if self.grid.grid[i][j] == player_token
                    && self.grid.grid[i + 1][j + 1] == player_token
                    && self.grid.grid[i + 2][j + 2] == player_token
                    && self.grid.grid[i + 3][j + 3] == player_token
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
                if self.grid.grid[i][j] == player_token
                    && self.grid.grid[i + 1][j - 1] == player_token
                    && self.grid.grid[i + 2][j - 2] == player_token
                    && self.grid.grid[i + 3][j - 3] == player_token
                {
                    println!("Victoire détectée en diagonale (de droite à gauche) !");
                    return true; // Victoire détectée
                }
            }
        }

        false
    }

    pub fn check_time(&self, player: CurrentPlayer, duration: Duration) -> bool {
        let timer = match player {
            CurrentPlayer::Player1 => &self.timer_player1,
            CurrentPlayer::Player2 => &self.timer_player2,
        };

        if timer.lock().unwrap().elapsed_time() >= duration {
            return true;
        }

        false
    }

    // Fonction pour vérifier la victoire
    fn check_victory(&self, player_token: char) -> bool 
    {
        // Vous pouvez réutiliser vos fonctions de vérification de victoire ici
        Gameplay::check_line_victory(self, player_token)
            || Gameplay::check_column_victory(self, player_token)
            || Gameplay::check_column_victory(self, player_token)
            || Gameplay::check_diagonal_victory(self, player_token)
    }

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
            _ => {
                println!("Choix invalide, choisissez à nouveau.");
                Self::choose_mod()
            }
        }
    }

    // Fonction principale pour jouer la partie
    pub fn play(&mut self) 
    {
        self.current_player = CurrentPlayer::Player1;

        let mut first_time = true;

        let mut time_finish = false;

        let mut tokens_places_player1 = 0;

        let mut game_mod = Self::choose_mod();

        // TODO : j'ai l'impression que le chrono commence avant, genre dès qu'on appelle choose_mod

        // Clonez les chronomètres pour les passer aux threads
        let timer_player1_clone = self.timer_player1.clone();
        let timer_player2_clone = self.timer_player2.clone();

        // Créez deux threads pour gérer les chronomètres
        let timer_thread1 = thread::spawn({
            let timer_player1_clone = timer_player1_clone.clone();
            move || {
                run_timer(timer_player1_clone);
            }
        });

        timer_player1_clone.lock().unwrap().reset();
        
        let timer_thread2 = thread::spawn({
            let timer_player2_clone = timer_player2_clone.clone();
            move || {
                run_timer(timer_player2_clone);
            }
        });
        
        
        loop 
        {
            while (!Gameplay::check_victory(&self, self.get_player(self.current_player).get_token()) || !Gameplay::check_victory(&self, self.ia.get_token())) && time_finish == false
            {
                // Enregistrez le moment où le tour du joueur commence
                let turn_start = Instant::now();
                println!("Tour de {}", self.get_player(self.current_player).name);
                self.grid.display_grid();

                match self.grid.add_token(self.get_player(self.current_player).token, 0) 
                {
                    // L'ajout du token a réussi
                    Ok(_) => 
                    {
                        if self.grid.ask_full() 
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
                                break;
                            }
                        }

                        match game_mod
                        {
                            GameMod::LocalVsLocal =>
                            {
                                if self.get_player(self.current_player) == &self.player1 
                                {
                                    self.current_player = CurrentPlayer::Player2;
                                    
                                    if first_time == true
                                    {
                                        timer_player2_clone.lock().unwrap().reset();
                                        first_time = false;
                                    }

                                } else 
                                {
                                    self.current_player = CurrentPlayer::Player1
                                };
                            }

                            GameMod::LocalVsIA =>
                            {
                                match self.grid.add_token(self.ia.get_token(), self.ia.make_random_move()) 
                                {
                                    // L'ajout du token a réussi
                                    Ok(_) => 
                                    {
                                        if self.grid.ask_full() 
                                        {
                                            println!("Partie terminée. Match nul !");
                                            break;
                                        }
                                        if tokens_places_player1 >= 4 
                                        {
                                            if Gameplay::check_victory(&self, self.ia.get_token()) 
                                            {
                                                self.grid.display_grid();
                                                println!("Partie terminée. IA a gagné !");
                                                break;
                                            }
                                        }
                                    }
                                    // L'ajout du token a échoué, affichez un message d'erreur
                                    Err(err) => 
                                    {
                                        println!("Erreur : {}", err);
                                    }
                                }
                            }
                        }
                    }

                    // L'ajout du token a échoué, affichez un message d'erreur
                    Err(err) => 
                    {
                        println!("Erreur : {}", err);
                    }
                }

                // Enregistrez le moment où le tour du joueur se termine
                let turn_end = Instant::now();

                // Calculez la durée du tour
                let turn_duration = turn_end - turn_start;

                // Vérifiez le chronomètre du joueur actuel
                if self.current_player == CurrentPlayer::Player1 {
                    let player1_time = timer_player1_clone.lock().unwrap().elapsed_time();
                    println!("Temps écoulé pour {}: {} seconds and {} nanoseconds", self.get_player(self.current_player).name, player1_time.as_secs(), player1_time.subsec_nanos());
                    timer_player2_clone.lock().unwrap().subtract_time(turn_duration);
                    if self.check_time(CurrentPlayer::Player1, Duration::from_secs(5)) {
                        println!("Le joueur {} a dépassé le temps limite, il perd la partie.", self.get_player(self.current_player).name);
                        // Ajoutez ici le code pour gérer la fin de la partie pour le joueur qui a dépassé le temps.
                        time_finish = true;
                    }
                } else {
                    let player2_time = timer_player2_clone.lock().unwrap().elapsed_time();
                    println!("Temps écoulé pour {}: {} seconds and {} nanoseconds", self.get_player(self.current_player).name, player2_time.as_secs(), player2_time.subsec_nanos());
                    timer_player1_clone.lock().unwrap().subtract_time(turn_duration);
                    if self.check_time(CurrentPlayer::Player2, Duration::from_secs(5)) {
                        println!("Le joueur {} a dépassé le temps limite, il perd la partie.", self.get_player(self.current_player).name);
                        // Ajoutez ici le code pour gérer la fin de la partie pour le joueur qui a dépassé le temps.
                        time_finish = true;
                    }
                }
            }

            // Demandez si les players veulent rejouer
            println!("Voulez-vous rejouer ? (oui/non)");
            let mut input = String::new();

            while input.trim().to_lowercase() != "oui" && input.trim().to_lowercase() != "non"
            {
                input.clear();
                io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée.");
            }

            if input.trim().to_lowercase() != "oui" 
            {
                break; // Sortez de la boucle infinie si la réponse n'est pas "oui"
            }

            self.grid.empty_grid();
            tokens_places_player1 = 0;

            game_mod = Self::choose_mod();

            timer_player1_clone.lock().unwrap().reset();

            time_finish = false;
            first_time = true;
        }

        // Terminer le thread du timer lorsque le jeu est terminé
        timer_thread1.join().unwrap();
        timer_thread2.join().unwrap();
    }
}
