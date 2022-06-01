use std::{
    fs::File,
    io::{BufWriter, Error, Write},
};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;

fn main() -> Result<(), Box<Error>> {
    let mut rng = ChaCha12Rng::seed_from_u64(20220601);
    let file = File::create("numbers.txt")?;
    let mut file = BufWriter::new(file);

    for _ in 0..20_000_000 {
        let num: i32 = rng.gen();
        writeln!(&mut file, "{}", num)?;
    }

    Ok(())
}
