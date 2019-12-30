use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    while true{
        println!("HELLO");
        println!("What action you want to perform");
        println!(" 1. Generate & View Card");
        println!(" 2. Quit");
        let mut main_input  = String::new();
        io::stdin().read_line(&mut main_input);
        let main_input : i32 = main_input.trim().parse().unwrap();
        if main_input == 1
        {
            while true{
                println!("Enter your full name Please !");
                let mut i_name = String::new();
                io::stdin().read_line(&mut i_name);
                println!("Enter your age Please !");
                let mut i_age = String::new();
                io::stdin().read_line(&mut i_age);
                println!("Enter your Course Please !");
                let mut i_course = String::new();
                io::stdin().read_line(&mut i_course);
                println!("Enter your Batch Please !");
                let mut i_batch = String::new();
                io::stdin().read_line(&mut i_batch);
                let output = Student_entry{name:i_name,age:i_age,course:i_course,batch_no:i_batch};
                thread::sleep(Duration::from_secs(2));
                println!(" => => => => printing card.......");
                thread::sleep(Duration::from_secs(1));
                println!("{:#?}",output.print_card());
                break
            }
        }
        else if main_input == 2 {
            println!("OK GOOD BYE NOW!");
            break
        }
        else{
            
            println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
            
            println!("Invalid Input");
            thread::sleep(Duration::from_secs(1));
            println!("please enter a valid input");
            println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        }
        thread::sleep(Duration::from_secs(1));
    }
}

#[derive(Debug)]
struct Student_entry {
    name : String,
    age : String,
    course : String,
    batch_no : String,
}

impl Student_entry{
    fn print_card(self) -> (){
        thread::sleep(Duration::from_secs(1));
        println!("----------------------------------------XXXXXXXXXXXXXXXXXXXXXXXXXXX-----------------------------------------------------");
        println!("----------------------------------------XX PIAIC STUDENT ID CARD XX-----------------------------------------------------");
        println!("----------------------------------------XXXXXXXXXXXXXXXXXXXXXXXXXXX-----------------------------------------------------");
        thread::sleep(Duration::from_secs(1));
        println!("");   
        thread::sleep(Duration::from_secs(1));
        println!("                                                    IMAGE                                        ");
        thread::sleep(Duration::from_secs(1));
        println!("                                        xxxxxxxxxxxxxxxxxxxxxxxxxx");
        println!("                                        x       Image will be    x");
        println!("                                        x       Uploaded Here    x");
        println!("                                        xxxxxxxxxxxxxxxxxxxxxxxxxx");
        println!("                         ");
        thread::sleep(Duration::from_secs(1));
        println!("                                        Name of Student : {}",self.name);
        println!("                                        Age of Student  :  {}",self.age);
        println!("                                        Enrolled Course :  {}",self.course);
        println!("                                        Batch Number    :  {}",self.batch_no);
        println!("------------------------------------------------------------------------------------------------------------------------");
    }
}