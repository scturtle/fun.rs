
pub fn annotate(board: &[&str]) -> Vec<String> {
    let n = board.len() as i8;
    if n == 0 { return Vec::new() }
    let m = board[0].len() as i8;
    board.iter().enumerate().map(|(x, r)|{
        r.chars().enumerate().map(|(y, c)|{
            if c != ' ' { c } else {
                let mut count = 0;
                for i in -1..2 {
                    for j in -1..2 {
                        let newx = x as i8 + i;
                        let newy = y as i8 + j;
                        if newx<0 || newx>=n || newy<0 ||
                            newy>=m || (i==0 && j==0) { continue }
                        let neighboor = board[newx as usize].chars().nth(newy as usize);
                        if neighboor == Some('*') { count += 1 }
                    }
                }
                if count == 0 {' '} else {
                    (count + '0' as u8) as char
                }
            }
        }).collect()
    }).collect()
}
