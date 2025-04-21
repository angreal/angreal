import angreal
import os
import subprocess
import webbrowser
import time
import shutil

# Import functions from task_api_docs.py for API documentation generation
# Add these imports so they're available for our new subcommands

venv_path = os.path.join(angreal.get_root(),'..','.venv')

cwd = os.path.join(angreal.get_root(),'..')
docs_dir = os.path.join(cwd,"docs")
rust_docs_dir = os.path.join(docs_dir, "static", "rust-docs")
py_docs_dir = os.path.join(docs_dir, "static", "py-docs")

# Helper function to check if Docker is installed
def is_docker_available():
    """Check if Docker is installed and available."""
    try:
        result = subprocess.run(
            ["docker --version"],
            shell=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE
        )
        return result.returncode == 0
    except Exception:
        return False

docs = angreal.command_group(name="docs", about="commands for generating documentation")

@docs()
@angreal.command(name="stop", about="stop the currently running hugo server")
def stop_hugo():
    # Get container ID for the Hugo server
    containers = subprocess.run(
        ["docker ps --filter ancestor=klakegg/hugo:0.111.3 --quiet"],
        shell=True, capture_output=True, text=True
    ).stdout.strip()

    if containers:
        # Stop the container(s)
        print("Stopping Hugo Docker container(s)...")
        subprocess.run(["docker stop " + containers], shell=True)
    else:
        # Fallback to killing local processes
        print("No Docker containers found, trying to kill local Hugo processes...")
        subprocess.run(["pkill -f hugo"], shell=True)

@docs()
@angreal.command(name="serve", about="starts the docs site in the background.")
@angreal.argument(name="open", long="open", short="o", takes_value=False,
                  help="open results in web browser", is_flag=True)
@angreal.argument(name="skip_api", long="skip-api", takes_value=False,
                  help="skip generating API docs before serving", is_flag=True)
def build_hugo(open=True, skip_api=False):
    """
    Serve the documentation locally using Hugo.

    Args:
        open: If True, open the documentation in a web browser
        skip_api: If True, skip generating API documentation before serving
    """
    if not skip_api:
        print("Generating API documentation...")
        try:
            # Import and run API docs generator directly
            from task_api_docs import generate_all_docs
            generate_all_docs()
        except Exception as e:
            print(f"Error generating API docs: {e}")
            print("Continuing with serve...")

    # Check if Docker is available
    if is_docker_available():
        # Start the Hugo server using Docker
        print("Starting Hugo server on http://localhost:12345/angreal/")
        server_process = subprocess.Popen(
            [
                "docker run --rm -it -v $(pwd)/docs:/src -p 12345:12345 " +
                "klakegg/hugo:0.111.3 serve -D -p 12345 --bind 0.0.0.0",
            ], cwd=cwd, shell=True
        )
    else:
        # Fallback to local Hugo if available
        if shutil.which("hugo"):
            print("Docker not available, falling back to local Hugo installation.")
            print("Starting Hugo server on http://localhost:12345/angreal/")
            server_process = subprocess.Popen(
                ["hugo serve -D -p 12345"], cwd=docs_dir, shell=True
            )
        else:
            print("ERROR: Neither Docker nor local Hugo installation found.")
            print("Please install Docker or Hugo to serve the documentation.")
            print("Docker installation: https://docs.docker.com/get-docker/")
            print("Hugo installation: https://gohugo.io/installation/")
            return

    # Wait a moment for the server to start
    time.sleep(1)

    if open:
        webbrowser.open_new("http://localhost:12345/angreal/")

    print("Hugo server is running. Press Ctrl+C to stop.")
    try:
        # Keep the server running until keyboard interrupt
        server_process.wait()
    except KeyboardInterrupt:
        print("Stopping Hugo server...")
        stop_hugo()

@docs()
@angreal.command(name="build", about="build the documentation site")
@angreal.argument(name="skip_api", long="skip-api", takes_value=False,
                  help="skip generating API docs before building", is_flag=True)
def build_docs(skip_api=False):
    """
    Build the documentation site.

    Args:
        skip_api: If True, skip generating API documentation before building
    """
    if not skip_api:
        print("Generating API documentation...")
        try:
            # Import and run API docs generator directly
            from task_api_docs import generate_all_docs
            generate_all_docs()
        except Exception as e:
            print(f"Error generating API docs: {e}")
            print("Continuing with build...")

    # Check if Docker is available
    print("Building documentation site...")
    if is_docker_available():
        # Build the documentation using Docker
        subprocess.run(
            ["docker run --rm -v $(pwd)/docs:/src klakegg/hugo:0.111.3 --minify"],
            cwd=cwd, shell=True, check=True
        )
        print(f"Documentation built successfully in {os.path.join(docs_dir, 'public')}")
    else:
        # Fallback to local Hugo if available
        if shutil.which("hugo"):
            print("Docker not available, falling back to local Hugo installation.")
            subprocess.run(["hugo", "--minify"], cwd=docs_dir, check=True)
            public_path = os.path.join(docs_dir, 'public')
            print(f"Documentation built successfully in {public_path}")
        else:
            print("ERROR: Neither Docker nor local Hugo installation found.")
            print("Please install Docker or Hugo to build the documentation.")
            print("Docker installation: https://docs.docker.com/get-docker/")
            print("Hugo installation: https://gohugo.io/installation/")
            return
    return
