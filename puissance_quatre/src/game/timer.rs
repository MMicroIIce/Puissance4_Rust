/* Module timer.rs
 * 
 * TODO : 
 * - Ajouter le timer
 * - Tout commenter
 * - Faire de la gestion d'erreur
 * - Simplifier, corriger, donner du sens (surtout d'un aspect modulable)
 * */

 use std::time::{Duration, Instant};
 use std::sync::{Arc, Mutex};
 use std::thread;
 
 /* Définition de la structure Timer : Vous avez créé une structure Timer qui contient un champ start_time de type Instant. 
  * elapsed_time est ajouté pour maintenir le temps écoulé depuis le démarrage du timer. */
 pub struct Timer
 {
     start_time: Instant,
     elapsed_time: Duration,
 }
 
 impl Timer {
     pub fn new_timer() -> Self
     {
         Timer {
             start_time: Instant::now(),
             elapsed_time: Duration::new(0, 0), // Initialisez le temps écoulé à zéro
         }
     }
 
     pub fn elapsed_time(&self) -> Duration
     {
         // Ajoutez le temps écoulé au temps de démarrage pour obtenir le temps total
         self.start_time.elapsed() + self.elapsed_time
     }
 
     pub fn reset(&mut self)
     {
         // Réinitialisez le temps de démarrage et le temps écoulé à zéro
         self.start_time = Instant::now();
         self.elapsed_time = Duration::new(0, 0);
     }
 
     pub fn subtract_time(&mut self, duration: Duration) {
         // Soustrayez la durée spécifiée du temps écoulé
         self.elapsed_time = self.elapsed_time.checked_sub(duration).unwrap_or_default();
     }
 }
 
 pub fn run_timer(timer: Arc<Mutex<Timer>>) {
     // Code du timer, déclenché par un thread
     loop {
         // Mettez à jour le timer
         {
             let mut timer = timer.lock().unwrap();
             let elapsed_time = timer.start_time.elapsed();
             //println!("Timer elapsed: {} seconds and {} nanoseconds", elapsed_time.as_secs(), elapsed_time.subsec_nanos());
         }
 
         // Mettez en pause le thread pendant 1 seconde
         thread::sleep(Duration::from_secs(1));
     }
 }
 