use rand::Rng;

#[derive(Debug)]
pub struct AsciiGenerator<'a, R:'a> {
    rng: &'a mut R,
}

impl<'a, R: Rng> Iterator for AsciiGenerator<'a, R> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        const GEN_ASCII_STR_CHARSET: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
              abcdefghijklmnopqrstuvwxyz\
              0123456789";
        Some(*choose(self.rng, GEN_ASCII_STR_CHARSET).unwrap() as char)
    }
}

fn choose<'a, T, R: Rng>(rng: &mut R, values: &'a [T]) -> Option<&'a T> {
    if values.is_empty() {
        None
    } else {
        Some(&values[rng.gen_range(0..values.len())])
    }
}

pub fn gen_ascii_chars<'a, R>(rng: &'a mut R) -> AsciiGenerator<'a, R> {
    AsciiGenerator { rng }
}

pub fn random_n_ascii_chars(n: u32) -> String {
    let mut rng = rand::thread_rng();
    gen_ascii_chars(&mut rng).take(n as usize).collect::<String>()
}
