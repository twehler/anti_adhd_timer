use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use notify_rust;
use std::{thread, time};
use std::process::Command;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::env;


fn set_xsct(temp: &str) {
    Command::new("xsct")
    .arg(temp)
    .spawn()
    .expect("Failed to execute xsct. Is it installed?");
}


// Function calculates duration between NaiveTime objects while wrapping around midnight
fn calculate_time_duration(start: NaiveTime, end: NaiveTime) -> chrono::Duration {
    let today = NaiveDate::from_ymd_opt(2026, 2, 9).unwrap(); // example fixed date

    let start_dt = NaiveDateTime::new(today, start);
    let mut end_dt = NaiveDateTime::new(today, end);

    // If end time is before start time, assume it is next day
    if end < start {
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



fn force_pomodoro_report(report_file_path: &str) {
    // esure the file exists (and close the handle immediately to avoid locking)
    {
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&report_file_path)
            .expect("Failed to create or open the pomodoro report file.");
    }

    println!("Waiting for Pomodoro Report... Please save and close the editor to continue.");

    // check if the user has Windows, Linux or MacOS (for future porting)
    let status = if cfg!(target_os = "windows") {
        Command::new("notepad.exe").arg(&report_file_path).status()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg("-a").arg("TextEdit").arg(&report_file_path).status()
    } else {
        // Use xdg-open for better Linux compatibility, or stick to mousepad
        let mut child = Command::new("mousepad")
            .arg(&report_file_path)
            .spawn()
            .expect("Cannot start Mousepad. Is it installed?");

        // Wait a split second for the window to appear, then force it on top
        thread::sleep(time::Duration::from_millis(500));
        println!("Report file location: {}", &report_file_path);

        // Get the PID of the mousepad process and find its window ID via wmctrl
        let pid = child.id(); // returns the PID as u32

        // wmctrl -l -p lists all windows with their PIDs
        let wmctrl_output = Command::new("wmctrl")
            .args(["-l", "-p"])
            .output()
            .expect("Cannot run wmctrl.");

        let output_str = String::from_utf8_lossy(&wmctrl_output.stdout);

        // Find the line whose PID matches our mousepad process
        let window_id = output_str.lines()
            .find(|line| line.contains(&pid.to_string()))
            .and_then(|line| line.split_whitespace().next())
            .expect("Could not find mousepad window ID.");

        // raise exactly that window by ID using -i
        Command::new("wmctrl")
            .args(["-i", "-r", window_id, "-b", "add,above"])
            .status()
            .expect("wmctrl failed.");

        // wait for the editor to close
        child.wait()

    };

    // handle the editor exit status
    match status {
        Ok(s) if s.success() => println!("Report saved. Break starts now."),
        Ok(_) => eprintln!("Editor closed without a success code, but continuing..."),
        Err(e) => eprintln!("Failed to open editor: {}", e),
    }
}



fn main() {

    // passing schedule file path as an argument
    let schedule_file_path: String = env::args().nth(1).expect("Please provide a schedule file path as an argument.");

    // Setting bedtime (24-hour format: hour, Minute)
    println!("Please enter your desired bed-time below in format HH:MM.");
    let bedtime = time_from_input();
    println!("Bedtime timer started! I'll remind you at {}.", bedtime);


    struct Task {
        name: String,
        beginning: chrono::NaiveTime,
        end: chrono::NaiveTime,
    }


    let schedule_file_path = schedule_file_path.trim();

    println!("Trying to open {}", schedule_file_path);

    let schedule_file = File::open(schedule_file_path).expect("File not found.");
    let reader = io::BufReader::new(schedule_file);

    println!("Using {} as a schedule today. Try to stay on time! For seeing your own progress, a text-editor will open after every pomodoro in an automatically created logfile, so you can log your activities. Go smash the day!", schedule_file_path);

    let mut tasks: Vec<Task> = vec![];
    let mut task_names: Vec<String> = vec![];

    for line in reader.lines() {
        // Unwrap the Result to get the actual String
        let line_content = line.expect("Could not read line from file");
        let line_content = line_content.trim();

        // Skip empty lines to prevent crashes
        if line_content.is_empty() { continue; }

        let line_parts: Vec<&str> = line_content.split('-').collect();

        // Safety check: ensure the line has 3 parts (Start-End-Name)
        if line_parts.len() < 3 {
            panic!("Line could not be parsed: {}", line_content);
        }

        // Parse the name
        let task_name = line_parts[2].to_string();

        // Split and parse the times
        let beginning_parts: Vec<&str> = line_parts[0].trim().split(':').collect();
        let end_parts: Vec<&str> = line_parts[1].trim().split(':').collect();

        let b_hour: u32 = beginning_parts[0].parse().expect("Invalid hour");
        let b_min: u32 = beginning_parts[1].parse().expect("Invalid minute");
        let e_hour: u32 = end_parts[0].parse().expect("Invalid hour");
        let e_min: u32 = end_parts[1].parse().expect("Invalid minute");

        let task_beginning = NaiveTime::from_hms_opt(b_hour, b_min, 0).unwrap();
        let task_end = NaiveTime::from_hms_opt(e_hour, e_min, 0).unwrap();

        let current_task = Task {
            name: task_name.clone(),
            beginning: task_beginning,
            end: task_end,
        };

        task_names.push(task_name);
        tasks.push(current_task);
    }


    println!("Your day-plan is now being tracked. Tasks for today: {:?}", task_names);


    // creating various flags
    let mut key_time_1_over = false;
    let mut key_time_2_over = false;
    let mut key_time_3_over = false;
    let mut bedtime_reminder_sent = false;

    // vector stores procedurally generated flags for individual tasks
    let mut task_beginning_reminder_sent: Vec<bool> = vec![false; tasks.len()];
    let mut task_end_reminder_sent: Vec<bool> = vec![false; tasks.len()];

    let mut pomodoro_start_reminder_sent = false;
    let mut pomodoro_pause_reminder_sent = false;

    // numeric flags
    let mut pomodoro_count: u16 = 1;
    let mut total_pomodoro_count: u16 = 0;
    let mut current_screen_temp = "6500";

    let mut now = chrono::Local::now().time();

    let mut pomodoro_start = now; // starting time of a pomodoro-unit
    let pomodoro_duration = 25;
    let mut pomodoro_end = pomodoro_start + chrono::Duration::minutes(pomodoro_duration);
    let small_break_duration = 5;
    let long_break_duration = 10;


    // Creating path for Pomodoro-report-file
    let pomodoro_report_path = schedule_file_path.replace(".txt", ".log");


    ///////////////////// entering main-loop //////////////////////

    loop {
        now = chrono::Local::now().time();


        for (task_index, t) in tasks.iter().enumerate() {

            // Shadowing "now" inside the for-loop to update to current time
            now = chrono::Local::now().time();


            // check if current task is running
            if now > t.beginning && now < t.end {

                // task beginning logic
                if !task_beginning_reminder_sent[task_index] {

                    println!("Task started: {}", t.name);
                    notify_rust::Notification::new()
                    // & because format!() marcro returns String, but summary expects slice:
                    .appname("Anti-ADHD Timer")
                    .summary(&format!("Task started: {}", t.name))
                    .icon("alarm-clock") // Standard Ubuntu icon name
                    .timeout(0)          // 0 means the notification won't disappear until clicked
                    .show()
                    .unwrap();
                    task_beginning_reminder_sent[task_index] = true;
                    task_end_reminder_sent[task_index] = false;

                    pomodoro_start = now;
                    pomodoro_start_reminder_sent = false;
                    pomodoro_end = pomodoro_start + chrono::Duration::minutes(pomodoro_duration);

                    pomodoro_count = 1;
                }


                ///// Pomodoro-Logic (25min of intense work, 5 min break afterwards)


                // Pomodoro Start
                if now >= pomodoro_start && !pomodoro_start_reminder_sent {

                    println!("Pomodoro {} of task {} has begun!", pomodoro_count, t.name);
                    notify_rust::Notification::new()
                        .appname("Anti-ADHD Timer")
                        .summary(&format!("Pomodoro {} of task {} has begun!.", pomodoro_count, t.name))
                        .body("25 minutes of focused work starting now!")
                        .icon("alarm-clock") // Standard Ubuntu icon name
                        .timeout(0)          // 0 means the notification won't disappear until clicked
                        .show()
                        .unwrap();

                    pomodoro_start_reminder_sent = true;
                    pomodoro_pause_reminder_sent = false;
                }


                // if pomodoro is over, take small break, reset pomodoro_start
                // only execute between the long breaks, which occur every 3 times
                if now > pomodoro_end && pomodoro_count % 3 != 0 && !pomodoro_pause_reminder_sent {
                    println!("Pomodoro over! 5 minutes of pause starting now.");
                    notify_rust::Notification::new()
                        .appname("Anti-ADHD Timer")
                        .summary(&format!("Pomodoro over! 5 minutes of pause starting now."))
                        .body("Move a little bit, get some water...")
                        .icon("alarm-clock") // Standard Ubuntu icon name
                        .timeout(0)          // 0 means the notification won't disappear until clicked
                        .show()
                        .unwrap();

                    // Let the screen blink in blue for a short time
                    for _ in 0..3 {
                        set_xsct("12000");
                        thread::sleep(time::Duration::from_millis(200));
                        set_xsct(current_screen_temp);
                        thread::sleep(time::Duration::from_millis(200));
                    }


                    // resetting pomodoro start
                    pomodoro_start = now + chrono::Duration::minutes(small_break_duration);

                    // resetting flags
                    pomodoro_pause_reminder_sent = true;
                    pomodoro_start_reminder_sent = false;

                    pomodoro_count = pomodoro_count + 1;
                    total_pomodoro_count = total_pomodoro_count + 1;

                    force_pomodoro_report(&pomodoro_report_path);
                }


                // every 3 pomodoros, take a long break
               if now > pomodoro_end && pomodoro_count % 3 == 0 && !pomodoro_pause_reminder_sent {
                    println!("Pomodoro over! Long break of 10 minutes starting now!");
                    notify_rust::Notification::new()
                        .appname("Anti-ADHD Timer")
                        .summary(&format!("Pomodoro over! Long break of 10 minutes starting now!."))
                        .body("Move a little more, hydrate or meditate for a short time.")
                        .icon("alarm-clock") // Standard Ubuntu icon name
                        .timeout(0)          // 0 means the notification won't disappear until clicked
                        .show()
                        .unwrap();

                    for _ in 0..3 {
                        set_xsct("12000");
                        thread::sleep(time::Duration::from_millis(200));
                        set_xsct(current_screen_temp);
                        thread::sleep(time::Duration::from_millis(200));
                    }

                    // resetting pomodoro start
                    pomodoro_start = now + chrono::Duration::minutes(long_break_duration);

                    // resetting flags
                    pomodoro_pause_reminder_sent = true;
                    pomodoro_start_reminder_sent = false;

                    pomodoro_count = pomodoro_count + 1;
                    total_pomodoro_count = total_pomodoro_count + 1;

                    force_pomodoro_report(&pomodoro_report_path);
                }

                // show how much time has elapsed until the end of the pomodoro
                // only show the same time once (instead of repeating the same amount of minutes every time the loop checks)
                if now < pomodoro_end && pomodoro_pause_reminder_sent == false {
                    let pomodoro_time_elapsed = calculate_time_duration(pomodoro_start, now).num_minutes();
                    println!("Pomodoro Status: {} minutes of {} elapsed.", pomodoro_time_elapsed, pomodoro_duration);
                }

            }



            ///// Task end logic (if current time is outside of task time interval)

            if now >= t.end && !task_end_reminder_sent[task_index] {

                // letting the screen blink in red for a short time
                for _ in 0..4 {
                        set_xsct("1300");
                        thread::sleep(time::Duration::from_millis(200));
                        set_xsct(current_screen_temp);
                        thread::sleep(time::Duration::from_millis(200));
                    }

                let next_task_option = tasks.get(task_index + 1);

                // at the end of each task, make a longer break
                // if there is a next task, announce it to the user
                // if  not, abandon the task logic
                // get() safely accesses the next task, to prevent overflow
                match next_task_option {

                    Some(next) => {

                        println!("Task {} has ended!", t.name);
                        notify_rust::Notification::new()
                        .appname("Anti-ADHD Timer")
                        .summary(&format!("Time for task {} is over. Prepare for next task: {}", t.name, next.name))
                        .body(&format!("The next task will begin at {}.", next.beginning))
                        .icon("alarm-clock") // Standard Ubuntu icon name
                        .timeout(0)          // 0 means the notification won't disappear until clicked
                        .show()
                        .unwrap();
                    },

                    None => {
                        println!("Task {} has ended! No more tasks scheduled for today.", t.name);
                        notify_rust::Notification::new()
                        .appname("Anti-ADHD Timer")
                        .summary(&format!("Time for task {} is over. No more tasks today!", t.name))
                        .body(&format!("Number of today's pomodoros: {}", total_pomodoro_count))
                        .icon("alarm-clock") // Standard Ubuntu icon name
                        .timeout(0)          // 0 means the notification won't disappear until clicked
                        .show()
                        .unwrap();
                    }
                }

                task_end_reminder_sent[task_index] = true;
                task_beginning_reminder_sent[task_index] = false;

                force_pomodoro_report(&pomodoro_report_path);
            }


            // reset flags
            if now < t.beginning {
                task_beginning_reminder_sent[task_index] = false;
            }

            if now < t.end {
                task_end_reminder_sent[task_index] = false;
            }

        } // end of for-loop (task-logic)




        /////////////// Bedtime-logic ///////////////

        let duration_until_bedtime = calculate_time_duration(now, bedtime);
        let duration_until_bedtime = duration_until_bedtime.num_minutes();

        // only execute if flag is false
        if duration_until_bedtime <= 180 && duration_until_bedtime > 150 && !key_time_1_over {
            key_time_1_over = true;
            current_screen_temp = "4000";
            set_xsct(current_screen_temp);
        }

        if duration_until_bedtime <= 150 && duration_until_bedtime > 120 && !key_time_2_over {

            key_time_2_over = true;
            current_screen_temp = "3000";
            set_xsct(current_screen_temp);
        }

        if duration_until_bedtime <= 120 && !key_time_3_over {

            key_time_3_over = true;
            current_screen_temp = "2000";
            set_xsct(current_screen_temp);
        }

        // Sending reminder
        if duration_until_bedtime <=60 && !bedtime_reminder_sent {
            notify_rust::Notification::new()
                .appname("Anti-ADHD Timer")
                .summary("Bedtime-Reminder")
                .body("Bedtime is in 1 hour!")
                .icon("alarm-clock") // Standard Ubuntu icon name
                .timeout(0)          // 0 means the notification won't disappear until clicked
                .show()
                .unwrap();
            bedtime_reminder_sent = true;
        }

        if now >= bedtime {
            notify_rust::Notification::new()
                .appname("Anti-ADHD Timer")
                .summary("Go to sleep! Your tomorrow-self will thank you.")
                .icon("alarm-clock") // Standard Ubuntu icon name
                .timeout(0)          // 0 means the notification won't disappear until clicked
                .show()
                .unwrap();

            set_xsct("1000");
            break; // Exit the program after the notification
        }

        // save CPU time
        thread::sleep(std::time::Duration::from_secs(20));
    }
}
