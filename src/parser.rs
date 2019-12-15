// # Parser

// ## Prelude

use lexer::Token;
use mech_core::{Hasher, Function};
#[cfg(not(feature = "no-std"))] use core::fmt;
#[cfg(feature = "no-std")] use alloc::fmt;
#[cfg(feature = "no-std")] use alloc::string::String;
#[cfg(feature = "no-std")] use alloc::vec::Vec;
use nom::alpha1 as nom_alpha1;
use nom::digit1 as nom_digit1;
use nom::AtEof as eof;
use nom::types::CompleteStr;

// ## Parser Node

#[derive(Clone, PartialEq)]
pub enum Node {
  Root{ children: Vec<Node> },
  Block{ children: Vec<Node> },
  Constraint{ children: Vec<Node> },
  Select { children: Vec<Node> },
  DataWatch { children: Vec<Node> },
  Insert { children: Vec<Node> },
  VariableDefine { children: Vec<Node> },
  TableDefine { children: Vec<Node> },
  AddRow { children: Vec<Node> },
  Column { children: Vec<Node> },
  IdentifierOrConstant { children: Vec<Node> },
  Table { children: Vec<Node> },
  Number { children: Vec<Node> },
  DigitOrComma {children: Vec<Node> },
  FloatingPoint {children: Vec<Node> },
  MathExpression { children: Vec<Node> },
  SelectExpression { children: Vec<Node> },
  FilterExpression { children: Vec<Node> },
  Comparator { children: Vec<Node> },
  InfixOperation { children: Vec<Node>},
  Repeat{ children: Vec<Node> },
  TableIdentifier{ children: Vec<Node> },
  Identifier{ children: Vec<Node> },
  Alpha{ children: Vec<Node> },
  DotIndex{ children: Vec<Node> },
  SubscriptIndex{ children: Vec<Node> },
  SubscriptList{ children: Vec<Node> },
  Subscript{ children: Vec<Node> },
  LogicOperator{ children: Vec<Node> },
  LogicExpression{ children: Vec<Node> },
  Range{ children: Vec<Node> },
  SelectAll{ children: Vec<Node> },
  Index{ children: Vec<Node> },
  Data{ children: Vec<Node> },
  SetData{ children: Vec<Node> },
  SetOperator{ children: Vec<Node> },
  AddOperator{ children: Vec<Node> },
  WatchOperator {children: Vec<Node>},
  Equality{ children: Vec<Node> },
  Expression{ children: Vec<Node> },
  AnonymousTable{ children: Vec<Node> },
  TableRow{ children: Vec<Node> },
  Binding{ children: Vec<Node> },
  Attribute{ children: Vec<Node> },
  TableHeader{ children: Vec<Node> },
  InlineTable{ children: Vec<Node> },
  Constant{ children: Vec<Node> },
  Infix{ children: Vec<Node> },
  Program{ children: Vec<Node> },
  Title{ children: Vec<Node> },
  Subtitle{ children: Vec<Node> },
  SectionTitle{ children: Vec<Node> },
  Head{ children: Vec<Node> },
  Body{ children: Vec<Node> },
  Statement{ children: Vec<Node> },
  StatementOrExpression{ children: Vec<Node> },
  DataOrConstant{ children: Vec<Node> },
  IdentifierCharacter{ children: Vec<Node> },
  Fragment{ children: Vec<Node> },
  Node{ children: Vec<Node> },
  NewLineOrEnd{ children: Vec<Node> },
  Alphanumeric{ children: Vec<Node> },
  Paragraph{ children: Vec<Node> },
  ParagraphText{ children: Vec<Node> },
  FormattedText{ children: Vec<Node> },
  InlineMechCode{ children: Vec<Node> },
  InlineCode{ children: Vec<Node> },
  Bold{ children: Vec<Node> },
  Italic{ children: Vec<Node> },
  Hyperlink{ children: Vec<Node> },
  BlockQuote{ children: Vec<Node> },
  CodeBlock{ children: Vec<Node> },
  MechCodeBlock{ children: Vec<Node> },
  UnorderedList{ children: Vec<Node> },
  ListItem{ children: Vec<Node> },
  String{ children: Vec<Node> },
  Word{ children: Vec<Node> },
  Section{ children: Vec<Node> },
  ProseOrCode{ children: Vec<Node> },
  Whitespace{ children: Vec<Node> },
  SpaceOrTab{ children: Vec<Node> },
  NewLine{ children: Vec<Node> },
  Text{ children: Vec<Node> },
  Punctuation{ children: Vec<Node> },
  L1Infix{ children: Vec<Node> },
  L2Infix{ children: Vec<Node> },
  L3Infix{ children: Vec<Node> },
  L1{ children: Vec<Node> },
  L2{ children: Vec<Node> },
  L3{ children: Vec<Node> },
  L4{ children: Vec<Node> },
  Function{ children: Vec<Node> },
  Negation{ children: Vec<Node> },
  ParentheticalExpression{ children: Vec<Node> },
  CommentSigil{ children: Vec<Node> },
  Comment{children: Vec<Node>},
  Any{children: Vec<Node>},
  Symbol{children: Vec<Node>},
  StateMachine{children: Vec<Node>},
  Transitions{children: Vec<Node>},
  Transition{children: Vec<Node>},
  Quantity{children: Vec<Node>},
  Token{token: Token, byte: u8},
  LessThanEqual,
  GreaterThanEqual,
  Equal,
  NotEqual,
  LessThan,
  GreaterThan,
  And,
  Or,
  Empty,
  Null,
}

