use chrono::{DateTime, Datelike, FixedOffset, Local, TimeZone, Timelike, Utc};
use git2::{Commit, DiffOptions, Repository, Sort};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;

// --- 原始数据结构 ---
#[derive(Serialize, Debug, Clone)]
struct UserInfo {
    name: String,
    email: String,
    timestamp: i64,
    offset_min: i32,
}

#[derive(Serialize, Debug, Clone)]
struct FileChangeDetail {
    path: String,
    old_path: Option<String>,
    status: String,
}

#[derive(Serialize, Debug, Clone)]
struct CommitReport {
    hash: String,
    parent_hashes: Vec<String>,
    author: UserInfo,
    committer: UserInfo,
    summary: String,
    body: Option<String>,
    is_merge: bool,
    signature_verified: bool,
    total_insertions: usize,
    total_deletions: usize,
    files_changed_count: usize,
    changes: Vec<FileChangeDetail>,
}

// --- 严格匹配要求的分析结果结构 ---

#[derive(Serialize, Debug)]
struct ProjectMetadata {
    generated_at: String,
    local_timezone: String, // 新增：本地时区名称
    total_commits: usize,
    total_authors: usize,
    bus_factor: usize,
    last_commit_time: String, // 新增：最后一次提交的时间
    time_range: TimeRange,
}

#[derive(Serialize, Debug)]
struct TimeRange {
    start: String,
    end: String,
}

#[derive(Serialize, Debug)]
struct WorkloadStats {
    total_insertions: usize,
    total_deletions: usize,
    net_lines: i64,
    refactor_ratio: f64,
    code_retention_rate: f64,
}

#[derive(Serialize, Debug)]
struct AuthorLeaderboardEntry {
    name: String,
    email: String,
    commit_count: usize,
    insertions: usize,
    deletions: usize,
    impact_score: f64,
    active_days: usize,
    avg_cycle_time_hours: f64,
    preferred_work_hours: String,
    role: String,
}

#[derive(Serialize, Debug)]
struct TemporalTrends {
    daily_commits: HashMap<String, usize>,
    // 改为 BTreeMap 确保输出时按时间顺序排列 0-23
    hourly_distribution: BTreeMap<String, usize>,
    weekly_distribution: HashMap<String, usize>,
}

#[derive(Serialize, Debug)]
struct FileSystemAnalysis {
    hotspots: Vec<HotspotEntry>,
    language_distribution: HashMap<String, f64>,
    file_coupling: Vec<(String, String, f64)>,
}

#[derive(Serialize, Debug)]
struct HotspotEntry {
    path: String,
    change_count: usize,
    unique_authors: usize,
    risk_level: String,
}

#[derive(Serialize, Debug)]
struct EngineeringQuality {
    commit_type_distribution: HashMap<String, usize>,
    avg_files_per_commit: f64,
    gpg_signed_percentage: f64,
    potential_secrets_found: usize,
}

#[derive(Serialize, Debug)]
struct FinalAnalysisReport {
    project_metadata: ProjectMetadata,
    workload_stats: WorkloadStats,
    author_leaderboard: Vec<AuthorLeaderboardEntry>,
    temporal_trends: TemporalTrends,
    file_system_analysis: FileSystemAnalysis,
    engineering_quality: EngineeringQuality,
}

fn main() -> Result<(), Box<dyn Error>> {
    // 检查并创建输出目录
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
        println!("已创建输出目录: output/");
    }

    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME)?;

    let mut reports = Vec::new();
    println!("正在从 Git 历史采集数据...");

    for id_result in revwalk {
        let id = id_result?;
        let commit = repo.find_commit(id)?;
        if let Ok(report) = analyze_commit(&repo, &commit) {
            reports.push(report);
        }
    }

    // 1. 保存原始数据到 output/
    let raw_file = File::create(output_dir.join("raw_commits.json"))?;
    serde_json::to_writer_pretty(raw_file, &reports)?;

    // 2. 执行分析
    println!("正在计算多维度分析指标...");
    let final_report = perform_final_analysis(&reports);

    // 3. 保存分析报告到 output/
    let summary_file = File::create(output_dir.join("processed_summary.json"))?;
    serde_json::to_writer_pretty(summary_file, &final_report)?;

    println!("✅ 分析完成！文件已存入 output/ 文件夹。");
    Ok(())
}

