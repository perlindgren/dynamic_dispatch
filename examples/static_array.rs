fn a(data: &[u8]) {
    println!("a {:?}", data);
}

fn b(data: &[u8]) {
    println!("b {:?}", data);
}

#[derive(Copy, Clone)]
struct Msg {
    msg: &'static dyn Fn(&[u8]) -> (),
}
unsafe impl Sync for Msg {}

static A: [Option<Msg>; 64] = {
    let mut arr = [None; 64];
    arr[0] = Some(Msg { msg: &a });
    arr[1] = Some(Msg { msg: &b });
    arr[2] = Some(Msg { msg: &a });

    arr[32] = Some(Msg {
        msg: &|data: &[u8]| {
            println!("closure {:?}", data);
        },
    });
    arr
};

fn main() {
    println!("Test of static array");

    for i in A.iter() {
        match i {
            Some(f) => (f.msg)(&[1, 2, 3]),
            _ => {}
        }
    }
}
