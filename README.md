# Planifie

This project aims to develop a minimalist CLI app for personal organization.
Planifie operates on markdown files. There are only two types of data: topics
and tasks. Each file/folder represents a topic. For example, you may have the
following file structure:

```
projects/
  planifie.md
  voxel-platformer.md
  cubos.md
finances.md
```

A topic is a regular markdown file with one quirk: tasks associated to this
topic are defined beneath the `## Tasks` subtitle, if there is one. Other than
that the file can be treated as a regular markdown file you would use for
taking notes on something.

So where do tasks fit in here? Tasks are used to represent goals you want to
achieve on some topic. They are stored in a list with checkboxes for each task.
Tasks must have a title and can have a due/completion date, a description or
even sub-tasks. Here is an example `## Tasks` section:

```md
## Tasks

- [X] Vacuum the dining room (01/09/21) 
- [ ] Wash the dishes (01/09/21) 
- [ ] Due dates are optional
- [ ] Big project coming up (05/09/21)
  Here is a very interesting
  multi line description.
  And here are the subtasks:

  - [X] Do x (01/09/21)
  - [ ] Do y
  - [ ] Do z (02/09/21-04/09/21)
```

Sub-tasks are the same as normal tasks: they can have their own description
and even their own sub-tasks. If a task you've created has grown really big
and you think it deserves its own topic, you can promote the task to a topic,
moving the description and every sub-task to the new topic. The reverse is also
possible, you can collapse an entire topic to a single task. 

## Commands

So, how can you use Planifie? This system can be manipulated through its CLI,
the following commands are available:

- `plan --add-task, -a <task name>` - Adds a new task to a topic. This command
  can take as options `--due, -d <due date/interval>`, which specifies the due
  date or interval, `--description, -m <description>`, which specifies the
  task's description and `--edit, -e`, which opens `$EDITOR` and allows the
  user to edit the task manually. 
- `plan --add-topic, -A <topic name>` - Creates a new topic. Can take as option
  `--edit, -e`, which allows the user to edit the topic's file manually.
- `plan --remove-task, -r <task name>` - Removes a task and its sub-tasks.
- `plan --remove-topic, -R <topic name>` - Removes a topic and its sub-topics.
- `plan --expand, -E <task name>` - Expands a task into a topic.
- `plan --collapse, -C <topic name>` - Collapses a topic into a task on its
  parent topic.
- `plan --edit, -e <topic/task>` - Allows the user to edit a topic or task
  manually. The files can also be edited manually without using `plan`.
- `plan --due, -d <task> [due date/interval]` - Sets the due date of a task. If
  no date or interval is specified, the task is set to due today. Can also take
  as options `--weekly, -w`, which makes the task due by the end of the week and
  `--monthly, -m`, which makes the the dask due by the end of the month.
- `plan --update, -u` - Makes all tasks that were due before today and were not
  completed due today. This command can be set to run automatically everyday
  (eg: cron job).
- `plan --status, -s` - Lists all tasks either due today, this week or this
  month. The interval can be set using the `--weekly, -W` and `--monthly, -M`
  options, the default being today. It can also take `--descriptio, -m` as an
  option, which makes the tasks' descriptions visible.

## Example usage

Here is some example usage of the commands shown above:

```sh
# Create a new topic. The topic `projects` is also created automatically, but
# since its only used as a folder, no markdown file is created, only a folder.
# If you were to add a topic as a child topic to an already existing topic with
# a markdown file, the already existing topic would turn into a folder and its
# markdown file moved to within that folder and be renamed to `about.md`.
$ plan -A "projects/planifie"

# Add a simple task to the previously created topic, and then open the editor
# so that we can write a description for it.
$ plan -ae "projects/planifie/Do x" 
$ plan -a "projects/planifie/Do y" -m "This task has a
                                       multi-line description"
$ plan -a "projects/planifie/Do z" -d 02/09/21
$ plan -a "projects/planifie/Do x/My subtask" -d today

$ plan -s
projects/planifie - Do x
$ plan -sm
projects/planifie - Do x
  <"Do x" description here>

# Makes "Do y" due today.
$ plan -d "projects/planifie/Do y"
$ plan -d "projects/planifie/Do y" today
# Schedule "Do y" to be due 01/01/22. 
$ plan -d "projects/planifie/Do y" 01/01/22
# Schedule "Do y" to be due 01/01/22 to 06/01/22
$ plan -d "projects/planifie/Do y" 01/01/22-06/01/22

# Expand "Do x" into a topic named "x" (will be put on projects/planifie/x).
$ plan -E "projects/Do x" "x"

# Collapse the "projects/planifie" topic into a task on "projects". This
# operation is applied recursively to every sub-topic, so that the "x" topic
# is collapsed into "planifie" first, and then "planifie" is collapsed into
# "projects", as a task named "Develop Planifie".
$ plan -C "projects/planifie" "Develop Planifie"

# Remove the task and its sub-tasks.
$ plan -r "projects/Develop Planifie"

# Removes the whole "projects" topic.
$ plan -R "projects"
```
