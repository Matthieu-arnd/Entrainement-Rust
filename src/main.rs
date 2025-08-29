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
    mission = Task::new(String::from("tache1"), TaskStatus::ToDo, 1, date);
    println!("{:#?}",mission);

}
