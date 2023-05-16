#[derive(Debug, Clone, Copy)]
struct SomeStruct {
    num: i32,
}

fn print_some_struct(the_struct: &SomeStruct) {
    println!("{:?}", the_struct);
}

fn mutate_struct(the_struct: &mut SomeStruct) {
    the_struct.num = 5
}

fn biggest<'a>(a: &'a SomeStruct, b: &'a SomeStruct) -> &'a SomeStruct {
    if a.num > b.num {
        a
    } else {
        b
    }
}

fn main() {
    let mut some_struct: SomeStruct = SomeStruct { num: 3 };
    let bigger: &SomeStruct;
    {
        let other_struct: SomeStruct = SomeStruct { num: 5 };
        bigger = biggest(&some_struct, &other_struct);
    }
    print_some_struct(&bigger);
    // print_some_struct(&some_struct);
    // mutate_struct(&mut some_struct);
    // print_some_struct(&some_struct);
}
