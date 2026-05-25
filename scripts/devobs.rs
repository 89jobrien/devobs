#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! clap = { version = "4", features = ["derive"] }
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! serde_yaml = "0.9"
//! glob = "0.3"
//! regex = "1"
//! reqwest = { version = "0.12", features = ["blocking", "json"] }
//! ```

use clap::{Parser, Subcommand};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

// ── Config ──────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct Config {
    projects_glob: String,
    candidates_path: String,
    required_fields: Vec<String>,
    valid_statuses: Vec<String>,
    valid_languages: Vec<String>,
    valid_relationship_types: Vec<String>,
    enrich: EnrichConfig,
}

#[derive(Deserialize)]
struct EnrichConfig {
    model: String,
    min_confidence: f64,
    github_owner: String,
}

fn load_config() -> Config {
    let script_dir = Path::new(file!()).parent().unwrap_or(Path::new("."));
    let candidates = [
        script_dir.join("devobs.yaml"),
        std::path::PathBuf::from("scripts/devobs.yaml"),
    ];
    for p in &candidates {
        if p.exists() {
            let text = fs::read_to_string(p)
                .unwrap_or_else(|e| panic!("read {}: {e}", p.display()));
            return serde_yaml::from_str(&text)
                .unwrap_or_else(|e| panic!("parse {}: {e}", p.display()));
        }
    }
    panic!("devobs.yaml not found");
}

// ── Frontmatter helpers ─────────────────────────────────────────────

fn frontmatter_re() -> Regex {
    Regex::new(r"(?s)\A---\s*\n(.*?)\n---\s*\n").unwrap()
}

fn parse_frontmatter(path: &str) -> Option<serde_yaml::Value> {
    let text = fs::read_to_string(path).ok()?;
    let re = frontmatter_re();
    let caps = re.captures(&text)?;
    serde_yaml::from_str(caps.get(1)?.as_str()).ok()
}

fn first_paragraph(path: &str) -> String {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return String::new(),
    };
    let re = frontmatter_re();
    let body = match re.find(&text) {
        Some(m) => &text[m.end()..],
        None => &text,
    };
    for line in body.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            return trimmed.to_string();
        }
    }
    String::new()
}

fn project_name(path: &str) -> String {
    Path::new(path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .replace("PROJECT.", "")
        .replace(".md", "")
}

fn as_string_vec(val: &serde_yaml::Value, key: &str) -> Vec<String> {
    match val.get(key) {
        Some(serde_yaml::Value::Sequence(seq)) => seq
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
        Some(serde_yaml::Value::String(s)) => vec![s.clone()],
        _ => vec![],
    }
}

fn project_files(cfg: &Config) -> Vec<String> {
    let mut files: Vec<String> = glob::glob(&cfg.projects_glob)
        .expect("bad glob")
        .filter_map(|e| e.ok())
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    files.sort();
    files
}

// ── CLI ─────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "devobs", about = "devobs vault tooling")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Emit kgx ingest JSON payloads to stdout
    Ingest,
    /// Validate frontmatter, cross-refs, and consistency
    Lint,
    /// Call LLM to suggest new project relationships
    Enrich,
    /// Apply approved enrichment candidates to PROJECT frontmatter
    Apply,
    /// Generate kgx wiki summary pages per project
    WikiSummaries,
    /// Generate kgx wiki domain/language cluster pages
    WikiDomains,
}

fn main() {
    let cli = Cli::parse();
    let cfg = load_config();
    match cli.cmd {
        Cmd::Ingest => cmd_ingest(&cfg),
        Cmd::Lint => cmd_lint(&cfg),
        Cmd::Enrich => cmd_enrich(&cfg),
        Cmd::Apply => cmd_apply(&cfg),
        Cmd::WikiSummaries => cmd_wiki_summaries(&cfg),
        Cmd::WikiDomains => cmd_wiki_domains(&cfg),
    }
}

// ── ingest ──────────────────────────────────────────────────────────

#[derive(Serialize)]
struct IngestPayload {
    doc_id: String,
    title: String,
    source: String,
    raw_content: String,
    entities: Vec<serde_json::Value>,
    relations: Vec<serde_json::Value>,
}

fn cmd_ingest(cfg: &Config) {
    let files = project_files(cfg);
    if files.is_empty() {
        eprintln!("No PROJECT notes found");
        std::process::exit(1);
    }
    for path in &files {
        let fm = match parse_frontmatter(path) {
            Some(f) => f,
            None => {
                eprintln!("Skipping {path}: no valid frontmatter");
                continue;
            }
        };
        let name = project_name(path);
        let desc = first_paragraph(path);
        let langs = as_string_vec(&fm, "language_stack");
        let domains = as_string_vec(&fm, "domain");
        let status = fm
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let mut entities = vec![serde_json::json!({
            "name": name,
            "type": "project",
            "supporting_text": desc,
        })];
        let mut relations = vec![];

        for lang in &langs {
            entities.push(serde_json::json!({
                "name": lang,
                "type": "language",
                "supporting_text": format!("Language used by {name}"),
            }));
            relations.push(serde_json::json!({
                "source": name,
                "target": lang,
                "type": "uses_language",
                "confidence": 1.0,
                "supporting_text": format!("{name} uses {lang}"),
            }));
        }
        for domain in &domains {
            entities.push(serde_json::json!({
                "name": domain,
                "type": "domain",
                "supporting_text": format!("Domain of {name}"),
            }));
            relations.push(serde_json::json!({
                "source": name,
                "target": domain,
                "type": "belongs_to_domain",
                "confidence": 1.0,
                "supporting_text": format!("{name} belongs to {domain} domain"),
            }));
        }

        if let Some(serde_yaml::Value::Sequence(rels)) =
            fm.get("project_relationships")
        {
            for rel in rels {
                let rtype = rel
                    .get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("related_to");
                let target =
                    rel.get("target").and_then(|v| v.as_str()).unwrap_or("");
                if !target.is_empty() {
                    relations.push(serde_json::json!({
                        "source": name,
                        "target": target,
                        "type": rtype,
                        "confidence": 0.9,
                        "supporting_text": format!("{name} {rtype} {target}"),
                    }));
                }
            }
        }

        let raw = format!(
            "{desc} Status: {status}. Languages: {}.",
            langs.join(", ")
        );
        let payload = IngestPayload {
            doc_id: format!("project_{name}"),
            title: format!("PROJECT.{name}"),
            source: path.clone(),
            raw_content: raw,
            entities,
            relations,
        };
        println!("{}", serde_json::to_string(&payload).unwrap());
    }
}

// ── lint ────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct LintFinding {
    file: String,
    project: String,
    severity: String,
    message: String,
}

