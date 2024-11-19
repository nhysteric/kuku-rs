use kuku_rs::kuku::KuKuTable;
use rand_core::{RngCore, SeedableRng};
use rand_pcg::Pcg64Mcg;

#[test]
pub fn kuku_test() {
    let mut table = KuKuTable::new(10, 2, 2, 2, 2, [0; 16]);
    let items = generate_random_u8_array(15, 100);
    items.iter().for_each(|&x| {
        if let Err(msg) = table.insert(&x) {
            println!("{:?}", msg);
        }
        println!("fill_rate: {:?}", table.fill_rate());
    });
    println!("table:{:?}", table.get_table());
    println!("stash:{:?}", table.get_stash());
    items.iter().for_each(|&x| {
        println!("{:?}", x);
        match table.query(&x) {
            Some((is_table, index)) => {
                if is_table {
                    println!("find in table at index {}", index);
                } else {
                    println!("find in stash at index {}", index);
                }
            }
            None => println!("not find"),
        }
    });
}

pub fn generate_random_u8_array(n: usize, seed: u64) -> Vec<[u8; 16]> {
    let mut rng = Pcg64Mcg::seed_from_u64(seed);
    let mut result = Vec::with_capacity(n);

    for _ in 0..n {
        let mut arr = [0u8; 16];
        rng.fill_bytes(&mut arr);
        result.push(arr);
    }
    result
}
