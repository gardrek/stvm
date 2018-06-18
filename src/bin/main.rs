extern crate stvm;
use stvm::{Lang, STVM};

fn main() -> () {
    let zero = 0u8;
    let neg_two = -2i8;
    assert_eq!(neg_two as u8, zero.wrapping_add(neg_two as u8));
    assert_eq!(-(std::i8::MIN + 1), std::i8::MAX);
    assert_eq!(-(std::isize::MIN + 1), std::isize::MAX);

    //let mut main_vm = STVM::from_file("LostKingdomBF/LostKng.b");
    let mut main_vm = STVM::from_file("tictactoe.bf");
    //let mut main_vm = STVM::from_file("life.bf");
    //let mut main_vm = STVM::from_file("hanoi.bf");
    //let mut main_vm = STVM::from_file("mandelbrot.bf");

    //let mut main_vm = STVM::from_code(Lang::LISP, "(add (add 2 3 ) 5) \n(add 123 456 7890)(print \"I'm a string! Say what?\")");

    //let mut main_vm = STVM::from_code(Lang::BF, "-+");
    //let mut main_vm = STVM::new_test();
    //let mut main_vm = STVM::from_code(Lang::BF,
        //"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."
    //);

    //*
    let mut simpletest = STVM::from_code(Lang::BF, "[]+++++>++<[->>+<<]>>>+++[-]++[<]++[>]");
    simpletest.debug_print();
    println!("\n\n--------\n{:?}\n", simpletest);
    simpletest.run();
    simpletest.debug_print();
    println!("{:?}", simpletest);;

    let mut multiply = STVM::from_code(Lang::BF, "+++++[>+++<-]>");
    multiply.run();
    // */

    //println!("{}", main_vm.read());

    /*let mut main_vm = STVM::from_code(
        ">++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.>->
        +++..+++.>-.<<+[>[+>+]>>]<--------------.>>.+++.------.--------.>+.>+."
    );*/

    /*
    let mut main_vm = STVM::from_code(
        Lang::BF,
        "++++++++++[>+++>++++>+++++++++++>++++++++<<<<-]>++>++++>++>+++.<\
         .--------.+.+++++.++++++++++.<<.>>>>++[<<---------.>>-]<<<<.>>--\
         --.++++++++++.-----------.++.++++++++.<<.>>++++++.++++.>>++++[<<\
         ----->>-]<<.>>++++[<<++++>>-]<<+.++.++++++.<.<.>>>>++++[<<---->>\
         -]<<.+++++++++++.>>++++[<<---->>-]<<-.+++.--.<<.>>++++++++.+++++\
         +++++++.<<.>>---.-------.++++++++.<++.>>[-]<[-]<[-]<[-]<",
    );
    // */

    /*let mut main_vm = STVM::from_code(
        ">>>>--<-<<+[+[<+>--->->->-<<<]>]<<--.<++++++.<<-..<<.<+.>>.>>.<<<.+++.>>.>>-.<<<+."
    );*/

    //println!("{:?}\n--------\n", main_vm);

    main_vm.debug_print();
    println!("\n\n--------\n{:?}\n", main_vm);
    main_vm.run();
    println!();
    main_vm.debug_print();
    println!("\n\n--------\n{:?}\n", main_vm);
}