fn cmd_lint(cfg: &Config) {
    let files = project_files(cfg);
    if files.is_empty() {
        eprintln!("No PROJECT notes found");
        std::process::exit(1);
    }
    let valid_statuses: BTreeSet<&str> =
        cfg.valid_statuses.iter().map(|s| s.as_str()).collect();
    let valid_languages: BTreeSet<&str> =
        cfg.valid_languages.iter().map(|s| s.as_str()).collect();
    let valid_rel_types: BTreeSet<&str> =
        cfg.valid_relationship_types.iter().map(|s| s.as_str()).collect();

    let mut findings: Vec<LintFinding> = vec![];
    let mut project_names: BTreeSet<String> = BTreeSet::new();

    for path in &files {
        let name = project_name(path);
        project_names.insert(name.clone());

        let fm = match parse_frontmatter(path) {
            Some(f) => f,
            None => {
                findings.push(LintFinding {
                    file: path.clone(),
                    project: name,
                    severity: "error".into(),
                    message: "no valid YAML frontmatter found".into(),
                });
                continue;
            }
        };

        // Required fields
        for field in &cfg.required_fields {
            if fm.get(field.as_str()).is_none() {
                findings.push(LintFinding {
                    file: path.clone(),
                    project: name.clone(),
                    severity: "error".into(),
                    message: format!("missing required field: {field}"),
                });
            }
        }

        // Status
        if let Some(status) = fm.get("status").and_then(|v| v.as_str()) {
            if !valid_statuses.contains(status) {
                findings.push(LintFinding {
                    file: path.clone(),
                    project: name.clone(),
                    severity: "error".into(),
                    message: format!(
                        "invalid status '{status}' (expected: {})",
                        cfg.valid_statuses.join(", ")
                    ),
                });
            }
        }

        // Languages
        for lang in as_string_vec(&fm, "language_stack") {
            if !valid_languages.contains(lang.as_str()) {
                findings.push(LintFinding {
                    file: path.clone(),
                    project: name.clone(),
                    severity: "warning".into(),
                    message: format!(
                        "unknown language '{lang}' in language_stack"
                    ),
                });
            }
        }

        // Repo
        if let Some(repo) = fm.get("repo") {
            if let Some(map) = repo.as_mapping() {
                let url = map
                    .get(serde_yaml::Value::String("url".into()))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if !url.is_empty() && !url.contains(&cfg.enrich.github_owner) {
                    findings.push(LintFinding {
                        file: path.clone(),
                        project: name.clone(),
                        severity: "warning".into(),
                        message: format!(
                            "repo URL doesn't contain {}: {url}",
                            cfg.enrich.github_owner
                        ),
                    });
                }
                let repo_path = map
                    .get(serde_yaml::Value::String("path".into()))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if !repo_path.is_empty() {
                    let expanded = repo_path.replace("~/", &format!(
                        "{}/",
                        std::env::var("HOME").unwrap_or_default()
                    ));
                    if !Path::new(&expanded).is_dir() {
                        findings.push(LintFinding {
                            file: path.clone(),
                            project: name.clone(),
                            severity: "warning".into(),
                            message: format!(
                                "repo path does not exist: {repo_path}"
                            ),
                        });
                    }
                }
            } else if repo.as_str().is_some() {
                findings.push(LintFinding {
                    file: path.clone(),
                    project: name.clone(),
                    severity: "warning".into(),
                    message: "repo field should be a mapping with url/path"
                        .into(),
                });
            }
        }

        // Relationships
        if let Some(serde_yaml::Value::Sequence(rels)) =
            fm.get("project_relationships")
        {
            for rel in rels {
                let rtype =
                    rel.get("type").and_then(|v| v.as_str()).unwrap_or("");
                let target =
                    rel.get("target").and_then(|v| v.as_str()).unwrap_or("");
                if !rtype.is_empty() && !valid_rel_types.contains(rtype) {
                    findings.push(LintFinding {
                        file: path.clone(),
                        project: name.clone(),
                        severity: "warning".into(),
                        message: format!(
                            "unknown relationship type '{rtype}' -> {target}"
                        ),
                    });
                }
                if target.is_empty() {
                    findings.push(LintFinding {
                        file: path.clone(),
                        project: name.clone(),
                        severity: "error".into(),
                        message: "relationship missing target".into(),
                    });
                }
            }
        }

        // Domain should be a list
        if let Some(domain) = fm.get("domain") {
            if !domain.is_sequence() && !domain.is_null() {
                findings.push(LintFinding {
                    file: path.clone(),
                    project: name.clone(),
                    severity: "warning".into(),
                    message: "domain should be a list".into(),
                });
            }
        }
    }

    // Cross-reference: relationship targets should exist
    for path in &files {
        let fm = match parse_frontmatter(path) {
            Some(f) => f,
            None => continue,
        };
        let name = project_name(path);
        if let Some(serde_yaml::Value::Sequence(rels)) =
            fm.get("project_relationships")
        {
            for rel in rels {
                let target =
                    rel.get("target").and_then(|v| v.as_str()).unwrap_or("");
                if !target.is_empty()
                    && !project_names.contains(target)
                {
                    findings.push(LintFinding {
                        file: path.clone(),
                        project: name.clone(),
                        severity: "warning".into(),
                        message: format!(
                            "relationship target '{target}' has no PROJECT note"
                        ),
                    });
                }
            }
        }
    }

    let errors: Vec<&LintFinding> =
        findings.iter().filter(|f| f.severity == "error").collect();
    let warnings: Vec<&LintFinding> =
        findings.iter().filter(|f| f.severity == "warning").collect();

    let output = serde_json::json!({
        "total_projects": files.len(),
        "errors": errors.len(),
        "warnings": warnings.len(),
        "findings": findings,
    });
    println!("{}", serde_json::to_string_pretty(&output).unwrap());

    if !errors.is_empty() {
        eprintln!(
            "\nLINT FAILED: {} error(s), {} warning(s)",
            errors.len(),
            warnings.len()
        );
        std::process::exit(1);
    } else if !warnings.is_empty() {
        eprintln!("\nLINT PASSED with {} warning(s)", warnings.len());
    } else {
        eprintln!("\nLINT PASSED: all clean");
    }
}

// ── enrich ──────────────────────────────────────────────────────────

