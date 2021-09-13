use std::fs;
use std::io;
use std::fmt;
use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use chrono::{prelude::*, offset::TimeZone, DateTime, NaiveDateTime, Local};
use regex::Regex;

// Stores settings
#[derive(Debug)]
pub struct Manager {
    root_dir: PathBuf,
    tab_size: usize,
}

#[derive(Debug)]
pub struct Topic<'a> {
    manager: &'a Manager,
    folder_path: PathBuf,
    file_path: Option<PathBuf>,
    name: String,
    tasks: Vec<Task>,
    subtopics: Vec<Topic<'a>>,
}

#[derive(Debug)]
pub struct Task {
    done: bool,
    name: String,
    due: Due,
    special: Option<String>,
    description: Option<String>,
    subtasks: Vec<Task>,
}

#[derive(Debug)]
pub enum Due {
    Never,
    Day(DateTime<Local>),
    Interval(DateTime<Local>, DateTime<Local>),
}

impl PartialEq for Due {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Due::Never, Due::Never) => true,
            (Due::Day(a), Due::Day(b)) => a == b,
            (Due::Interval(a, c), Due::Interval(b, d)) => a == b && c == d,
            _ => false,
        }
    }
}

impl Eq for Due {
    
}

impl PartialOrd for Due {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Due {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        use Due::*;

        match (self, other) {
            (Never, Never) => Equal,
            (Never, _) => Greater,
            (_, Never) => Less,
            (Due::Day(a), Due::Day(b)) => a.cmp(b),
            (Due::Day(a), Due::Interval(b, _)) => a.cmp(b),
            (Due::Interval(a, _), Due::Day(b)) => a.cmp(b),
            (Due::Interval(a, c), Due::Interval(b, d)) => a.cmp(b).then(c.cmp(d)),
        }
    }
}

fn parse_datetime(when: &str) -> Option<DateTime<Local>> {
    if let Some(dt) = NaiveDateTime::parse_from_str(when, "%d/%m/%y %H:%M").ok() {
        Some(Local.from_local_datetime(&dt).unwrap())
    }
    else if let Some(d) = NaiveDate::parse_from_str(when, "%d/%m/%y").ok() {
        Some(Local.from_local_datetime(&d.and_hms(0, 0, 0)).unwrap())
    }
    else {
        println!("'{}'", when);
        None
    }
}

fn datetime_to_str(dt: &DateTime<Local>) -> String {
    if dt.hour() != 0 || dt.minute() != 0 {
        format!("{} {:02}/{:02}/{:02} {:02}:{:02}",
            dt.weekday(),
            dt.day(), dt.month(), dt.year() % 100,
            dt.hour(), dt.minute())
    }
    else {
        format!("{} {:02}/{:02}/{:02}",
            dt.weekday(),
            dt.day(), dt.month(), dt.year() % 100)
    }
}

impl Due {
    fn parse(when: &str) -> Option<Due> {
        let parts: Vec<_> = when.split("-").collect();
        match parts.len() {
            1 => Some(Due::Day(parse_datetime(parts[0])?)),
            2 => Some(Due::Interval(parse_datetime(parts[0])?, parse_datetime(parts[1])?)),
            _ => None,
        }
    }

    pub fn active_on(&self, begin: &DateTime<Local>, end: &DateTime<Local>) -> bool {
        match self {
            Due::Never => false,
            Due::Day(dt) => begin <= dt && dt <= end,
            Due::Interval(a, b) => begin <= b && a <= end,
        }
    }   
}

impl fmt::Display for Due {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Due::Never => Ok(()),
            Due::Day(dt) => write!(f, "{}", datetime_to_str(dt)),
            Due::Interval(a, b) => write!(f, "{}-{}", datetime_to_str(a), datetime_to_str(b)),
        }
    }
}

