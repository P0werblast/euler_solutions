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
}