fn cmd_enrich(cfg: &Config) {
    let files = project_files(cfg);

    #[derive(Serialize)]
    struct ProjectSummary {
        name: String,
        description: String,
        languages: Vec<String>,
        domains: Vec<String>,
        existing_relationships: Vec<serde_json::Value>,
        existing_targets: Vec<String>,
    }

    let mut projects: Vec<ProjectSummary> = vec![];
    for path in &files {
        let fm = match parse_frontmatter(path) {
            Some(f) => f,
            None => continue,
        };
        let name = project_name(path);
        let desc = first_paragraph(path);
        let langs = as_string_vec(&fm, "language_stack");
        let domains = as_string_vec(&fm, "domain");

        let mut existing_rels = vec![];
        let mut existing_targets = BTreeSet::new();
        if let Some(serde_yaml::Value::Sequence(rels)) =
            fm.get("project_relationships")
        {
            for rel in rels {
                let rtype =
                    rel.get("type").and_then(|v| v.as_str()).unwrap_or("");
                let target =
                    rel.get("target").and_then(|v| v.as_str()).unwrap_or("");
                existing_rels.push(
                    serde_json::json!({"type": rtype, "target": target}),
                );
                existing_targets.insert(target.to_string());
            }
        }

        projects.push(ProjectSummary {
            name,
            description: desc,
            languages: langs,
            domains,
            existing_relationships: existing_rels,
            existing_targets: existing_targets.into_iter().collect(),
        });
    }

    eprintln!("Gathered {} projects", projects.len());

    let summaries: String = projects
        .iter()
        .map(|p| {
            format!(
                "- **{}**: {} [{}] domains: {} existing rels: {}",
                p.name,
                p.description,
                p.languages.join(", "),
                p.domains.join(", "),
                serde_json::to_string(&p.existing_relationships).unwrap(),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let rel_types_json =
        serde_json::to_string(&cfg.valid_relationship_types).unwrap();

    let prompt = format!(
        r#"You are analyzing a portfolio of {} open-source projects by the same developer. Your job is to suggest NEW relationships between projects that are not already captured.

## Projects

{summaries}

## Valid relationship types

{rel_types_json}

## Rules

1. Only suggest relationships between projects in this list.
2. Do NOT repeat relationships that already exist (check existing rels).
3. Each suggestion needs a confidence score (0.0-1.0). Only suggest >= {}.
4. Provide a brief rationale for each suggestion.
5. Focus on conceptual/architectural connections, shared patterns, or workflow integrations — not just "both use Rust".
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

Return ONLY the JSON array, no other text."#,
        projects.len(),
        cfg.enrich.min_confidence,
    );

    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set");

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&serde_json::json!({
            "model": cfg.enrich.model,
            "max_completion_tokens": 4096,
            "messages": [{"role": "user", "content": prompt}],
        }))
        .send()
        .expect("OpenAI request failed");

    let resp_json: serde_json::Value =
        resp.json().expect("failed to parse response");
    let text = resp_json["choices"][0]["message"]["content"]
        .as_str()
        .expect("no content in response")
        .trim();

    // Strip markdown code fences if present
    let json_text = if text.starts_with("```") {
        let re = Regex::new(r"(?s)^```\w*\n(.*)\n```$").unwrap();
        re.captures(text)
            .map(|c| c.get(1).unwrap().as_str())
            .unwrap_or(text)
    } else {
        text
    };

    let suggestions: Vec<serde_json::Value> =
        serde_json::from_str(json_text).expect("failed to parse LLM JSON");
    eprintln!("Got {} suggestions from LLM", suggestions.len());

    let project_names: BTreeSet<&str> =
        projects.iter().map(|p| p.name.as_str()).collect();

    let mut valid = vec![];
    for s in &suggestions {
        let src = s["source"].as_str().unwrap_or("");
        let tgt = s["target"].as_str().unwrap_or("");
        let rtype = s["type"].as_str().unwrap_or("");
        let conf = s["confidence"].as_f64().unwrap_or(0.0);

        if !project_names.contains(src) {
            eprintln!("  SKIP: unknown source '{src}'");
            continue;
        }
        if !project_names.contains(tgt) {
            eprintln!("  SKIP: unknown target '{tgt}'");
            continue;
        }
        if !cfg
            .valid_relationship_types
            .iter()
            .any(|t| t == rtype)
        {
            eprintln!("  SKIP: invalid type '{rtype}'");
            continue;
        }
        if conf < cfg.enrich.min_confidence {
            eprintln!("  SKIP: low confidence {conf} for {src}->{tgt}");
            continue;
        }

        let proj = projects.iter().find(|p| p.name == src).unwrap();
        if proj.existing_targets.iter().any(|t| t == tgt) {
            eprintln!("  SKIP: {src}->{tgt} already exists");
            continue;
        }

        valid.push(s.clone());
    }

    eprintln!(
        "\n{} valid candidates (of {} total)",
        valid.len(),
        suggestions.len()
    );
    println!("{}", serde_json::to_string_pretty(&valid).unwrap());
}

// ── apply ───────────────────────────────────────────────────────────

fn cmd_apply(cfg: &Config) {
    let text = fs::read_to_string(&cfg.candidates_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", cfg.candidates_path));

    // Strip ANSI codes and find JSON array start
    let ansi_re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    let clean = ansi_re.replace_all(&text, "");
    let idx = clean
        .find('[')
        .expect("No JSON array found in candidates file");
    let candidates: Vec<serde_json::Value> =
        serde_json::from_str(&clean[idx..])
            .expect("failed to parse candidates JSON");

    eprintln!("Loaded {} candidates", candidates.len());

    // Group by source
    let mut by_source: BTreeMap<String, Vec<&serde_json::Value>> =
        BTreeMap::new();
    for c in &candidates {
        let src = c["source"].as_str().unwrap_or("").to_string();
        by_source.entry(src).or_default().push(c);
    }

    let re = frontmatter_re();
    let mut updated = 0;

    for (src, rels) in &by_source {
        let path = format!("02_Projects/PROJECT.{src}.md");
        let file_text = match fs::read_to_string(&path) {
            Ok(t) => t,
            Err(_) => {
                eprintln!("  ERROR: {path} not found");
                continue;
            }
        };

        let caps = match re.captures(&file_text) {
            Some(c) => c,
            None => continue,
        };

        let mut fm: serde_yaml::Value =
            match serde_yaml::from_str(caps.get(1).unwrap().as_str()) {
                Ok(v) => v,
                Err(_) => continue,
            };

        let existing = fm
            .get("project_relationships")
            .and_then(|v| v.as_sequence())
            .cloned()
            .unwrap_or_default();

        let existing_keys: BTreeSet<(String, String)> = existing
            .iter()
            .map(|r| {
                (
                    r.get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    r.get("target")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                )
            })
            .collect();

        let mut new_rels = existing.clone();
        let mut added = 0;
        for rel in rels {
            let rtype = rel["type"].as_str().unwrap_or("");
            let target = rel["target"].as_str().unwrap_or("");
            let rationale =
                rel["rationale"].as_str().unwrap_or("LLM-suggested");
            let key = (rtype.to_string(), target.to_string());
            if !existing_keys.contains(&key) {
                let mut entry = serde_yaml::Mapping::new();
                entry.insert(
                    serde_yaml::Value::String("type".into()),
                    serde_yaml::Value::String(rtype.into()),
                );
                entry.insert(
                    serde_yaml::Value::String("target".into()),
                    serde_yaml::Value::String(target.into()),
                );
                entry.insert(
                    serde_yaml::Value::String("notes".into()),
                    serde_yaml::Value::String(rationale.into()),
                );
                new_rels.push(serde_yaml::Value::Mapping(entry));
                added += 1;
            }
        }

        if added == 0 {
            eprintln!("  skipped: {src} (no new rels)");
            continue;
        }

        fm["project_relationships"] =
            serde_yaml::Value::Sequence(new_rels);
        let new_fm = serde_yaml::to_string(&fm).unwrap();
        let body = &file_text[caps.get(0).unwrap().end()..];
        let new_text = format!("---\n{new_fm}---\n{body}");
        fs::write(&path, new_text)
            .unwrap_or_else(|e| panic!("write {path}: {e}"));
        eprintln!("  updated: {src} (+{added} rels)");
        updated += 1;
    }

    eprintln!("\nUpdated {updated} PROJECT notes");
}

// ── ASCII graph renderer ────────────────────────────────────────────

const MAX_GRAPH_NODES: usize = 15;
const MAX_LABEL_LEN: usize = 18;

struct GraphEdge {
    src: usize,
    tgt: usize,
    label: String,
}

fn truncate_label(s: &str) -> String {
    if s.len() <= MAX_LABEL_LEN {
        s.to_string()
    } else {
        format!("{}...", &s[..MAX_LABEL_LEN - 3])
    }
}

fn render_fallback(nodes: &[&str], edges: &[GraphEdge]) -> String {
    let mut lines = vec![];
    for e in edges {
        lines.push(format!(
            "{} -> {} [{}]",
            nodes[e.src], nodes[e.tgt], e.label
        ));
    }
    if lines.is_empty() {
        return String::new();
    }
    lines.join("\n")
}

/// Assign layers via BFS from roots (nodes with no incoming edges).
/// Returns node -> layer mapping.
fn assign_layers(
    node_count: usize,
    edges: &[GraphEdge],
) -> Vec<usize> {
    let mut in_degree = vec![0usize; node_count];
    let mut adj: Vec<Vec<usize>> = vec![vec![]; node_count];
    for e in edges {
        in_degree[e.tgt] += 1;
        adj[e.src].push(e.tgt);
    }

    let mut layers = vec![0usize; node_count];
    let mut queue: Vec<usize> = (0..node_count)
        .filter(|&i| in_degree[i] == 0)
        .collect();
    if queue.is_empty() {
        // Cycle — just use node order
        queue.push(0);
    }

    let mut visited = vec![false; node_count];
    let mut q_idx = 0;
    while q_idx < queue.len() {
        let n = queue[q_idx];
        q_idx += 1;
        if visited[n] {
            continue;
        }
        visited[n] = true;
        for &tgt in &adj[n] {
            let new_layer = layers[n] + 1;
            if new_layer > layers[tgt] {
                layers[tgt] = new_layer;
            }
            in_degree[tgt] -= 1;
            if in_degree[tgt] == 0 {
                queue.push(tgt);
            }
        }
    }
    // Assign unvisited nodes to layer 0
    for i in 0..node_count {
        if !visited[i] && !queue.contains(&i) {
            layers[i] = 0;
        }
    }
    layers
}

/// Find connected components in the graph (undirected connectivity).
fn connected_components(
    node_count: usize,
    edges: &[GraphEdge],
) -> Vec<Vec<usize>> {
    let mut adj: Vec<Vec<usize>> = vec![vec![]; node_count];
    for e in edges {
        adj[e.src].push(e.tgt);
        adj[e.tgt].push(e.src);
    }

    let mut visited = vec![false; node_count];
    let mut components = vec![];

    for start in 0..node_count {
        if visited[start] {
            continue;
        }
        let mut comp = vec![];
        let mut stack = vec![start];
        while let Some(n) = stack.pop() {
            if visited[n] {
                continue;
            }
            visited[n] = true;
            comp.push(n);
            for &neighbor in &adj[n] {
                if !visited[neighbor] {
                    stack.push(neighbor);
                }
            }
        }
        comp.sort();
        components.push(comp);
    }
    components
}

fn render_graph(nodes: &[&str], edges: &[GraphEdge]) -> String {
    if nodes.is_empty() || edges.is_empty() {
        return String::new();
    }
    if nodes.len() > MAX_GRAPH_NODES {
        return render_fallback(nodes, edges);
    }

    // Filter to only nodes that participate in at least one edge
    let mut active: BTreeSet<usize> = BTreeSet::new();
    for e in edges {
        active.insert(e.src);
        active.insert(e.tgt);
    }
    let active_list: Vec<usize> = active.iter().copied().collect();

    if active_list.is_empty() {
        return String::new();
    }

    // Build remapped node list and edges
    let remap: BTreeMap<usize, usize> = active_list
        .iter()
        .enumerate()
        .map(|(new, &old)| (old, new))
        .collect();
    let sub_nodes: Vec<&str> = active_list.iter().map(|&i| nodes[i]).collect();
    let sub_edges: Vec<GraphEdge> = edges
        .iter()
        .filter(|e| active.contains(&e.src) && active.contains(&e.tgt))
        .map(|e| GraphEdge {
            src: remap[&e.src],
            tgt: remap[&e.tgt],
            label: e.label.clone(),
        })
        .collect();

    // Count distinct sources
    let sources: BTreeSet<usize> = sub_edges.iter().map(|e| e.src).collect();

    // Multi-source graphs (cluster graphs) — split into per-source
    // subgraphs. Each gets its own box-and-arrow rendering. If the
    // total exceeds complexity, use fallback.
    if sources.len() > 1 {
        // Split into connected components, render simple ones with
        // boxes, complex ones with fallback
        let components = connected_components(sub_nodes.len(), &sub_edges);
        let mut parts = vec![];

        for comp in &components {
            let comp_remap: BTreeMap<usize, usize> = comp
                .iter()
                .enumerate()
                .map(|(new, &old)| (old, new))
                .collect();
            let comp_nodes: Vec<&str> =
                comp.iter().map(|&i| sub_nodes[i]).collect();
            let comp_edges: Vec<GraphEdge> = sub_edges
                .iter()
                .filter(|e| comp_remap.contains_key(&e.src))
                .map(|e| GraphEdge {
                    src: comp_remap[&e.src],
                    tgt: comp_remap[&e.tgt],
                    label: e.label.clone(),
                })
                .collect();

            if comp_edges.is_empty() {
                continue;
            }

            // Simple components (single source, <=5 nodes) get boxes
            let comp_sources: BTreeSet<usize> =
                comp_edges.iter().map(|e| e.src).collect();
            if comp_sources.len() == 1 && comp_nodes.len() <= 5 {
                let layers =
                    assign_layers(comp_nodes.len(), &comp_edges);
                let max_depth = *layers.iter().max().unwrap_or(&0);
                let mut layer_counts = vec![0usize; max_depth + 1];
                for &l in &layers {
                    layer_counts[l] += 1;
                }
                let max_width =
                    *layer_counts.iter().max().unwrap_or(&1);
                let rendered = if max_depth >= max_width {
                    render_td(&comp_nodes, &comp_edges, &layers)
                } else {
                    render_lr(&comp_nodes, &comp_edges, &layers)
                };
                if !rendered.is_empty() {
                    parts.push(rendered);
                }
            } else {
                // Complex component — use fallback
                parts.push(render_fallback(&comp_nodes, &comp_edges));
            }
        }
        return parts.join("\n\n");
    }

    // Single source graph (ego graphs) — full box-and-arrow
    let layers = assign_layers(sub_nodes.len(), &sub_edges);
    let max_depth = *layers.iter().max().unwrap_or(&0);

    let mut layer_counts = vec![0usize; max_depth + 1];
    for &l in &layers {
        layer_counts[l] += 1;
    }
    let max_width = *layer_counts.iter().max().unwrap_or(&1);

    if max_depth >= max_width {
        render_td(&sub_nodes, &sub_edges, &layers)
    } else {
        render_lr(&sub_nodes, &sub_edges, &layers)
    }
}

// ── LR renderer ─────────────────────────────────────────────────────

fn render_lr(
    nodes: &[&str],
    edges: &[GraphEdge],
    layers: &[usize],
) -> String {
    let max_layer = *layers.iter().max().unwrap_or(&0);

    // Group nodes by layer, sorted alphabetically within each layer
    let mut layer_nodes: Vec<Vec<usize>> = vec![vec![]; max_layer + 1];
    for (i, &l) in layers.iter().enumerate() {
        layer_nodes[l].push(i);
    }
    for ln in &mut layer_nodes {
        ln.sort_by_key(|&i| nodes[i]);
    }

    // Box dimensions: name padded with 1 space each side, 3 rows tall
    let box_width = |idx: usize| -> usize { nodes[idx].len() + 4 };

    // Compute column x-offsets for each layer
    let mut layer_x = vec![0usize; max_layer + 1];
    for l in 0..=max_layer {
        if l == 0 {
            layer_x[l] = 0;
        } else {
            // Max box width in previous layer + space for labels + gap
            let prev_max_w = layer_nodes[l - 1]
                .iter()
                .map(|&i| box_width(i))
                .max()
                .unwrap_or(6);
            // Find max label width for edges from layer l-1 to layer l
            let max_label = edges
                .iter()
                .filter(|e| layers[e.src] == l - 1 && layers[e.tgt] == l)
                .map(|e| truncate_label(&e.label).len())
                .max()
                .unwrap_or(0);
            layer_x[l] = layer_x[l - 1] + prev_max_w + max_label + 6;
        }
    }

    // Assign y-positions: each node gets 3 rows (box) + 1 row gap
    let mut node_y = vec![0usize; nodes.len()];
    for ln in &layer_nodes {
        let mut y = 0;
        for &idx in ln {
            node_y[idx] = y;
            y += 4; // 3 rows for box + 1 gap
        }
    }

    // Canvas size
    let total_width = layer_x.last().copied().unwrap_or(0)
        + layer_nodes
            .last()
            .and_then(|ln| ln.iter().map(|&i| box_width(i)).max())
            .unwrap_or(10)
        + 2;
    let total_height = nodes
        .iter()
        .enumerate()
        .map(|(i, _)| node_y[i] + 3)
        .max()
        .unwrap_or(3)
        + 1;

    let mut canvas = vec![vec![' '; total_width]; total_height];

    // Helper: draw text at position
    let put = |canvas: &mut Vec<Vec<char>>, y: usize, x: usize, s: &str| {
        for (i, ch) in s.chars().enumerate() {
            if y < canvas.len() && x + i < canvas[y].len() {
                canvas[y][x + i] = ch;
            }
        }
    };

    // Draw boxes
    for (idx, &name) in nodes.iter().enumerate() {
        let x = layer_x[layers[idx]];
        let y = node_y[idx];
        let w = box_width(idx);

        // Top border
        put(&mut canvas, y, x, "┌");
        for i in 1..w - 1 {
            put(&mut canvas, y, x + i, "─");
        }
        put(&mut canvas, y, x + w - 1, "┐");

        // Middle row with name
        put(&mut canvas, y + 1, x, "│");
        put(&mut canvas, y + 1, x + 1, &format!(" {name} "));
        put(&mut canvas, y + 1, x + w - 1, "│");

        // Bottom border
        put(&mut canvas, y + 2, x, "└");
        for i in 1..w - 1 {
            put(&mut canvas, y + 2, x + i, "─");
        }
        put(&mut canvas, y + 2, x + w - 1, "┘");
    }

    // Group edges by source for branching
    let mut edges_by_src: BTreeMap<usize, Vec<&GraphEdge>> = BTreeMap::new();
    for e in edges {
        edges_by_src.entry(e.src).or_default().push(e);
    }

    for (&src, src_edges) in &edges_by_src {
        let src_x = layer_x[layers[src]] + box_width(src);
        let src_y = node_y[src] + 1; // middle row

        if src_edges.len() == 1 {
            // Single edge: simple horizontal line
            let e = src_edges[0];
            let label = truncate_label(&e.label);
            let tgt_x = layer_x[layers[e.tgt]];
            let tgt_y = node_y[e.tgt] + 1;

            // Change right border to ├
            put(&mut canvas, src_y, src_x - 1, "├");

            if src_y == tgt_y {
                // Same row — straight line
                for x in src_x..tgt_x {
                    put(&mut canvas, src_y, x, "─");
                }
                put(&mut canvas, src_y, tgt_x - 1, ">");
                // Label above the line
                let label_x = src_x + 1;
                if src_y > 0 {
                    put(&mut canvas, src_y - 1, label_x, &label);
                }
            } else {
                // Different row — horizontal then vertical then horizontal
                let mid_x = src_x + 1;
                put(&mut canvas, src_y, src_x, "─");

                if tgt_y > src_y {
                    put(&mut canvas, src_y, mid_x, "┐");
                    for y in src_y + 1..tgt_y {
                        put(&mut canvas, y, mid_x, "│");
                    }
                    put(&mut canvas, tgt_y, mid_x, "└");
                } else {
                    put(&mut canvas, src_y, mid_x, "┘");
                    for y in tgt_y + 1..src_y {
                        put(&mut canvas, y, mid_x, "│");
                    }
                    put(&mut canvas, tgt_y, mid_x, "┌");
                }

                let tgt_x_pos = layer_x[layers[e.tgt]];
                for x in mid_x + 1..tgt_x_pos {
                    put(&mut canvas, tgt_y, x, "─");
                }
                put(&mut canvas, tgt_y, tgt_x_pos - 1, ">");
                // Label
                let label_x = mid_x + 2;
                if tgt_y > 0 {
                    put(&mut canvas, tgt_y - 1, label_x, &label);
                }
            }
        } else {
            // Multiple edges: branching from source
            put(&mut canvas, src_y, src_x - 1, "├");
            let rail_x = src_x + 1;
            put(&mut canvas, src_y, src_x, "─");

            let mut target_ys: Vec<(usize, &GraphEdge)> = src_edges
                .iter()
                .map(|&e| (node_y[e.tgt] + 1, e))
                .collect();
            target_ys.sort_by_key(|&(y, _)| y);

            // Draw vertical rail
            let min_y = target_ys.first().map(|&(y, _)| y).unwrap_or(src_y);
            let max_y = target_ys.last().map(|&(y, _)| y).unwrap_or(src_y);
            let rail_start = src_y.min(min_y);
            let rail_end = src_y.max(max_y);

            for y in rail_start..=rail_end {
                put(&mut canvas, y, rail_x, "│");
            }

            // Junction at source row
            if src_y == rail_start {
                put(&mut canvas, src_y, rail_x, "┬");
            } else if src_y == rail_end {
                put(&mut canvas, src_y, rail_x, "┴");
            } else {
                put(&mut canvas, src_y, rail_x, "┤");
            }

            for (i, &(tgt_y, e)) in target_ys.iter().enumerate() {
                let label = truncate_label(&e.label);
                let tgt_x = layer_x[layers[e.tgt]];

                // Junction character on the rail
                if tgt_y == rail_start && tgt_y != src_y {
                    put(&mut canvas, tgt_y, rail_x, "┌");
                } else if i == target_ys.len() - 1 && tgt_y == rail_end {
                    put(&mut canvas, tgt_y, rail_x, "└");
                } else if tgt_y != src_y {
                    put(&mut canvas, tgt_y, rail_x, "├");
                }

                // Horizontal line to target
                for x in rail_x + 1..tgt_x {
                    put(&mut canvas, tgt_y, x, "─");
                }
                put(&mut canvas, tgt_y, tgt_x - 1, ">");

                // Label above connector
                if tgt_y > 0 {
                    put(&mut canvas, tgt_y - 1, rail_x + 2, &label);
                }
            }
        }
    }

    // Convert canvas to string, trimming trailing spaces per line
    canvas
        .iter()
        .map(|row| row.iter().collect::<String>().trim_end().to_string())
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_string()
}

// ── TD renderer ─────────────────────────────────────────────────────

fn render_td(
    nodes: &[&str],
    edges: &[GraphEdge],
    layers: &[usize],
) -> String {
    let max_layer = *layers.iter().max().unwrap_or(&0);

    // Group nodes by layer
    let mut layer_nodes: Vec<Vec<usize>> = vec![vec![]; max_layer + 1];
    for (i, &l) in layers.iter().enumerate() {
        layer_nodes[l].push(i);
    }
    for ln in &mut layer_nodes {
        ln.sort_by_key(|&i| nodes[i]);
    }

    let box_width = |idx: usize| -> usize { nodes[idx].len() + 4 };
    let box_height = 3usize;

    // Compute x-position for each node within its layer
    // Nodes in each layer are spread horizontally with 4-char gaps
    let mut node_x = vec![0usize; nodes.len()];
    let mut layer_widths = vec![0usize; max_layer + 1];
    for (l, ln) in layer_nodes.iter().enumerate() {
        let mut x = 0;
        for &idx in ln {
            node_x[idx] = x;
            x += box_width(idx) + 4;
        }
        layer_widths[l] = if x >= 4 { x - 4 } else { 0 };
    }

    // Center layers relative to the widest
    let max_total_width = *layer_widths.iter().max().unwrap_or(&0);
    for (l, ln) in layer_nodes.iter().enumerate() {
        let offset = (max_total_width.saturating_sub(layer_widths[l])) / 2;
        for &idx in ln {
            node_x[idx] += offset;
        }
    }

    // Y positions: each layer is box_height + 3 (connector space) apart
    let layer_spacing = box_height + 3;
    let mut node_y = vec![0usize; nodes.len()];
    for (i, &l) in layers.iter().enumerate() {
        node_y[i] = l * layer_spacing;
    }

    // Account for labels placed beside vertical connectors (src_cx + 2 + label)
    let max_label_overflow = edges
        .iter()
        .map(|e| {
            let src_cx = node_x[e.src] + box_width(e.src) / 2;
            src_cx + 2 + truncate_label(&e.label).len() + 1
        })
        .max()
        .unwrap_or(0);
    let total_width = nodes
        .iter()
        .enumerate()
        .map(|(i, _)| node_x[i] + box_width(i))
        .max()
        .unwrap_or(10)
        .max(max_label_overflow)
        + 2;
    let total_height = nodes
        .iter()
        .enumerate()
        .map(|(i, _)| node_y[i] + box_height)
        .max()
        .unwrap_or(3)
        + 1;

    let mut canvas = vec![vec![' '; total_width]; total_height];

    let put = |canvas: &mut Vec<Vec<char>>, y: usize, x: usize, s: &str| {
        for (i, ch) in s.chars().enumerate() {
            if y < canvas.len() && x + i < canvas[y].len() {
                canvas[y][x + i] = ch;
            }
        }
    };

    // Draw boxes
    for (idx, &name) in nodes.iter().enumerate() {
        let x = node_x[idx];
        let y = node_y[idx];
        let w = box_width(idx);

        put(&mut canvas, y, x, "┌");
        for i in 1..w - 1 {
            put(&mut canvas, y, x + i, "─");
        }
        put(&mut canvas, y, x + w - 1, "┐");

        put(&mut canvas, y + 1, x, "│");
        put(&mut canvas, y + 1, x + 1, &format!(" {name} "));
        put(&mut canvas, y + 1, x + w - 1, "│");

        put(&mut canvas, y + 2, x, "└");
        for i in 1..w - 1 {
            put(&mut canvas, y + 2, x + i, "─");
        }
        put(&mut canvas, y + 2, x + w - 1, "┘");
    }

    // Group edges by source
    let mut edges_by_src: BTreeMap<usize, Vec<&GraphEdge>> = BTreeMap::new();
    for e in edges {
        edges_by_src.entry(e.src).or_default().push(e);
    }

    for (&src, src_edges) in &edges_by_src {
        let src_cx = node_x[src] + box_width(src) / 2; // center x of source
        let src_bot = node_y[src] + box_height; // bottom of source box

        // Replace bottom-center with ┬
        put(&mut canvas, src_bot - 1, src_cx, "┬");

        if src_edges.len() == 1 {
            let e = src_edges[0];
            let label = truncate_label(&e.label);
            let tgt_cx = node_x[e.tgt] + box_width(e.tgt) / 2;
            let tgt_top = node_y[e.tgt];

            // Vertical line down from source
            for y in src_bot..tgt_top - 1 {
                put(&mut canvas, y, src_cx, "│");
            }

            if src_cx == tgt_cx {
                // Straight down
                put(&mut canvas, tgt_top - 1, tgt_cx, "v");
                // Label beside the vertical
                put(&mut canvas, src_bot, src_cx + 2, &label);
            } else {
                // L-bend: down then across
                let bend_y = src_bot + 1;
                put(&mut canvas, src_bot, src_cx, "│");

                if tgt_cx > src_cx {
                    put(&mut canvas, bend_y, src_cx, "└");
                    for x in src_cx + 1..tgt_cx {
                        put(&mut canvas, bend_y, x, "─");
                    }
                    put(&mut canvas, bend_y, tgt_cx, "┐");
                } else {
                    put(&mut canvas, bend_y, src_cx, "┘");
                    for x in tgt_cx + 1..src_cx {
                        put(&mut canvas, bend_y, x, "─");
                    }
                    put(&mut canvas, bend_y, tgt_cx, "┌");
                }

                for y in bend_y + 1..tgt_top - 1 {
                    put(&mut canvas, y, tgt_cx, "│");
                }
                put(&mut canvas, tgt_top - 1, tgt_cx, "v");
                // Label beside vertical
                put(&mut canvas, src_bot, src_cx + 2, &label);
            }
        } else {
            // Multiple targets: vertical down, then horizontal rail, then
            // drops to each target

            let rail_y = src_bot + 1;
            put(&mut canvas, src_bot, src_cx, "│");

            let mut target_xs: Vec<(usize, &GraphEdge)> = src_edges
                .iter()
                .map(|&e| (node_x[e.tgt] + box_width(e.tgt) / 2, e))
                .collect();
            target_xs.sort_by_key(|&(x, _)| x);

            let min_x = target_xs.first().map(|&(x, _)| x).unwrap_or(src_cx);
            let max_x = target_xs.last().map(|&(x, _)| x).unwrap_or(src_cx);

            // Draw horizontal rail
            for x in min_x..=max_x {
                put(&mut canvas, rail_y, x, "─");
            }

            // Source meets the rail
            if src_cx >= min_x && src_cx <= max_x {
                put(&mut canvas, rail_y, src_cx, "┴");
            }

            // Rail ends
            if min_x < src_cx {
                put(&mut canvas, rail_y, min_x, "┌");
            }
            if max_x > src_cx {
                put(&mut canvas, rail_y, max_x, "┐");
            }

            // Drop points and labels
            for &(tgt_cx, e) in &target_xs {
                let label = truncate_label(&e.label);
                let tgt_top = node_y[e.tgt];

                // Junction on rail
                if tgt_cx != min_x && tgt_cx != max_x && tgt_cx != src_cx {
                    put(&mut canvas, rail_y, tgt_cx, "┬");
                }

                // Vertical drop
                for y in rail_y + 1..tgt_top - 1 {
                    put(&mut canvas, y, tgt_cx, "│");
                }
                put(&mut canvas, tgt_top - 1, tgt_cx, "v");

                // Label beside the drop
                put(&mut canvas, rail_y + 1, tgt_cx + 1, &format!(" {label}"));
            }
        }
    }

    canvas
        .iter()
        .map(|row| row.iter().collect::<String>().trim_end().to_string())
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_string()
}

// ── wiki-summaries ──────────────────────────────────────────────────

fn kgx_wiki_write(category: &str, title: &str, summary: &str, content: &str) {
    let mut child = Command::new("kgx")
        .args(["wiki", "write", "--category", category, "--title", title, "--summary", summary])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run kgx");

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(content.as_bytes());
    }

    let output = child.wait_with_output().expect("kgx wait failed");
    if !output.status.success() {
        eprintln!(
            "  ERROR: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    } else {
        eprintln!("  wrote: {title}");
    }
}

fn cmd_wiki_summaries(cfg: &Config) {
    let files = project_files(cfg);
    if files.is_empty() {
        eprintln!("No PROJECT notes found");
        std::process::exit(1);
    }

    for path in &files {
        let fm = match parse_frontmatter(path) {
            Some(f) => f,
            None => continue,
        };
        let name = project_name(path);
        let desc = first_paragraph(path);
        let status = fm
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let langs = as_string_vec(&fm, "language_stack");
        let domains = as_string_vec(&fm, "domain");

        let mut lines = vec![
            "---".to_string(),
            format!("title: {name}"),
            format!("source_document: project_{name}"),
            format!("tags: [summary, project, {}]", langs.join(", ")),
            "---".to_string(),
            String::new(),
            format!("# {name}"),
            String::new(),
            format!("**Status:** {status}"),
            format!("**Languages:** {}", langs.join(", ")),
            format!("**Domains:** {}", domains.join(", ")),
            String::new(),
            desc.clone(),
            String::new(),
        ];

        if let Some(serde_yaml::Value::Sequence(rels)) =
            fm.get("project_relationships")
        {
            if !rels.is_empty() {
                lines.push("## Relationships".into());
                lines.push(String::new());
                for rel in rels {
                    let rtype = rel
                        .get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("related_to");
                    let target = rel
                        .get("target")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let notes = rel
                        .get("notes")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    lines.push(format!(
                        "- [[{target}]] ({rtype}): {notes}"
                    ));
                }
                lines.push(String::new());
            }
        }

        // Ego graph: this project + all direct relationship targets
        if let Some(serde_yaml::Value::Sequence(rels)) =
            fm.get("project_relationships")
        {
            if !rels.is_empty() {
                let mut graph_nodes = vec![name.as_str()];
                let mut graph_edges = vec![];
                let mut node_idx: BTreeMap<String, usize> = BTreeMap::new();
                node_idx.insert(name.clone(), 0);

                for rel in rels {
                    let rtype = rel
                        .get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("related_to");
                    let target = rel
                        .get("target")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if target.is_empty() {
                        continue;
                    }
                    let tgt_idx = *node_idx
                        .entry(target.to_string())
                        .or_insert_with(|| {
                            graph_nodes.push(target);
                            graph_nodes.len() - 1
                        });
                    graph_edges.push(GraphEdge {
                        src: 0,
                        tgt: tgt_idx,
                        label: rtype.to_string(),
                    });
                }

                let graph = render_graph(&graph_nodes, &graph_edges);
                if !graph.is_empty() {
                    lines.push("## Relationship Graph".into());
                    lines.push(String::new());
                    lines.push("```".into());
                    lines.push(graph);
                    lines.push("```".into());
                    lines.push(String::new());
                }
            }
        }

        let repo_url = fm
            .get("repo")
            .and_then(|v| v.get("url"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        lines.push("## References".into());
        lines.push(String::new());
        if !repo_url.is_empty() {
            lines.push(format!("- Repo: {repo_url}"));
        }
        lines.push(format!("- Source note: {path}"));
        lines.push(String::new());

        let content = lines.join("\n");
        let summary_text = format!("{name}: {desc} [{status}]");
        kgx_wiki_write("summary", &name, &summary_text, &content);
    }

    eprintln!("Done writing project summaries.");
}

// ── wiki-domains ────────────────────────────────────────────────────

struct ProjectEntry {
    name: String,
    desc: String,
    status: String,
    #[allow(dead_code)]
    langs: Vec<String>,
    rels: Vec<serde_json::Value>,
}

fn cmd_wiki_domains(cfg: &Config) {
    let files = project_files(cfg);

    let mut domain_map: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut lang_map: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut entries: Vec<ProjectEntry> = vec![];

    for path in &files {
        let fm = match parse_frontmatter(path) {
            Some(f) => f,
            None => continue,
        };
        let name = project_name(path);
        let desc = first_paragraph(path);
        let status = fm
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let langs = as_string_vec(&fm, "language_stack");
        let domains = as_string_vec(&fm, "domain");

        let mut rels_json = vec![];
        if let Some(serde_yaml::Value::Sequence(rels)) =
            fm.get("project_relationships")
        {
            for rel in rels {
                rels_json.push(serde_json::json!({
                    "type": rel.get("type").and_then(|v| v.as_str()).unwrap_or(""),
                    "target": rel.get("target").and_then(|v| v.as_str()).unwrap_or(""),
                    "notes": rel.get("notes").and_then(|v| v.as_str()).unwrap_or(""),
                }));
            }
        }

        let idx = entries.len();
        for d in &domains {
            domain_map.entry(d.clone()).or_default().push(idx);
        }
        for l in &langs {
            lang_map.entry(l.clone()).or_default().push(idx);
        }

        entries.push(ProjectEntry {
            name,
            desc,
            status,
            langs,
            rels: rels_json,
        });
    }

    // Domain cluster pages
    for (domain, indices) in &domain_map {
        let projects: Vec<&ProjectEntry> =
            indices.iter().map(|&i| &entries[i]).collect();
        let active: Vec<&&ProjectEntry> =
            projects.iter().filter(|p| p.status == "active").collect();
        let inactive: Vec<&&ProjectEntry> =
            projects.iter().filter(|p| p.status != "active").collect();

        let mut lines = vec![
            "---".to_string(),
            format!("title: Domain - {domain}"),
            format!("tags: [topic, domain, {domain}]"),
            "---".to_string(),
            String::new(),
            format!("# {domain}"),
            String::new(),
            format!(
                "{} projects in this domain ({} active, {} inactive/archived).",
                projects.len(),
                active.len(),
                inactive.len()
            ),
            String::new(),
            "## Active Projects".into(),
            String::new(),
        ];

        let mut sorted_active: Vec<&&&ProjectEntry> = active.iter().collect();
        sorted_active.sort_by_key(|p| &p.name);
        for p in sorted_active {
            lines.push(format!("- **[[{}]]** -- {}", p.name, p.desc));
        }

        if !inactive.is_empty() {
            lines.push(String::new());
            lines.push("## Inactive / Archived".into());
            lines.push(String::new());
            let mut sorted: Vec<&&&ProjectEntry> = inactive.iter().collect();
            sorted.sort_by_key(|p| &p.name);
            for p in sorted {
                lines.push(format!("- [[{}]] -- {}", p.name, p.desc));
            }
        }

        // Internal relationships
        let cluster_names: BTreeSet<&str> =
            projects.iter().map(|p| p.name.as_str()).collect();
        let mut internal_rels = vec![];
        let mut seen = BTreeSet::new();
        for p in &projects {
            for rel in &p.rels {
                let target = rel["target"].as_str().unwrap_or("");
                if cluster_names.contains(target) {
                    let rtype = rel["type"].as_str().unwrap_or("");
                    let notes = rel["notes"].as_str().unwrap_or("");
                    let key = (p.name.as_str(), rtype, target);
                    if seen.insert(format!("{}-{}-{}", key.0, key.1, key.2)) {
                        internal_rels.push((
                            p.name.clone(),
                            rtype.to_string(),
                            target.to_string(),
                            notes.to_string(),
                        ));
                    }
                }
            }
        }

        if !internal_rels.is_empty() {
            lines.push(String::new());
            lines.push("## Internal Relationships".into());
            lines.push(String::new());
            for (src, rtype, tgt, notes) in &internal_rels {
                lines.push(format!(
                    "- [[{src}]] --{rtype}--> [[{tgt}]]: {notes}"
                ));
            }

            // Cluster graph
            let cluster_node_names: Vec<&str> =
                projects.iter().map(|p| p.name.as_str()).collect();
            let cluster_idx: BTreeMap<&str, usize> = cluster_node_names
                .iter()
                .enumerate()
                .map(|(i, &n)| (n, i))
                .collect();
            let cluster_edges: Vec<GraphEdge> = internal_rels
                .iter()
                .filter_map(|(src, rtype, tgt, _)| {
                    let si = cluster_idx.get(src.as_str())?;
                    let ti = cluster_idx.get(tgt.as_str())?;
                    Some(GraphEdge {
                        src: *si,
                        tgt: *ti,
                        label: rtype.clone(),
                    })
                })
                .collect();

            if !cluster_edges.is_empty() {
                let graph =
                    render_graph(&cluster_node_names, &cluster_edges);
                if !graph.is_empty() {
                    lines.push(String::new());
                    lines.push("## Cluster Graph".into());
                    lines.push(String::new());
                    lines.push("```".into());
                    lines.push(graph);
                    lines.push("```".into());
                }
            }
        }

        lines.push(String::new());
        let content = lines.join("\n");
        let summary_text = format!(
            "{domain} domain: {} projects ({} active)",
            projects.len(),
            active.len()
        );
        kgx_wiki_write("topic", &format!("domain-{domain}"), &summary_text, &content);
    }

    // Language ecosystem pages
    for (lang, indices) in &lang_map {
        let projects: Vec<&ProjectEntry> =
            indices.iter().map(|&i| &entries[i]).collect();
        let active: Vec<&&ProjectEntry> =
            projects.iter().filter(|p| p.status == "active").collect();
        let inactive: Vec<&&ProjectEntry> =
            projects.iter().filter(|p| p.status != "active").collect();

        let mut lines = vec![
            "---".to_string(),
            format!("title: Language - {lang}"),
            format!("tags: [topic, language, {lang}]"),
            "---".to_string(),
            String::new(),
            format!("# {lang} ecosystem"),
            String::new(),
            format!(
                "{} projects using {lang} ({} active, {} inactive/archived).",
                projects.len(),
                active.len(),
                inactive.len()
            ),
            String::new(),
            "## Active Projects".into(),
            String::new(),
        ];

        let mut sorted_active: Vec<&&&ProjectEntry> = active.iter().collect();
        sorted_active.sort_by_key(|p| &p.name);
        for p in sorted_active {
            lines.push(format!("- **[[{}]]** -- {}", p.name, p.desc));
        }

        if !inactive.is_empty() {
            lines.push(String::new());
            lines.push("## Inactive / Archived".into());
            lines.push(String::new());
            let mut sorted: Vec<&&&ProjectEntry> = inactive.iter().collect();
            sorted.sort_by_key(|p| &p.name);
            for p in sorted {
                lines.push(format!("- [[{}]] -- {}", p.name, p.desc));
            }
        }

        lines.push(String::new());
        let content = lines.join("\n");
        let summary_text = format!(
            "{lang} ecosystem: {} projects ({} active)",
            projects.len(),
            active.len()
        );
        kgx_wiki_write("topic", &format!("lang-{lang}"), &summary_text, &content);
    }

    eprintln!("Done writing domain and language pages.");
}
