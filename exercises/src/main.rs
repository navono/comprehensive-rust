fn main() {
    println!("Hello, world 🌍!");

    first();
}

fn first() {
    //  rust 默认是不可变的
    let mut x = 5;

    // 查找 api 文档： rustup doc --std::fmt
    println!("The value of x is: {}", x);

    while x != 1 {
        if x %2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }

        println!("-> : {x}");
    }

    println!();
}

