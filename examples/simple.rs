use randlib::{Random, RandomSeedSource};

fn main() {
    let src = RandomSeedSource::SystemTime;
    let mut rand = Random::new(src);

    for _ in 0..10 {
        println!("{}", rand.random());
    }

    println!("i128: {}", rand.rand_i128());
    println!("i64: {}", rand.rand_i64());
    println!("i32: {}", rand.rand_i32());
    println!("i16: {}", rand.rand_i16());
    println!("i8: {}", rand.rand_i8());

    println!("u64: {}", rand.rand_u64());
    println!("u32: {}", rand.rand_u32());
    println!("u16: {}", rand.rand_u16());
    println!("u8: {}", rand.rand_u8());

    println!("bool: {}", rand.rand_bool());

    println!("f32: {}", rand.rand_f32());
    println!("f64: {}", rand.rand_f64());
}
