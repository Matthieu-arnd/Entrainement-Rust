use std::string;

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
}

fn main() {
    let mission  = Task::init();
    println!("{:#?}",mission);

}
