# JobClock

JobClock is a CLI (Command Line Interface) tool developed in Rust designed for tracking time spent on various tasks. It offers a simple yet effective way to manage work sessions, allowing users to start a session, end it, and log tasks with their names during the session. This tool is especially useful for freelancers, developers, and anyone looking to measure the time dedicated to different projects or tasks.

## Features

- **Start a Session:** Begin tracking time with a single command.
- **End a Session:** Conclude the session and get a summary of all tasks completed along with the total time spent.
- **Task Logging:** Add specific tasks to your session to track how much time you spend on each.
- **Git Integration:** Collect all Git commit messages made during the session with a simple command.

## Installation

To install JobClock, ensure you have Rust and Cargo installed on your system. Then, run the following command:

```bash
cargo install jobclock
```

## Usage

### Starting a Session

To begin a session, use the `start` command. This command initiates the tracking period.

```console
jobclock start
```

**Note:** Once a session has started, you cannot start another session until the current one has ended.

### Adding a Task

To add a task to your current session, use the `task` command followed by the task name.

```console
jobclock task <name>
```

Replace `<name>` with the actual name of your task.

### Collecting Git Commit Messages

To collect all Git commit messages made during the current session, use the `git` command.

```console
jobclock git
```

This command will log all commit messages to your session summary.

### Ending a Session

To end the current session and receive a summary of all tasks, commit messages, and the total time spent, use the `end` command.

```console
jobclock end
```

Upon ending a session, JobClock will provide an output similar to the following:

```console
Job ended
Timeline:
  13-03-2024 20:00:00 - Begin session
  13-03-2024 20:45:00 - Job: Add frontend feature
  13-03-2024 21:00:00 - End session
Total time: 1h 5m 0s
Hours: 1.08
```

## License
[LICENSE](LICENSE)