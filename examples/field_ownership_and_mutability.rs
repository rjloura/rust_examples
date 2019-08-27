#[derive(Debug)]
struct MultiFields {
    fieldone: String,
    fieldtwo: String,
}

fn move_me(s: String) {
    println!("{}", s);
}

// You CAN move the ownership of a single field without moving other fields
fn move_different_fields() {
    let mf = MultiFields {
        fieldone: String::from("hello"),
        fieldtwo: String::from("world"),
    };

    move_me(mf.fieldone);
    println!("That before move: {}", mf.fieldtwo);

    // Will not compile: Borrowed after move
    // println!("MutliFields after 'fieldone' was moved: {:?}", mf);

    // This works because only the 'fieldone' field was moved.
    move_me(mf.fieldtwo);

    // Will not compile: Borrowed after move
    // println!("{} {}", mf.fieldone, mf.fieldtwo);
}

fn mut_one(s: &mut String) {
    s.push('1');
    println!("Appending '1': {}", s);
}

fn mut_two(s: &mut String) {
    s.push('2');
    println!("Appending '2': {}", s);
}

fn mut_different_fields() {
    // The structure itself must be mutable in order to take a mutable
    // reference of its fields.  i.e. this won't work:
    //
    // let mf = MultiFields { ... }
    //
    let mut mf = MultiFields {
        fieldone: String::from("hello"),
        fieldtwo: String::from("world"),
    };

    mut_one(&mut mf.fieldone);
    mut_two(&mut mf.fieldtwo);

    // Borrow ok:
    println!(
        "mut different fields final result: {} {}",
        mf.fieldone, mf.fieldtwo
    );
}

fn main() {
    mut_different_fields();
    move_different_fields();
}
