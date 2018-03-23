
struct Foo<'a> {
    x: &'a i32,
}

//A struct with a reference
//but no lifetime will not work
//struct Boo {
//    x: &i32,
//}

fn main() {
    // Mutability
    println!("Simple program!");
    let v = vec![1, 2, 3];
    let mut v2 = v;
    v2[0] = 100;
    println!("V2= {:?} \n", v2);

    let a = [1, 2, 3];
    let mut m = a;
    m[0] = 100;
    println!("m= {:?} \n", m);

    let x = 5;
    let mut y = x; //copy into mutable
    println!("x={} y={} \n", x, y);
    y = y + 1;
    println!("x={} y={} \n", x, y);

    //let x = 100;
    //let y = mut x; //will not work

    //let x = 100;
    //let y = &mut x; //will not work

    // References and borrowing
    let mut x = 100;
    {
        let y = &mut x;
        *y = *y + 1;
        // will not work as &mut of x (i.e) is still in scope
        //println!("x={}", x);
    }
    println!("x={}\n", x);

    //Lifetimes
    let y = &5; // this is the same as `let _y = 5; let y = &_y;`
    let f = Foo { x: y };

    println!("{}", f.x);
}