impl Manager {
    // Loads/generates the configuration
    pub fn new(config_path: &Path) -> Result<Self, io::Error> {
        // Get directory path from config file
        let root_dir = if let Ok(c) = fs::read_to_string(config_path) {
            PathBuf::from(c)
        }
        else {
            let default = dirs::document_dir()
                .expect("Couldn't get default planifie directory, documents directory not found")
                .join("plan");
            if fs::write(config_path, default.to_str().unwrap()).is_err() {
                println!("Couldn't write default configuration file on path {}", config_path.to_str().unwrap())
            }
            default
        };

        // Create directory if necessary
        fs::create_dir_all(&root_dir)?;

        Ok(Self {
            root_dir: root_dir,
            tab_size: 2,
        })
    }

    // Loads the root topic
    pub fn load<'a>(&'a self) -> Result<Topic<'a>, io::Error> {
        Topic::load(&self, &self.root_dir).transpose().unwrap()
            .map(|mut t| { t.name = "root".to_owned(); t })
    } 
}

impl<'a> Topic<'a> {
    // Parses a topic from a markdown file
    fn parse(manager: &'a Manager, path: &Path) -> Result<Topic<'a>, io::Error> {
        let task_re = Regex::new(r"^(.+)\s*\((.*)\)(?:\s*@\s*(.*))?$").unwrap();
        
        let content = fs::read_to_string(path)?;
        let name = path.file_stem().unwrap().to_str().unwrap();
        let name = if name == "about" { path.parent().unwrap().file_name().unwrap().to_str().unwrap() } else { name };
        let mut tasks: Vec<Task> = Vec::new();
        let mut within_code_block = false;
        let mut within_task = false;

        for mut line in content.lines() {
            // Get line indentation
            let mut indentation = 0;
            while let Some(l) = line.strip_prefix(&" ".repeat(manager.tab_size)) {
                line = l;
                indentation += 1;
            }
            line = line.trim();
            
            // For escaping purposes
            for _ in line.matches("```") {
                within_code_block = !within_code_block;
            }

            if within_code_block || line.len() == 0 {
                continue;
            }

            // Parse a task
            let done = if let Some(l) = line.strip_prefix("- [ ]") {
                line = l.trim();
                within_task = true;
                false
            }
            else if let Some(l) = line.strip_prefix("- [X]") {
                line = l.trim();
                within_task = true;
                true
            }
            // Parse task's description
            else {
                let mut task = tasks.last_mut()
                    .map(|u| if indentation > 0 { Some(u) } else { None })
                    .flatten();
                if task.is_none() || !within_task {
                    within_task = false;
                }
                else {
                    let mut task = task.unwrap();

                    loop {
                        if task.subtasks.len() == 0 || indentation == 1 {
                            if let Some(d) = &task.description {
                                task.description = Some(d.to_owned() + "\n" + line);
                            }
                            else {
                                task.description = Some(line.to_owned());
                            }
                            break
                        }
                        else {
                            task = task.subtasks.last_mut().unwrap();
                            indentation -= 1;
                        }
                    }
                }
                
                continue;
            };

            // Extract name and due date
            let (name, due, special) = match task_re.captures(line) {
                Some(c) => {
                    let name = c.get(1).unwrap().as_str();
                    let when = c.get(2).unwrap().as_str();
                    let special = c.get(3).map(|s| s.as_str().to_owned());
                    let due = Due::parse(when).unwrap_or(Due::Never);
                    (name, due, special)
                },
                None => (line, Due::Never, None)
            };
            let name = name.trim().to_owned();

            let mut tasks = &mut tasks;
            loop {
                if tasks.len() == 0 || indentation == 0 {
                    if tasks.iter().find(|t| t.name == name).is_some() {
                        println!("Two tasks under the same parent can't have \
                                  the same name, ignoring the all but the \
                                  first one");
                        continue;
                    }

                    tasks.push(Task {
                        done: done,
                        name: name,
                        due: due,
                        special: special,
                        description: None,
                        subtasks: Vec::new(),
                    });
                    break
                }
                else {
                    tasks = &mut tasks.last_mut().unwrap().subtasks;
                    indentation -= 1;
                }
            }
        }
        
        Ok(Topic {
            manager: manager,
            folder_path: path.parent().unwrap().to_owned(),
            file_path: Some(path.to_owned()),
            name: name.to_owned(),
            tasks: tasks,
            subtopics: Vec::new(),
        })
    }

    // Writes changes in a topic to its markdown file 
    fn write(&self) -> Result<(), io::Error> {
        Ok(())
    }

    // Loads a topic and its subtopics from a path
    fn load(manager: &'a Manager, path: &Path) -> Result<Option<Topic<'a>>, io::Error> {
        let md = fs::metadata(path);
        if md.is_err() {
            return Ok(None);
        }
        let md = md.unwrap();
        
        if md.is_file() {
            // Ensure it is a markdown file
            match path.extension().map(|s| s.to_str()).flatten() {
                Some("md") => {},
                _ => return Ok(None)
            }

            // Parse topic file
            Ok(Some(Topic::parse(manager, path)?))
        }
        else {
            // Load all topics
            let subtopics: Result<Vec<_>, _> = fs::read_dir(path)?
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.file_name().map(OsStr::to_str).flatten() != Some("about.md"))
                .map(|p| Topic::load(manager, &p))
                .collect();
            let subtopics: Vec<Topic> = subtopics?.into_iter()
                .filter_map(|s| s)
                .collect();

            // Check if there is an 'about.md' file inside the folder
            let file_path = path.join("about").with_extension("md");
            let topic = Topic::parse(manager, &file_path).ok();
            if let Some(mut topic) = topic {
                topic.subtopics = subtopics;
                Ok(Some(topic))
            }
            else {
                let name = path.file_name().unwrap().to_str().unwrap();

                Ok(Some(Topic {
                    manager:  manager,
                    folder_path: path.to_owned(),
                    file_path: None,
                    name: name.to_owned(),
                    tasks: Vec::new(),
                    subtopics: subtopics,
                }))
            }
        }
    }

    // Sorts groups of tasks in files by their due date
    fn sort(&self) {
        // TODO
    }

    // Recursively writes changes in this topic and its subtopics to their markdown files
    // Any extra subtopic on the filesystem is deleted
    // Possible changes:
    // - task addition and removal
    // - change task due date, special params and description
    // - add topic / remove topic
    fn store(&self) {
        // TODO
    }

    // Gets all tasks in this topic
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    // Gets all tasks (subtasks too) in the topic and subtopics
    pub fn tasks_rec(&self) -> Vec<(String, &Task)> {
        let top_tasks = self.subtopics.iter()
            .map(|t| t.tasks_rec().into_iter()
                .map(|(n, u)| ([t.name.clone(), n].join("/"), u))
                .collect::<Vec<_>>())
            .fold(self.tasks.iter()
                .map(|t| (t.name.clone(), t))
                .collect(),
                |a, b| [a, b].concat());
        let sub_tasks: Vec<_> = self.tasks.iter()
            .map(|t| t.subtasks_rec())
            .flatten()
            .collect();
        return [top_tasks, sub_tasks].concat();
    }
}

impl Task {
    pub fn done(&self) -> bool {
        self.done
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn due(&self) -> &Due {
        &self.due
    }

    pub fn desc(&self) -> Option<&str> {
        if let Some(d) = &self.description {
            Some(d)
        }
        else {
            None
        }
    }
    
    pub fn subtasks_rec(&self) -> Vec<(String, &Task)> {
        self.subtasks.iter()
            .map(|t| [vec![(t.name.clone(), t)], t.subtasks_rec()].concat())
            .flatten()
            .map(|(n, t)| ([self.name.clone(), n].join("/"), t))
            .collect()
    }

    pub fn due_compare(lhs: &Self, rhs: &Self) -> Ordering {
        Ordering::Equal
    }
}

