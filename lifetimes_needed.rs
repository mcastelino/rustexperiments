// This example illustrates why lifetimes are needed
// when the compiler cannot (or will not) elide them

fn foo<'a>(x: &'a u32, _y: &'a u32) -> &'a u32 {
    x
}

fn foox<'a, 'b>(x: &'a u32, _y: &'b u32) -> &'a u32 {
    x
}

fn fooy<'a, 'b>(_x: &'a u32, y: &'b u32) -> &'b u32 {
    y
}

fn main() {
    let x = 12;
    let y = 13;

    foo(&x, &y);

    let _z: &u32 = {
        let y = 42;
        foox(&x, &y)
        //foo(&x, &y) //will fail
        //fooy(&x, &y) //will fail
    };

    let _z: &u32 = {
        let x = 42;
        fooy(&x, &y)
    };
}
