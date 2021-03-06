use crate::code::Code;
use crate::code::Code::*;
use regex::Regex;

pub fn parse(data: &str) -> Result<Vec<Code>, String> {
    let mut code = vec![];
    for line in data.lines() {
        code.push(parse_line(line)?);
    }
    Ok(code)
}

fn parse_line(line: &str) -> Result<Code, String> {
    if Regex::new(r"flip type \d+").unwrap().is_match(line) {
        Ok(FlipType(line[10..].parse().unwrap()))
    } else if Regex::new(r"jump \d+").unwrap().is_match(line) {
        Ok(Jump(line[5..].parse().unwrap()))
    } else if Regex::new(r"jump addr \d+").unwrap().is_match(line) {
        Ok(JumpAddr(line[10..].parse().unwrap()))
    } else if Regex::new(r"swap \d+ \d+").unwrap().is_match(line) {
        let mut split = line[5..].split(' ');
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        Ok(Swap(first.parse().unwrap(), second.parse().unwrap()))
    } else if Regex::new(r"copy \d+ \d+").unwrap().is_match(line) {
        let mut split = line[5..].split(' ');
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        Ok(Copy(first.parse().unwrap(), second.parse().unwrap()))
    } else if Regex::new(r"add \d+ \d+").unwrap().is_match(line) {
        let mut split = line[4..].split(' ');
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        Ok(Add(first.parse().unwrap(), second.parse().unwrap()))
    } else if Regex::new(r"sub \d+ \d+").unwrap().is_match(line) {
        let mut split = line[4..].split(' ');
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        Ok(Sub(first.parse().unwrap(), second.parse().unwrap()))
    } else if Regex::new(r"mul \d+ \d+").unwrap().is_match(line) {
        let mut split = line[4..].split(' ');
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        Ok(Mul(first.parse().unwrap(), second.parse().unwrap()))
    } else if Regex::new(r"div \d+ \d+").unwrap().is_match(line) {
        let mut split = line[4..].split(' ');
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        Ok(Div(first.parse().unwrap(), second.parse().unwrap()))
    } else if Regex::new(r"print \d+").unwrap().is_match(line) {
        Ok(Print(line[6..].parse().unwrap()))
    } else if Regex::new(r"exit \d+").unwrap().is_match(line) {
        Ok(Exit(line[5..].parse().unwrap()))
    } else if line.eq("exit") {
        Ok(Exit(0))
    } else if Regex::new(r"jump addr \d+").unwrap().is_match(line) {
        Ok(JumpAddr(line[10..].parse().unwrap()))
    } else if Regex::new(r"jump addr cmp \d+ \d+ \d+ \d+ \d+").unwrap().is_match(line) {
        let mut split = line[14..].split(' ');
        let left = split.next().unwrap().parse().unwrap();
        let right = split.next().unwrap().parse().unwrap();
        let lt = split.next().unwrap().parse().unwrap();
        let eq = split.next().unwrap().parse().unwrap();
        let gt = split.next().unwrap().parse().unwrap();
        Ok(JumpAddrCmp(left, right, lt, eq, gt))
    } else if Regex::new(r"del \d+").unwrap().is_match(line) {
        Ok(Del(line[4..].parse().unwrap()))
    } else if Regex::new(r"_\d+").unwrap().is_match(line) {
        Ok(IntData(line[1..].parse().unwrap()))
    } else if line.len() == 1 {
        Ok(CharData(line.chars().next().unwrap()))
    } else if line.is_empty() {
        Ok(Nop)
    } else {
        Ok(Nop)
    }
}

#[cfg(test)]
mod tests {
    use crate::code::Code::{FlipType, IntData, CharData};
    use crate::parse::parse_line;

    #[test]
    fn flip_type() {
        assert_eq!(FlipType(123), parse_line("flip type 123").unwrap());
    }

    #[test]
    fn int_data() {
        assert_eq!(IntData(321), parse_line("_321").unwrap());
    }

    #[test]
    fn char_data() {
        assert_eq!(CharData('b'), parse_line("b").unwrap());
    }
}
