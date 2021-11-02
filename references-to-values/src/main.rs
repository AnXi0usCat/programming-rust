use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn show_ref(table: &Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn main() {
    let mut table = Table::new();
    table.insert("Beliermo".to_string(), vec!["foo".to_string(),
                                                    "bar".to_string(),
                                                    "baz".to_string()]);
    table.insert("Beliermo2".to_string(), vec!["foo2".to_string(),
                                                     "bar2".to_string(),
                                                     "baz2".to_string()]);
    table.insert("Beliermo3".to_string(), vec!["foo3".to_string(),
                                                     "bar3".to_string(),
                                                     "baz3".to_string()]);

    // fails like a boss

    //show(table);
    //    |          ----- value moved here
    // 26 |     assert_eq!(table["Beliermo"][0], "foo")
    //    |                ^^^^^ value borrowed here after move
    //assert_eq!(table["Beliermo"][0], "foo")

    show_ref(&table);
    assert_eq!(table["Beliermo"][0], "foo");
}