impl fmt::Debug for Node {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    print_recurse(self, 0);
    Ok(())
  }
}

pub fn print_recurse(node: &Node, level: usize) {
  spacer(level);
  let children: Option<&Vec<Node>> = match node {
    Node::Root{children} => {print!("Root\n"); Some(children)},
    Node::Block{children} => {print!("Block\n"); Some(children)},
    Node::Constraint{children} => {print!("Constraint\n"); Some(children)},
    Node::Select{children} => {print!("Select\n"); Some(children)},
    Node::DataWatch{children} => {print!("DataWatch\n"); Some(children)},
    Node::Insert{children} => {print!("Insert\n"); Some(children)},
    Node::MathExpression{children} => {print!("MathExpression\n"); Some(children)},
    Node::SelectExpression{children} => {print!("SelectExpression\n"); Some(children)},
    Node::Comparator{children} => {print!("Comparator\n"); Some(children)},
    Node::FilterExpression{children} => {print!("FilterExpression\n"); Some(children)},
    Node::AnonymousTable{children} => {print!("AnonymousTable\n"); Some(children)},
    Node::TableRow{children} => {print!("TableRow\n"); Some(children)},
    Node::Table{children} => {print!("Table\n"); Some(children)},
    Node::Number{children} => {print!("Number\n"); Some(children)},
    Node::DigitOrComma{children} => {print!("DigitOrComma\n"); Some(children)},
    Node::FloatingPoint{children} => {print!("FloatingPoint\n"); Some(children)},
    Node::Alphanumeric{children} => {print!("Alphanumeric\n"); Some(children)},
    Node::Word{children} => {print!("Word\n"); Some(children)},
    Node::Paragraph{children} => {print!("Paragraph\n"); Some(children)},
    Node::ParagraphText{children} => {print!("ParagraphText\n"); Some(children)},
    Node::FormattedText{children} => {print!("FormattedText\n"); Some(children)},
    Node::InlineMechCode{children} => {print!("InlineMechCode\n"); Some(children)},
    Node::InlineCode{children} => {print!("InlineCode\n"); Some(children)},
    Node::MechCodeBlock{children} => {print!("MechCodeBlock\n"); Some(children)},
    Node::Bold{children} => {print!("Bold\n"); Some(children)},
    Node::Italic{children} => {print!("Italic\n"); Some(children)},
    Node::Hyperlink{children} => {print!("Hyperlink\n"); Some(children)},
    Node::BlockQuote{children} => {print!("BlockQuote\n"); Some(children)},
    Node::CodeBlock{children} => {print!("CodeBlock\n"); Some(children)},
    Node::UnorderedList{children} => {print!("UnorderedList\n"); Some(children)},
    Node::ListItem{children} => {print!("ListItem\n"); Some(children)},
    Node::String{children} => {print!("String\n"); Some(children)},
    Node::VariableDefine{children} => {print!("VariableDefine\n"); Some(children)},
    Node::TableDefine{children} => {print!("TableDefine\n"); Some(children)},
    Node::AddRow{children} => {print!("AddRow\n"); Some(children)},
    Node::Column{children} => {print!("Column\n"); Some(children)},
    Node::Binding{children} => {print!("Binding\n"); Some(children)},
    Node::InlineTable{children} => {print!("InlineTable\n"); Some(children)},
    Node::TableHeader{children} => {print!("TableHeader\n"); Some(children)},
    Node::Attribute{children} => {print!("Attribute\n"); Some(children)},
    Node::IdentifierOrConstant{children} => {print!("IdentifierOrConstant\n"); Some(children)},
    Node::InfixOperation{children} => {print!("Infix\n"); Some(children)},
    Node::Repeat{children} => {print!("Repeat\n"); Some(children)},
    Node::Identifier{children} => {print!("Identifier\n"); Some(children)},
    Node::TableIdentifier{children} => {print!("TableIdentifier\n"); Some(children)},
    Node::DotIndex{children} => {print!("DotIndex\n"); Some(children)},
    Node::SubscriptIndex{children} => {print!("SubscriptIndex\n"); Some(children)},
    Node::SubscriptList{children} => {print!("SubscriptList\n"); Some(children)},
    Node::Subscript{children} => {print!("Subscript\n"); Some(children)},
    Node::LogicOperator{children} => {print!("LogicOperator\n"); Some(children)},
    Node::LogicExpression{children} => {print!("LogicExpression\n"); Some(children)},
    Node::Range{children} => {print!("Range\n"); Some(children)},
    Node::SelectAll{children} => {print!("SelectAll\n"); Some(children)},
    Node::Index{children} => {print!("Index\n"); Some(children)},
    Node::Equality{children} => {print!("Equality\n"); Some(children)},
    Node::Data{children} => {print!("Data\n"); Some(children)},
    Node::SetData{children} => {print!("SetData\n"); Some(children)},
    Node::SetOperator{children} => {print!("SetOperator\n"); Some(children)},
    Node::AddOperator{children} => {print!("AddOperator\n"); Some(children)},
    Node::WatchOperator{children} => {print!("WatchOperator\n"); Some(children)},
    Node::Infix{children} => {print!("Infix\n"); Some(children)},
    Node::Expression{children} => {print!("Expression\n"); Some(children)},
    Node::Constant{children} => {print!("Constant\n"); Some(children)},
    Node::Program{children} => {print!("Program\n"); Some(children)},
    Node::IdentifierCharacter{children} => {print!("IdentifierCharacter\n"); Some(children)},
    Node::Title{children} => {print!("Title\n"); Some(children)},
    Node::Subtitle{children} => {print!("Subtitle\n"); Some(children)},
    Node::SectionTitle{children} => {print!("SectionTitle\n"); Some(children)},
    Node::Section{children} => {print!("Section\n"); Some(children)},
    Node::Statement{children} => {print!("Statement\n"); Some(children)},
    Node::StatementOrExpression{children} => {print!("StatementOrExpression\n"); Some(children)},
    Node::DataOrConstant{children} => {print!("DataOrConstant\n"); Some(children)},
    Node::NewLineOrEnd{children} => {print!("NewLineOrEnd\n"); Some(children)},
    Node::Fragment{children} => {print!("Fragment\n"); Some(children)},
    Node::Body{children} => {print!("Body\n"); Some(children)},
    Node::Head{children} => {print!("Head\n"); Some(children)},
    Node::Node{children} => {print!("Node\n"); Some(children)},
    Node::Text{children} => {print!("Text\n"); Some(children)},
    Node::Punctuation{children} => {print!("Punctuation\n"); Some(children)},
    Node::L1Infix{children} => {print!("L1Infix\n"); Some(children)},
    Node::L2Infix{children} => {print!("L2Infix\n"); Some(children)},
    Node::L3Infix{children} => {print!("L3Infix\n"); Some(children)},
    Node::L1{children} => {print!("L1\n"); Some(children)},
    Node::L2{children} => {print!("L2\n"); Some(children)},
    Node::L3{children} => {print!("L3\n"); Some(children)},
    Node::L4{children} => {print!("L4\n"); Some(children)},
    Node::Function{children} => {print!("Function\n"); Some(children)},
    Node::Negation{children} => {print!("Negation\n"); Some(children)},
    Node::ParentheticalExpression{children} => {print!("ParentheticalExpression\n"); Some(children)},
    Node::ProseOrCode{children} => {print!("ProseOrCode\n"); Some(children)},
    Node::Whitespace{children} => {print!("Whitespace\n"); Some(children)},
    Node::SpaceOrTab{children} => {print!("SpaceOrTab\n"); Some(children)},
    Node::NewLine{children} => {print!("NewLine\n"); Some(children)},
    Node::Token{token, byte} => {print!("Token({:?} ({:?}))\n", token, byte); None},
    Node::CommentSigil{children} => {print!("CommentSigil\n"); Some(children)},
    Node::Comment{children} => {print!("Comment\n"); Some(children)},
    Node::Any{children} => {print!("Any\n"); Some(children)},
    Node::Symbol{children} => {print!("Symbol\n"); Some(children)},
    Node::Quantity{children} => {print!("Quantity\n"); Some(children)},
    Node::StateMachine{children} => {print!("StateMachine\n"); Some(children)},
    Node::Transitions{children} => {print!("Transitions\n"); Some(children)},
    Node::Transition{children} => {print!("Transition\n"); Some(children)},
    Node::LessThan => {print!("LessThan\n",); None},
    Node::GreaterThan => {print!("GreaterThan\n",); None},
    Node::GreaterThanEqual => {print!("GreaterThanEqual\n",); None},
    Node::LessThanEqual => {print!("LessThanEqual\n",); None},
    Node::Equal => {print!("Equal\n",); None},
    Node::NotEqual => {print!("NotEqual\n",); None},
    Node::And => {print!("And\n",); None},
    Node::Or => {print!("Or\n",); None},
    _ => {print!("Unhandled Node"); None},
  };  
  match children {
    Some(childs) => {
      for child in childs {
        print_recurse(child, level + 1)
      }
    },
    _ => (),
  }    
}

