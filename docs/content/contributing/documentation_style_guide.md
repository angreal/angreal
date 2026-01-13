---
title: Documentation Style Guide
weight: 20
---

# Documentation Style Guide

This guide establishes the writing standards for Angreal documentation. Following these guidelines ensures consistency across all documentation and creates a better experience for readers.

## Diátaxis Framework

Angreal documentation follows the Diátaxis framework, which organizes content into four distinct categories. Each piece of documentation should belong to exactly one category.

### Tutorials

Tutorials guide newcomers through their first experiences with Angreal. They are learning-oriented and focus on teaching through doing rather than explaining concepts in depth.

A good tutorial takes the reader from zero to a working result. It includes every step needed to complete the exercise, avoids explaining why things work the way they do, and focuses on building confidence through success. The tutorial at `/tutorials/your_first_angreal.md` demonstrates this approach.

Tutorials belong in the `/tutorials/` directory.

### How-to Guides

How-to guides show experienced users how to accomplish specific tasks. They are goal-oriented and assume the reader already understands the basics.

A good how-to guide has a clear objective, provides the steps needed to achieve that objective, and includes enough context to adapt the solution to different situations. It does not explain underlying concepts at length or teach fundamentals.

How-to guides belong in the `/how-to-guides/` directory.

### Explanations

Explanations help readers understand concepts, design decisions, and architectural choices. They are understanding-oriented and focus on the "why" rather than the "how."

A good explanation illuminates a topic without requiring the reader to do anything. It provides context, discusses trade-offs, and connects ideas to help readers build mental models. The explanation at `/explanation/task_discovery.md` demonstrates this approach.

Explanations belong in the `/explanation/` directory.

### Reference

Reference documentation provides precise technical specifications. It is information-oriented and serves as a lookup resource during work.

A good reference page is accurate, complete, and consistently structured. It describes what something is and how it behaves without teaching or explaining why. The API documentation in `/reference/python-api/` demonstrates this approach.

Reference documentation belongs in the `/reference/` directory.

## Prosaic Writing Style

Documentation should read as flowing technical prose rather than fragmented lists. This section establishes the standards for achieving that style.

### Prefer Paragraphs Over Bullets

When explaining concepts or describing processes, write complete paragraphs that develop ideas across connected sentences. Readers understand relationships better when ideas flow naturally from one to the next.

Instead of:

```
## Features

- Fast virtual environment creation
- Package installation
- Python version management
```

Write:

```
## Features

The virtual environment integration provides fast environment creation,
efficient package installation, and Python version management. These
capabilities work together to streamline the development workflow.
```

### When Bullets Are Appropriate

Bullets remain appropriate for genuinely list-like content. Use them for:

Parameter tables in reference documentation, where readers scan for specific options. Enumerated choices where the items are truly independent and parallel. Feature comparison tables that benefit from visual alignment. Checklists or step sequences in how-to guides where order matters.

### Lead with Context

Before presenting code examples, explain what the code demonstrates and why it matters. After the code, highlight important details the reader should notice.

Instead of:

```python
@angreal.command(name="build")
def build():
    pass
```

Write:

The `@angreal.command` decorator transforms a Python function into a CLI command. The `name` parameter determines what users type to invoke the command.

```python
@angreal.command(name="build")
def build():
    pass
```

After applying this decorator, users can run `angreal build` to execute the function.

### Use Active Voice

Write in active voice to create clear, direct sentences. Active voice identifies who or what performs actions, making documentation easier to follow.

Instead of "The file is loaded by Angreal" write "Angreal loads the file."

Instead of "Arguments can be added with the decorator" write "The decorator adds arguments to your command."

### Present Tense

Use present tense throughout documentation. Present tense feels immediate and describes current behavior accurately.

Instead of "The command will execute" write "The command executes."

Instead of "This was designed to" write "This design allows."

## Code Examples

Code examples teach by showing. Follow these guidelines to make them effective.

### Complete and Runnable

Every code example should be complete enough that a reader could copy it into a file and run it. Include necessary imports and show realistic values rather than placeholders.

Instead of:

```python
@angreal.command(...)
def example():
    # do something
    pass
```

Write:

```python
import angreal

@angreal.command(name="greet", about="Say hello")
def greet():
    print("Hello, world!")
```

### Progressive Complexity

When teaching a concept, start with the simplest example that demonstrates the idea. Add complexity in subsequent examples only after establishing the foundation.

### Comments for Clarity

Add comments to explain non-obvious behavior, but avoid commenting obvious operations. Comments should explain "why" more often than "what."

## Accuracy

Documentation must match current behavior. Inaccurate documentation is worse than no documentation because it misleads readers.

### Verify Against Code

When documenting API behavior, verify claims against the actual implementation. If the documentation describes a parameter, that parameter must exist with the described behavior.

### Test Examples

Run code examples to verify they work. Broken examples frustrate readers and damage trust in the documentation.

### Version Awareness

When behavior changes between versions, document which version introduced the change. Avoid showing outdated version numbers in examples.

## Structure and Navigation

### Consistent Headings

Use sentence case for headings. Start with the most important information. Keep headings concise.

### Cross-References

Link to related documentation when mentioning concepts explained elsewhere. Use the "See Also" pattern at the end of pages to guide readers to related content.

### Front Matter

Every documentation page requires YAML front matter with at least a title. Include weight to control ordering within a section.

```yaml
---
title: "Page Title Here"
weight: 10
---
```

## Hugo-Specific Formatting

Angreal documentation uses Hugo with the Geekdoc theme. These notes cover theme-specific features.

### Hints

Use hints for important callouts:

```
{{</* hint type=note */>}}
This is helpful information.
{{</* /hint */>}}

{{</* hint type=warning */>}}
Be careful about this.
{{</* /hint */>}}
```

### Code Highlighting

Hugo automatically highlights code blocks with language identifiers:

````
```python
def example():
    pass
```
````

## Review Checklist

Before submitting documentation changes, verify:

The content belongs in its Diátaxis category. Prose flows in paragraphs rather than excessive bullets. Code examples are complete and runnable. Technical claims match current implementation. Cross-references link to existing pages. Front matter includes required fields.
