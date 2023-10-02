pub struct Grid {
    column: i8,
    row: i8,
    tab: Vec<Vec<String>>,
}

pub fn grid_create(column_nb: i8, row_nb: i8) -> Grid {
    let tab = vec![vec![" ".to_string(); column_nb as usize]; row_nb as usize];
    
    let game_grid = Grid {
        column: column_nb,
        row: row_nb,
        tab: tab,
    };
    
    return game_grid;
}

/*fn grid_create(column: i8, row: i8) -> grid {
    let game_grid = grid{column, row};
    return game_grid;
}*/

pub fn grid_main(game_grid: &Grid) {
    println!("Nombre de colonnes : {}", game_grid.column);
    println!("Nombre de lignes : {}", game_grid.row);
    grid_print(game_grid);
}

fn grid_print(game_grid: &Grid) {
    println!("Tableau : ");
    println!("-----------------------------");
    for row in &game_grid.tab {
        for cell in row {
            print!("|");
            print!(" {} ", cell);
        }
        println!("|");
        println!("-----------------------------"); // Nouvelle ligne après chaque ligne de la grille
    }
}