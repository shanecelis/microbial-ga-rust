use rand::Rng;
use rand::distributions::{Distribution, Standard};

const REC : f32 = 0.1;
const MUT : f32 = 0.1;
fn eval(gene : &[i32]) -> i32 {
    gene[0]
}

fn rnd() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn microbial_tournament(genes: &mut [&mut[i32]]) {
    // let A,B,W,L,i : usize;
    let P = genes.len();
    let D = P;
    let A : usize = (P as f32 * rnd()) as usize; // Choose A randomly
    let B = (A + 1 + (D as f32 * rnd()) as usize) % P; // B from Deme, %P..
    let (W, L) = if eval(&genes[A]) > eval(&genes[B]) { (A, B) } else { (B, A) };
    let N = genes[W].len();
    for i in 0..N {

        // ..for wrap-around
        // W=Winner L=Loser
        // walk down N genes
        // RECombn rate
        // Copy from Winner
        // MUTation rate
        // Flip a bit
        if rnd()<REC { genes[L][i]=genes[W][i]; }
        if rnd()<MUT { genes[L][i]^=1; }
    }
}

fn main() {
    let mut genes = [[0i32; 5]; 10];
    let mut rng = rand::thread_rng();
    for gene in &mut genes {
        rng.fill(gene);
    }
    let mut genes_slice : Vec<&mut [i32]> = genes.iter_mut().map(|g| g.as_mut_slice()).collect();
    microbial_tournament(&mut genes_slice);

    println!("Hello, world!");
}
