use std::{env, process};

use decompress_cap::{cc128_decompress_mem, HWPerms, UPerms};

fn parse_input_cap() -> (u64, u64) {
    // let input = String::from("0xffff0000000140060000000000000000");
    let input = {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            eprintln!("Usage: decompress_cap <YOUR_CAPABILITY>");
            process::exit(1);
        }
        args[1].clone()
    };

    let mut cap_val: String = match input.strip_prefix("0x") {
        Some(cap_val) => cap_val.to_owned(),
        None => input,
    };
    if cap_val.len() > 32 {
        panic!("Capability must not larger than 128 bits.")
    }
    if cap_val.len() < 17 {
        cap_val = "0".repeat(32 - cap_val.len()) + &cap_val
    };

    {
        let (l, r) = cap_val.split_at(cap_val.len() - 16);
        (
            u64::from_str_radix(l, 16).unwrap(),
            u64::from_str_radix(r, 16).unwrap(),
        )
    }
}

fn main() {
    let (pesbt, cursor) = parse_input_cap();
    println!(
        "Decompressing pesbt = {:016x?}, cursor = {:016x?}",
        pesbt, cursor
    );

    let result = &mut cc128_decompress_mem(pesbt, cursor, false);

    let perm = decompress_cap::cc128_get_perms(result) as u32;
    let perm_names: Vec<_> = HWPerms::from_bits_truncate(perm)
        .iter_names()
        .map(|x| x.0)
        .collect();
    println!("Permissions: 0x{:x?} ({})", perm, perm_names.join("|"));

    let uperm = decompress_cap::cc128_get_uperms(result) as u32;
    let uperm_names: Vec<_> = UPerms::from_bits_truncate(uperm)
        .iter_names()
        .map(|x| x.0)
        .collect();
    println!("User Perms: 0x{:x?} ({})", uperm, uperm_names.join("|"));
    println!("Base:   0x{:016x}", result.cr_base);
    println!("Offset: 0x{:016x}", result._cr_cursor - result.cr_base);
    println!("Cursor: 0x{:016x}", result._cr_cursor);
    println!("Length: 0x{:016x}", result._cr_top - result.cr_base as u128);
    println!("Top:    0x{:016x}", result._cr_top);
    println!("Sealed: {}", decompress_cap::cc128_is_cap_sealed(result));
    println!(
        "Valid decompress: {}",
        if result.cr_bounds_valid == 1 {
            "yes"
        } else {
            "no"
        }
    );
}
