#[allow(dead_code)]
pub fn increase_count_window(input: &[u16]) -> u64 {
    let mut peekable = input.windows(3).peekable();

    let mut i = 0;

    while let Some(next) = peekable.next() {
        if let Some(peeked) = peekable.peek() {
            if peeked.iter().sum::<u16>() > next.iter().sum() {
                i += 1;
            }
        }
    }
    i
}
