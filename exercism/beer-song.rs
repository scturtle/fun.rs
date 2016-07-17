
pub fn verse(n : u16) -> String {
    let s = format!("{} bottle{2} of beer on the wall, {} bottle{2} of beer.\n",
                    if n == 0 {"No more".to_string()} else {n.to_string()},
                    if n == 0 {"no more".to_string()} else {n.to_string()},
                    if n == 1 {""} else {"s"});
    if n <= 0 {
        s + "Go to the store and buy some more, 99 bottles of beer on the wall.\n"
    } else {
        s + &format!("Take {} down and pass it around, {} bottle{} of beer on the wall.\n",
                     if n == 1 {"it"} else {"one"},
                     if n == 1 {"no more".to_string()} else {(n-1).to_string()},
                     if n == 2 {""} else {"s"})
    }
}

pub fn sing(fr : u16, to : u16) -> String {
    (to..fr+1).rev().map(|i|verse(i)).collect::<Vec<_>>().join("\n")
}
