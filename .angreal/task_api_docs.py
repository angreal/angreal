"""
API documentation generator functions for Angreal.

This module contains functions to generate API documentation for both
Python and Rust components of Angreal. These functions are called by the
docs commands in task_docs.py rather than being exposed as commands themselves.
"""

import angreal
import os
import subprocess
import sys
import re
import glob
import ast
import inspect

cwd = os.path.join(angreal.get_root(), '..')
docs_dir = os.path.join(cwd, "docs")
api_docs_dir = os.path.join(docs_dir, "content", "api_reference", "py_angreal")

def extract_docstring(node):
    """Extract docstring from an AST node."""
    if not isinstance(node, (ast.FunctionDef, ast.ClassDef, ast.Module)):
        return None
    
    if node.body and isinstance(node.body[0], ast.Expr) and isinstance(node.body[0].value, ast.Str):
        docstring = node.body[0].value.s
        
        # Format docstring - preserve code blocks, params, etc.
        docstring = docstring.replace('\n    ', '\n')  # Fix indentation
        
        # Identify and format restructured text params
        import re
        
        # Format restructured text parameters
        docstring = re.sub(r':param\s+([^:]+):', r'**\1**:', docstring)
        docstring = re.sub(r':return:', r'**Returns**:', docstring)
        docstring = re.sub(r':raises?\s+([^:]+):', r'**Raises \1**:', docstring)
        docstring = re.sub(r':type\s+([^:]+):', r'*Type \1*:', docstring)
        
        # Format code blocks and examples
        docstring = re.sub(r'i\.e\.\s*::', r'*Example*:\n```python', docstring)
        if 'i.e. ::' in docstring and '```' not in docstring:
            docstring += '\n```'
            
        # Clean up extra newlines
        docstring = re.sub(r'\n{3,}', r'\n\n', docstring)
        
        return docstring
    
    return None

def parse_function_args(node):
    """Parse function arguments from an AST node."""
    args = []
    
    for arg in node.args.args:
        args.append(arg.arg)
    
    if node.args.vararg:
        args.append(f"*{node.args.vararg.arg}")
    
    if node.args.kwarg:
        args.append(f"**{node.args.kwarg.arg}")
    
    # Handle defaults (very simplified)
    return ", ".join(args)

def generate_markdown_for_module(module_path, output_dir):
    """Generate markdown documentation for a Python module using AST parsing."""
    # Get the module name from the path
    module_name = os.path.basename(module_path).replace('.py', '')
    if module_name == '__init__':
        # Skip __init__.py files directly
        return
    
    try:
        # Parse the module using AST
        with open(module_path, 'r') as f:
            module_content = f.read()
        
        module_ast = ast.parse(module_content)
        
        # Create markdown file
        md_path = os.path.join(output_dir, f"{module_name}.md")
        with open(md_path, 'w') as f:
            f.write(f"---\ntitle: {module_name}\n---\n\n")
            f.write(f"# {module_name}\n\n")
            
            # Add module docstring if available
            module_docstring = extract_docstring(module_ast)
            if module_docstring:
                f.write(f"{module_docstring.strip()}\n\n")
            
            # Find functions and classes
            functions = []
            classes = []
            
            for node in module_ast.body:
                if isinstance(node, ast.FunctionDef) and not node.name.startswith('_'):
                    functions.append(node)
                elif isinstance(node, ast.ClassDef) and not node.name.startswith('_'):
                    classes.append(node)
            
            # Document functions
            if functions:
                f.write("## Functions\n\n")
                for func in functions:
                    f.write(f"### {func.name}\n\n")
                    
                    # Get simplified function signature
                    args = parse_function_args(func)
                    f.write(f"```python\n{func.name}({args})\n```\n\n")
                    
                    # Add docstring
                    func_docstring = extract_docstring(func)
                    if func_docstring:
                        f.write(f"{func_docstring.strip()}\n\n")
            
            # Document classes
            if classes:
                f.write("## Classes\n\n")
                for cls in classes:
                    f.write(f"### {cls.name}\n\n")
                    
                    # Add class docstring
                    cls_docstring = extract_docstring(cls)
                    if cls_docstring:
                        f.write(f"{cls_docstring.strip()}\n\n")
                    
                    # Find methods
                    methods = [n for n in cls.body if isinstance(n, ast.FunctionDef)]
                    
                    if methods:
                        f.write("#### Methods\n\n")
                        for method in methods:
                            # Only include __init__, __call__, and non-private methods
                            if method.name.startswith('_') and method.name not in ['__init__', '__call__', '__getattr__']:
                                continue
                                
                            f.write(f"##### {method.name}\n\n")
                            
                            # Get simplified method signature
                            args = parse_function_args(method)
                            f.write(f"```python\n{method.name}({args})\n```\n\n")
                            
                            # Add docstring
                            method_docstring = extract_docstring(method)
                            if method_docstring:
                                f.write(f"{method_docstring.strip()}\n\n")
        
        print(f"Generated documentation for {module_name}")
        return True
    except Exception as e:
        print(f"Error generating documentation for {module_path}: {e}")
        return False

