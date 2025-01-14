extern crate mech_syntax;
extern crate mech_core;

use mech_syntax::compiler::{Compiler, Node, Element};
use mech_syntax::formatter::Formatter;
use mech_core::Block;
use mech_core::{Change, Transaction};
use mech_core::{Value, Index};
use mech_core::Hasher;
use mech_core::Core;
use mech_core::make_quantity;
use std::time::{Duration, SystemTime};

fn compile_test(input: String, test: Value) {
  let mut compiler = Compiler::new();
  let mut core = Core::new(10, 10);
  compiler.compile_string(input);
  core.register_blocks(compiler.blocks);
  core.step();
  let table = Hasher::hash_str("test");
  let row = Index::Index(1);
  let column = Index::Index(1);
  let actual = core.index(table, &row, &column);
  match actual {
    Some(value) => {
      assert_eq!(*value, test);
    },
    None => assert_eq!(0,1),
  }
}

fn main() {
  let input = String::from(r#"
block
  #i = [x: 2]
  #h = [53; 100; 85]
  #x = [400; 0; 0; 0; 0; 0]
 
block
  #i.x{#i < 6} := #i.x + 1

block 
  ~ #i.x
  i = #i
  i2 = #i / 2
  ir = math/round(column: i2)
  #x{i,:} := #x{i - 1,:} + #h{ir,:}
"#);
  
  //let value = Value::Number(make_quantity(780000,-4,0));
  //compile_test(input.clone(), value);

  let mut compiler = Compiler::new();
  let mut formatter = Formatter::new();
  let mut core = Core::new(1_000, 250);
  let programs = compiler.compile_string(input.clone());

  println!("{:?}", programs);
 

  core.register_blocks(compiler.blocks.clone());
  //println!("{:?}", compiler.parse_tree);
  println!("{:?}", compiler.unparsed);
  //println!("{:?}", compiler.syntax_tree);
  //println!("{:?}", core.runtime);
  core.step();
  println!("{:?}", core);
  //println!("{:?}", core.runtime);
  /*let block_ast = match &programs[0].sections[0].elements[1] {
  Element::Block((id, node)) => node,
    _ => &Node::Null,
  };
  formatter.format(&block_ast);*/
  
  
  //let now = SystemTime::now();
  /*let change = Change::Set{table: 0x132537277, 
                            row: Index::Index(1), 
                            column: Index::Index(3),
                            value: Value::from_u64(42),
                          };
  let txn = Transaction::from_change(change.clone());

  core.process_transaction(&txn);*/
  //println!("{:?}", core);
  //println!("{:?}", core.runtime);
  /*
  match now.elapsed() {
    Ok(elapsed) => {
      // it prints '2'
      let time: f32 = elapsed.as_millis() as f32;
      println!("{}ms", time / n as f32);
    }
    Err(e) => {
      // an error occurred!
      println!("Error: {:?}", e);
    }
  }*/
  //println!("{:?}", core);

}

/*
This program doesn't execute correctly.
block
  #i = [x: 2]
  #h = [53; 100; 85]
  #p = [|x   y|
         400 500 
         0   0
         0   0
         0   0]
  #angle = [10; 20; 30]
 
block
  #i.x{#i.x <= 6} := #i.x + 1

block 
  ~ #i.x
  i = #i
  i2 = i / 2
  ir = math/round(column: i2)
  a = #angle{i2,:}
  #p.x{i} := #p.x{i - 1} + #h{i2,:} * math/sin(degrees: a)
  #p.y{i} := #p.y{i - 1} - #h{i2,:} * math/cos(degrees: a)

  */