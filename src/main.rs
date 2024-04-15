use std::{process::Command, str};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
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
    println!("  begin       - Start a new job session");
    println!("  end         - End the current job session");
    println!("  task <name> - Add a new task to the current job session");
    println!("  status      - Show the current job session status");
    println!("  git         - Extract tasks from git commits");
}

fn version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("Jobclock v{}", version);
}

#[cfg(test)]
fn persistent_folder() -> std::path::PathBuf {
    let mut path = std::path::PathBuf::new();
    path.push("tmp");
    path
}

#[cfg(not(test))]
fn persistent_folder() -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    path.push("jobclock");
    path
}

fn persistent_file() -> std::path::PathBuf {
    let mut path = persistent_folder();
    path.push("session.json");
    path
}

struct Commit {
    date: chrono::DateTime<chrono::Local>,
    title: String,
}

fn get_commits() -> Vec<Commit> {
    match Command::new("git").args(["log"]).output() {
        Ok(output) => {
            if output.status.success() {
                let mut commits = vec![];

                let mut parts = str::from_utf8(&output.stdout)
                    .unwrap_or("")
                    .split("\n\n")
                    .peekable();

                while parts.peek().is_some() {
                    // commit header: "commit <hash>\nAuthor: <author>\nDate: <date>\n\n<title>"

                    let header = parts.next().unwrap().split('\n').collect::<Vec<&str>>();
                    let date = header[2]
                        .to_string()
                        .replace("Date: ", "")
                        .trim()
                        .to_string();
                    let title = parts.next().unwrap().to_string().trim().to_string();
                    let date = date.split(' ').skip(1).collect::<Vec<&str>>().join(" ");

                    let date = chrono::DateTime::parse_from_str(&date, "%b %d %H:%M:%S %Y %z")
                        .unwrap()
                        .into();

                    commits.push(Commit { date, title });
                }

                commits
            } else {
                eprintln!("There was an error!\n");

                eprintln!(
                    "Git command failed with error: {}",
                    str::from_utf8(&output.stderr).unwrap_or("Unknown error")
                );
                Vec::new()
            }
        }
        Err(e) => {
            eprintln!("Failed to execute git command: {}", e);
            Vec::new()
        }
    }
}

fn get_commit_titles_since(start_date: chrono::DateTime<chrono::Local>) -> Vec<Task> {
    let commits = get_commits();
    let mut tasks = vec![];

    for commit in commits {
        if commit.date > start_date {
            tasks.push(Task {
                name: commit.title,
                created_at: commit.date,
            });
        }
    }

    tasks
}

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

    fn get_tasks_clone_sorted(&self) -> Vec<Task> {
        let mut tasks = self.tasks.clone();
        tasks.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        tasks
    }

    fn begin(&mut self) {
        if self.working {
            println!("Job session already started");
        } else {
            println!("Job session started");
            self.start_time = chrono::Local::now();
            self.tasks.clear();
            self.working = true;
        }
    }

    fn end(&mut self) {
        if self.working {
            println!("Job session ended");
            println!("Timeline:");
            println!(
                "  {} - Begin job session",
                chrono::Local::now().format("%d-%m-%Y %H:%M:%S")
            );

            let tasks = self.get_tasks_clone_sorted();

            for task in tasks {
                println!(
                    "  {} - Task: {}",
                    task.created_at.format("%d-%m-%Y %H:%M:%S"),
                    task.name
                );
            }

            let end_time = chrono::Local::now();
            println!(
                "  {} - End job session",
                end_time.format("%d-%m-%Y %H:%M:%S")
            );

            let duration = end_time - self.start_time;
            let total_seconds = duration.num_seconds();
            let hours = total_seconds / 3600;
            let minutes = (total_seconds % 3600) / 60;
            let seconds = total_seconds % 60;
            println!("Total time: {}h {}m {}s", hours, minutes, seconds);

            let task_summary = self
                .tasks
                .iter()
                .map(|task| task.name.as_str())
                .collect::<Vec<&str>>()
                .join(". ");
            if task_summary.is_empty() {
                println!("No tasks added");
            } else {
                println!("\nSummary:\n{}", task_summary + ".");
            }

            let hours = total_seconds as f64 / 3600.0;
            println!("Hours: {:.2}", hours);

            self.working = false;
            self.tasks = vec![];
        } else {
            println!("No job session to end");
        }
    }

    fn task(&mut self, name: &str) {
        if !self.working {
            println!("No job session started");
        } else {
            if name.is_empty() {
                println!("Task name is required");
                return;
            }
            let task = Task {
                name: name.to_string(),
                created_at: chrono::Local::now(),
            };
            self.add_task(task);
            println!("Task '{}' added to job session", name);
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn save(&self) {
        if !persistent_folder().exists() {
            std::fs::create_dir_all(persistent_folder()).unwrap();
        }
        let data = serde_json::to_string(&self).unwrap();
        std::fs::write(persistent_file(), data).unwrap();
    }

    fn load() -> Session {
        let data = std::fs::read_to_string(persistent_file()).unwrap();
        serde_json::from_str(&data).unwrap()
    }

    fn status(&self) {
        if self.working {
            println!(
                "Job session started at {}",
                self.start_time.format("%d-%m-%Y %H:%M:%S")
            );

            let tasks = self.get_tasks_clone_sorted();

            println!("Tasks:");
            if tasks.is_empty() {
                println!("  No tasks added");
            }
            for task in tasks {
                println!(
                    "  {} - {}",
                    task.created_at.format("%d-%m-%Y %H:%M:%S"),
                    task.name
                );
            }

            let duration = chrono::Local::now() - self.start_time;
            let total_seconds = duration.num_seconds();
            let hours = total_seconds / 3600;
            let minutes = (total_seconds % 3600) / 60;
            let seconds = total_seconds % 60;
            println!("Total time: {}h {}m {}s", hours, minutes, seconds);
        } else {
            println!("No job session started");
        }
    }

    fn extract_from_git(&mut self) {
        if !self.working {
            println!("No job session started");
            return;
        }

        let start_time = self.start_time;
        let commit_titles = get_commit_titles_since(start_time);
        for task in commit_titles.iter() {
            self.add_task((*task).clone());
        }

        println!("Extracted {} tasks from git commits", commit_titles.len());
    }
}

fn main() {
    let mut session = Session::new();
    if persistent_file().exists() {
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
        "task" => {
            session.task(args.as_str());
        }
        "help" => {
            usage();
            return;
        }
        "version" => {
            version();
            return;
        }
        "status" => {
            session.status();
            return;
        }
        "git" => {
            session.extract_from_git();
        }
        _ => {
            println!("ERROR: Invalid command entered: {}", args);
            usage();
            return;
        }
    }

    session.save();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_begin() {
        let mut session = Session::new();
        if persistent_file().exists() {
            session = Session::load();
        } else {
            session.save();
        }
        session.begin();
        assert_eq!(session.working, true);
        session.task("Test");

        for task in &session.tasks {
            assert_eq!(task.name, "Test");
        }

        session.end();
        assert_eq!(session.working, false);
        assert_eq!(session.tasks.len(), 0);
    }

    #[test]
    fn test_get_commits() {
        let commits = get_commits();
        println!("Commits {:?}", commits.len());
    }
}
