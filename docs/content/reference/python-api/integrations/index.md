---
title: Integrations API Reference
weight: 30
---

# Integrations API Reference

Angreal provides integration modules that connect with external tools and systems to enhance your project automation capabilities. Each integration wraps a commonly-used development tool, providing a Python interface that works naturally within Angreal tasks.

## Available Integrations

### Virtual Environment Integration

The `angreal.integrations.venv` module provides ultra-fast virtual environment and package management powered by UV. This integration handles environment creation, package installation, and Python version management with significantly better performance than traditional tools.

### Git Integration

The `angreal.integrations.git` module wraps Git operations for version control automation. It provides both high-level methods for common operations like commit and push, and low-level access for arbitrary Git commands.

### Flox Integration

The `angreal.integrations.flox` module provides cross-language development environment management through Flox, a Nix-based tool. Unlike the virtual environment integration which focuses on Python, Flox supports multi-language projects and includes services management for databases and other dependencies.

### Docker Compose Integration

The `angreal.integrations.docker` module provides Docker container orchestration through Docker Compose. It enables starting, stopping, and managing multi-container applications defined in compose files.

## Choosing an Integration

Different integrations serve different needs. For Python-only projects, the virtual environment integration offers the simplest path with the best performance. Multi-language projects benefit from Flox's ability to manage packages across Python, Node.js, Rust, and other ecosystems. Projects requiring containerized services should use Docker Compose for its networking and isolation capabilities.

The integration pages provide detailed API documentation, usage examples, and guidance for each module.
