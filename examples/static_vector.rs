fn a(data: &[u8]) {
    println!("a {:?}", data);
}

fn b(data: &[u8]) {
    println!("b {:?}", data);
}

type Msg = dyn Fn(&[u8]) -> ();

static mut V: Vec<&Msg> = Vec::new();

fn v_push(v: &'static Msg) {
    unsafe {
        V.push(v);
    }
}

fn v_run() {
    let v = unsafe { V.clone() };
    for f in v {
        f(&[1, 2, 4]);
    }
}

fn main() {
    println!("Test of static vector");

    v_push(&a);
    v_push(&b);
    v_push(&a);
    v_push(&|data| {
        println!("closure {:?}", data);
    });
    v_run();
}
