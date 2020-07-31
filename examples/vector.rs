fn a(data: &[u8]) {
    println!("a {:?}", data);
}

fn b(data: &[u8]) {
    println!("b {:?}", data);
}

type Msg = dyn Fn(&[u8]) -> ();

fn main() {
    println!("Test of vector");
    let mut vector: Vec<&Msg> = vec![];
    vector.push(&a);
    vector.push(&b);
    vector.push(&a);
    vector.push(&|data| {
        println!("closure {:?}", data);
    });

    for f in vector {
        f(&[1, 2, 4]);
    }
}
