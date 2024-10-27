pub mod solutions {
    use std::io;

    pub fn problem_1(){
        println!("Problem 1:");
        println!("**********");


        let mut input: String = "".to_owned();
        println!("Input a number:");
        io::stdin().read_line(&mut input).unwrap();

        println!("Checking result against:{input}");

        let input_number = input.trim().parse::<u32>().unwrap();
        let mut result = 0;

        for i in 0..input_number{
            if i%3 == 0 || i%5 == 0{
                result += i;
            }
        }        

        println!("Result: {result}");

        println!("**********");
    }

    pub fn problem_2 (){
        println!("Problem 1:");
        println!("**********");

        // fibo series starts with 1 and 2
        let mut result: u32 = 2;
        let mut previous: u32 = 1;
        let mut current : u32 = 2;

        println!("{previous}");
        println!("{current}");

        while current < 4000000 {

            let next = current + previous;
            println!("{next}");

            if next < 4000000 && next % 2 == 0{
                result += next;
            }

            previous = current;
            current = next;
        }

        println!("Result: {result}");
        println!("**********");
    }
}