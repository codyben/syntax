#[macro_use]
extern crate mech_syntax;
extern crate mech_core;

use mech_syntax::lexer::Lexer;
use mech_syntax::parser::{Parser, ParseStatus, Node};
use mech_syntax::compiler::Compiler;
use mech_core::{Hasher, Core};

macro_rules! compile_string {
  ($func:ident, $test:tt) => (
    #[test]
    fn $func() {
      let mut lexer = Lexer::new();
      let mut parser = Parser::new();
      let mut compiler = Compiler::new();
      lexer.add_string(String::from($test));
      let tokens = lexer.get_tokens();
      parser.text = String::from($test);
      parser.add_tokens(&mut tokens.clone());
      parser.build_parse_tree();
      assert_eq!(parser.status, ParseStatus::Ready);
      compiler.build_syntax_tree(parser.parse_tree);
      let ast = compiler.syntax_tree.clone();
      compiler.compile_blocks(ast);
      assert_eq!(parser.status, ParseStatus::Ready);
    }
  )
}

macro_rules! test_math {
  ($func:ident, $input:tt, $test:tt) => (
    #[test]
    fn $func() {
      let mut compiler = Compiler::new();
      let mut core = Core::new(10, 10);
      let input = String::from($input);
      compiler.compile_string(input);
      core.register_blocks(compiler.blocks);
      core.step();
      let table = Hasher::hash_str("test");
      let row = 1;
      let column = 1;
      let test = $test;
      assert_eq!(core.index(table,row,column).unwrap().as_u64().unwrap(),test);
    }
  )
}

compile_string!(empty, "");

// ## Constant

compile_string!(constant_digit, "1");

// ## Table

compile_string!(table, "#table");
compile_string!(table_define, "#table = [x y z]");
//compile_string!(table_index_bracket_index, "#table[1]");
//compile_string!(table_index_dot_index_name, "#table.field");


compile_string!(table_define_program, "# A Working Program

## Section Two

  #gravity = 9");

test_math!(math_constant,"#test = 10", 10);
test_math!(math_add,"#test = 1 + 1", 2);
test_math!(math_multiply,"#test = 2 * 2", 4);
test_math!(math_divide,"#test = 4 / 2", 2);
test_math!(math_multiple_rows_select,"
block
  #ball = [x: 15 y: 9 vx: 18 vy: 0]
block
  #test = #ball.x + #ball.y * #ball.vx",177);
test_math!(math_single_cell_table_math,"# Bouncing Balls
Define the environment
  #ball = [x: 15 y: 9 vx: 18 vy: 9]
  #system/timer = [resolution: 1000]
  #gravity = 10
  
Define the timestep
  #dt = #system/timer.resolution

Now update the block positions
  #x = #ball.x + #ball.vx
  #y = #ball.y + #ball.vy
  #test = #ball.vy + #gravity * #dt",10009);

test_math!(filter_less_than,"block 
  #x = [x: 5001 y: 456]
  #boundary = 5000
block
  iy = #x.y < #boundary
  #x.y[iy] := 12
block
  #test = #x.y",12);