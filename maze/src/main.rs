use rand::prelude::Rng;

fn main() {
    maze(11);
}

fn maze(size:usize) {
    let mut rng = rand::thread_rng();

    let mut walls: Vec<Vec<bool>> = Vec::new();
    for _ in 0..size {
        let mut row = Vec::new();
        for _ in 0..size{
            row.push(false);
        }
        walls.push(row);
    }

    for i in 0..size{
        walls[0][i] = true;
        walls[i][0] = true;
        walls[size-1][i] = true;
        walls[i][size-1] = true;
    }

    for i in (2..size-2).step_by(2) {
        for j in (2..size-2).step_by(2){
            walls[i][j] = true;
            let direction = rng.gen_range(0..4);
            match direction {
                0 => {walls[i-1][j]=true;},
                1 => {walls[i+1][j]=true;},
                2 => {walls[i][j-1]=true;},
                3 => {walls[i][j+1]=true;},
                _ => {},
            }
        }
    }

    for i in 0..size {
        for j in 0..size{
            if walls[i][j]{
                print!("ZZ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}