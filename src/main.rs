#[derive(serde::Serialize, serde::Deserialize)]
struct Task {
    name: String,
    created_at: chrono::DateTime<chrono::Local>,
}

fn usage() {
    println!(
        "Usage: {} <subcommand> [args]",
        std::env::args().nth(0).unwrap()
    );
    println!("Subcommands: ");
    println!("  begin - Start a new job session");
    println!("  end - End the current job session");
    println!("  task <name> - Add a new task to the current job session");
}

const PERSISTENT_FILE: &str = "session.json";

#[derive(serde::Serialize, serde::Deserialize)]
struct Session {
    tasks: Vec<Task>,
    start_time: chrono::DateTime<chrono::Local>,
    working: bool,
}

impl Session {
    fn new() -> Session {
        Session {
            tasks: vec![],
            start_time: chrono::Local::now(),
            working: false,
        }
    }

    fn begin(&mut self) {
        if self.working {
            println!("Job already started");
        } else {
            println!("Job started");
            self.start_time = chrono::Local::now();
            self.working = true;
        }
    }

    fn end(&mut self) {
        if self.working {
            println!("Job ended");
            println!("Timeline:");
            println!(
                "  {} - Begin session",
                chrono::Local::now().format("%d-%m-%Y %H:%M:%S")
            );

            for task in &self.tasks {
                println!(
                    "  {} - Job: {}",
                    task.created_at.format("%d-%m-%Y %H:%M:%S"),
                    task.name
                );
            }

            let end_time = chrono::Local::now();
            println!("  {} - End session", end_time.format("%d-%m-%Y %H:%M:%S"));

            let duration = end_time - self.start_time;
            println!(
                "Total time: {}h {}m {}s",
                duration.num_hours(),
                duration.num_minutes(),
                duration.num_seconds()
            );

            let hours = duration.num_hours()
                + (duration.num_minutes() / 60)
                + (duration.num_seconds() / 3600);
            println!("Hours: {}", hours);

            self.working = false;
            self.tasks = vec![];
        } else {
            println!("No job to end");
        }
    }

    fn job(&mut self, name: &str) {
        if !self.working {
            println!("No job started");
        } else {
            if name.is_empty() {
                println!("Job name is required");
                return;
            }
            let task = Task {
                name: name.to_string(),
                created_at: chrono::Local::now(),
            };
            self.tasks.push(task);
            println!("Job added");
        }
    }

    fn save(&self) {
        let data = serde_json::to_string(&self).unwrap();
        std::fs::write(PERSISTENT_FILE, data).unwrap();
    }

    fn load() -> Session {
        let data = std::fs::read_to_string(PERSISTENT_FILE).unwrap();
        serde_json::from_str(&data).unwrap()
    }
}

fn main() {
    let mut session = Session::new();
    if std::path::Path::new(PERSISTENT_FILE).exists() {
        session = Session::load();
    } else {
        session.save();
    }

    let subcommand = std::env::args().nth(1).unwrap_or("".to_string());

    if subcommand.is_empty() {
        println!("ERROR: No subcommand found");
        usage();
        return;
    }

    let args = std::env::args().skip(2).collect::<Vec<String>>().join(" ");

    match subcommand.as_str() {
        "begin" => {
            session.begin();
        }
        "end" => {
            session.end();
        }
        "job" => {
            session.job(args.as_str());
        }
        "help" => {
            usage();
            return;
        }
        _ => {
            println!("ERROR: Invalid command entered: {}", args);
            usage();
            return;
        }
    }

    session.save();
}
