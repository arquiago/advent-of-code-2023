use std;

fn checkNum(grid: &[Vec<char>], i: usize, j: usize) -> bool {
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for (dx, dy) in &directions {
        let new_i = i as isize + dx;
        let new_j = j as isize + dy;

        if new_i >= 0 && new_i < grid.len() as isize && new_j >= 0 && new_j < grid[0].len() as isize {
            let new_i = new_i as usize;
            let new_j = new_j as usize;

            if !grid[new_i][new_j].is_digit(10) && grid[new_i][new_j] != '.' {
                return true;
            }
        }
    }
    false
}


fn main() {
    let input = std::fs::read_to_string("/home/archago/aoc/aoc3/input.txt").unwrap();
    let mut grid : Vec<Vec<char>> = Vec::new();
    for line in input.lines(){
        let mut row = Vec::new();
        for c in line.chars(){
            row.push(c);
        }
        grid.push(row);
    }
    let mut acc : u64 = 0;
    let mut aux = 0;
    let mut flagadd = false;
    let mut cont1 = 0;
    let mut cont2 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len(){
            if let Some(digit) = grid[i][j].to_digit(10){
                aux = aux * 10;
                aux += digit;
                println!("{}", aux);
                if checkNum(&grid, i, j){
                    flagadd = true;
                }
            }
            else{
                cont1 += 1;
                if flagadd {
                    println!("aux eh {}", aux);
                    cont2 += 1;
                    acc += aux as u64;
                }
                aux = 0;
                flagadd = false;
            }
            if j == grid[i].len() - 1{
                println!("entrou aux: {}", aux);
                if flagadd {
                    acc += aux as u64;
                    aux = 0;
                }
                flagadd = false;
                aux = 0;
            }
        }
    }
   println!("cont1 eh {} e cont2 eh {}", cont1, cont2);
   println!("resposta: {}", acc); 
}
