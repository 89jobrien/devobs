#!/usr/bin/env python3
"""Generate kgx wiki topic pages for each domain cluster."""
import glob
import json
import re
import subprocess
import sys
from collections import defaultdict
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

    # Collect domain -> projects and language -> projects
    domain_map = defaultdict(list)
    lang_map = defaultdict(list)

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

        entry = {
            "name": name,
            "desc": desc,
            "status": status,
            "langs": langs,
            "rels": rels,
        }

        for d in domains:
            domain_map[d].append(entry)
        for l in langs:
            lang_map[l].append(entry)

    # Write domain cluster pages
    for domain, projects in sorted(domain_map.items()):
        active = [p for p in projects if p["status"] == "active"]
        inactive = [p for p in projects if p["status"] != "active"]

        lines = [
            "---",
            f"title: Domain - {domain}",
            f"tags: [topic, domain, {domain}]",
            "---",
            "",
            f"# {domain}",
            "",
            f"{len(projects)} projects in this domain "
            f"({len(active)} active, {len(inactive)} inactive/archived).",
            "",
            "## Active Projects",
            "",
        ]

        for p in sorted(active, key=lambda x: x["name"]):
            lines.append(f"- **[[{p['name']}]]** — {p['desc']}")

        if inactive:
            lines.append("")
            lines.append("## Inactive / Archived")
            lines.append("")
            for p in sorted(inactive, key=lambda x: x["name"]):
                lines.append(f"- [[{p['name']}]] — {p['desc']}")

        # Cross-domain relationships within cluster
        internal_rels = []
        cluster_names = {p["name"] for p in projects}
        for p in projects:
            for rel in p.get("rels", []):
                target = rel.get("target", "")
                if target in cluster_names:
                    internal_rels.append(
                        (p["name"], rel["type"], target, rel.get("notes", ""))
                    )

        if internal_rels:
            lines.append("")
            lines.append("## Internal Relationships")
            lines.append("")
            seen = set()
            for src, rtype, tgt, notes in internal_rels:
                key = (src, rtype, tgt)
                if key not in seen:
                    seen.add(key)
                    lines.append(f"- [[{src}]] --{rtype}--> [[{tgt}]]: {notes}")

        lines.append("")
        content = "\n".join(lines)
        summary_text = (
            f"{domain} domain: {len(projects)} projects "
            f"({len(active)} active)"
        )
        kgx_wiki_write("topic", f"domain-{domain}", summary_text, content)

    # Write language ecosystem pages
    for lang, projects in sorted(lang_map.items()):
        active = [p for p in projects if p["status"] == "active"]
        inactive = [p for p in projects if p["status"] != "active"]

        lines = [
            "---",
            f"title: Language - {lang}",
            f"tags: [topic, language, {lang}]",
            "---",
            "",
            f"# {lang} ecosystem",
            "",
            f"{len(projects)} projects using {lang} "
            f"({len(active)} active, {len(inactive)} inactive/archived).",
            "",
            "## Active Projects",
            "",
        ]

        for p in sorted(active, key=lambda x: x["name"]):
            domains = ", ".join(p.get("langs", []))
            lines.append(f"- **[[{p['name']}]]** — {p['desc']}")

        if inactive:
            lines.append("")
            lines.append("## Inactive / Archived")
            lines.append("")
            for p in sorted(inactive, key=lambda x: x["name"]):
                lines.append(f"- [[{p['name']}]] — {p['desc']}")

        lines.append("")
        content = "\n".join(lines)
        summary_text = (
            f"{lang} ecosystem: {len(projects)} projects "
            f"({len(active)} active)"
        )
        kgx_wiki_write("topic", f"lang-{lang}", summary_text, content)

    print("Done writing domain and language pages.", file=sys.stderr)


if __name__ == "__main__":
    main()
