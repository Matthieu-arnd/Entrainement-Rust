use std::string;
use std::io;

#[derive(Debug)]
enum TaskStatus {
    ToDo,
    InProgress,
    Done,
} 
#[derive(Debug)]
struct Date{
    day:i32,
    month: i32,
    year: i32,
}
#[derive(Debug)]
struct Task{
    name: String,
    status: TaskStatus,
    priority: i32,
    //description: String, //à faire 
    date: Date,
}
impl Date {
    fn init() -> Self{
        Self { day: 15, month: 09, year: 2025 }
    }
    fn new(day:i32, month:i32,year:i32) -> Self
    {
        Self { day: day, month: month, year: year }
    }
}

impl Task {
    fn init() -> Self {
        Self {
            name : String::from("Task 0"),
            status : TaskStatus::ToDo,
            priority :0,
            date : Date::init(),
        }
    }

    fn new(name:String, status:TaskStatus, priority:i32, date:Date) -> Self{
        Self { name : name.to_string(), 
               status:status, 
               priority: priority, 
               date: date, 
            }
    }
}

fn main() {
    let mut mission  = Task::init();
    let date = Date::init();
    print!("Entrer le nom de la tâche\r\n");
    let mut name = String::new();
    let mut priority = String::new();
    let mut status = String::new();
    let mut enum_status = TaskStatus::ToDo;
    io::stdin().read_line(&mut name).expect("Erreur de lecture");
    print!("Entrer la prioritée\r\n");
    io::stdin().read_line(&mut priority).expect("Erreur de lecture");
    print!("Entrer le status\r\n");
    io::stdin().read_line(&mut status).expect("Erreur de lecture");
 
    match status.trim(){
        "ToDo" => enum_status=TaskStatus::ToDo,
        "InProgress" => enum_status=TaskStatus::InProgress,
        "Done" => enum_status=TaskStatus::Done ,
        _ => println!("Status incorrect") ,
    }
    mission = Task::new(name.trim().to_string(), enum_status, priority.trim().parse().unwrap(), date);
    println!("{:#?}",mission);

}
