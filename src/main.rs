use chrono::{Datelike, TimeZone, Utc, Timelike};
use git2::{Commit, DiffOptions, Repository, Sort};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

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
    status: String,
}

#[derive(Serialize, Debug, Clone)]
struct CommitReport {
    hash: String,
    author: UserInfo,
    summary: String,
    is_merge: bool,
    total_insertions: usize,
    total_deletions: usize,
    changes: Vec<FileChangeDetail>,
}

// --- 增强后的处理结果结构 ---

#[derive(Serialize, Debug)]
struct ProjectSummary {
    total_commits: usize,
    total_lines_added: usize,
    total_lines_deleted: usize,
    
    // 1. 人员贡献
    author_contributions: HashMap<String, usize>,
    
    // 2. 时间维度分析
    daily_activity: HashMap<String, usize>,      // 日期 (YYYY-MM-DD) -> 提交数
    hour_distribution: [usize; 24],             // 0-23点分布 (工作习惯)
    
    // 3. 文件热点
    file_hotspots: Vec<(String, usize)>,
    
    // 4. 代码构成分析
    language_distribution: HashMap<String, usize>, // 后缀名 -> 修改次数
    
    // 5. 提交类型分析 (基于 Conventional Commits)
    commit_types: HashMap<String, usize>,        // feat, fix, docs 等
}

fn main() -> Result<(), Box<dyn Error>> {
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME)?;

    let mut reports = Vec::new();

    for id_result in revwalk {
        let id = id_result?;
        let commit = repo.find_commit(id)?;
        if let Ok(report) = analyze_commit(&repo, &commit) {
            reports.push(report);
        }
    }

    // 保存原始数据
    let raw_file = File::create("raw_commits.json")?;
    serde_json::to_writer_pretty(raw_file, &reports)?;

    // 执行深度分析
    let summary = process_data(&reports);

    // 保存增强版分析结果
    let processed_file = File::create("processed_summary.json")?;
    serde_json::to_writer_pretty(processed_file, &summary)?;

    println!("✅ 分析完成！原始记录与深度汇总已生成。");
    Ok(())
}

fn process_data(reports: &[CommitReport]) -> ProjectSummary {
    let mut author_map = HashMap::new();
    let mut daily_map = HashMap::new();
    let mut hour_array = [0; 24];
    let mut file_map = HashMap::new();
    let mut lang_map = HashMap::new();
    let mut type_map = HashMap::new();
    let mut total_added = 0;
    let mut total_deleted = 0;

    for r in reports {
        // 时间分析
        let dt = Utc.timestamp_opt(r.author.timestamp, 0).unwrap();
        let date_str = format!("{}-{:02}-{:02}", dt.year(), dt.month(), dt.day());
        *daily_map.entry(date_str).or_insert(0) += 1;
        hour_array[dt.hour() as usize] += 1;

        if !r.is_merge {
            // 代码量统计
            *author_map.entry(r.author.name.clone()).or_insert(0) += r.total_insertions;
            total_added += r.total_insertions;
            total_deleted += r.total_deletions;

            // 提交类型分析 (匹配开头如 "feat:", "fix:")
            let c_type = r.summary.split(':').next().unwrap_or("other").to_lowercase();
            if ["feat", "fix", "docs", "style", "refactor", "chore", "test"].contains(&c_type.as_str()) {
                *type_map.entry(c_type).or_insert(0) += 1;
            } else {
                *type_map.entry("other".to_string()).or_insert(0) += 1;
            }
        }

        // 文件与语言分析
        for change in &r.changes {
            *file_map.entry(change.path.clone()).or_insert(0) += 1;
            
            let extension = change.path.split('.')
                .last()
                .unwrap_or("no_ext")
                .to_string();
            *lang_map.entry(extension).or_insert(0) += 1;
        }
    }

    let mut hotspots: Vec<(String, usize)> = file_map.into_iter().collect();
    hotspots.sort_by(|a, b| b.1.cmp(&a.1));

    ProjectSummary {
        total_commits: reports.len(),
        total_lines_added: total_added,
        total_lines_deleted: total_deleted,
        author_contributions: author_map,
        daily_activity: daily_map,
        hour_distribution: hour_array,
        file_hotspots: hotspots.into_iter().take(10).collect(),
        language_distribution: lang_map,
        commit_types: type_map,
    }
}

// 该函数保持不变，仅根据新结构简化了返回值
fn analyze_commit(repo: &Repository, commit: &Commit) -> Result<CommitReport, Box<dyn Error>> {
    let author_sig = commit.author();
    let current_tree = commit.tree()?;
    let parent_tree = if commit.parent_count() > 0 {
        Some(commit.parent(0)?.tree()?)
    } else {
        None
    };

    let mut opts = DiffOptions::new();
    let mut diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&current_tree), Some(&mut opts))?;
    let mut find_opts = git2::DiffFindOptions::new();
    find_opts.renames(true);
    diff.find_similar(Some(&mut find_opts))?;

    let stats = diff.stats()?;
    let mut changes = Vec::new();

    diff.foreach(
        &mut |delta, _| {
            let path = delta.new_file().path().or(delta.old_file().path())
                .map(|p| p.to_string_lossy().into_owned()).unwrap_or_default();
            changes.push(FileChangeDetail { path, status: format!("{:?}", delta.status()) });
            true
        },
        None, None, None
    )?;

    Ok(CommitReport {
        hash: commit.id().to_string(),
        author: UserInfo {
            name: author_sig.name().unwrap_or("Unknown").to_string(),
            email: author_sig.email().unwrap_or("Unknown").to_string(),
            timestamp: author_sig.when().seconds(),
            offset_min: author_sig.when().offset_minutes(),
        },
        summary: commit.summary().unwrap_or("").to_string(),
        is_merge: commit.parent_count() > 1,
        total_insertions: stats.insertions(),
        total_deletions: stats.deletions(),
        changes,
    })
}