fn perform_final_analysis(reports: &[CommitReport]) -> FinalAnalysisReport {
    let mut author_map: HashMap<String, (UserInfo, Vec<&CommitReport>)> = HashMap::new();
    let mut daily_commits = HashMap::new();
    // 使用 BTreeMap 确保输出 JSON 时 Key 是有序的 (0时, 1时...)
    let mut hourly_dist = BTreeMap::new();
    for i in 0..24 {
        hourly_dist.insert(format!("{:02}", i), 0);
    }

    let mut weekly_dist = HashMap::new();
    let mut file_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut file_change_counts: HashMap<String, usize> = HashMap::new();
    let mut lang_counts: HashMap<String, usize> = HashMap::new();
    let mut type_dist = HashMap::new();

    let mut total_ins = 0;
    let mut total_del = 0;
    let mut gpg_count = 0;

    for r in reports {
        // 时间适配 (按开发者提交时的当地时间统计习惯)
        let offset = FixedOffset::east_opt(r.author.offset_min * 60)
            .unwrap_or(FixedOffset::east_opt(0).unwrap());
        let dt = Utc
            .timestamp_opt(r.author.timestamp, 0)
            .unwrap()
            .with_timezone(&offset);

        *daily_commits
            .entry(dt.format("%Y-%m-%d").to_string())
            .or_insert(0) += 1;
        let hour_key = format!("{:02}时", dt.hour());
        *hourly_dist.entry(hour_key).or_insert(0) += 1;
        *weekly_dist.entry(dt.format("%a").to_string()).or_insert(0) += 1;

        total_ins += r.total_insertions;
        total_del += r.total_deletions;
        if r.signature_verified {
            gpg_count += 1;
        }

        let c_type = r
            .summary
            .split(':')
            .next()
            .unwrap_or("unknown")
            .to_lowercase();
        let c_type = if ["feat", "fix", "docs", "refactor", "chore"].contains(&c_type.as_str()) {
            c_type
        } else {
            "unknown".to_string()
        };
        *type_dist.entry(c_type).or_insert(0) += 1;

        let entry = author_map
            .entry(r.author.email.clone())
            .or_insert((r.author.clone(), Vec::new()));
        entry.1.push(r);

        for change in &r.changes {
            *file_change_counts.entry(change.path.clone()).or_insert(0) += 1;
            file_map
                .entry(change.path.clone())
                .or_insert(HashSet::new())
                .insert(r.author.email.clone());
            let ext = change.path.split('.').last().unwrap_or("other").to_string();
            *lang_counts.entry(ext).or_insert(0) += 1;
        }
    }

    let mut leaderboard = Vec::new();
    for (email, (info, commits)) in author_map {
        let ins: usize = commits.iter().map(|c| c.total_insertions).sum();
        let del: usize = commits.iter().map(|c| c.total_deletions).sum();
        let active_days: HashSet<String> = commits
            .iter()
            .map(|c| {
                Utc.timestamp_opt(c.author.timestamp, 0)
                    .unwrap()
                    .format("%Y-%m-%d")
                    .to_string()
            })
            .collect();

        let impact = (commits.len() as f64 * 0.3)
            + ((ins as f64 + 1.0).log10() * 5.0)
            + (active_days.len() as f64 * 0.2);

        leaderboard.push(AuthorLeaderboardEntry {
            name: info.name,
            email,
            commit_count: commits.len(),
            insertions: ins,
            deletions: del,
            impact_score: (impact * 10.0).round() / 10.0,
            active_days: active_days.len(),
            avg_cycle_time_hours: 0.0,
            preferred_work_hours: "Standard".to_string(),
            role: if commits.len() > reports.len() / 5 {
                "Maintainer"
            } else {
                "Contributor"
            }
            .to_string(),
        });
    }
    leaderboard.sort_by(|a, b| b.impact_score.partial_cmp(&a.impact_score).unwrap());

    // 获取本地当前时区和最后一次提交时间
    let now_local = Local::now();
    let local_tz = now_local.format("%Z").to_string();
    let last_commit_str = if let Some(latest) = reports.first() {
        let last_offset = FixedOffset::east_opt(latest.author.offset_min * 60)
            .unwrap_or(FixedOffset::east_opt(0).unwrap());
        Utc.timestamp_opt(latest.author.timestamp, 0)
            .unwrap()
            .with_timezone(&last_offset)
            .to_rfc3339()
    } else {
        "N/A".to_string()
    };

    FinalAnalysisReport {
        project_metadata: ProjectMetadata {
            generated_at: now_local.to_rfc3339(),
            local_timezone: local_tz,
            total_commits: reports.len(),
            total_authors: leaderboard.len(),
            bus_factor: 1,
            last_commit_time: last_commit_str,
            time_range: TimeRange {
                start: reports
                    .last()
                    .map(|r| r.author.timestamp.to_string())
                    .unwrap_or_default(),
                end: reports
                    .first()
                    .map(|r| r.author.timestamp.to_string())
                    .unwrap_or_default(),
            },
        },
        workload_stats: WorkloadStats {
            total_insertions: total_ins,
            total_deletions: total_del,
            net_lines: (total_ins as i64 - total_del as i64),
            refactor_ratio: if total_ins > 0 {
                (total_del as f64 / total_ins as f64 * 100.0).round() / 100.0
            } else {
                0.0
            },
            code_retention_rate: 0.0,
        },
        author_leaderboard: leaderboard,
        temporal_trends: TemporalTrends {
            daily_commits,
            hourly_distribution: hourly_dist,
            weekly_distribution: weekly_dist,
        },
        file_system_analysis: FileSystemAnalysis {
            hotspots: file_change_counts
                .into_iter()
                .take(10)
                .map(|(path, count)| {
                    let authors = file_map.get(&path).map(|s| s.len()).unwrap_or(0);
                    HotspotEntry {
                        path,
                        change_count: count,
                        unique_authors: authors,
                        risk_level: if authors > 2 { "High" } else { "Low" }.to_string(),
                    }
                })
                .collect(),
            language_distribution: HashMap::new(),
            file_coupling: Vec::new(),
        },
        engineering_quality: EngineeringQuality {
            commit_type_distribution: type_dist,
            avg_files_per_commit: if reports.len() > 0 {
                reports.iter().map(|r| r.files_changed_count).sum::<usize>() as f64
                    / reports.len() as f64
            } else {
                0.0
            },
            gpg_signed_percentage: if reports.len() > 0 {
                (gpg_count as f64 / reports.len() as f64 * 100.0)
            } else {
                0.0
            },
            potential_secrets_found: 0,
        },
    }
}

