use rand::Rng;
use rand::distributions::Uniform;

const REC : f32 = 0.1;
const MUT : f32 = 0.3;

fn eval(gene : &[i32]) -> i32 {
    gene[0]
}

fn rnd() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

/// The microbial genetic algorithm by Harvey (2009) rewritten in rust.
///
/// Inman Harvey. 2009. The microbial genetic algorithm. In Proceedings of the
/// 10th European conference on Advances in artificial life: Darwin meets von
/// Neumann - Volume Part II (ECAL'09). Springer-Verlag, Berlin, Heidelberg,
/// 126–133.
fn microbial_tournament(genes: &mut [&mut [i32]]) {
    let p = genes.len();                               // Populazation size
    let d = p;                                         // Let deme size be same as pop
    let a : usize = (p as f32 * rnd()) as usize;       // Choose a randomly
    let b = (a + 1 + (d as f32 * rnd()) as usize) % p; // b from Deme, %p..
                                                       // ..for wrap-around
    let (w, l) =                                       // W=Winner L=Loser
        if eval(&genes[a]) > eval(&genes[b]) {
            (a, b)
        } else {
            (b, a)
        };
    let n = genes[w].len();
    for i in 0..n {                  // Walk down n genes

        if rnd()<REC {               // RECombn rate
            genes[l][i]=genes[w][i]; // Copy from Winner
        }
        if rnd()<MUT {               // MUTation rate
            genes[l][i]^=1;          // Flip a bit
        }
    }
}

fn main() {
    let mut genes = [[0i32; 5]; 10];
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(0, 1);
    for gene in &mut genes {
        *gene = (&mut rng).sample_iter(&dist).take(5).collect::<Vec<i32>>().try_into().unwrap();
    }
    let mut genes_slice : Vec<&mut [i32]> = genes.iter_mut().map(|g| g.as_mut_slice()).collect();

    for gen in 0..100 {
        println!("gen {}: {:?}", gen, genes_slice);
        microbial_tournament(&mut genes_slice);
    }

    println!("Hello, world!");
}
