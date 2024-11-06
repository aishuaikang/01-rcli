use rand::seq::SliceRandom;

pub fn process_gen_pass(
    length: u8,
    lowercase: bool,
    uppercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    println!("{lowercase} - {uppercase}");
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if lowercase {
        chars.extend_from_slice(b"abcdefghijkmnopqrstuvwxyz");
        let c = chars
            .choose(&mut rng)
            .expect("在这个上下文中，字符不会为空。");
        password.push(*c);
    }

    if uppercase {
        chars.extend_from_slice(b"ABCDEFGHJKLMNPQRSTUVWXYZ");
        let c = chars
            .choose(&mut rng)
            .expect("在这个上下文中，字符不会为空。");
        password.push(*c);
    }

    if number {
        chars.extend_from_slice(b"123456789");
        let c = chars
            .choose(&mut rng)
            .expect("在这个上下文中，字符不会为空。");
        password.push(*c);
    }

    if symbol {
        chars.extend_from_slice(b"!@#$%^&*()-_=+");
        let c = chars
            .choose(&mut rng)
            .expect("在这个上下文中，字符不会为空。");
        password.push(*c);
    }

    for _ in 0..(length - password.len() as u8) {
        // let idx = rng.gen_range(0..chars.len());
        let c = chars
            .choose(&mut rng)
            .expect("在这个上下文中，字符不会为空。");
        password.push(*c)
    }

    password.shuffle(&mut rng);

    println!("{}", String::from_utf8(password).unwrap());

    Ok(())
}
