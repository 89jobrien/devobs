#!/usr/bin/env python3
"""Apply approved enrichment candidates to PROJECT note frontmatter.

Reads candidates from .ctx/enrich_candidates.json, adds them as
project_relationships entries in the corresponding PROJECT notes.
"""
import glob
import json
import re
import sys
import yaml

FRONT_MATTER_RE = re.compile(r"^---\s*\n(.*?)\n---\s*\n", re.DOTALL)


def load_candidates(path: str) -> list[dict]:
    with open(path) as f:
        text = f.read()
    # Strip ANSI escape codes from dotenvx output
    text = re.sub(r"\x1b\[[0-9;]*m", "", text)
    # Find the first '[' — skip any non-JSON preamble lines
    idx = text.find("[")
    if idx == -1:
        print("No JSON array found in candidates file", file=sys.stderr)
        sys.exit(1)
    return json.loads(text[idx:])


def update_project_file(path: str, new_rels: list[dict]) -> bool:
    with open(path) as f:
        text = f.read()

    m = FRONT_MATTER_RE.match(text)
    if not m:
        return False

    fm = yaml.safe_load(m.group(1))
    if fm is None:
        return False

    existing = fm.get("project_relationships", [])
    existing_keys = {
        (r.get("type"), r.get("target")) for r in existing
    }

    added = 0
    for rel in new_rels:
        key = (rel["type"], rel["target"])
        if key not in existing_keys:
            existing.append({
                "type": rel["type"],
                "target": rel["target"],
                "notes": rel.get("rationale", "LLM-suggested"),
            })
            existing_keys.add(key)
            added += 1

    if added == 0:
        return False

    fm["project_relationships"] = existing

    # Rebuild the file
    new_fm = yaml.dump(fm, default_flow_style=False, sort_keys=False,
                       allow_unicode=True)
    body = text[m.end():]
    new_text = f"---\n{new_fm}---\n{body}"

    with open(path, "w") as f:
        f.write(new_text)

    return True


def main():
    candidates_path = ".ctx/enrich_candidates.json"
    candidates = load_candidates(candidates_path)
    print(f"Loaded {len(candidates)} candidates", file=sys.stderr)

    # Group by source project
    by_source: dict[str, list[dict]] = {}
    for c in candidates:
        src = c["source"]
        by_source.setdefault(src, []).append(c)

    updated = 0
    for src, rels in sorted(by_source.items()):
        path = f"02_Projects/PROJECT.{src}.md"
        try:
            if update_project_file(path, rels):
                print(f"  updated: {src} (+{len(rels)} rels)",
                      file=sys.stderr)
                updated += 1
            else:
                print(f"  skipped: {src} (no new rels)", file=sys.stderr)
        except FileNotFoundError:
            print(f"  ERROR: {path} not found", file=sys.stderr)

    print(f"\nUpdated {updated} PROJECT notes", file=sys.stderr)


if __name__ == "__main__":
    main()
