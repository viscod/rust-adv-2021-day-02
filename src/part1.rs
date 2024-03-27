#[allow(dead_code)]
pub fn increase_count(input: &[u16]) -> u64 {
    let mut peekable = input.iter().peekable();

    let mut i = 0;

    while let Some(next) = peekable.next() {
        if let Some(peeked) = peekable.peek() {
            if *peeked > next {
                i += 1;
            }
        }
    }
    i
}
