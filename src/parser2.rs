use nom::alpha1 as nom_alpha1;
use nom::digit1 as nom_digit1;
use nom::AtEof as eof;
use nom::types::CompleteStr;
use lexer::{Lexer, Token};
use parser::{ParseStatus, Node};
use lexer::Token::{HashTag, Alpha, Period, LeftBracket, RightBracket, Newline,
                   Digit, Space, Equal, Plus, EndOfStream, Dash, Asterisk, Slash};
use mech_core::{Hasher, Function};
use alloc::fmt;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone)]
pub struct Parser {
  pub tokens: Vec<Token>,
  pub parse_tree: Node,
  pub unparsed: String,
  pub text: String,
}

impl Parser {

  pub fn new() -> Parser {
    Parser {
      text: String::from(""),
      tokens: Vec::new(),
      unparsed: String::from(""),
      parse_tree: Node::Root{ children: Vec::new()  },
    }
  }

  pub fn add_tokens(&mut self, tokens: &mut Vec<Token>) {
    self.tokens.append(tokens);
  }

  pub fn parse(&mut self, text: &str) {
    let parse_tree = program(CompleteStr(text));
    match parse_tree {
      Ok((rest, tree)) => {
        self.unparsed = rest.to_string();
        self.parse_tree = tree;
      },
      _ => (), 
    }
  }
   
}

impl fmt::Debug for Parser {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    
    write!(f, "┌───────────────────────────────────────┐\n").unwrap();
    write!(f, "│ Parser\n").unwrap();
    write!(f, "│ Length: {:?}\n", self.tokens.len()).unwrap();
    write!(f, "├───────────────────────────────────────┤\n").unwrap();
    for (ix, token) in self.tokens.iter().enumerate() {
      let c1 = " "; //if self.position == ix + 1 { ">" } else { " " };
      let c2 = " "; //if self.last_match == ix + 1 { ">" } else { " " };
      write!(f, "│ {:}{:} {:?}\n", c1, c2, token).unwrap();
    }
    write!(f, "├───────────────────────────────────────┤\n").unwrap();
    write!(f, "{:?}", self.parse_tree);
    write!(f, "└───────────────────────────────────────┘\n").unwrap();
    Ok(())
  }
}

macro_rules! leaf {
  ($name:ident, $byte:expr, $token:expr) => (
    named!($name<CompleteStr, Node>,
      do_parse!(
        byte: tag!($byte) >> 
        (Node::Token{token: $token, byte: (byte.as_bytes())[0]})
      )
    );
  )
}

leaf!{hashtag, "#", Token::HashTag}
leaf!{period, ".", Token::Period}
leaf!{colon, ":", Token::Colon}
leaf!{comma, ",", Token::Comma}
leaf!{left_bracket, "[", Token::LeftBracket}
leaf!{right_bracket, "]", Token::RightBracket}
leaf!{left_parenthesis, "(", Token::LeftParenthesis}
leaf!{right_parenthesis, ")", Token::RightParenthesis}
leaf!{left_brace, "{", Token::LeftBrace}
leaf!{right_brace, "}", Token::RightBrace}
leaf!{equal, "=", Token::Equal}
leaf!{less_than, "<", Token::LessThan}
leaf!{greater_than, ">", Token::GreaterThan}
leaf!{exclamation, "!", Token::Exclamation}
leaf!{question, "?", Token::Question}
leaf!{plus, "+", Token::Plus}
leaf!{dash, "-", Token::Dash}
leaf!{asterisk, "*", Token::Asterisk}
leaf!{slash, "/", Token::Slash}
leaf!{caret, "^", Token::Caret}
leaf!{space, " ", Token::Space}
leaf!{tab, "\t", Token::Tab}
leaf!{tilde, "~", Token::Tilde}
leaf!{grave, "`", Token::Grave}
leaf!{bar, "|", Token::Bar}
leaf!{quote, "\"", Token::Quote}
leaf!{ampersand, "&", Token::Ampersand}
leaf!{semicolon, ";", Token::Semicolon}
leaf!{new_line_char, "\n", Token::Newline}
leaf!{carriage_return, "\r", Token::CarriageReturn}

// ## The Basics

named!(word<CompleteStr, Node>,
  do_parse!(
    bytes: nom_alpha1 >>
    (Node::Word{children: bytes.chars().map(|b| Node::Token{token: Token::Alpha, byte: b as u8}).collect()})));

named!(number<CompleteStr, Node>,
  do_parse!(
    bytes: nom_digit1 >>
    (Node::Number{children: bytes.chars().map(|b| Node::Token{token: Token::Digit, byte: b as u8}).collect()})));

named!(text<CompleteStr, Node>,
  do_parse!(
    word: many1!(alt!(word | space | number)) >>
    (Node::Text{children: word})));

named!(identifier<CompleteStr, Node>,
  do_parse!(
    identifier: map!(tuple!(count!(word,1), many0!(alt!(dash | slash | word | number))), |tuple| {
      let (mut word, mut rest) = tuple;
      word.append(&mut rest);
      word
    }) >>
    (Node::Identifier{children: identifier})));

