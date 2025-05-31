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
