use std::io;
extern crate rand;
use rand::Rng;

pub const RIGHT: usize = 0;
pub const DOWN: usize = 1;
pub const LEFT: usize = 2;
pub const UP: usize = 3;
 
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
    visited: bool,  
}

#[derive(Debug, Clone, Copy)]
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
    // println!("tree before: {:?}", tree);
    // println!("");
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
    // println!("tree after: {:?}", tree);
    // println!("");
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
    // let boarder:Point = Point{
    //     x:0,
    //     y:0,
    //     visited:true,
    // };
    // let dummy:Edge = Edge {
    //     p: boarder,
    //     d: 0,
    //     used: true,
    //     deleted: false,
    //     dumdum: true,
    // } ;
    while count_of_roots != 1 {
        let random_index = rng.gen::<usize>() % (size * size);
        let random_dir = rng.gen::<usize>() % 4;
        // println!("this is random_index: {:?}", random_index);
        // println!("this is random_dir: {:?}", random_dir);
        // println!("");
        // //println!("this is graph:  {:?}", graph);
        // println!("this is dumdum: {:?}", graph[random_index][random_dir].dumdum);
        // println!("this is used: {:?}", graph[random_index][random_dir].used);
        // println!("this is deleted: {:?}", graph[random_index][random_dir].deleted);
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
                    // println!("random_index: {:?}", random_index);
                    // println!("random_dir: {:?}", random_dir);
                    // println!("got right:");
                    // println!("a: {:?}", a);
                    // println!("b: {:?}", b);
                    graph[random_index][random_dir].deleted = true;
                    //graph[random_index+1][random_dir].deleted = true;
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
                    // println!("random_index: {:?}", random_index);
                    // println!("random_dir: {:?}", random_dir);
                    // println!("got down:");
                    // println!("a: {:?}", a);
                    // println!("b: {:?}", b);
                    graph[random_index][random_dir].deleted = true;
                    //graph[random_index+size][random_dir].deleted = true;
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
                    // println!("random_index: {:?}", random_index);
                    // println!("random_dir: {:?}", random_dir);
                    // println!("got left:");
                    // println!("a: {:?}", a);
                    // println!("b: {:?}", b);
                    graph[random_index][random_dir].deleted = true;
                    // if !graph[random_index-1][random_dir].dumdum {
                    //     graph[random_index-1][(random_dir + 2) % 4].deleted = true;
                    // }
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
                    // println!("random_index: {:?}", random_index);
                    // println!("random_dir: {:?}", random_dir);
                    // println!("got up:");
                    // println!("a: {:?}", a);
                    // println!("b: {:?}", b);
                    graph[random_index][random_dir].deleted = true;
                    // if !graph[random_index-size][random_dir].dumdum {
                    //     graph[random_index-size][(random_dir + 2) % 4].deleted = true;
                    // }
                    tree = union(a,b,tree.clone());
                    count_of_roots = count_of_roots - 1;
                }
                //BELOW: IF in same set, then set the used field for that index & direction combo.
                else{
                    graph[random_index][random_dir].used = true;
                }
            }
            // println!("count_of_roots: {:?}", count_of_roots);
        }
    }
    // println!("final tree: {:?}", tree);
    return graph.clone(); 
}


fn print_maze(size: usize, graph: Vec<Vec<Edge>>){
    for i in 0 .. size {
        print!("    =");
        for j in 0 .. size {
            if i != 0 {
                if (!graph[i * size + j][3].deleted && !graph[(i-1) * size + j][1].deleted) || (graph[i * size + j][3].dumdum){
                        print!("====");
                    }
                    else {
                        print!("   =");
                    }
            }
            else {
                if !graph[i * size + j][3].deleted {
                    print!("====");
                }
                else {
                        print!("   =");
                }
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
            else if (!graph[i * size + j][0].deleted && !graph[(i * size + j) + 1][2].deleted) ||  graph[i * size + j][0].dumdum {
                print!("   |");
            }
            else{
                print!("    ");
            }
        }
        if i == size - 1 {
            print!("    -");
            for j in 0 .. size {
                print!("====");
            }
        }
        println!("");    
    }
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
    // println!("this is tree: {:?}", tree);
    // println!("length of graph: {:?}", tree.len());
    // println!("tree[15]: {:?}", tree[15][0]);
    // println!("length of inner: {:?}", tree[1].len());
    let mut graph2 = init_maze(size.clone());
    // for x in graph2.iter(){
    //     println!("BEFORE inside iter(){:?}", x);
    //     println!("");
    //     println!("");
    // }


    //print_maze(size.clone(), graph2.clone());
    // println!("length of graph: {:?}", graph2.len());
    // println!("length of inner: {:?}", graph2[1].len());
    // println!("graph[2][1]: {:?}", graph2[2][1]);
    // println!("graph[15][3]: {:?}", graph2[15][3]);
    // println!("graph[16][0]: {:?}", graph2[15][0]);
    // println!("graph[16][1]: {:?}", graph2[15][1]);
    graph2 = mk_maze(size.clone(), graph2.clone(), tree.clone());
    //println!("final graph: {:?}", graph2);

    // for x in graph2.iter(){
    //     println!("inside iter(){:?}", x);
    //     println!("");
    // }

    print_maze(size.clone(), graph2);
    

}