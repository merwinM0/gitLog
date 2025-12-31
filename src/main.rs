use git2::{Commit, DiffOptions, Repository, Sort};
use serde::Serialize;
use std::error::Error;

// --- 数据结构定义 ---

#[derive(Serialize, Debug)]
struct UserInfo {
    name: String,
    email: String,
    timestamp: i64,
    offset_min: i32,
}

#[derive(Serialize, Debug)]
struct FileChangeDetail {
    path: String,
    old_path: Option<String>,
    status: String,
}

#[derive(Serialize, Debug)]
struct CommitReport {
    hash: String,
    parent_hashes: Vec<String>,
    author: UserInfo,
    committer: UserInfo,
    summary: String,
    body: Option<String>,
    is_merge: bool,
    signature_verified: bool,
    // 统计数据
    total_insertions: usize,
    total_deletions: usize,
    files_changed_count: usize,
    changes: Vec<FileChangeDetail>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // 1. 初始化仓库
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;

    // 设置遍历规则：从 HEAD 开始，按时间顺序
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME)?;

    let mut reports = Vec::new();

    // 2. 遍历所有提交
    for id_result in revwalk {
        let id = id_result?;
        let commit = repo.find_commit(id)?;

        // 执行深度分析
        match analyze_commit(&repo, &commit) {
            Ok(report) => reports.push(report),
            Err(e) => eprintln!("跳过提交 {} 的分析: {}", id, e),
        }
    }

    // 3. 序列化为 JSON 并输出
    let json_output = serde_json::to_string_pretty(&reports)?;
    println!("{}", json_output);

    eprintln!("\n成功采集 {} 条提交数据。", reports.len());
    Ok(())
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

    // --- 修正部分开始 ---
    let mut opts = DiffOptions::new();
    let mut diff =
        repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&current_tree), Some(&mut opts))?;

    // 使用 find_similar 来检测重命名
    let mut find_opts = git2::DiffFindOptions::new();
    find_opts.renames(true);
    diff.find_similar(Some(&mut find_opts))?;
    // --- 修正部分结束 ---

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
