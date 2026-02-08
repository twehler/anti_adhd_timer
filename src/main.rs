use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use notify_rust;
use std::{thread, io};
use std::process::Command;


fn set_xsct(temp: &str) {
    Command::new("xsct")
    .arg(temp)
    .spawn()
    .expect("Failed to execute xsct. Is it installed?");
}

// Function subtracts NaiveTime objects while wrapping around midnight
fn calculate_time_duration(start: NaiveTime, end: NaiveTime) -> chrono::Duration {
    let today = NaiveDate::from_ymd_opt(2026, 2, 6).unwrap(); // example fixed date

    let start_dt = NaiveDateTime::new(today, start);
    let mut end_dt = NaiveDateTime::new(today, end);

    // If end time is before start time, assume it is next day
    if end <= start {
        end_dt = end_dt + chrono::Duration::days(1);
    }

    end_dt - start_dt
}


// function promts a user-input and returns a NaiveTime object
fn time_from_input() -> NaiveTime {
    let mut raw_input = String::new();
    io::stdin()
        .read_line(&mut raw_input)
        .expect("Failed to read line. Input has to be in format: HH:MM");

    let input = raw_input.trim();
    let parts: Vec<&str> = input.split(':').collect();
    let hour: u32 = parts[0].parse().expect("Hour is not a number!");
    let minute: u32 = parts[1].parse().expect("Minute is not a number!");

    if parts.len() != 2 {
        panic!("Input must be in format HH:MM");
    }

    NaiveTime::from_hms_opt(hour, minute, 0).unwrap()
}




fn main() {
    // Setting bedtime (24-hour format: hour, Minute)

    println!("Please enter your desired bed-time below in format HH:MM.");

    let bedtime = time_from_input();

    println!("Bedtime timer started! I'll remind you at {}.", bedtime);

    struct Task {
        name: String,
        beginning: chrono::NaiveTime,
        end: chrono::NaiveTime,
    }

    println!("What is the name of a task that you want to accomplish today? Write its name below in alphabet letters and underscores. Type <skip> to end input");

    // the first task is added to the 'tasks' vector as a starting value
    let mut task_name_input = String::new();

    io::stdin()
        .read_line(&mut task_name_input)
        .expect("Failed to read line. Only alphabet letters and underscores are allowed.");

    let task_name_input = task_name_input.trim();

    println!("What should be its beginning time? Enter in format HH:MM");
    let task_beginning_time = time_from_input();

    println!("What should be its end time? Enter in format HH:MM");
    let task_end_time = time_from_input();

    let first_task = Task {
        name: task_name_input.to_string(),
        beginning: task_beginning_time,
        end: task_end_time,
    };

    // putting first task into a vector (appending it later with other tasks)
    let mut tasks: Vec<Task> = vec![first_task];

    let mut input_finished = false;

    // entering loop
    while !input_finished {

        println!("What would be the another task? Write its name below in alphabet letters and underscores. Type <finish> to end input.");

        let mut task_name_input = String::new();

        // reading input & removing white spaces
        io::stdin()
            .read_line(&mut task_name_input)
            .expect("Failed to read line. Only alphabet letters and underscores are allowed.");
        let task_name_input = task_name_input.trim();

        if task_name_input == "finish" {
            input_finished = true;
            break;
        }
        else {

        println!("What should be its beginning time? Enter in format HH:MM");
        let task_beginning_time = time_from_input();

        println!("What should be its end time? Enter in format HH:MM");
        let task_end_time = time_from_input();

        let current_task = Task {
            name: task_name_input.to_string(), // convert the slice task_name_input to a String
            beginning: task_beginning_time,
            end: task_end_time,
        };

        tasks.push(current_task);
        }}





    // Creating flags to avoid flickering bug
    let mut key_time_1_over = false;
    let mut key_time_2_over = false;
    let mut key_time_3_over = false;
    let mut reminder_sent = false;

    // main-loop
    loop {
        let now = chrono::Local::now().time();

        let mut task_index = 0;

        for (task_index, t) in tasks.iter().enumerate() {

            if now >= t.beginning {
                println!("Task {} started!", t.name);

                notify_rust::Notification::new()
                .summary(&format!("Stop everything! It's time to do this task now: {}", t.name))
                .body("Bedtime is in 1 hour!")
                .icon("alarm-clock") // Standard Ubuntu icon name
                .timeout(0)          // 0 means the notification won't disappear until clicked
                .show()
                .unwrap();
            }

            let next_task_option = tasks.get(task_index + 1);

            if now >= t.end {
                
                match next_task_option {

                println!("Task {} has ended!", t.name);
                notify_rust::Notification::new()
                .summary(&format!("Time for task {} is over. The next task will be {} and it  will begin at {}.", 
                    t.name,
                    next_task.name,
                    next_task.beginning
                    ))
                .body("Bedtime is in 1 hour!")
                .icon("alarm-clock") // Standard Ubuntu icon name
                .timeout(0)          // 0 means the notification won't disappear until clicked
                .show()
                .unwrap();
            }

        task_index += 1;
        }














        // "Shadowing" feature in Rust
        let duration_until_bedtime = calculate_time_duration(bedtime, now);
        let duration_until_bedtime = duration_until_bedtime.num_seconds();


        // only execute if flag is false
        if duration_until_bedtime <= 12 && duration_until_bedtime > 9 && !key_time_1_over {
            set_xsct("4000");
            key_time_1_over = true;
        }

        if duration_until_bedtime <= 9 && duration_until_bedtime > 6 && !key_time_2_over {
            set_xsct("3000");
            key_time_2_over = true;
        }

        if duration_until_bedtime <= 6 && !key_time_3_over {
            set_xsct("2000");
            key_time_3_over = true;
        }

        // Sending reminder
        if duration_until_bedtime <=3 && !reminder_sent {
            notify_rust::Notification::new()
                .summary("Bedtime-Reminder")
                .body("Bedtime is in 1 hour!")
                .icon("alarm-clock") // Standard Ubuntu icon name
                .timeout(0)          // 0 means the notification won't disappear until clicked
                .show()
                .unwrap();
            reminder_sent = true;
        }

        if now >= bedtime {
            notify_rust::Notification::new()
                .summary("Bed time!")
                .body("Go to sleep! Your tomorrow-self will thank you.")
                .icon("alarm-clock") // Standard Ubuntu icon name
                .timeout(0)          // 0 means the notification won't disappear until clicked
                .show()
                .unwrap();

            set_xsct("1000");
            break; // Exit the program after the notification
        }

        // 3. Wait a bit before checking again to save CPU
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
