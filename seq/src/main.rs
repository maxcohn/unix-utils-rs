use std::env;


fn main() {
    
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 1 {
        // Single number: 1-num

        let num = args.get(0)
            .expect("Failed to read args")
            .parse::<isize>()
            .expect("Failed to parse argument");

        for i in 1 ..= num {
            println!("{}", i);
        }
    } else if args.len() == 2 {
        // Two numbers: num1-num2

        let num1 = args.get(0)
            .expect("Failed to read args")
            .parse::<isize>()
            .expect("Failed to parse argument");

        let num2 = args.get(1)
            .expect("Failed to read args")
            .parse::<isize>()
            .expect("Failed to parse argument");

        for i in num1 ..= num2 {
            println!("{}", i);
        }
    } else if args.len() == 3 {
        // Three numbers: num1-num2, incrememnted by incr

        let num1 = args.get(0)
            .expect("Failed to read args")
            .parse::<isize>()
            .expect("Failed to parse argument");
        
        let rev;
        let incr = match args.get(1).expect("Failed to read args").parse::<isize>() {
            Ok(i) => {
                if i < 0 {
                    rev = true;
                    (-i) as usize
                } else {
                    rev = false;
                    i as usize
                }
            },
            Err(_) => panic!("Failed to parse argument")
        };

        let num2 = args.get(2)
            .expect("Failed to read args")
            .parse::<isize>()
            .expect("Failed to parse argument");
        
        if rev {
            if num1 >= num2 {
                for i in (num2 ..= num1).rev().step_by(incr) {
                    println!("{}", i);
                }
            }
        } else {
            for i in (num1 ..= num2).step_by(incr) {
                println!("{}", i);
            }
        }

    }
}
