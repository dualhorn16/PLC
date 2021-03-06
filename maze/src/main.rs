#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;
extern crate rand;
use rand::Rng;
use std::collections::HashMap;
use std::io::prelude::*;
use std::collections::LinkedList;

pub const RIGHT: usize = 0;
pub const DOWN: usize = 1;
pub const LEFT: usize = 2;
pub const UP: usize = 3;

#[allow(dead_code)]
pub struct Solution {
    path: Vec<Vec<usize>>,
}
 
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Point {
    x: usize,
    y: usize,
    visited: bool,  
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Edge {
    p: Point,
    d: usize,
    used: bool,
    deleted: bool,
    dumdum: bool,
}

pub struct Size {
    size:usize,
}
#[allow(dead_code)]
pub struct Board {
    points: Vec<Vec<Point>>
}

fn find(mut a: usize, mut tree: Vec<Vec<usize>>) -> (usize) {
    let mut temp = a.clone();
    while tree[temp][0] != 999999{
            temp = tree[temp][0];
    }
    if a != temp{
        let mut k = tree[a][0];
        while k != temp{
            tree[a][0] = temp;
            a = k;
            k = tree[k][0];
        }
    }
    return temp;
}

fn union(a: usize, b: usize, mut tree: Vec<Vec<usize>>) -> (Vec<Vec<usize>>) {
    let rank_a = tree[a][1];
    let rank_b = tree[b][1];
    if rank_a < rank_b {
        tree[a][0] = b;
    }
    else if rank_a > rank_b {
        tree[b][0] = a;
    }
    else if rank_a == rank_b {
        tree[a][1] = tree[a][1] + 1;
        tree[b][0] = a;
    }
    return tree;
}

fn init_maze(input: usize) ->(Vec<Vec<Edge>>) {
    
    println!("A maze of size {} is coming up!", input);

    let total = input * input;

    let boarder:Point = Point{
        x:0,
        y:0,
        visited:true,
    };
    
    let dummy:Edge = Edge {
        p: boarder,
        d: 0,
        used: true,
        deleted: false,
        dumdum: true,
    } ;
    
    let mut board:Vec<Vec<Point>> = vec![]; 
    let mut graph:Vec<Vec<Edge>> = vec![vec![dummy;4]; total];

    for i in 0 .. input {
        let mut innervec:Vec<Point> = vec![];
        for j in 0 .. input {
            
            let temp = Point{
                x: j,
                y: i,
                visited: false,
            };
            
            let pindex = i*input+j;
            
            if j < input-1 {
                graph[pindex][RIGHT] = Edge{
                    p:temp.clone(),
                    d:RIGHT,
                    used: false,
                    deleted: false,
                    dumdum: false,
                };
            }
            else {
                graph[pindex][RIGHT] = dummy.clone();
            }
            if i < input-1 {
                graph[pindex][DOWN] = Edge{
                    p:temp.clone(),
                    d:DOWN,
                    used: false,
                    deleted: false,
                    dumdum: false,
                };
            }
            else {
                graph[pindex][DOWN] = dummy.clone();
            }
            if j > 0 {
                graph[pindex][LEFT] = graph[pindex-1][RIGHT];
            }
            else {
                graph[pindex][LEFT] = dummy.clone();
            }
            if i > 0 {
                graph[pindex][UP] = graph[pindex-input][DOWN];
            }
            else {
                graph[pindex][UP] = dummy.clone();
            }


            innervec.push(temp.clone());
        }
        board.push(innervec);
        
    }
    return graph.clone();
}

fn mk_maze(size:usize, mut graph: Vec<Vec<Edge>>, mut tree: Vec<Vec<usize>>) ->(Vec<Vec<Edge>>) {
    
    let mut count_of_roots = size * size;
    let mut rng = rand::thread_rng();
   
    while count_of_roots != 1 {
        let random_index = rng.gen::<usize>() % (size * size);
        let random_dir = rng.gen::<usize>() % 4;
        
        if (!graph[random_index][random_dir].dumdum) || (!graph[random_index][random_dir].used &&
            !graph[random_index][random_dir].deleted)
            {
            //BELOW: IF statements to find direction and index of neighboring cell (the shared edge)
            //BELOW: If direction of edge is RIGHT, then find box to RIGHT of index.
            if random_dir == RIGHT{
                //BELOW: Get the roots of both the neighboring boxes.
                let a = find(random_index, tree.clone());
                let b = find(random_index+1, tree.clone());
                //BELOW: IF not in same set then set delete and combine the sets and decrease num of sets by 1.
                if a != b {
                    
                    graph[random_index][RIGHT].deleted = true;
                    graph[random_index+1][LEFT].deleted = true;
                    tree = union(a,b,tree.clone());
                    count_of_roots = count_of_roots - 1;
                }
                //BELOW: IF in same set, then set the used field for that index & direction combo.
                else{
                    graph[random_index][random_dir].used = true;
                }                           
            }
            //BELOW: If direction of edge is DOWN, then find box to DOWN of index.
            else if random_dir == DOWN{
                //BELOW: Get the roots of both the neighboring boxes.
                let a = find(random_index,tree.clone());
                let b = find(random_index+size,tree.clone());
                //BELOW: IF not in same set then set delete and combine the sets and decrease num of sets by 1.
                if a != b {
                    
                    graph[random_index][DOWN].deleted = true;
                    graph[random_index+size][UP].deleted = true;
                    tree = union(a,b,tree.clone());
                    count_of_roots = count_of_roots - 1;
                }
                //BELOW: IF in same set, then set the used field for that index & direction combo.
                else {
                    graph[random_index][random_dir].used = true;
                }
            }
            //BELOW: If direction of edge is LEFT, then find box to left of index.
            else if random_dir == LEFT{
                //BELOW: Get the roots of both the neighboring boxes.
                let a = find(random_index, tree.clone());
                let b = find(random_index-1, tree.clone());
                //BELOW: IF not in same set then set delete and combine the sets and decrease num of sets by 1.
                if a != b {
                    
                    graph[random_index][LEFT].deleted = true;
                    graph[random_index-1][RIGHT].deleted = true;
                    tree = union(a,b,tree.clone());
                    count_of_roots = count_of_roots - 1;
                }
                //BELOW: IF in same set, then set the used field for that index & direction combo.
                else{
                    graph[random_index][random_dir].used = true;
                }
            }
            //BELOW: ELSE is for if direction is up.
            else{
                //BELOW: Get the roots of both the neighboring boxes.
                let a = find(random_index,tree.clone());
                let b = find(random_index-size,tree.clone());
                //BELOW: IF not in same set then set delete and combine the sets and decrease num of sets by 1.
                if a != b {
                   
                    graph[random_index][UP].deleted = true;
                    graph[random_index-size][DOWN].deleted = true;
                    tree = union(a,b,tree.clone());
                    count_of_roots = count_of_roots - 1;
                }
                //BELOW: IF in same set, then set the used field for that index & direction combo.
                else{
                    graph[random_index][random_dir].used = true;
                }
            }
        }
    }
    //println!("final tree: {:?}", tree);
    return clear_used(graph.clone(),size.clone()); 
}

//THIS vvvv function was used to initially ensure the proper printing of maze boxes before creation.
//Function still works, but is no longer called so to avoid warning at compile time it has been commented out.

fn print_maze(size: usize, graph: Vec<Vec<Edge>>){
    for i in 0 .. size {
        print!("    =");
        for j in 0 .. size {
            if i != 0 {
                if (!graph[i * size + j][3].deleted) || (graph[i * size + j][3].dumdum){
                        print!("====");
                    }
                    else {
                        print!("   =");
                    }
            }
            else {
                print!("====");
            }
        }
        println!("");
        if i == 0 {
            print!("Start");
        }
        else {
            print!("    |");
        }
        for j in 0 .. size {
            if i == size-1 && j == size-1{
                println!("    End");
                
            }
            else if (!graph[i * size + j][0].deleted) ||  graph[i * size + j][0].dumdum {
                print!("   |");
            }
            else{
                print!("    ");
            }
        }
        if i == size - 1 {
            print!("    =");
            for j in 0 .. size {
                print!("====");
            }
        }
        println!("");    
    }
}

fn print_position(position: usize, size: usize, graph: Vec<Vec<Edge>>){
    for i in 0 .. size {
        print!("    =");
        for j in 0 .. size {
            if i != 0 {
                if (!graph[i * size + j][3].deleted) || (graph[i * size + j][3].dumdum){
                        print!("====");
                    }
                    else {
                        print!("   =");
                    }
            }
            else {
                print!("====");
            }
        }
        println!("");
        if i == 0 {
            print!("Start");
        }
        else {
            print!("    |");
        }
        for j in 0 .. size {
            if i == size-1 && j == size-1{
                println!("    End");
                
            }
            else if (!graph[i * size + j][0].deleted) ||  graph[i * size + j][0].dumdum {
                if position == i * size + j{
                    print!(" x |");
                }
                else {
                    print!("   |");
                }
            }
            else{
                if position == i * size + j{
                    print!(" x  ");
                }
                else {
                    print!("    ");
                }
            }
        }
        if i == size - 1 {
            print!("    =");
            for j in 0 .. size {
                print!("====");
            }
        }
        println!("");    
    }
}

//THIS vvvvv function was initial attempt to find a maze solution using recursion.  This does actually work, however
//only for trivially small mazes, has a bug somewhere, and thought it more efficient to go a different route with 
//solution as opposed to banging head against wall.

fn find_solution(x: usize, y: usize, size: usize, mut graph: Vec<Vec<Edge>>) -> bool{

    let mut maze_solution = HashMap::new();
    if x == size - 1 && y == size - 1{ return true; }

    for i in 0 .. 4 {
        if graph[y * size + x][i].deleted && !graph[y * size + x][i].used{
            if graph[y * size + x][i].used {
                return false;
            }

            graph[y * size + x][i].used = true;
            
            if i == 0 {
                graph[y * size + x + 1][LEFT].used = true;

                if find_solution(x+1,y,size,graph.clone()){
                    maze_solution.insert(y * size + x, "right");
                    for (index, direction) in &maze_solution {
                        println!("{}: \"{}\"", index, direction);
                    }
                    return true;
                }
            }
            if i == 1 {
                graph[(y+1) * size + x][UP].used = true;

                if find_solution(x,y+1,size,graph.clone()){
                    maze_solution.insert(y * size + x, "down");
                    for (index, direction) in &maze_solution {
                        println!("{}: \"{}\"", index, direction);
                    }
                    return true;
                }
            }
            if i == 2 {
                graph[y * size + x - 1][RIGHT].used = true;

                if find_solution(x-1,y,size,graph.clone()){
                    maze_solution.insert(y * size + x, "left");
                    for (index, direction) in &maze_solution {
                        println!("{}: \"{}\"", index, direction);
                    }
                    return true;
                }
            }
            if i == 3 {
                graph[(y-1) * size + x][DOWN].used = true;

                if find_solution(x,y-1,size,graph.clone()){
                    maze_solution.insert(y * size + x, "up");
                    for (index, direction) in &maze_solution {
                        println!("{}: \"{}\"", index, direction);
                    }
                    return true;
                }
            }
        }
    }

    return false;
}

//THIS vvv function uses a 'right hand on wall' technique to find solution.  Each node visited is added to Vec, and 
//after solution found, the dead ends, non-main path part of visits are pruned off the list.

fn find_solution2(x: usize, y: usize, size: usize, graph: Vec<Vec<Edge>>) -> Vec<usize>{
    
    let mut not_solved = true;
    let final_position = size * size - 1;
    let mut heading = RIGHT;
    let mut loc = y * size + x;
    let mut solver_hist = Vec::new();

    while not_solved {

        let mut try_dir = 0;

        for i in 1 .. 5 {
            //Determine preference of direction traveled based on heading of traveler.  This is what determines
            //the right hand on wall technique.
            if heading == RIGHT {
                match i {
                    1 =>{
                        try_dir = 1;
                        },
                    2 =>{
                        try_dir = 0;
                        },
                    3 =>{
                        try_dir = 3;
                        },
                    4 =>{
                        try_dir = 2;
                        },
                    _ => unimplemented!(),
                }
            }
            else if heading == DOWN {
                //println!("this is DOWN and I: {:?}", i);
                match i {
                    1 =>{
                        try_dir = 2;
                        },
                    2 =>{
                        try_dir = 1;
                        },
                    3 =>{
                        try_dir = 0;
                        },
                    4 =>{
                        try_dir = 3;
                        },
                    _ => unimplemented!(),
                }
            }
            else if heading == UP {
                match i {
                    1 =>{
                        try_dir = 0;
                        },
                    2 =>{
                        try_dir = 3;
                        },
                    3 =>{
                        try_dir = 2;
                        },
                    4 =>{
                        try_dir = 1;
                        },
                    _ => unimplemented!(),
                }
            }
            else if heading == LEFT {
                match i {
                    1 =>{
                        try_dir = 3;
                        },
                    2 =>{
                        try_dir = 2;
                        },
                    3 =>{
                        try_dir = 1;
                        },
                    4 =>{
                        try_dir = 0;
                        },
                    _ => unimplemented!(),
                }
            }
            //After determining preference, then see if that choice is available to move to.

            if graph[loc][try_dir].deleted {
                let mut temp:Vec<usize> = vec![loc;1];
                
                //if going backwards then do this 
                if i != 4 {
                    solver_hist.append(&mut temp);
                }
                
                match try_dir {
                    RIGHT => loc = loc + 1,
                    DOWN => loc = loc + size,
                    LEFT => loc = loc - 1,
                    UP => loc = loc - size,
                    _ => unimplemented!(),
                }
                heading = try_dir;

                if loc == final_position {
                    temp = vec![loc;1];
                    solver_hist.append(&mut temp);
                    not_solved = false; 
                }
                break; 
            }
        }
    }
    
    let range = 0 .. solver_hist.len();
    let mut sol = vec![];
    let mut new_range = 9999 .. 10000;
    for index in range {
        let examine = solver_hist[index].clone();
        let range2 = index + 1 .. solver_hist.len();
        if index >= new_range.start && index <= new_range.end {    
            continue; 
        }
        else {
            sol.push(examine);
        }
        for index2 in range2 {
            let examine2 = solver_hist[index2].clone();
            if examine == examine2 {
                new_range = index .. (index2);
            }
        }
    }
    return sol;
}

fn clear_used(mut graph: Vec<Vec<Edge>>, size: usize) -> Vec<Vec<Edge>> {
    let total = size * size;
    for x in 0 .. total {
        for i in 0 .. 3 {
            if !graph[x][i].dumdum {
                graph[x][i].used = false;
            }
        }
    }
    return graph.clone();
}


fn main() {    
    
    let mut input = String::new();
    println!("Please enter the size of maze requested: ");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from console");

    let trimmed = input.trim();
    let mut size:usize = 0;
    match trimmed.parse::<usize>() {
        Ok(i) => size = i,
        Err(..) => println!("This was not a valid maze size: {}", trimmed)
    };

    let total_boxes = Size{
        size:(size*size),
    };
    //sets up 2d vec of vecs for use as the tree.  vec[x][0] = parent of x, and vec[x][1] = rank of x.
    let mut tree:Vec<Vec<usize>> = vec![vec![999999;2];total_boxes.size];
    //makes sure the rank of each box in tree is zero before beginning
    for innervec in tree.iter_mut(){
        innervec[1] = 0;
    } 
    
    let mut graph2 = init_maze(size.clone());

    graph2 = mk_maze(size.clone(), graph2.clone(), tree.clone());

    print_position(0, size.clone(), graph2.clone());
    
    let mut not_solved = true;
    let mut position:usize = 0;
    let mut movement:String = "".to_string();
    let mut player_moves = LinkedList::new();
    player_moves.push_back(position);
    
    while not_solved {
        
        //command "clear" used for linux/unix/mac os.   If windows this has to be changed to "clr", cuz microsoft.
        std::process::Command::new("clear").status().unwrap();
        
        print_position(position, size.clone(), graph2.clone());
        
        let mut dir = String::new();
        
        println!("W,A,S,D Controls movement thru maze, enter a direction to go:");
        io::stdin()
            .read_line(&mut dir)
            .expect("failed to read from console");

        let trimmed = dir.trim();
        
        match trimmed.parse::<String>() {
            Ok(i) => movement = i,
            Err(..) => println!("You did not enter a valid direction: {}", trimmed)
        };
        
        if movement == "w" || movement == "W"{
            if graph2[position][3].deleted {
                position = position - size;
                player_moves.push_back(position);
            }
        } 
        else if movement == "a" || movement == "A"{
            if graph2[position][2].deleted {
                position = position - 1;
                player_moves.push_back(position);
            }
        } 
        else if movement == "s" || movement == "S"{
            if graph2[position][1].deleted {
                position = position + size;
                player_moves.push_back(position);
            }
        } 
        else if movement == "d" || movement == "D"{
            if graph2[position][0].deleted {
                position = position + 1 ;
                player_moves.push_back(position);
            }
        } 
        else{
            println!("You did not enter a valid direction: {}", movement); 
        }
        if position == size * size - 1 {

            not_solved = false;
            std::process::Command::new("clear").status().unwrap();
            print_position(position, size.clone(), graph2.clone());
            println!("");
            println!("\\ \\   / / __ \\| |  | | \\ \\        / / __ \\| \\ | | |");
            println!(" \\ \\_/ / |  | | |  | |  \\ \\  /\\  / / |  | |  \\| | |");
            println!("  \\   /| |  | | |  | |   \\ \\/  \\/ /| |  | | . ` | |");
            println!("   | | | |__| | |__| |    \\  /\\  / | |__| | |\\  |_|");
            println!("   |_|  \\____/ \\____/      \\/  \\/   \\____/|_| \\_(_)");
            println!("");
            println!("");
            
        } 

    }

    if not_solved == false {
        //calculate and print score!
    }
    
    graph2 = clear_used(graph2.clone(),size.clone());
    let solution = find_solution2(0, 0, size.clone(), graph2.clone());
    
    print!("HERE WAS THE IDEAL SOLUTION: ");
    println!("{:?}", solution);
    println!("this is player move history: {:?}", player_moves);

}