use nom::{
    branch::alt, bytes::complete::take, character::complete::char, combinator::map, multi::many1,
    sequence::tuple, IResult,
};

fn parse_abc(input: &str) -> IResult<&str, char> {
    alt((char('a'), char('b'), char('c')))(input)
}

fn parse_abc_as_numbers(input: &str) -> IResult<&str, u8> {
    fn transform_letter(ch: char) -> u8 {
        match ch {
            'a' => 5,
            'b' => 16,
            'c' => 8,
            _ => 0,
        }
    }
    alt((
        map(char('a'), transform_letter),
        map(char('b'), transform_letter),
        map(char('c'), transform_letter),
    ))(input)
}

fn parse_abc_to_ac(input: &str) -> IResult<&str, (char, char)> {
    tuple((char('a'), char('b'), char('c')))(input)
        .map(|(rest, result)| (rest, (result.0, result.2)))
}

fn parse_variable_text(input: &str) -> IResult<&str, (char, &str)> {
    tuple((char('n'), take(2usize)))(input)
}

fn repeated_text(input: &str) -> IResult<&str, Vec<&str>> {
    many1(take(3usize))(input)
}

fn main() {
    println!("a: {:?}", parse_abc("a"));
    println!("x: {:?}", parse_abc("x"));
    println!("bjk: {:?}", parse_abc("bjk"));

    println!("a: {:?}", parse_abc_as_numbers("a"));
    println!("x: {:?}", parse_abc_as_numbers("x"));
    println!("bjk: {:?}", parse_abc_as_numbers("bjk"));

    println!("abc: {:?}", parse_abc_to_ac("abc"));
    println!("bjk: {:?}", parse_abc_to_ac("bjk"));

    println!("nhgj: {:?}", parse_variable_text("nhgj"));
    println!("ahgj: {:?}", parse_variable_text("ahgj"));
    println!("ng: {:?}", parse_variable_text("ng"));

    println!(": {:?}", repeated_text(""));
    println!("abc: {:?}", repeated_text("abc"));
    println!("abcabcabc: {:?}", repeated_text("abcabcabc"));
}
