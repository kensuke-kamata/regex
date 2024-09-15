mod engine;
mod helper;

use engine::print;
use helper::DynError;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), DynError> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() <= 2 {
        eprintln!("usage: {} regex file", args[0]);
        return Err("invalid arguments".into());
    } else {
        match_file(&args[1], &args[2])?;
    }

    Ok(())
}

fn match_file(expr: &str, file: &str) -> Result<(), DynError> {
    let f = File::open(file)?;
    let reader = BufReader::new(f);

    print(expr)?;
    println!();

    for line in reader.lines() {
        let line = line?;
        for (i, _) in line.char_indices() {
            if engine::is_match(expr, &line[i..], true)? {
                println!("{line}");
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        engine::is_match,
        helper::{safe_add, SafeAdd},
    };

    #[test]
    fn test_safe_add() {
        let n: usize = 10;
        assert_eq!(Some(30), n.safe_add(&20));

        let n: usize = !0; // 2^64 - 1 (64 bits CPU)
        assert_eq!(None, n.safe_add(&1));

        let mut n: usize = 10;
        assert!(safe_add(&mut n, &20, || ()).is_ok());

        let mut n: usize = !0;
        assert!(safe_add(&mut n, &1, || ()).is_err());
    }

    #[test]
    fn test_matching() {
        // パースエラー
        assert!(is_match("+b", "bbb", true).is_err());
        assert!(is_match("*b", "bbb", true).is_err());
        assert!(is_match("|b", "bbb", true).is_err());
        assert!(is_match("?b", "bbb", true).is_err());

        // パース成功、マッチ成功
        assert!(is_match("abc|def", "def", true).unwrap());
        assert!(is_match("(abc)*", "abcabc", true).unwrap());
        assert!(is_match("(ab|cd)+", "abcdcd", true).unwrap());
        assert!(is_match("abc?", "ab", true).unwrap());
        assert!(is_match("((((a*)*)*)*)", "aaaaaaaaa", true).unwrap());
        assert!(is_match("(a*)*b", "aaaaaaaaab", true).unwrap());
        assert!(is_match("(a*)*b", "b", true).unwrap());
        assert!(is_match("a**b", "aaaaaaaaab", true).unwrap());
        assert!(is_match("a**b", "b", true).unwrap());

        // パース成功、マッチ失敗
        assert!(!is_match("abc|def", "efa", true).unwrap());
        assert!(!is_match("(ab|cd)+", "", true).unwrap());
        assert!(!is_match("abc?", "acb", true).unwrap());
    }
}
