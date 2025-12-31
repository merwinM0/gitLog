use git2::{Repository, Revwalk};

fn main() -> Result<(), git2::Error> {
    // 1. 打开当前工作目录下的 Git 仓库
    let repo = Repository::open(".")?;

    // 2. 创建一个 Revwalk 实例，用于遍历提交历史
    let mut revwalk = repo.revwalk()?;

    // 3. 告诉 git2 从 HEAD 开始遍历
    revwalk.push_head()?;

    // 4. 设置遍历顺序（从新到旧）
    revwalk.set_sorting(git2::Sort::TIME)?;

    println!("{:<40} | {:<20} | {}", "Commit ID", "Author", "Message");
    println!("{}", "-".repeat(80));

    // 5. 迭代每一个 commit ID
    for id in revwalk {
        let id = id?;
        let commit = repo.find_commit(id)?;

        // 获取作者信息
        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown");

        // 获取提交信息的摘要（第一行）
        let message = commit.summary().unwrap_or("No message");

        println!("{:.8} | {:<20} | {}", id, author_name, message);
    }

    Ok(())
}
