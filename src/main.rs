#[derive(Copy, Clone, Debug)]
struct Foo {
    bar: i32
}

enum FooResult <'a> {
    Ref(&'a Foo),
    Cp(Foo),
}

// will assign from lifetime B into A
fn get_max<'a, 'b>(a: &'a mut Foo, b: &'b Foo) -> &'a mut Foo {
    if b.bar > a.bar {
        // copy B into A
        *a = *b;
    }
    a
}

// will return new structs
fn get_max2(a: Foo, b: Foo) -> Foo {
    if a.bar > b.bar {
        a
    } else {
        b
    }
}

// will return lifetime A reference or a new lifetime A struct
fn get_max3<'a, 'b>(a: &'a Foo, b: &'b Foo) -> FooResult<'a> {
    if a.bar > b.bar {
        FooResult::Ref( a )
    } else {
        FooResult::Cp( b.clone() )
    }
}

fn main() {

    // lifetime A
    let mut a = Foo {
        bar: 102,
    };

    println!("A: {:?}", a);

    let max = {
        // lifetime B
        let b = Foo {
            bar: 151,
        };
        println!("B: {:?}", b);
        // Copy B into A
        get_max(&mut a, &b )
    };

    println!("Max: {:?}", max);
    println!("A changed to: {:?}", a);

    // lifetime A
    let c = Foo{
        bar: 42,
    };

    println!("C: {:?}", c);

    let max2 = {
        // lifetime B
        let d = Foo {
            bar: 31
        };

        println!("D: {:?}", d);
        // return new Foo in lifetime B to A
        get_max2( c, d )
    };

    println!("Max: {:?}", max2);

    let e = Foo {
        bar: 13,
    };

    println!("E: {:?}", e);

    let max3 = {

        let f = Foo {
            bar: 31,
        };
        println!("F: {:?}", f);
        // return new copy
        get_max3( &e, &f )
    };

    match max3 {
        FooResult::Ref( max ) => println!( "Max from reference: {:?}", max ),
        FooResult::Cp( max ) => println!( "Max from copy: {:?}", max ),
    };

    let h = Foo {
        bar: 72,
    };

    println!("H: {:?}", h);

    let max4 = {

        let i = Foo {
            bar: 52,
        };
        println!("I: {:?}", i);
        // return reference
        get_max3( &h, &i )
    };

    match max4 {
        FooResult::Ref( max ) => println!( "Max from reference: {:?}", max ),
        FooResult::Cp( max ) => println!( "Max from copy: {:?}", max ),
    };


}
