use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

static LOWERCASE: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
static UPPERCASE: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
static NUMBER: &[u8] = b"123456789";
static SYMBOL: &[u8] = b"!@#$%^&*()-_=+";

pub fn process_gen_pass(
    length: u8,
    lowercase: bool,
    uppercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if lowercase {
        chars.extend_from_slice(LOWERCASE);
        let c = LOWERCASE
            .choose(&mut rng)
            .expect("在这个上下文中，字符不会为空。");
        password.push(*c);
    }

    if uppercase {
        chars.extend_from_slice(UPPERCASE);
        let c = UPPERCASE
            .choose(&mut rng)
            .expect("在这个上下文中，字符不会为空。");
        password.push(*c);
    }

    if number {
        chars.extend_from_slice(NUMBER);
        let c = NUMBER
            .choose(&mut rng)
            .expect("在这个上下文中，字符不会为空。");
        password.push(*c);
    }

    if symbol {
        chars.extend_from_slice(SYMBOL);
        let c = SYMBOL
            .choose(&mut rng)
            .expect("在这个上下文中，字符不会为空。");
        password.push(*c);
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("在这个上下文中，字符不会为空。");
        password.push(*c)
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    println!("{}", password);

    let password_strength = zxcvbn(&password, &[]);
    eprintln!("{:?}", Into::<u8>::into(password_strength.score()));

    Ok(())
}
