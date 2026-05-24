#!/usr/bin/env python3
"""Lint devobs vault: frontmatter validation, schema checks, consistency."""
import glob
import json
import os
import re
import sys
import yaml

FRONT_MATTER_RE = re.compile(r"^---\s*\n(.*?)\n---\s*\n", re.DOTALL)

REQUIRED_FIELDS = ["type", "status", "language_stack", "domain", "repo",
                    "primary_machine", "tags"]
VALID_STATUSES = {"active", "inactive", "archived"}
VALID_LANGUAGES = {"rust", "go", "python", "shell", "nix", "typescript",
                   "javascript", "nushell"}
VALID_REL_TYPES = {
    "depends_on", "bootstraps", "observes", "consumes_from", "feeds_into",
    "adjacent_to", "sibling_of", "replaces", "replaced_by", "complements",
    "informed_by", "informs", "indexes", "indexed_by",
    "aligns_schema_with", "control_plane_for", "provisioned_by",
}


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


def lint_project(path: str, fm: dict) -> list[dict]:
    findings = []
    name = path.split("/")[-1].replace("PROJECT.", "").replace(".md", "")

    def finding(severity: str, msg: str):
        findings.append({"file": path, "project": name,
                         "severity": severity, "message": msg})

    # Required fields
    for field in REQUIRED_FIELDS:
        if field not in fm:
            finding("error", f"missing required field: {field}")

    # Status validation
    status = fm.get("status")
    if status and status not in VALID_STATUSES:
        finding("error", f"invalid status '{status}' "
                f"(expected: {', '.join(sorted(VALID_STATUSES))})")

    # Language stack validation
    langs = fm.get("language_stack", [])
    if isinstance(langs, str):
        langs = [langs]
    for lang in langs:
        if lang not in VALID_LANGUAGES:
            finding("warning",
                    f"unknown language '{lang}' in language_stack")

    # Repo validation
    repo = fm.get("repo", {})
    if isinstance(repo, dict):
        url = repo.get("url", "")
        repo_path = repo.get("path", "")
        if url and "89jobrien" not in url:
            finding("warning", f"repo URL doesn't contain 89jobrien: {url}")
        if repo_path:
            expanded = os.path.expanduser(repo_path)
            if not os.path.isdir(expanded):
                finding("warning", f"repo path does not exist: {repo_path}")
    elif repo:
        finding("warning", "repo field should be a mapping with url/path")

    # Relationship validation
    rels = fm.get("project_relationships", [])
    if isinstance(rels, list):
        for rel in rels:
            rtype = rel.get("type", "")
            target = rel.get("target", "")
            if rtype and rtype not in VALID_REL_TYPES:
                finding("warning",
                        f"unknown relationship type '{rtype}' -> {target}")
            if not target:
                finding("error", "relationship missing target")

    # Domain should be a list
    domains = fm.get("domain")
    if domains is not None and not isinstance(domains, list):
        finding("warning", "domain should be a list")

    return findings


def main():
    files = sorted(glob.glob("02_Projects/PROJECT.*.md"))
    if not files:
        print("No PROJECT notes found", file=sys.stderr)
        sys.exit(1)

    all_findings = []
    project_names = set()

    for path in files:
        name = path.split("/")[-1].replace("PROJECT.", "").replace(".md", "")
        project_names.add(name)

        fm = parse_frontmatter(path)
        if fm is None:
            all_findings.append({
                "file": path, "project": name,
                "severity": "error",
                "message": "no valid YAML frontmatter found",
            })
            continue

        all_findings.extend(lint_project(path, fm))

    # Cross-reference: relationship targets should exist as projects
    for path in files:
        fm = parse_frontmatter(path)
        if fm is None:
            continue
        name = path.split("/")[-1].replace("PROJECT.", "").replace(".md", "")
        for rel in fm.get("project_relationships", []):
            target = rel.get("target", "")
            if target and target not in project_names:
                all_findings.append({
                    "file": path, "project": name,
                    "severity": "warning",
                    "message": f"relationship target '{target}' has no "
                               f"PROJECT note",
                })

    # Output
    errors = [f for f in all_findings if f["severity"] == "error"]
    warnings = [f for f in all_findings if f["severity"] == "warning"]

    print(json.dumps({
        "total_projects": len(files),
        "errors": len(errors),
        "warnings": len(warnings),
        "findings": all_findings,
    }, indent=2))

    if errors:
        print(f"\nLINT FAILED: {len(errors)} error(s), "
              f"{len(warnings)} warning(s)", file=sys.stderr)
        sys.exit(1)
    elif warnings:
        print(f"\nLINT PASSED with {len(warnings)} warning(s)",
              file=sys.stderr)
    else:
        print("\nLINT PASSED: all clean", file=sys.stderr)


if __name__ == "__main__":
    main()
