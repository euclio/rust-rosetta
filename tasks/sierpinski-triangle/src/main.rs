fn main() {
    const ORDER: i32 = 4;
    const HEIGHT: i32 = 1 << ORDER;

    for y in (0..HEIGHT).rev() {
        for _ in 0..y {
            print!(" ");
        }

        for x in 0..(HEIGHT - y) {
            let fill = if x & y != 0 {
                ' '
            } else {
                '*'
            };

            print!("{} ", fill);
        }

        println!();
    }
}
