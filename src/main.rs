use std::io;

fn main() {
    
    
    println!("Celsius To Fahrenheit And Fahrenheit To Celsius Converter");

    loop {

        let mut _output: f64 = 0.0;
        let mut _user_input: String = String::new();    
        let mut user_choice: String = String::new();

        println!("Please select the value tempearture type you'll be inputing, select 1 for celsius select 2 for fahrenheit");
    
        io::stdin().read_line(&mut user_choice)
            .expect("Failed to read user input");
    
        let user_choice: u8 = user_choice.trim().parse()
            .expect("Please enter a numerical input.");

        println!("user_choice: {}", user_choice);
        
        if user_choice == 1 as u8{
            println!("Enter value in celsius: ");
            _user_input = get_user_input();
            _output = celsius_to_farenheit(&_user_input);
            println!("The fahrenheit equivalent of the celsius value: {} is: {}.", _user_input, _output);
            break;
        }else if user_choice == 2 as u8{
            println!("Enter value in fahrenheit: ");
            _user_input = get_user_input();
            _output = fanrenhiet_to_celsius(&_user_input);
            println!("The celsius equivalent of the fahrenheit value: {} is: {}.", _user_input, _output);
            break;
        }else{
            println!("Please input a valid choice");
            continue;
        }
    }

}

fn fanrenhiet_to_celsius(input: &str)->f64{
    let mut _result: f64 = 0 as f64;
    
    let input: f64 = input.trim().parse()
        .expect("Please input the correct temperature numerical");

    _result = ((input - 32 as f64) * 5 as f64)/9 as f64;
    _result
}


fn celsius_to_farenheit(input: &str)->f64{
    let mut _output: f64 = 0 as f64;

    let input:f64 = input.trim().parse()
        .expect("Please provide the correct numerical celsius temperature");
    
    _output = ((9 as f64 * input as f64)/5 as f64) + 32 as f64;
    _output
}

fn get_user_input()->String{
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input)
        .expect("failed to read user input");
    user_input
}