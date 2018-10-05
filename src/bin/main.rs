extern crate stvm;

use stvm::{Lang, STVM};

use std::env;

fn wait_for_input() -> () {
    use std::io::{self, Read};
    let mut buffer = [0u8; 1];
    let mut stdin = io::stdin();
    stdin.lock();
    match stdin.read(&mut buffer) {
        Err(e) => panic!(e),
        _ => (),
    }
}

fn main() -> () {
    let mut lang:Option<Lang> = None;

    let mut positional: Vec<String> = vec![];

    let mut i = 0;
    for argument in env::args() {
        let s = argument.as_ref();
        if i != 0 {
            match s {
                "" => {},
                _ => match s.chars().next().unwrap() {
                    '-' => match s {
                        "--bf" => lang = Some(Lang::BF),
                        _ => panic!("unkown flag {}", s),
                    },
                    _ => positional.push(argument.clone()),
                },
            };
        };
        i += 1;
    };

    if positional.len() > 1 {
        panic!("too many arguments");
    };

    if let Some(lang) = lang {
        // A language was selected
        if positional.len() > 0 {
            // the first argument is a filename
            let file = &positional[0];
            let mut main_vm = STVM::from_file(lang, file);
            println!("Press enter to run program.");
            wait_for_input();

            let e = main_vm.run();
            println!();
            println!();

            match e {
                Err(e) => match e {
                    stvm::VMError::Halt => println!("Program halted."),
                    _ => println!("{:?}", e),
                }
                _ => println!("OK"),
            }
            //println!("{:?} {:?}", lang, file);
        //} else {
            // with no argument, assume program is on stdin?
        };
    //} else {
        // with no language selected, what?
    };

/*
    let zero = 0u8;
    let neg_two = -2i8;
    assert_eq!(neg_two as u8, zero.wrapping_add(neg_two as u8));
    assert_eq!(-(std::i8::MIN + 1), std::i8::MAX);
    assert_eq!(-(std::isize::MIN + 1), std::isize::MAX);


    let mut test = STVM::new_test();
    println!("\n\n--------\n{:?}\n", test);
    test.debug_print();
    println!("{:?}", test.run());
    println!("\n\n--------\n{:?}\n", test);

    /*
    let mut simpletest = STVM::from_code(Lang::BF, "[]+++++>++<[->>+<<]>>>+++[-]++[<]++[>]");
    simpletest.debug_print();
    println!("\n\n--------\n{:?}\n", simpletest);
    simpletest.run();
    simpletest.debug_print();
    println!("\n\n--------\n{:?}\n", simpletest);;

    let mut multiply = STVM::from_code(Lang::BF, "+++++[>+++<-]>");
    multiply.run();
    // */

    //let mut main_vm = STVM::from_file(Lang::BF, "LostKingdomBF/LostKng.b");
    //let mut main_vm = STVM::from_file(Lang::BF, "tictactoe.bf");
    //let mut main_vm = STVM::from_file(Lang::BF, "life.bf");
    //let mut main_vm = STVM::from_file(Lang::BF, "hanoi.bf");
    //let mut main_vm = STVM::from_file(Lang::BF, "oobrain.min.bf");
    //let mut main_vm = STVM::from_file(Lang::BF, "mandelbrot.bf");

    //let mut main_vm = STVM::from_code(Lang::LISP, "(add (add 2 3 ) 5) \n(add 123 456 7890)(print \"I'm a string! Say what?\")");

    //let mut main_vm = STVM::from_code(Lang::BF, "-+");
    //let mut main_vm = STVM::new_test();
    //let mut main_vm = STVM::from_code(Lang::BF,
        //"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."
    //);


    //println!("{}", main_vm.read());

    /*
    let mut main_vm = STVM::from_code(
        Lang::BF,
        ">++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.>->
        +++..+++.>-.<<+[>[+>+]>>]<--------------.>>.+++.------.--------.>+.>+."
    );
    // */

    //*
    let mut main_vm = STVM::from_code(
        Lang::BF,
        "Sphinx \
         ++++++++++[>+++>++++>+++++++++++>++++++++<<<<-]>++>++++>++>+++.<\
         .--------.+.+++++.++++++++++.<<.>>>>++[<<---------.>>-]<<<<.>>--\
         --.++++++++++.-----------.++.++++++++.<<.>>++++++.++++.>>++++[<<\
         ----->>-]<<.>>++++[<<++++>>-]<<+.++.++++++.<.<.>>>>++++[<<---->>\
         -]<<.+++++++++++.>>++++[<<---->>-]<<-.+++.--.<<.>>++++++++.+++++\
         +++++++.<<.>>---.-------.++++++++.<++.>>[-]<[-]<[-]<[-]<",
    );
    // */

    /*
    let mut main_vm = STVM::from_code(
        Lang::BF,
        ">>>>--<-<<+[+[<+>--->->->-<<<]>]<<--.<++++++.<<-..<<.<+.>>.>>.<<<.+++.>>.>>-.<<<+."
    );
    // */

    /*
    let mut main_vm = STVM::from_code(
        Lang::BF,
        "++++"
    );
    // */

    /*
        "-[------->+<]>.-[->++++<]>.-[--->++<]>+.+++.+.++++++++.+[---->+<\
         ]>++.---[----->++<]>.---------.--.+.++++++++++++.-[->+++++<]>."
    */

    /*
    let mut main_vm = STVM::from_code(
        Lang::BF,
        ">>>++[
    <++++++++[
        <[<++>-]>>[>>]+>>+[
            -[->>+<<<[<[<<]<+>]>[>[>>]]]
            <[>>[-]]>[>[-<<]>[<+<]]+<<
        ]<[>+<-]>>-
    ]<.[-]>>
]
\"Random\" byte generator using the Rule 30 automaton.
Doesn't terminate; you will have to kill it.
To get x bytes you need 32x+4 cells.
Turn off any newline translation!
Daniel B Cristofani (cristofdathevanetdotcom)
http://www.hevanet.com/cristofd/brainfuck/",
    );
    // */

    println!("\n\n--------\n");
    //main_vm.debug_print();
    //println!("\n\n--------\n{:?}\n", main_vm);
    //println!("\n\n--------\n{:?}\n", main_vm.bytecode);

    println!();
    println!("Press enter to run program");
    use std::io::{self, Read};
    let mut buffer = [0u8; 1];
    let mut stdin = io::stdin();
    stdin.lock();
    match stdin.read(&mut buffer) {
        Err(e) => panic!(e),
        _ => (),
    }

    let e = main_vm.run();
    println!();
    println!();

    match e {
        Err(e) => match e {
            stvm::VMError::Halt => println!("Program halted."),
            _ => println!("{:?}", e),
        }
        _ => println!("OK?????"),
    }

    println!("Press enter to print debug");
    stdin.lock();
    match stdin.read(&mut buffer) {
        Err(e) => panic!(e),
        _ => (),
    }

    println!();
    //main_vm.debug_print();
    println!("\n\n--------\n{:?}\n", main_vm);
*/
}
