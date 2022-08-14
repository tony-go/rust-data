mod float;

fn main () {
    let n: f32 = 42.42;
    let n_bits = n.to_bits();
    
    let res = float::decode(n_bits);
    assert_eq!(res, n);
    println!("Decoed: {}", res);
}
