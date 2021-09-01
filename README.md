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

So, how can you use Planifie? This system can be manipulated through its CLI,
the following commands are available:

<TODO>

