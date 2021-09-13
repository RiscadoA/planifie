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

Topics are simple markdown files. Where do tasks fit in here? Tasks are used to
represent goals you want to achieve on some topic. Every list item with a
checkbox (e.g.: `- [ ] Example task`) is considered a task. Tasks must have a
name and can have a due/completion date, a description or even sub-tasks. Here
is an example topic file:

```md
## General tasks

Home stuff:

- [X] Vacuum the dining room (01/09/21) 
- [ ] Wash the dishes (01/09/21)

Other stuff:

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

## Usage

So, how can you use Planifie? This system can be manipulated through its CLI,
the following commands are available:

- `plan update` - Performs automation on special parameters (eg.: `@ repeat`).
  This command can/should be set to run automatically everyday.
- `plan show [WHEN]` - Shows the tasks that are due on a certain time frame.
  This time frame can be specified through the `WHEN` argument. This argument
  can take the following forms: `dd/mm/yy`, `dd/mm/yy-dd/mm/yy`, `daily`,
  `weekly`, `monthly`, `all`. This command also supports `-m` flag, which shows
  the task's descriptions too, the `-d` flag which hides every task that is
  already done and the `-t` flag which shows only the top tasks (subtasks are
  hidden).
- `plan -c, --config PATH` - Overrides the default planifie configuration file
  path.
- `plan -h, --help` - Shows an help message.
- `plan -t, --task TASK_NAME` - Specifies that we will be operating on task.
- `plan -T, --topic TOPIC_NAME` - Specifies that we will be operating on a
  topic.
- `plan -a, --add` - Creates a new task or topic. Requires `-t`/`-T`.
- `plan -r, --remove` - Removes a task or topic. Requires `-t`/`-T`.
- `plan -m, --description [DESC]` - Sets or shows a task's description.
  Requires `-t` or `-s`.
- `plan -d, --due [WHEN]` - Sets a task due date/interval. If no date/interval
  is specified, 'today' is used as default. Requires `-t`.
- `plan -E, --expand` - Expands a task into a topic. Requires `-t` and `-T`.
- `plan -C, --collapse` - Collapses a topic into a task. Requires `-t` and
  `-T`.
- `plan -e, --edit` - Allows the user to edit a topic or task manually using
  `$EDITOR`. The files can also be edited manually without using `plan`.
- `plan -s, --status [WHEN]` - Lists all tasks due on a certain date/interval.
  If no date/interval is specified, 'today' is used as default. If `-m` is used,
  the tasks' descriptions are also shown.

## Example usage

Here is some example usage of the commands shown above:

```sh
# Create a new topic. The topic `projects` is also created automatically, but
# since its only used as a folder, no markdown file is created, only a folder.
# If you were to add a topic as a child topic to an already existing topic with
# a markdown file, the already existing topic would turn into a folder and its
# markdown file moved to within that folder and be renamed to `about.md`.
$ plan -aT "projects/planifie"

# Add a simple task to the previously created topic, and then open the editor
# so that we can write a description for it.
$ plan -aet "projects/planifie/Do x" 
$ plan -at "projects/planifie/Do y" -m "This task has a
                                       multi-line description"
$ plan -at "projects/planifie/Do z" -d 02/09/21
$ plan -at "projects/planifie/Do x/My subtask" -d today

$ plan -s
projects/planifie - Do x
$ plan -ms
projects/planifie - Do x
  <"Do x" description here>

# Makes "Do y" due today.
$ plan -td "projects/planifie/Do y"
$ plan -td "projects/planifie/Do y" today
# Schedule "Do y" to be due 01/01/22. 
$ plan -td "projects/planifie/Do y" 01/01/22
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
