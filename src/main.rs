use clap::{App, Arg};
use phf::phf_map;

use std::convert::TryInto;
use std::io::{Error, ErrorKind, Read};
use std::process;

mod baseperm;

static DECODE_MAP: phf::Map<&'static str, &'static (baseperm::DecodeContext + Sync)> = phf_map! {
    "base64" => &(baseperm::Base64 {}),
    "base64-urlsafe" => &(baseperm::Base64Urlsafe {}),
    "base32" => &(baseperm::Base32 {}),
};

fn permute(input: &str, ctx: &baseperm::DecodeContext) -> Result<Vec<String>, Error> {
    let decoded = match ctx.decode(input) {
        Some(result) => result,
        None => {
            return Err(Error::new(ErrorKind::Other, "Couldn't decode input"));
        }
    };

    let padding: u8 = ((((((decoded.len() * 8) / ctx.bitness()) + 1) * ctx.bitness())
        - (decoded.len() * 8))
        % ctx.bitness())
    .try_into()
    .unwrap();

    // NOTE(ww): This should probably be implemented on each DecodeContext;
    // all of our current encodings use = for alignment but future ones might not.
    let (body_str, alignment) = input.split_at(input.find('=').unwrap_or(input.len()));

    let mut body = body_str.to_owned().into_bytes();
    let mask = (1 << padding) - 1;

    let idx = ctx
        .alphabet()
        .iter()
        .position(|&x| x == *body.last().unwrap())
        .unwrap();

    // NOTE(ww): This is probably always a no-op, but I haven't taken
    // the time to prove it to myself yet.
    let base_idx = idx & !mask;

    let mut results = Vec::new();
    for i in 0..2_usize.pow(padding as u32) {
        // NOTE(ww): This also should never happen.
        if base_idx + i >= ctx.alphabet().len() {
            continue;
        }

        let last_byte = body.last_mut().unwrap();
        *last_byte = ctx.alphabet()[base_idx + i];

        let mut permuted = String::new();
        permuted.push_str(std::str::from_utf8(&body).unwrap());
        permuted.push_str(alignment);

        results.push(permuted);
    }

    Ok(results)
}

fn run() -> Result<(), Error> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("encoding")
                .help("sets the encoding")
                .short("e")
                .long("encoding")
                .multiple(false)
                .possible_values(&DECODE_MAP.keys().cloned().collect::<Vec<&str>>())
                .default_value("base64"),
        )
        .get_matches();

    let encoding = matches.value_of("encoding").unwrap();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    // NOTE(ww): This is a bit dumb: the base64 mod doesn't like trailing newlines.
    if input.ends_with('\n') {
        input.pop();
    }

    if input.is_empty() {
        return Ok(());
    }

    let results = permute(&input, DECODE_MAP.get(encoding).cloned().unwrap())?;

    for result in results.iter() {
        println!("{}", result);
    }

    Ok(())
}

fn main() {
    process::exit(match run() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("Fatal: {}", e);
            1
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute_base64() {
        // 8 bytes (4 pad), 16 (2 pad), 24 (0 pad)
        let strings = vec![("f", 4), ("fo", 2), ("foo", 0)];
        for (string, pad) in strings.iter() {
            let input = base64::encode(string);
            let results = permute(&input, &baseperm::Base64 {}).unwrap();

            assert_eq!(results.len(), 2_usize.pow(*pad));
        }
    }

    #[test]
    fn test_permute_base64_urlsafe() {
        // 8 bytes (4 pad), 16 (2 pad), 24 (0 pad)
        let strings = vec![("f", 4), ("fo", 2), ("foo", 0)];
        for (string, pad) in strings.iter() {
            let input = base64::encode(string);
            let results = permute(&input, &baseperm::Base64 {}).unwrap();

            assert_eq!(results.len(), 2_usize.pow(*pad));
        }
    }

    #[test]
    fn test_permute_base32() {
        // 8 bytes (2 pad), 16 (4 pad), 24 (1 pad), 32 (3 pad), 40 (0 pad)
        let strings = vec![("f", 2), ("fo", 4), ("foo", 1), ("fooo", 3), ("foooo", 0)];
        for (string, pad) in strings.iter() {
            let input = base32::encode(
                base32::Alphabet::RFC4648 { padding: true },
                string.as_bytes(),
            );
            let results = permute(&input, &baseperm::Base32 {}).unwrap();

            assert_eq!(results.len(), 2_usize.pow(*pad));
        }
    }
}
