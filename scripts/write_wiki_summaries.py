#!/usr/bin/env python3
"""Generate kgx wiki summary pages for each PROJECT note."""
import glob
import json
import re
import subprocess
import sys
import yaml

FRONT_MATTER_RE = re.compile(r"^---\s*\n(.*?)\n---\s*\n", re.DOTALL)


def parse_frontmatter(path: str) -> dict | None:
    with open(path) as f:
        text = f.read()
    m = FRONT_MATTER_RE.match(text)
    if not m:
        return None
    try:
        return yaml.safe_load(m.group(1))
    except yaml.YAMLError:
        return None


def first_paragraph(path: str) -> str:
    with open(path) as f:
        text = f.read()
    m = FRONT_MATTER_RE.match(text)
    body = text[m.end():] if m else text
    for line in body.splitlines():
        line = line.strip()
        if line and not line.startswith("#"):
            return line
    return ""


def kgx_wiki_write(category: str, title: str, summary: str, content: str):
    proc = subprocess.run(
        ["kgx", "wiki", "write",
         "--category", category,
         "--title", title,
         "--summary", summary],
        input=content, capture_output=True, text=True,
    )
    if proc.returncode != 0:
        print(f"  ERROR: {proc.stderr.strip()}", file=sys.stderr)
    else:
        print(f"  wrote: {title}", file=sys.stderr)


def main():
    files = sorted(glob.glob("02_Projects/PROJECT.*.md"))
    if not files:
        print("No PROJECT notes found", file=sys.stderr)
        sys.exit(1)

    for path in files:
        fm = parse_frontmatter(path)
        if fm is None:
            continue

        name = path.split("/")[-1].replace("PROJECT.", "").replace(".md", "")
        desc = first_paragraph(path)
        status = fm.get("status", "unknown")
        langs = fm.get("language_stack", [])
        domains = fm.get("domain", [])
        rels = fm.get("project_relationships", [])

        if isinstance(langs, str):
            langs = [langs]
        if isinstance(domains, str):
            domains = [domains]

        # Build wiki page content
        lines = [
            "---",
            f"title: {name}",
            f"source_document: project_{name}",
            f"tags: [summary, project, {', '.join(langs)}]",
            "---",
            "",
            f"# {name}",
            "",
            f"**Status:** {status}",
            f"**Languages:** {', '.join(langs)}",
            f"**Domains:** {', '.join(domains)}",
            "",
            f"{desc}",
            "",
        ]

        if rels:
            lines.append("## Relationships")
            lines.append("")
            for rel in rels:
                rtype = rel.get("type", "related_to")
                target = rel.get("target", "")
                notes = rel.get("notes", "")
                lines.append(
                    f"- [[{target}]] ({rtype}): {notes}"
                )
            lines.append("")

        repo_url = ""
        repo = fm.get("repo", {})
        if isinstance(repo, dict):
            repo_url = repo.get("url", "")

        lines.append("## References")
        lines.append("")
        if repo_url:
            lines.append(f"- Repo: {repo_url}")
        lines.append(f"- Source note: {path}")
        lines.append("")

        content = "\n".join(lines)
        summary_text = f"{name}: {desc} [{status}]"
        kgx_wiki_write("summary", name, summary_text, content)

    print("Done writing project summaries.", file=sys.stderr)


if __name__ == "__main__":
    main()