# Removed command decorator
def generate_python_docs():
    """
    Generate markdown documentation for the Python API.
    """
    # Ensure output directory exists
    os.makedirs(api_docs_dir, exist_ok=True)
    
    # Update the index file
    with open(os.path.join(api_docs_dir, "_index.md"), 'w') as f:
        f.write("---\ntitle: Python API Reference\n---\n\n")
        f.write("# Python API Reference\n\n")
        f.write("This documentation covers the Python API for Angreal.\n\n")
        f.write("{{% children %}}\n")
    
    # Find all Python modules to document
    python_dir = os.path.join(cwd, "python", "angreal")
    
    # Process main package
    generate_markdown_for_module(os.path.join(python_dir, "__init__.py"), api_docs_dir)
    
    # Process direct modules
    for module_path in glob.glob(os.path.join(python_dir, "*.py")):
        if not os.path.basename(module_path) == "__init__.py":
            generate_markdown_for_module(module_path, api_docs_dir)
    
    # Process integrations
    integrations_dir = os.path.join(python_dir, "integrations")
    if os.path.exists(integrations_dir):
        # Create integrations directory and index
        integ_docs_dir = os.path.join(api_docs_dir, "integrations")
        os.makedirs(integ_docs_dir, exist_ok=True)
        
        with open(os.path.join(integ_docs_dir, "_index.md"), 'w') as f:
            f.write("---\ntitle: Integrations\n---\n\n")
            f.write("# Integrations\n\n")
            f.write("Python integrations for external tools and systems.\n\n")
            f.write("{{% children %}}\n")
        
        # Process integration modules
        for module_path in glob.glob(os.path.join(integrations_dir, "*.py")):
            if not os.path.basename(module_path) == "__init__.py":
                generate_markdown_for_module(module_path, integ_docs_dir)
        
        # Process Docker integration if present
        docker_dir = os.path.join(integrations_dir, "docker")
        if os.path.exists(docker_dir):
            docker_docs_dir = os.path.join(integ_docs_dir, "docker")
            os.makedirs(docker_docs_dir, exist_ok=True)
            
            with open(os.path.join(docker_docs_dir, "_index.md"), 'w') as f:
                f.write("---\ntitle: Docker Integration\n---\n\n")
                f.write("# Docker Integration\n\n")
                f.write("Python bindings for Docker functionality.\n\n")
                f.write("{{% children %}}\n")
            
            for module_path in glob.glob(os.path.join(docker_dir, "*.py")):
                if not os.path.basename(module_path) == "__init__.py":
                    generate_markdown_for_module(module_path, docker_docs_dir)
    
    print("Python API documentation generated successfully")

# Removed command decorator
def generate_rust_docs():
    """
    Generate Rust API documentation using cargo doc.
    """
    subprocess.run(["cargo", "doc", "--no-deps"], cwd=cwd, check=True)
    print(f"Rust API documentation generated successfully at {os.path.join(cwd, 'target', 'doc')}")
    print("To view locally, run: cargo doc --open")
    print("Online documentation available at https://docs.rs/angreal")

# Removed command decorator
def generate_all_docs():
    """
    Generate both Rust and Python API documentation.
    """
    generate_rust_docs()
    generate_python_docs()
    print("All API documentation generated successfully")