pub fn spacer(width: usize) {
  let limit = if width > 0 {
    width - 1
  } else {
    width
  };
  for _ in 0..limit {
    print!("│");
  }
  print!("├");
}

// ## Parser

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
    let parse_tree = parse_mech(CompleteStr(text));
    match parse_tree {
      Ok((rest, tree)) => {
        self.unparsed = rest.to_string();
        self.parse_tree = tree;
      },
      _ => (), 
    }
  }

  pub fn parse_block(&mut self, text: &str) {
    let parse_tree = parse_block(CompleteStr(text));
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

leaf!{at, "@", Token::At}
leaf!{hashtag, "#", Token::HashTag}
leaf!{period, ".", Token::Period}
leaf!{colon, ":", Token::Colon}
leaf!{comma, ",", Token::Comma}
leaf!{apostrophe, "'", Token::Apostrophe}
leaf!{left_bracket, "[", Token::LeftBracket}
leaf!{right_bracket, "]", Token::RightBracket}
leaf!{left_parenthesis, "(", Token::LeftParenthesis}
leaf!{right_parenthesis, ")", Token::RightParenthesis}
leaf!{left_brace, "{", Token::LeftBrace}
leaf!{right_brace, "}", Token::RightBrace}
leaf!{equal, "=", Token::Equal}
leaf!{left_angle, "<", Token::LessThan}
leaf!{right_angle, ">", Token::GreaterThan}
leaf!{exclamation, "!", Token::Exclamation}
leaf!{question, "?", Token::Question}
leaf!{plus, "+", Token::Plus}
leaf!{dash, "-", Token::Dash}
leaf!{underscore, "_", Token::Underscore}
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

named!(word<CompleteStr, Node>, do_parse!(
  bytes: nom_alpha1 >>
  (Node::Word{children: bytes.chars().map(|b| Node::Token{token: Token::Alpha, byte: b as u8}).collect()})));

named!(number<CompleteStr, Node>, do_parse!(
  bytes: nom_digit1 >>
  (Node::Number{children: bytes.chars().map(|b| Node::Token{token: Token::Digit, byte: b as u8}).collect()})));

named!(punctuation<CompleteStr, Node>, do_parse!(
  punctuation: alt!(period | exclamation | question | comma | colon | semicolon | dash | apostrophe | left_parenthesis | right_parenthesis | left_angle | right_angle | left_brace | right_brace) >>
  (Node::Punctuation{children: vec![punctuation]})));

named!(symbol<CompleteStr, Node>, do_parse!(
  punctuation: alt!(ampersand | bar | at | slash | hashtag | equal | tilde | plus | asterisk | caret | underscore) >>
  (Node::Symbol{children: vec![punctuation]})));

named!(text<CompleteStr, Node>, do_parse!(
  word: many1!(alt!(word | space | number | punctuation | symbol)) >>
  (Node::Text{children: word})));

named!(paragraph_rest<CompleteStr, Node>, do_parse!(
  word: many1!(alt!(word | space | number | punctuation | symbol | quote)) >>
  (Node::Text{children: word})));

  named!(paragraph_starter<CompleteStr, Node>, do_parse!(
  word: many1!(alt!(word | number | quote | left_angle | right_angle | period | exclamation | question | comma | colon | semicolon | left_parenthesis | right_parenthesis)) >>
  (Node::Text{children: word})));

named!(identifier<CompleteStr, Node>, do_parse!(
  identifier: map!(tuple!(count!(word,1), many0!(alt!(dash | slash | word | number | underscore))), |tuple| {
    let (mut word, mut rest) = tuple;
    word.append(&mut rest);
    word
  }) >>
  (Node::Identifier{children: identifier})));

named!(carriage_newline<CompleteStr, Node>, do_parse!(
  tag!("\r\n") >>
  (Node::Null)));

named!(newline<CompleteStr, Node>, do_parse!(
  alt!(new_line_char | carriage_newline) >>
  (Node::Null)));

named!(whitespace<CompleteStr, Node>, do_parse!(
  many0!(space) >> newline >>
  (Node::Null)));

named!(floating_point<CompleteStr, Node>, do_parse!(
  period >> bytes: nom_digit1 >>
  (Node::FloatingPoint{children: bytes.chars().map(|b| Node::Token{token: Token::Digit, byte: b as u8}).collect()})));

named!(quantity<CompleteStr, Node>, do_parse!(
  quantity: map!(tuple!(number, opt!(floating_point), opt!(identifier)),|tuple| {
    let (front, floating_point, unit) = tuple;
    let mut quantity = vec![front];
    match floating_point {
      Some(point) => quantity.push(point),
      _ => (),
    };
    match unit {
      Some(unit) => quantity.push(unit),
      _ => (),
    };
    quantity
  }) >>
  (Node::Quantity{children: quantity})));

named!(constant<CompleteStr, Node>, do_parse!(
  constant: alt!(string | quantity) >>
  (Node::Constant{children: vec![constant]})));

named!(empty<CompleteStr, Node>, do_parse!(
  underscore >>
  (Node::Empty)));

// ## Blocks

// ### Data

named!(select_all<CompleteStr, Node>, do_parse!(
  colon >> 
  (Node::SelectAll{children: vec![]})));

named!(subscript<CompleteStr, Node>, do_parse!(
  subscript: alt!(select_all | constant | expression) >> many0!(space) >> opt!(comma) >> many0!(space) >>
  (Node::Subscript{children: vec![subscript]})));

named!(subscript_index<CompleteStr, Node>, do_parse!(
  left_brace >> subscripts: many1!(subscript) >> right_brace >>
  (Node::SubscriptIndex{children: subscripts})));

named!(dot_index<CompleteStr, Node>, do_parse!(
  period >> index: map!(tuple!(identifier,opt!(subscript_index)),|tuple|{
    let (identifier, subscript) = tuple;
    let mut index = vec![identifier];
    match subscript {
      Some(subscript) => index.push(subscript),
      None => (),
    };
    index
  }) >>
  (Node::DotIndex{children: index})));

named!(index<CompleteStr, Node>, do_parse!(
  index: alt!(dot_index | subscript_index) >>
  (Node::Index{children: vec![index]})));

named!(data<CompleteStr, Node>, do_parse!(
  data: map!(tuple!(alt!(table | identifier), many0!(index)), |tuple| {
    let (mut source, mut indices) = tuple;
    let mut data = vec![source];
    data.append(&mut indices);
    data
  }) >>
  (Node::Data { children: data })));

// ### Tables

named!(table<CompleteStr, Node>, do_parse!(
  hashtag >> table_identifier: identifier >>
  (Node::Table { children: vec![table_identifier] })));

named!(binding<CompleteStr, Node>, do_parse!(
binding_id: identifier >> colon >> many0!(space) >> 
bound: alt!(empty | expression | identifier | constant ) >> many0!(space) >> opt!(comma) >> many0!(space) >>
(Node::Binding { children: vec![binding_id, bound] })));

named!(table_column<CompleteStr, Node>, do_parse!(
  many0!(alt!(space | tab)) >> item: alt!(empty | data | expression | quantity) >> opt!(comma) >> opt!(alt!(space | tab)) >>
  (Node::Column { children: vec![item] })));

named!(table_row<CompleteStr, Node>,
do_parse!(
  many0!(alt!(space | tab)) >> columns: many1!(table_column) >> opt!(semicolon) >> opt!(newline) >>
  (Node::TableRow { children: columns })));

named!(attribute<CompleteStr, Node>, do_parse!(
  identifier: identifier >> many0!(space) >> opt!(comma) >> many0!(space) >>
  (Node::Attribute { children: vec![identifier] })));

named!(table_header<CompleteStr, Node>, do_parse!(
  bar >> attributes: many1!(attribute) >> bar >> many0!(space) >> opt!(newline) >>
  (Node::TableHeader { children: attributes })));

named!(anonymous_table<CompleteStr, Node>, do_parse!(
  left_bracket >> many0!(space) >> table: map!(tuple!(opt!(table_header),many0!(table_row)),|tuple|{
    let (table_header, mut table_rows) = tuple;
    let mut table = vec![];
    match table_header {
      Some(table_header) => table.push(table_header),
      _ => (),
    };
    table.append(&mut table_rows);
    table
  }) >> right_bracket >>
  (Node::AnonymousTable { children: table })));

named!(inline_table<CompleteStr, Node>, do_parse!(
  left_bracket >> bindings: many1!(binding) >> right_bracket >>
  (Node::InlineTable { children: bindings })));

// ### Statements

named!(comment_sigil<CompleteStr, Node>, do_parse!(tag!("//") >> (Node::Null)));

named!(comment<CompleteStr, Node>, do_parse!(
  comment_sigil >> comment: text >>
  (Node::Comment { children: vec![comment] })));

named!(add_row_operator<CompleteStr, Node>, do_parse!(tag!("+=") >> (Node::Null)));

named!(add_row<CompleteStr, Node>, do_parse!(
  table_id: table >> space >> add_row_operator >> space >> table: alt!(inline_table | anonymous_table) >>
  (Node::AddRow { children: vec![table_id, table] })));

named!(set_operator<CompleteStr, Node>, do_parse!(tag!(":=") >> (Node::Null)));

named!(set_data<CompleteStr, Node>, do_parse!(
  table: data >> space >> set_operator >> space >> expression: expression >>
  (Node::SetData { children: vec![table, expression] })));

named!(variable_define<CompleteStr, Node>, do_parse!(
  variable: identifier >> space >> equal >> space >> expression: expression >>
  (Node::VariableDefine { children: vec![variable, expression] })));

named!(table_define<CompleteStr, Node>, do_parse!(
  table: table >> space >> equal >> space >> expression: expression >>
  (Node::TableDefine { children: vec![table, expression] })));

named!(watch_operator<CompleteStr, Node>, do_parse!(
  tilde >> 
  (Node::Null)));

named!(until_operator<CompleteStr, Node>, do_parse!(
  tag!("~|") >> 
  (Node::Null)));

named!(as_soon_as<CompleteStr, Node>, do_parse!(
  tag!("|~") >> 
  (Node::Null)));

named!(data_watch<CompleteStr, Node>, do_parse!(
  watch_operator >> space >> watch: alt!(variable_define | filter_expression | logic_expression | data ) >>
  (Node::DataWatch { children: vec![watch] })));

named!(statement<CompleteStr, Node>, do_parse!(
  statement: alt!(table_define | variable_define | data_watch | set_data | add_row | comment) >>
  (Node::Statement { children: vec![statement] })));

// ### Expressions

// #### Math Expressions

named!(parenthetical_expression<CompleteStr, Node>, do_parse!(
  left_parenthesis >> l1: l1 >> right_parenthesis >>
  (Node::ParentheticalExpression { children: vec![l1] })));

named!(negation<CompleteStr, Node>, do_parse!(
  dash >> negated: alt!(data | constant) >>
  (Node::Negation { children: vec![negated] })));

named!(function<CompleteStr, Node>, do_parse!(
  function_nodes: map!(tuple!(identifier, left_parenthesis, many1!(binding), right_parenthesis),|tuple|{
    let (identifier, _, mut bindings, _) = tuple;
    let mut function = vec![identifier];
    function.append(&mut bindings);
    function
  }) >>
  (Node::Function { children: function_nodes })));

named!(l1_infix<CompleteStr, Node>, do_parse!(
  space >> op: alt!(plus | dash) >> space >> l2: l2 >>
  (Node::L1Infix { children: vec![op, l2] })));

named!(matrix_multiply<CompleteStr, Node>, do_parse!(
  tag!("**") >> 
  (Node::Null)));

named!(l2_infix<CompleteStr, Node>, do_parse!(
  space >> op: alt!(asterisk | slash | matrix_multiply) >> space >> l3: l3 >>
  (Node::L2Infix { children: vec![op, l3] })));

named!(l3_infix<CompleteStr, Node>, do_parse!(
  space >> op: caret >> space >> l4: l4 >>
  (Node::L3Infix { children: vec![op, l4] })));

named!(l4<CompleteStr, Node>, do_parse!(
  l4: alt!(function | data | quantity | negation | parenthetical_expression) >>
  (Node::L4 { children: vec![l4] })));

named!(l3<CompleteStr, Node>, do_parse!(
  l4: map!(tuple!(l4, many0!(l3_infix)), |tuple| {
    let (mut l, mut infix) = tuple;
    let mut math = vec![l];
    math.append(&mut infix);
    math
  }) >>
  (Node::L3 { children: l4 })));

named!(l2<CompleteStr, Node>, do_parse!(
  l3: map!(tuple!(l3, many0!(l2_infix)), |tuple| {
    let (mut l, mut infix) = tuple;
    let mut math = vec![l];
    math.append(&mut infix);
    math
  }) >>
  (Node::L2 { children: l3 })));

named!(l1<CompleteStr, Node>, do_parse!(
  l2: map!(tuple!(l2, many0!(l1_infix)), |tuple| {
    let (mut l, mut infix) = tuple;
    let mut math = vec![l];
    math.append(&mut infix);
    math
  }) >>
  (Node::L1 { children: l2 })));

named!(math_expression<CompleteStr, Node>, do_parse!(
  l1: l1 >>
  (Node::MathExpression { children: vec![l1] })));

// #### Filter Expressions

named!(not_equal<CompleteStr, Node>, do_parse!(tag!("!=") >> (Node::NotEqual)));

named!(equal_to<CompleteStr, Node>, do_parse!(tag!("==") >> (Node::Equal)));

named!(less_than_equal<CompleteStr, Node>, do_parse!(tag!("<=") >> (Node::LessThanEqual)));

named!(greater_than_equal<CompleteStr, Node>, do_parse!(tag!(">=") >> (Node::GreaterThanEqual)));

named!(less_than<CompleteStr, Node>, do_parse!(tag!("<") >> (Node::LessThan)));

named!(greater_than<CompleteStr, Node>, do_parse!(tag!(">") >> (Node::GreaterThan)));

named!(comparator<CompleteStr, Node>, do_parse!(
  comparator: alt!(greater_than_equal | less_than_equal | equal_to | not_equal | less_than | greater_than) >>
  (Node::Comparator { children: vec![comparator] })));

named!(filter_expression<CompleteStr, Node>, do_parse!(
  lhs: alt!(data | constant) >> space >> comp: comparator >> space >> rhs: alt!(data | constant) >>
  (Node::FilterExpression { children: vec![lhs, comp, rhs] })));

// State Machine

named!(state_machine<CompleteStr, Node>, do_parse!(
  source: data >> question >> whitespace >> transitions: transitions >> whitespace >>
  (Node::StateMachine { children: vec![source, transitions] })));

named!(transitions<CompleteStr, Node>, do_parse!(
  transitions: many1!(transition) >>
  (Node::Transitions { children:transitions })));

named!(transition<CompleteStr, Node>, do_parse!(
  many1!(space) >> state: alt!(string | constant | empty) >> many1!(space) >> tag!("=>") >> many1!(space) >> next: alt!(identifier | string | constant | empty) >> many0!(space) >> opt!(newline) >>
  (Node::Transition { children: vec![state, next] })));

// #### Logic Expressions

named!(or<CompleteStr, Node>, do_parse!(bar >> (Node::Or)));

named!(and<CompleteStr, Node>, do_parse!(ampersand >> (Node::And)));

named!(logic_operator<CompleteStr, Node>, do_parse!(
  operator: alt!(and | or) >>
  (Node::LogicOperator { children: vec![operator] })));

named!(logic_expression<CompleteStr, Node>, do_parse!(
  lhs: alt!(filter_expression | data | constant) >> many0!(space) >> op: logic_operator >> many0!(space) >> rhs: alt!(logic_expression | filter_expression | data | constant) >>
  (Node::LogicExpression { children: vec![lhs, op, rhs] })));

// #### Other Expressions

named!(range<CompleteStr, Node>, do_parse!(
  start: math_expression >> many0!(space) >> colon >> many0!(space) >> end: math_expression >>
  (Node::Range { children: vec![start,end] })));

named!(string<CompleteStr, Node>, do_parse!(
  quote >> text: many0!(text) >> quote >>
  (Node::String { children: text })));

named!(expression<CompleteStr, Node>, do_parse!(
  expression: alt!(state_machine | string | range | logic_expression | filter_expression | inline_table | anonymous_table | math_expression) >>
  (Node::Expression { children: vec![expression] })));

// ### Block Basics

named!(constraint<CompleteStr, Node>, do_parse!(
  space >> space >> statement_or_expression: statement >> many0!(space) >> opt!(newline) >>
  (Node::Constraint { children: vec![statement_or_expression] })));

named!(block<CompleteStr, Node>, do_parse!(
  constraints: many1!(constraint) >> many0!(whitespace) >>
  (Node::Block { children: constraints })));

// ## Markdown

named!(title<CompleteStr, Node>, do_parse!(
  hashtag >> space >> text: text >> many0!(whitespace) >>
  (Node::Title { children: vec![text] })));

named!(subtitle<CompleteStr, Node>, do_parse!(
  hashtag >> hashtag >> space >> text: text >> many0!(whitespace) >>
  (Node::Subtitle { children: vec![text] })));

named!(sectiontitle<CompleteStr, Node>, do_parse!(
  hashtag >> hashtag >> hashtag >> space >> text: text >> many0!(whitespace) >>
  (Node::SectionTitle { children: vec![text] })));

named!(inline_code<CompleteStr, Node>, do_parse!(
  grave >> text: text >> grave >> opt!(space) >>
  (Node::InlineCode { children: vec![text] })));

named!(paragraph_text<CompleteStr, Node>, do_parse!(
  paragraph: map!(tuple!(paragraph_starter, opt!(paragraph_rest)), |tuple| {
    let (mut word, mut text) = tuple;
    let mut paragraph = vec![word];
    match text {
      Some(text) => paragraph.push(text),
      _ => (),
    };
    paragraph
  }) >> many0!(space) >>
  (Node::ParagraphText { children: paragraph })));

named!(paragraph<CompleteStr, Node>, do_parse!(
  paragraph_elements: many1!(alt!(inline_mech_code | inline_code | paragraph_text)) >> opt!(newline) >> many0!(whitespace) >>
  (Node::Paragraph { children: paragraph_elements })));

named!(unordered_list<CompleteStr, Node>, do_parse! (
  list_items: many1!(list_item) >> opt!(whitespace) >>
  (Node::UnorderedList{children: list_items})));

named!(list_item<CompleteStr, Node>, do_parse! (
  dash >> space >> list_item: paragraph >> opt!(newline) >>
  (Node::ListItem{children: vec![list_item]})));

named!(formatted_text<CompleteStr, Node>, do_parse!(
  formatted: many0!(alt!(paragraph_rest | carriage_return | new_line_char)) >>
  (Node::FormattedText { children: formatted })));

named!(code_block<CompleteStr, Node>, do_parse!(
  grave >> grave >> grave >> newline >> text: formatted_text >> grave >> grave >> grave >> newline >> many0!(whitespace) >>
  (Node::CodeBlock { children: vec![text] })));

// Mechdown

named!(inline_mech_code<CompleteStr, Node>, do_parse!(
  left_bracket >> left_bracket >> expression: expression >> right_bracket >> right_bracket >> opt!(space) >>
  (Node::InlineMechCode { children: vec![expression] })));

named!(mech_code_block<CompleteStr, Node>, do_parse!(
  grave >> grave >> grave >> tag!("mech:") >> directive: word >> newline >> mech_block: block >> grave >> grave >> grave >> newline >> many0!(whitespace) >>
  (Node::MechCodeBlock { children: vec![directive, mech_block] })));

// ## Start Here

named!(section<CompleteStr, Node>, do_parse!(
  section: map!(tuple!(opt!(subtitle), many0!(alt!(block | code_block | mech_code_block | paragraph | unordered_list))), |tuple| {
    let (mut section_title, mut section_body) = tuple;
    let mut section = vec![];
    match section_title {
      Some(subtitle) => section.push(subtitle),
      _ => (),
    };
    section.append(&mut section_body);
    section
  }) >> many0!(whitespace) >>
  (Node::Section { children: section })));

named!(body<CompleteStr, Node>, do_parse!(
  many0!(whitespace) >> sections: many1!(section) >>
  (Node::Body { children: sections })));

named!(fragment<CompleteStr, Node>, do_parse!(
  statement: statement >>
  (Node::Fragment { children: vec![statement] })));

named!(program<CompleteStr, Node>, do_parse!(
  program: map!(tuple!(opt!(title),body), |tuple| {
    let (title, body) = tuple;
    let mut program = vec![];
    match title {
      Some(title) => program.push(title),
      None => (),
    };
    program.push(body);
    program
  } ) >> opt!(whitespace) >>
  (Node::Program { children: program })));

named!(parse_mech<CompleteStr, Node>, do_parse!(
  program: alt!(many1!(fragment) | many1!(program)) >>
  (Node::Root { children: program })));

named!(raw_constraint<CompleteStr, Node>, do_parse!(
  statement_or_expression: statement >> many0!(space) >> opt!(newline) >>
  (Node::Constraint { children: vec![statement_or_expression] })));

named!(parse_block<CompleteStr, Node>, do_parse!(
  constraints: many1!(raw_constraint) >> many0!(whitespace) >>
  (Node::Block { children: constraints })));