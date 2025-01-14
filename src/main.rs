use serde::{Deserialize, Serialize };
use std::io::{self,Write};

#[derive(Serialize, Deserialize)]
struct Task{
    date:String,
    time:String,
    task_name:String,
    task_id:u32,
    completed_task:bool,
}

fn adding_new_task(tasks:&mut Vec<Task>, date:String, time:String, task_name:String){
    let task_id_collection = tasks.len() as u32 + 1;
    tasks.push(Task{date, time, task_name, task_id:task_id_collection, completed_task:false});
}

fn task_list(tasks:& Vec<Task>){
    if tasks.is_empty(){
        println!("There is no tasks recorded, please trying implementing a task.");
} else{
    println!("Task List:");
    for task in tasks{
        println!("Task  Schedule For:{}. Time:{}. Task Identification:{}. Task name:{}. Completed:{}.",
        task.date, task.time, task.task_id, task.task_name, 
        if task.completed_task {"Yes"} else {"No"}
    );
    }
}
}

fn removing_task(tasks:&mut Vec<Task>, task_id:u32){
    if let Some(pos) = tasks.iter().position(|t| t.task_id == task_id){
        tasks.remove(pos);
        println!("You have successfully remove task Task Name:{}. with Task Id:{}.", tasks[pos].task_name, task_id);
    } else{
        println!("Task cannot be found.");
    }
}

fn task_completed(tasks: &mut Vec<Task>, task_id:u32){
    if let Some(task) = tasks.iter_mut().find(|t| t.task_id == task_id){
        task.completed_task = true;
        println!("The Task {} with an id {} have been marked as completed",task.task_name, task_id);
    } else {
        println!("The task with id {} not found.",task_id);
    }
}

fn create_pin()-> String {
    loop{
    let mut pin = String::new();
    println!("Please create your PIN:");
    io::stdout().flush().unwrap();
    io::stdin()
    .read_line(&mut pin)
    .expect("Failed to read line");
let pin = pin.trim();


if pin.len() > 5 {
    println!("The PIN You Are Creating Cannot Be More Than Or less than 5 Digit.Please Try Again.");
} else if !pin.chars().all(|c| c.is_digit(10)){
    println!("Please Your Pin Must Contain Only Digit,Try Again.");
} else{
    return pin.to_string();
}
};
}
fn check_pin(inputed_pin: &str) -> bool{
    let mut entered_pin = String::new();
    println!("Please input your pin you have used to register to gain access to your tasks record.");
    println!("Please  Input Your Pin Here:");
    io::stdout().flush().unwrap();
    io::stdin()
    .read_line(&mut entered_pin)
    .expect("Failed to read line.");
entered_pin.trim() == inputed_pin
}

fn main(){
    println!("WELCOME TO EXCEL TO DO LIST APPLICATION!");
    let mut name = String::new();
    println!("Please Input Your Name:");
    io::stdout().flush().unwrap();
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line.");
println!("Please we encourage you to create a pin for security purposes (Please not more than 5 digit).");

let _pin = create_pin();
println!("Hi,{}You Have Successfully Created A Pin And Please Don't Forget Your Pin Because Without Your Pin You Cannot Gain Access Your Task Maneger.",name);
let mut tasks: Vec<Task> = Vec::new();

loop{
    println!("Please select any of the option listed below:");

    println!("1.Create or Add Task.");
    println!("2.View Your Task List.");
    println!("3.Mark Your Task as Completed.");
    println!("4.Delete Task.");
    println!("5.Exist.");
    let mut option = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut option).unwrap();
    let option:u32 = match option.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please What You Choose Is Not Included In The Option Above, We Suggest You Enter Any Of The Number Above. ");
            continue;
        }
    };

    match option {
        1 => {
            let mut date = String::new();
            println!("Please Input a Specific Date For Your Task:");
            io::stdin().read_line(&mut date).expect("Failed to Read line.");
            let date = date.trim().to_string();

            let mut time = String::new();
            println!("Please Input Time For Your Task:");
            io::stdin().read_line(&mut time).expect("Failed to Read line.");
            let time = time.trim().to_string();
            
            let mut task_name = String::new();
            println!("Please Enter Task Name: ");
            io::stdin().read_line(&mut task_name).unwrap();
            let task_name = task_name.trim().to_string();

            adding_new_task(&mut tasks, date, time,task_name);
            println!("Your New Task Have Been Added Successfuly!");
        }
        2 => task_list(&tasks),
        3 =>{
            let mut task_id = String::new();
            println!("Please input the task Id you want to mark as completed:");
            io::stdin().read_line(&mut task_id).unwrap();
            let task_id:u32 = task_id.trim().parse().unwrap_or(0);

            task_completed(&mut tasks, task_id);
        }
        4 =>{   
            let mut task_id = String::new();
            println!("If You Want To Delete Any Task Please Input The Task Id:");
            io::stdin().read_line(&mut task_id).unwrap();
            let task_id:u32 = task_id.trim().parse().unwrap_or(0);

            removing_task(&mut tasks, task_id);
        }

        5 =>{
            println!("{} You Have Successfully Exist The Task Manager.",name);
            break;
        }
        _=> println!("You Have Made An Invalid Option. Please Try Again."),
    }
}
}
    

