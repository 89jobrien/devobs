#!/usr/bin/env python3
"""Parse PROJECT notes and emit kgx ingest JSON payloads to stdout.

Each line is a complete JSON object ready to pipe into `kgx ingest`.
"""
import glob
import json
import re
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
    # Skip frontmatter
    m = FRONT_MATTER_RE.match(text)
    body = text[m.end():] if m else text
    for line in body.splitlines():
        line = line.strip()
        if line and not line.startswith("#"):
            return line
    return ""


def build_payload(path: str, fm: dict) -> dict:
    name = path.split("/")[-1].replace("PROJECT.", "").replace(".md", "")
    desc = first_paragraph(path)
    langs = fm.get("language_stack", [])
    domains = fm.get("domain", [])
    status = fm.get("status", "unknown")

    if isinstance(langs, str):
        langs = [langs]
    if isinstance(domains, str):
        domains = [domains]

    entities = [
        {"name": name, "type": "project", "supporting_text": desc},
    ]
    for lang in langs:
        entities.append({
            "name": lang,
            "type": "language",
            "supporting_text": f"Language used by {name}",
        })
    for domain in domains:
        entities.append({
            "name": domain,
            "type": "domain",
            "supporting_text": f"Domain of {name}",
        })

    relations = []
    for lang in langs:
        relations.append({
            "source": name,
            "target": lang,
            "type": "uses_language",
            "confidence": 1.0,
            "supporting_text": f"{name} uses {lang}",
        })
    for domain in domains:
        relations.append({
            "source": name,
            "target": domain,
            "type": "belongs_to_domain",
            "confidence": 1.0,
            "supporting_text": f"{name} belongs to {domain} domain",
        })

    # Cross-project relations from frontmatter
    for rel in fm.get("project_relationships", []):
        rel_type = rel.get("type", "related_to")
        target = rel.get("target", "")
        if target:
            relations.append({
                "source": name,
                "target": target,
                "type": rel_type,
                "confidence": 0.9,
                "supporting_text": f"{name} {rel_type} {target}",
            })

    raw = f"{desc} Status: {status}. Languages: {', '.join(langs)}."
    return {
        "doc_id": f"project_{name}",
        "title": f"PROJECT.{name}",
        "source": path,
        "raw_content": raw,
        "entities": entities,
        "relations": relations,
    }


def main():
    files = sorted(glob.glob("02_Projects/PROJECT.*.md"))
    if not files:
        print("No PROJECT notes found", file=sys.stderr)
        sys.exit(1)

    for path in files:
        fm = parse_frontmatter(path)
        if fm is None:
            print(f"Skipping {path}: no valid frontmatter", file=sys.stderr)
            continue
        payload = build_payload(path, fm)
        print(json.dumps(payload))


if __name__ == "__main__":
    main()
