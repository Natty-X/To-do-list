use std::io;

fn main() {
    println!("\nMY TO_DO_LIST!");
   
    let mut task_list = UserInput {
        input: Vec::new(),
    };

    loop {
        println!("use the numbers mapped functions to call specific functions");
        println!("1-CREATE\t 2-READ\t 3-UPDATE/ADD\t 4-DELETE\t");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Enter from the above opeerations");
        
        match user_input.trim() {
            "1" => {
                task_list.create_task(); 
            }
            "2" => {
                task_list.print_list();
            }
            "3"=> {
                task_list.update_list();
            } 
            "4" => {
                task_list.delete_task();
            }
            "00"=> {
                println!("exitinnngg");
                break
            }
            _   => println!("Invalid input, Enter the numbers listed above [1, 2, 3, 4]"),
        };

    }

}

struct UserInput {
    input: Vec<String>,
}

impl UserInput {
    fn create_task(&mut self) {
        loop {
            let task = Self::read_input();

            if task == "exit" {
                break
            } else if task.is_empty() {
                println!("Field cannot be empty, enter a task");
                continue;
            } else {
                self.input.push(task);
            }
        }
}
    fn read_input() -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");

        input.trim().to_string()
    } 

    fn print_list(&self) {

        let mut counter = 0;
        for task in &self.input {
            counter += 1;

            println!("{}. {:?}", counter, task);
        }
    }

    fn update_list(&mut self) {

        if self.input.is_empty() {
            println!("You havent created a to_do_list yet, create one");
        } else {
        println!("{:#?}", self.input); 
        }
        
        let new_task = Self::read_input();
        self.input.push(new_task);

    }

    fn delete_task(&mut self) {
        self.print_list();

        if self.input.is_empty() {
            println!("You havent created your to_do_list yet, create one");

        }
        let value = Self::read_input();

        let task = &mut self.input;
        match value.trim() {
            "0" => { task.remove(0); },
            "1" => { task.remove(1); },
            "2" => { task.remove(2); },

             _ => println!("task not found"),
        }
    }
}
