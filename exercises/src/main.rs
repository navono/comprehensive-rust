fn main() {
    // first();
    scalar_types();
    compound_types();
    reference_types();
    slices();
}

fn first() {
    //  rust 默认是不可变的
    let mut x = 5;

    // 查找 api 文档： rustup doc --std::fmt
    println!("The value of x is: {}", x);

    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }

        println!("-> : {x}");
    }

    println!();
}

fn scalar_types() {
    println!("\nscalar types");
    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");

    println!("{:?}", b"abc");
    println!("{:?}", &[97, 98, 99]);
}

fn compound_types() {
    println!("\ncompound types");
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {:?}", a);
    println!("a: {:#?}", a);

    let t: (i8, bool) = (42, true);
    println!("t: {:?}", t);
    println!("t.0: {:?}", t.0);
    println!("t.1: {:?}", t.1);

    let meal = ("Falafel", 40);
    let (name, price) = meal;
    println!("{} costs {} price", name, price);
}

fn reference_types() {
    // println!("\nreference types");
    // let mut x = 5;
    // let y = &mut x;
    // *y += 1;
    // println!("x: {:?}", x);
    // println!("y: {:?}", y);

    println!("\nreference types");
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");
}

fn slices() {
    println!("\nslices");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice: {:?}", slice);

    let s1 = &a[0..a.len()];
    println!("s1 = &a[0..a.len()]: {:?}", s1);

    let s2 = &a[..a.len()];
    println!("s2 = &a[..a.len()] : {:?}", s2);
}