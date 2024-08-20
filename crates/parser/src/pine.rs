use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, multispace0},
    combinator::{map, opt},
    sequence::{delimited, preceded, tuple},
    IResult,
};
use crate::{Ast, Parser};

fn parse_function_declaration(input: &str) -> IResult<&str, String> {
    let (input, name) = alpha1(input)?; // 解析函数名
    let (input, _) = tag("(")(input)?;  // 解析左括号
    let (input, params) = take_until(")")(input)?; // 解析参数
    let (input, _) = tag(")")(input)?;  // 解析右括号
    let (input, _) = multispace0(input)?; // 可选空格
    let (input, _) = tag("=>")(input)?;  // 解析 `=>`

    // 构造替换后的字符串格式: "name = |params|"
    let result = format!("{} = |{}|", name, params);
    Ok((input, result))
}

fn process_line(input: &str) -> String {
    match parse_function_declaration(input) {
        Ok((remaining, replaced)) => format!("{}{}", replaced, remaining), // 如果匹配，替换并保留剩余部分
        Err(_) => input.to_string(), // 如果不匹配，保持原样
    }
}

pub(crate) fn replace_function_format_in_code(input: &str) -> String {
    input.lines().map(process_line).collect::<Vec<_>>().join("\n")
}
