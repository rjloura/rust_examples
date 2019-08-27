// This example shows how to match an enum field of a struct based soley on
// the variant and not the variant's value.

#[derive(PartialEq, Debug)]
enum MatchMe {
    OneMatch,
    TwoMatch,
    StrMatch(String),
}

#[derive(Debug)]
struct MatchStruct {
    state: MatchMe,
}

fn match_me(m: MatchMe, ms: &MatchStruct) -> bool {
    if std::mem::discriminant(&m) == std::mem::discriminant(&ms.state) {
        println!("Found match for: {:?}", m);
        return true;
    }

    println!("No match found for: {:?}", m);
    return false;
}

fn main() {
    let ms = MatchStruct {
        state: MatchMe::OneMatch,
    };

    let msstr = MatchStruct {
        state: MatchMe::StrMatch(String::from("foo")),
    };

    // No Match
    assert!(!match_me(MatchMe::TwoMatch, &ms));

    // Match
    assert!(match_me(MatchMe::OneMatch, &ms));

    // No Match
    assert!(!match_me(MatchMe::OneMatch, &msstr));

    // Even though you are only matching the variant and not the value we
    // still need to provide a dummy value here. AFAIK there is no way to
    // refer to a variant that takes a value without specifying some value.
    //
    // Match
    assert!(match_me(MatchMe::StrMatch(String::from("")), &msstr));
}
