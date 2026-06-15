---
title: "Your First Angreal"
weight: 10
---

# Your First Angreal

In this tutorial, you'll create a complete Angreal project from scratch. We'll build a "meeting notes" system that demonstrates both task automation and project templating.

{{< hint type=note >}}
**Time Required**: 15-20 minutes

**Prerequisites**: Python 3.8+, Angreal installed, basic Python knowledge
{{< /hint >}}

## What You'll Build

A meeting notes system with:
- A project template for consistent meeting structure
- Automated task for taking timestamped notes
- Customizable meeting details (name, cadence, agenda)

## Project Structure

Angreal renders templates with [Tera](https://keats.github.io/tera/docs/), a
Jinja2-like template engine, so names wrapped in `{{ }}` get filled in from the
values you supply during `init`.

Here's what we'll create:

```
meeting_notes/
├── angreal.toml           # Template configuration
└──  {{ name }}/            # Template directory (Tera variable)
    ├── .angreal/          # Angreal tasks directory
    │   ├── init.py        # Post-initialization script
    │   └── task_notes.py  # Note-taking task
    └── README.md          # Meeting overview template
```

## Step 1: Create the Template Structure

First, create the project directory:

```bash
mkdir meeting_notes
cd meeting_notes
```

## Step 2: Define Template Variables

Create `angreal.toml` to define the variables users will provide:

```toml
# Template variables with defaults
name = "weekly_standup"
cadence = "weekly"
standing_agenda = "Updates, blockers, and next steps"
```

## Step 3: Create the README Template

Create a directory with a Tera template name:

```bash
mkdir "{{ name }}"
```

{{< hint type=warning >}}
**Note**: Angreal uses [Tera](https://keats.github.io/tera/docs/) templating engine. The syntax is similar to Jinja2, but there are some differences.
{{< /hint >}}

Create `{{ name }}/README.md`:

```markdown
# {{ name }}

## Meeting Cadence
{{ cadence }}

## Standing Agenda
{{ standing_agenda }}

## Notes
Meeting notes will be stored in this directory with timestamps.
```

## Step 4: Create the Initialization Script

Create the `.angreal` directory:

```bash
mkdir "{{ name }}/.angreal"
```

Create `{{ name }}/.angreal/init.py`:

```python
"""Post-initialization script for meeting notes template."""

def init():
    """Run after template is rendered."""
    print("Meeting notes project initialized!")
    print("Run 'angreal notes' to take meeting notes")
    print("See README.md for meeting details")
```

## Step 5: Create the Note-Taking Task

Create `{{ name }}/.angreal/task_notes.py`:

```python
"""Task for taking meeting notes."""
import angreal
import datetime
import os
import subprocess
import tempfile
from pathlib import Path

@angreal.command(name="notes", about="Take meeting notes")
@angreal.argument(
    name="editor",
    long="editor",
    short="e",
    help="Editor to use (default: $EDITOR or vi)"
)
@angreal.argument(
    name="now",
    long="now",
    is_flag=True,
    help="Open editor immediately"
)
def take_notes(editor=None, now=False):
    """Create a timestamped meeting notes file."""
    # Generate filename with current timestamp
    timestamp = datetime.datetime.now().strftime("%Y-%m-%d_%H-%M")
    filename = f"notes_{timestamp}.md"

    # Determine editor
    if not editor:
        editor = os.environ.get("EDITOR", "vi")

    # Create initial content
    initial_content = f"""# Meeting Notes - {timestamp}

## Attendees
-

## Agenda Items
{angreal.get_context().get('standing_agenda', 'No agenda set')}

## Discussion


## Action Items
- [ ]

## Next Meeting
"""

    if now:
        # Create temporary file with initial content
        with tempfile.NamedTemporaryFile(mode='w', suffix='.md', delete=False) as tf:
            tf.write(initial_content)
            temp_path = tf.name

        # Open in editor
        try:
            subprocess.run([editor, temp_path], check=True)

            # Read edited content
            with open(temp_path, 'r') as f:
                content = f.read()

            # Save to final file
            with open(filename, 'w') as f:
                f.write(content)

            print(f"Notes saved to {filename}")

        except subprocess.CalledProcessError:
            print(f"Editor '{editor}' failed. Check your EDITOR environment variable.")
        finally:
            # Clean up temp file
            Path(temp_path).unlink(missing_ok=True)
    else:
        # Just create the file
        with open(filename, 'w') as f:
            f.write(initial_content)
        print(f"Created {filename}")
        print("Use 'angreal notes --now' to open in editor")

@angreal.command(name="list", about="List all meeting notes")
def list_notes():
    """List all meeting notes files."""
    notes = sorted(Path.cwd().glob("notes_*.md"))

    if not notes:
        print("No meeting notes found")
        return

    print("Meeting Notes:")
    for note in notes:
        # Extract date from filename
        date_str = note.stem.replace("notes_", "")
        print(f"  - {date_str}: {note.name}")
```

Note the call to `angreal.get_context()` in the agenda section: it returns the
template variables the user supplied during `init` (here, `standing_agenda`), so
the task can reuse the values the project was created with.

## Step 6: Test Your Template

### Initialize from the Template

Step 1 left you inside `meeting_notes`, so first move back up to its parent —
`angreal init` resolves a local template relative to the current directory:

```bash
cd ..   # back to the parent of meeting_notes
angreal init meeting_notes
```

`angreal init` takes a single argument — the template. It then prompts you for
each variable defined in `angreal.toml`. The value you give for `name` becomes
the name of the generated project directory, so enter `my_team_standup` at that
prompt. Pressing Enter at any prompt accepts the default shown in brackets, which
is why leaving `standing_agenda?` blank keeps the default agenda in the generated
README:

```text
name? ["weekly_standup"] > my_team_standup
cadence? ["weekly"] > daily
standing_agenda? ["Updates, blockers, and next steps"] >
Meeting notes project initialized!
Run 'angreal notes' to take meeting notes
See README.md for meeting details
```

Because you entered `my_team_standup` for `name`, Angreal renders the
`{{ name }}` template directory into a new directory called `my_team_standup`.

### Explore the Generated Project

```bash
cd my_team_standup
ls -la

# Output:
# .angreal/
# README.md
```

Check the README:

```bash
cat README.md

# Output:
# # my_team_standup
#
# ## Meeting Cadence
# daily
#
# ## Standing Agenda
# Updates, blockers, and next steps
# ...
```

### Use the Tasks

List available commands:

```bash
angreal --help
```

The two commands from `task_notes.py` appear in the output:

```text
Commands:
  notes  Take meeting notes
  list   List all meeting notes
```

Take notes. Running `angreal notes` (without `--now`) writes the file and prints
the line from the task's `print(f"Created {filename}")`:

```bash
angreal notes
```

```text
Created notes_2026-06-15_09-30.md
Use 'angreal notes --now' to open in editor
```

You can also open the editor immediately instead of just writing the file:

```bash
angreal notes --now
```

List all notes. `angreal list` reads the `notes_*.md` files and prints them:

```bash
angreal list
```

```text
Meeting Notes:
  - 2026-06-15_09-30: notes_2026-06-15_09-30.md
```

## Step 7: Share Your Template

Push to GitHub:

```bash
cd ../meeting_notes   # back to the template directory you created in Step 1
git init
git add .
git commit -m "Meeting notes Angreal template"
git remote add origin https://github.com/yourusername/meeting-notes-template.git
git push -u origin main
```

Others can now use your template by passing its URL to `angreal init`. They'll
answer the same prompts, and the `name` they enter becomes their project
directory:

```bash
angreal init https://github.com/yourusername/meeting-notes-template.git
```

## What You've Learned

How to create an Angreal template with:
- Template configuration (`angreal.toml`)
- Tera templated directories and files
- Post-initialization scripts
- Custom tasks with arguments

Key concepts:
- Templates use [Tera](https://keats.github.io/tera/) syntax (`{{ variable }}`)
- Tasks are Python files starting with `task_` in `.angreal/`
- The `@angreal.command` decorator creates CLI commands
- The `@angreal.argument` decorator adds command-line options


## Next Steps

- Add more tasks (archive notes, search notes, generate reports)
- Create templates for your own projects
- Explore the [How-to Guides](/angreal/how-to-guides/) for specific techniques
- Read the [API Reference](/angreal/reference/python-api) for all available features
