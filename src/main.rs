use git2::{Repository, Commit, DiffOptions};
use serde::{Serialize};

#[derive(Serialize, Debug)]
struct FileChange {
    path: String,
    status: String, // 例如: Added, Modified, Deleted
}

#[derive(Serialize, Debug)]
struct CommitReport {
    hash: String,
    author_name: String,
    author_email: String,
    committer_name: String,
    time: i64,             // Unix 时间戳
    time_offset: i32,      // 时区偏移（分钟）
    summary: String,       // 提交第一行
    body: String,          // 详细描述
    is_merge: bool,        // 是否为合并提交
    parent_count: usize,   // 父节点数量
    insertions: usize,     // 新增行数
    deletions: usize,      // 删除行数
    files_changed: usize,  // 变更文件数
    changed_file_details: Vec<FileChange>, // 具体变更的文件路径
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 打开仓库
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut reports = Vec::new();

    // 2. 遍历提交
    for id_result in revwalk {
        let id = id_result?;
        let commit = repo.find_commit(id)?;
        
        // 获取该提交的详细分析报告
        let report = analyze_commit(&repo, &commit)?;
        reports.push(report);
    }

    // 3. 输出数据 (这里以 JSON 格式打印，方便你后续导入分析工具)
    let json_output = serde_json::to_string_pretty(&reports)?;
    println!("{}", json_output);

    eprintln!("\n分析完成，共处理 {} 条提交记录。", reports.len());
    Ok(())
}

/// 深入分析单个提交，获取差异数据
fn analyze_commit(repo: &Repository, commit: &Commit) -> Result<CommitReport, git2::Error> {
    let author = commit.author();
    let committer = commit.committer();
    
    // 获取 Tree 对象用于对比
    let current_tree = commit.tree()?;
    
    // 获取父树（处理初始提交的情况）
    let parent_tree = if commit.parent_count() > 0 {
        Some(commit.parent(0)?.tree()?)
    } else {
        None
    };

    // 设置差异对比选项（可以根据需要过滤掉空格变化等）
    let mut diff_opts = DiffOptions::new();
    let diff = repo.diff_tree_to_tree(
        parent_tree.as_ref(),
        Some(&current_tree),
        Some(&mut diff_opts),
    )?;

    // 统计新增/删除行数
    let stats = diff.stats()?;
    
    // 提取变更的文件列表
    let mut changed_file_details = Vec::new();
    diff.foreach(
        &mut |delta, _| {
            let path = delta.new_file().path()
                .or_else(|| delta.old_file().path())
                .and_then(|p| p.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            let status = format!("{:?}", delta.status());
            changed_file_details.push(FileChange { path, status });
            true
        },
        None, None, None
    )?;

    Ok(CommitReport {
        hash: commit.id().to_string(),
        author_name: author.name().unwrap_or("Unknown").to_string(),
        author_email: author.email().unwrap_or("Unknown").to_string(),
        committer_name: committer.name().unwrap_or("Unknown").to_string(),
        time: commit.time().seconds(),
        time_offset: commit.time().offset_minutes(),
        summary: commit.summary().unwrap_or("").to_string(),
        body: commit.body().unwrap_or("").to_string(),
        is_merge: commit.parent_count() > 1,
        parent_count: commit.parent_count(),
        insertions: stats.insertions(),
        deletions: stats.deletions(),
        files_changed: stats.files_changed(),
        changed_file_details,
    })
}