fn analyze_commit(repo: &Repository, commit: &Commit) -> Result<CommitReport, Box<dyn Error>> {
    let author_sig = commit.author();
    let committer_sig = commit.committer();
    let author = UserInfo {
        name: author_sig.name().unwrap_or("Unknown").to_string(),
        email: author_sig.email().unwrap_or("Unknown").to_string(),
        timestamp: author_sig.when().seconds(),
        offset_min: author_sig.when().offset_minutes(),
    };
    let committer = UserInfo {
        name: committer_sig.name().unwrap_or("Unknown").to_string(),
        email: committer_sig.email().unwrap_or("Unknown").to_string(),
        timestamp: committer_sig.when().seconds(),
        offset_min: committer_sig.when().offset_minutes(),
    };
    let mut parent_hashes = Vec::new();
    for i in 0..commit.parent_count() {
        parent_hashes.push(commit.parent_id(i)?.to_string());
    }
    let current_tree = commit.tree()?;
    let parent_tree = if commit.parent_count() > 0 {
        Some(commit.parent(0)?.tree()?)
    } else {
        None
    };
    let mut opts = DiffOptions::new();
    let mut diff =
        repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&current_tree), Some(&mut opts))?;
    let mut find_opts = git2::DiffFindOptions::new();
    find_opts.renames(true);
    diff.find_similar(Some(&mut find_opts))?;
    let stats = diff.stats()?;
    let mut changes = Vec::new();
    diff.foreach(
        &mut |delta, _| {
            let old_path = delta
                .old_file()
                .path()
                .map(|p| p.to_string_lossy().into_owned());
            let new_path = delta
                .new_file()
                .path()
                .map(|p| p.to_string_lossy().into_owned());
            changes.push(FileChangeDetail {
                path: new_path.unwrap_or_else(|| old_path.clone().unwrap_or_default()),
                old_path,
                status: format!("{:?}", delta.status()),
            });
            true
        },
        None,
        None,
        None,
    )?;
    let signature_verified = repo.extract_signature(&commit.id(), None).is_ok();
    Ok(CommitReport {
        hash: commit.id().to_string(),
        parent_hashes,
        author,
        committer,
        summary: commit.summary().unwrap_or("").to_string(),
        body: commit.body().map(|s| s.to_string()),
        is_merge: commit.parent_count() > 1,
        signature_verified,
        total_insertions: stats.insertions(),
        total_deletions: stats.deletions(),
        files_changed_count: stats.files_changed(),
        changes,
    })
}
