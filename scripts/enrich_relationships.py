#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.12"
# dependencies = ["openai", "pyyaml"]
# ///
"""Use Claude to suggest cross-project relationships not in Cargo.toml.

Outputs candidates to stdout as JSON. Does NOT auto-ingest — human reviews
first, then approved candidates are added via `kgx graph add-edge`.

Requires: ANTHROPIC_API_KEY in environment.
"""
import glob
import json
import os
import re
import subprocess
import sys
import yaml

import openai

FRONT_MATTER_RE = re.compile(r"^---\s*\n(.*?)\n---\s*\n", re.DOTALL)

VALID_REL_TYPES = [
    "depends_on", "bootstraps", "observes", "consumes_from", "feeds_into",
    "adjacent_to", "sibling_of", "replaces", "replaced_by", "complements",
    "informed_by", "informs",
]


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


def gather_projects() -> list[dict]:
    projects = []
    for path in sorted(glob.glob("02_Projects/PROJECT.*.md")):
        fm = parse_frontmatter(path)
        if fm is None:
            continue
        name = path.split("/")[-1].replace("PROJECT.", "").replace(".md", "")
        desc = first_paragraph(path)
        langs = fm.get("language_stack", [])
        domains = fm.get("domain", [])
        rels = fm.get("project_relationships", [])

        if isinstance(langs, str):
            langs = [langs]
        if isinstance(domains, str):
            domains = [domains]

        existing_targets = {r.get("target") for r in rels}

        projects.append({
            "name": name,
            "description": desc,
            "languages": langs,
            "domains": domains,
            "existing_relationships": [
                {"type": r["type"], "target": r["target"]}
                for r in rels
            ],
            "existing_targets": list(existing_targets),
        })
    return projects


def call_llm(projects: list[dict]) -> list[dict]:
    client = openai.OpenAI()

    project_summaries = "\n".join(
        f"- **{p['name']}**: {p['description']} "
        f"[{', '.join(p['languages'])}] "
        f"domains: {', '.join(p['domains'])} "
        f"existing rels: {json.dumps(p['existing_relationships'])}"
        for p in projects
    )

    prompt = f"""You are analyzing a portfolio of {len(projects)} open-source
projects by the same developer. Your job is to suggest NEW relationships
between projects that are not already captured.

## Projects

{project_summaries}

## Valid relationship types

{json.dumps(VALID_REL_TYPES)}

## Rules

1. Only suggest relationships between projects in this list.
2. Do NOT repeat relationships that already exist (check existing rels).
3. Each suggestion needs a confidence score (0.0-1.0). Only suggest >= 0.6.
4. Provide a brief rationale for each suggestion.
5. Focus on conceptual/architectural connections, shared patterns, or
   workflow integrations — not just "both use Rust".
6. Be conservative. 10-20 high-quality suggestions > 50 weak ones.

## Output format

Return a JSON array of objects:
```json
[
  {{
    "source": "project_a",
    "target": "project_b",
    "type": "relationship_type",
    "confidence": 0.85,
    "rationale": "Why this relationship exists"
  }}
]
```

Return ONLY the JSON array, no other text."""

    response = client.chat.completions.create(
        model="gpt-5.5",
        max_completion_tokens=4096,
        messages=[{"role": "user", "content": prompt}],
    )

    text = response.choices[0].message.content.strip()
    # Extract JSON from response (handle markdown code blocks)
    if text.startswith("```"):
        text = re.sub(r"^```\w*\n", "", text)
        text = re.sub(r"\n```$", "", text)

    return json.loads(text)


def main():
    projects = gather_projects()
    print(f"Gathered {len(projects)} projects", file=sys.stderr)

    suggestions = call_llm(projects)
    print(f"Got {len(suggestions)} suggestions from Claude",
          file=sys.stderr)

    # Validate suggestions
    project_names = {p["name"] for p in projects}
    valid = []
    for s in suggestions:
        src = s.get("source", "")
        tgt = s.get("target", "")
        rtype = s.get("type", "")
        conf = s.get("confidence", 0)

        if src not in project_names:
            print(f"  SKIP: unknown source '{src}'", file=sys.stderr)
            continue
        if tgt not in project_names:
            print(f"  SKIP: unknown target '{tgt}'", file=sys.stderr)
            continue
        if rtype not in VALID_REL_TYPES:
            print(f"  SKIP: invalid type '{rtype}'", file=sys.stderr)
            continue
        if conf < 0.6:
            print(f"  SKIP: low confidence {conf} for {src}->{tgt}",
                  file=sys.stderr)
            continue

        # Check not already existing
        proj = next(p for p in projects if p["name"] == src)
        if tgt in proj["existing_targets"]:
            print(f"  SKIP: {src}->{tgt} already exists", file=sys.stderr)
            continue

        valid.append(s)

    print(f"\n{len(valid)} valid candidates (of {len(suggestions)} total)",
          file=sys.stderr)
    print(json.dumps(valid, indent=2))


if __name__ == "__main__":
    main()