named!(whitespace<CompleteStr, Node>,
  do_parse!(
    many0!(space) >> new_line_char >>
    (Node::Null)));

named!(constant<CompleteStr, Node>,
  do_parse!(
    constant: alt!(string | number) >>
    (Node::Constant{children: vec![constant]})));

// ## Blocks

// ### Data

named!(select_all<CompleteStr, Node>,
  do_parse!(
    colon >> 
    (Node::SelectAll{children: vec![]})));

named!(subscript<CompleteStr, Node>,
  do_parse!(
    subscript: alt!(select_all | constant) >> many0!(space) >> opt!(comma) >> many0!(space) >>
    (Node::SubscriptIndex{children: vec![subscript]})));

named!(subscript_index<CompleteStr, Node>,
  do_parse!(
    left_brace >> subscripts: many1!(subscript) >> right_brace >>
    (Node::SubscriptIndex{children: subscripts})));

named!(dot_index<CompleteStr, Node>,
  do_parse!(
    period >> column_name: identifier >> 
    (Node::DotIndex{children: vec![column_name]})));

named!(index<CompleteStr, Node>,
  do_parse!(
    index: alt!(dot_index | subscript_index) >>
    (Node::Index{children: vec![index]})));

named!(data<CompleteStr, Node>,
  do_parse!(
    data: map!(tuple!(alt!(table | identifier), many0!(index)), |tuple| {
      let (mut source, mut indices) = tuple;
      let mut data = vec![source];
      data.append(&mut indices);
      data
    }) >>
    (Node::Data { children: data })));

// ### Tables

named!(table<CompleteStr, Node>,
  do_parse!(
    hashtag >> table_identifier: identifier >>
    (Node::Table { children: vec![table_identifier] })));

named!(binding<CompleteStr, Node>,
  do_parse!(
    binding_id: identifier >> colon >> many0!(space) >> 
    bound: alt!(identifier | constant) >> many0!(space) >> opt!(comma) >> many0!(space) >>
    (Node::Binding { children: vec![binding_id, bound] })));

named!(inline_table<CompleteStr, Node>,
  do_parse!(
    left_bracket >> bindings: many1!(binding) >> right_bracket >>
    (Node::InlineTable { children: bindings })));

// ### Statements

named!(variable_define<CompleteStr, Node>,
  do_parse!(
    variable: identifier >> space >> equal >> space >> expression: expression >>
    (Node::VariableDefine { children: vec![variable, expression] })));

named!(table_define<CompleteStr, Node>,
  do_parse!(
    table: table >> space >> equal >> space >> expression: expression >>
    (Node::TableDefine { children: vec![table, expression] })));

named!(watch_operator<CompleteStr, Node>,
  do_parse!(
    tilde >> 
    (Node::Null)));

named!(data_watch<CompleteStr, Node>,
  do_parse!(
    watch_operator >> space >> watch: alt!(variable_define | data) >>
    (Node::DataWatch { children: vec![watch] })));

named!(statement<CompleteStr, Node>,
  do_parse!(
    statement: alt!(table_define | variable_define | data_watch) >>
    (Node::Statement { children: vec![statement] })));

// ### Expressions

named!(string<CompleteStr, Node>,
  do_parse!(
    quote >> text: text >> quote >>
    (Node::String { children: vec![text] })));

named!(expression<CompleteStr, Node>,
  do_parse!(
    expression: alt!(constant | inline_table | data) >>
    (Node::Expression { children: vec![expression] })));

// ### Block Basics

named!(constraint<CompleteStr, Node>,
  do_parse!(
    space >> space >>
    statement_or_expression: statement >> opt!(new_line_char) >>
    (Node::Constraint { children: vec![statement_or_expression] })));

named!(block<CompleteStr, Node>,
  do_parse!(
    constraints: many1!(constraint) >>
    (Node::Block { children: constraints })));

// ## Markdown

named!(title<CompleteStr, Node>,
  do_parse!(
    hashtag >> space >> text: text >> many0!(whitespace) >>
    (Node::Title { children: vec![text] })));

named!(subtitle<CompleteStr, Node>,
  do_parse!(
    hashtag >> hashtag >> space >> text: text >> many0!(whitespace) >>
    (Node::Subtitle { children: vec![text] })));

named!(paragraph<CompleteStr, Node>,
  do_parse!(
    text: text >> many0!(whitespace) >>
    (Node::Paragraph { children: vec![text] })));

named!(section<CompleteStr, Node>,
  do_parse!(
    section: map!(tuple!(opt!(subtitle), many0!(alt!(block | paragraph))), |tuple| {
      let (mut section_title, mut section_body) = tuple;
      let mut section = vec![];
      match section_title {
        Some(subtitle) => section.push(subtitle),
        _ => (),
      };
      section.append(&mut section_body);
      section
    }) >>
    (Node::Section { children: section })));

named!(body<CompleteStr, Node>,
  do_parse!(
    many0!(whitespace) >>
    sections: many1!(section) >>
    (Node::Body { children: sections })));

// ## Start Here

named!(program<CompleteStr, Node>,
  do_parse!(
    title: title >> body: body >> opt!(whitespace) >>
    (Node::Root { children: vec![title, body] })));