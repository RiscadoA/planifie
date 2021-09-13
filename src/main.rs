use std::path::PathBuf;
use std::io;
use clap::{App, Arg};

mod data;

fn main() {
    let matches = App::new("planifie")
        .version("0.1")
        .author("Ricardo Antunes <me@riscadoa.com>")
        .about("Minimalist self-organization CLI app")
        .arg(Arg::new("update")
            .short('u')
            .long("update")
            .about("Makes all tasks that were due before today and were not completed due today. \
                    This command can be set to run automatically everyday (eg: cron job)."))
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .about("Overrides the default planifie directory path.")
            .number_of_values(1)
            .takes_value(true)
            .value_name("PATH"))
        .arg(Arg::new("task")
            .short('t')
            .long("task")
            .about("Selects a task for operating on.")
            .group("selector")
            .takes_value(true)
            .value_name("TASK_NAME"))
        /*.arg(Arg::new("topic")
            .short('T')
            .long("topic")
            .about("Selects a topic for operating on.")
            .group("selector")
            .number_of_values(1)
            .takes_value(true)
            .value_name("TOPIC_NAME"))
        .arg(Arg::new("add")
            .short('a')
            .long("add")
            .about("Creates a new task/topic.")
            .requires("selector"))
        .arg(Arg::new("remove")
            .short('r')
            .long("remove")
            .about("Removes a new task/topic.")
            .requires("selector")
            .conflicts_with("add"))*/
        .arg(Arg::new("description")
            .short('m')
            .long("description")
            .value_name("DESC")
            .about("Sets a task/topic description.")
            .takes_value(true)
            .min_values(0)
            .max_values(1)
            .requires("selector"))
        /*.arg(Arg::new("due")
            .short('d')
            .long("due")
            .about("Sets a task due date/interval. If no date/interval is specified, 'today' is \
                    used as default.")
            .value_name("WHEN")
            .default_value("daily")
            .takes_value(true)
            .requires("selector"))
        .arg(Arg::new("expand")
            .short('E')
            .long("expand")
            .about("Expands a task into a topic.")
            .requires_all(&["task", "topic"]))
        .arg(Arg::new("collapse")
            .short('C')
            .long("collapse")
            .about("Collapses a topic into a task.")
            .requires_all(&["task", "topic"]))
        .arg(Arg::new("edit")
            .short('e')
            .long("edit")
            .about("Allows the user to edit a topic or task manually using $EDITOR.")
            .requires("selector"))
        .arg(Arg::new("status")
            .short('s')
            .long("status")
            .about("Lists all tasks due on a certain date/interval. If no date/interval is \
                    specified, 'daily' is used as default. If `-m` is used, the tasks' \
                    descriptions are also shown.")
            .value_name("WHEN")
            .default_value("daily")
            .number_of_values(1)
            .takes_value(true)
            .conflicts_with("selector"))*/
        .get_matches();

    // Get config file path
    let config_path = match matches.value_of("config") {
        Some(c) => PathBuf::from(c),
        _ => dirs::config_dir()
            .expect("No configuration folder could be found, set the configuration file path with \
                     the --config option")
            .join("planifie.conf")
    };

    // Init manager
    let manager = data::Manager::new(config_path.as_path()).expect("Couldn't create manager");
    let root = manager.load().expect("Couldn't load root topic");

    let mut tasks = root.tasks_rec();
    tasks.sort();
    tasks.iter()
        .for_each(|(n, t)| match t.due() {
            data::Due::Never => println!("{}", n),
            _ => println!("{} ({})", n, t.due()),
        });

    if matches.is_present("add") {
        /*if let Some(name) = matches.value_of("task") {
            cmd::add_task(name).unwrap();
        }
        else if let Some(name) = matches.value_of("topic") {

        }*/
    }
    
    if matches.is_present("remove") {

    }

    if matches.is_present("description") && !matches.is_present("status") {
        /*let name = matches.value_of("task").unwrap();
        let (mut topic, task_name) = manager.open_topic(name).expect("Topic not found");
        let task_name = task_name.expect("Task not found");
        let task = topic.task_mut(task_name).expect("Task not found");

        if let Some(desc) = matches.value_of("description") {
            task.description = Some(desc.to_owned());
            topic.write().unwrap();
        }
        else {
            println!("{}", task.description.as_ref().unwrap_or(&"".to_owned()));
        }*/
    }

    if matches.is_present("due") {
        
    }

    if matches.is_present("edit") {

    }

    if matches.is_present("expand") {
        
    }

    if matches.is_present("collapse") {
        
    }

    if matches.is_present("status") {
        
    }
}