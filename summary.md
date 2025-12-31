{
  // --- 1. 项目基础元数据 (Project Metadata) ---
  "project_metadata": {
    "generated_at": "2025-12-31T14:36:53Z", // 报告生成的确切时间
    "total_commits": 1250,                  // 仓库总提交数
    "total_authors": 8,                     // 唯一开发者数量
    "bus_factor": 2,                        // 【新增】核心人数：如果这2人离职，项目50%的代码就没人懂了
    "time_range": {
      "start": "2024-01-01T08:00:00Z",      // 项目首个提交时间
      "end": "2025-12-31T10:00:00Z"         // 项目最新提交时间
    }
  },

  // --- 2. 宏观工作量与重构质量 (Workload & Quality) ---
  "workload_stats": {
    "total_insertions": 150230,             // 累计总新增行数
    "total_deletions": 85400,               // 累计总删除行数
    "net_lines": 64830,                     // 现存有效行数
    "refactor_ratio": 0.57,                 // 重构系数：删除/新增，越高代表代码优化程度越高
    "code_retention_rate": 0.42             // 【新增】代码留存率：一年前写的代码现在还活着的比例
  },

  // --- 3. 开发者贡献排行榜 (Leaderboard) ---
  "author_leaderboard": [
    {
      "name": "merwinM0",
      "email": "18358511710@139.com",
      "commit_count": 450,
      "insertions": 85000,
      "deletions": 32000,
      "impact_score": 85.2,                 // 【新增】综合影响力：基于提交次数、行数、文件范围的加权分
      "active_days": 120,                   // 【新增】活跃天数：一年中有多少天提交过代码
      "avg_cycle_time_hours": 4.5,          // 【新增】平均研发周期：从本地写完到最终合入的时间差
      "preferred_work_hours": "Morning",    // 【画像】作息特征：晨型人/夜猫子/标准时间
      "role": "Maintainer"                  // 【画像】角色：核心维护者/重构者/新功能开发者
    }
  ],

  // --- 4. 时间维度趋势 (Temporal Trends) ---
  "temporal_trends": {
    "daily_commits": {                      // 用于绘制“GitHub 贡献墙”
      "2025-12-30": 5,
      "2025-12-31": 8
    },
    "hourly_distribution": [                // 24小时分布：0点到23点各有多少次提交
      0, 0, 0, 0, 0, 0, 5, 20, 100, 80, 70, 60, 20, 10, 80, 90, 110, 40, 10, 5, 20, 30, 10, 2
    ],
    "weekly_distribution": {                // 识别周末加班和周中忙碌程度
      "Mon": 200, "Tue": 220, "Wed": 180, "Thu": 190, "Fri": 210, "Sat": 20, "Sun": 30
    }
  },

  // --- 5. 文件与技术债分析 (File System & Risk) ---
  "file_system_analysis": {
    "hotspots": [                           // “震中”文件：最容易产生冲突和Bug的地方
      { 
        "path": "src/main.rs",
        "change_count": 156,                // 修改频率
        "unique_authors": 5,                // 修改过该文件的独立人数
        "risk_level": "High"                // 风险评估：人数多且修改频繁即为高风险
      }
    ],
    "language_distribution": {              // 后缀名统计：项目技术构成
      "rs": 85.5,
      "md": 10.2,
      "toml": 4.3
    },
    "file_coupling": [                      // 【新增】强耦合文件对：A改了B必改
      ["src/auth.rs", "src/models/user.rs", 0.92] // 0.92 代表 92% 的情况它们同时被修改
    ]
  },

  // --- 6. 工程规范与安全 (Quality & Compliance) ---
  "engineering_quality": {
    "commit_type_distribution": {           // 提交消息规范统计
      "feat": 450,                          // 新功能
      "fix": 120,                           // 修复
      "docs": 50,                           // 文档
      "refactor": 80,                       // 重构
      "chore": 30,                          // 杂务
      "unknown": 520                        // 不规范提交
    },
    "avg_files_per_commit": 3.2,            // 单次提交平均涉及文件数
    "gpg_signed_percentage": 92.5,          // 经过安全签名的比例
    "potential_secrets_found": 0            // 【新增】泄露风险检测：是否在日志中发现疑似密钥
  }
}{
  // --- 1. 项目基础元数据 (Project Metadata) ---
  "project_metadata": {
    "generated_at": "2025-12-31T14:36:53Z", // 报告生成的确切时间
    "total_commits": 1250,                  // 仓库总提交数
    "total_authors": 8,                     // 唯一开发者数量
    "bus_factor": 2,                        // 【新增】核心人数：如果这2人离职，项目50%的代码就没人懂了
    "time_range": {
      "start": "2024-01-01T08:00:00Z",      // 项目首个提交时间
      "end": "2025-12-31T10:00:00Z"         // 项目最新提交时间
    }
  },

  // --- 2. 宏观工作量与重构质量 (Workload & Quality) ---
  "workload_stats": {
    "total_insertions": 150230,             // 累计总新增行数
    "total_deletions": 85400,               // 累计总删除行数
    "net_lines": 64830,                     // 现存有效行数
    "refactor_ratio": 0.57,                 // 重构系数：删除/新增，越高代表代码优化程度越高
    "code_retention_rate": 0.42             // 【新增】代码留存率：一年前写的代码现在还活着的比例
  },

  // --- 3. 开发者贡献排行榜 (Leaderboard) ---
  "author_leaderboard": [
    {
      "name": "merwinM0",
      "email": "18358511710@139.com",
      "commit_count": 450,
      "insertions": 85000,
      "deletions": 32000,
      "impact_score": 85.2,                 // 【新增】综合影响力：基于提交次数、行数、文件范围的加权分
      "active_days": 120,                   // 【新增】活跃天数：一年中有多少天提交过代码
      "avg_cycle_time_hours": 4.5,          // 【新增】平均研发周期：从本地写完到最终合入的时间差
      "preferred_work_hours": "Morning",    // 【画像】作息特征：晨型人/夜猫子/标准时间
      "role": "Maintainer"                  // 【画像】角色：核心维护者/重构者/新功能开发者
    }
  ],

  // --- 4. 时间维度趋势 (Temporal Trends) ---
  "temporal_trends": {
    "daily_commits": {                      // 用于绘制“GitHub 贡献墙”
      "2025-12-30": 5,
      "2025-12-31": 8
    },
    "hourly_distribution": [                // 24小时分布：0点到23点各有多少次提交
      0, 0, 0, 0, 0, 0, 5, 20, 100, 80, 70, 60, 20, 10, 80, 90, 110, 40, 10, 5, 20, 30, 10, 2
    ],
    "weekly_distribution": {                // 识别周末加班和周中忙碌程度
      "Mon": 200, "Tue": 220, "Wed": 180, "Thu": 190, "Fri": 210, "Sat": 20, "Sun": 30
    }
  },

  // --- 5. 文件与技术债分析 (File System & Risk) ---
  "file_system_analysis": {
    "hotspots": [                           // “震中”文件：最容易产生冲突和Bug的地方
      { 
        "path": "src/main.rs",
        "change_count": 156,                // 修改频率
        "unique_authors": 5,                // 修改过该文件的独立人数
        "risk_level": "High"                // 风险评估：人数多且修改频繁即为高风险
      }
    ],
    "language_distribution": {              // 后缀名统计：项目技术构成
      "rs": 85.5,
      "md": 10.2,
      "toml": 4.3
    },
    "file_coupling": [                      // 【新增】强耦合文件对：A改了B必改
      ["src/auth.rs", "src/models/user.rs", 0.92] // 0.92 代表 92% 的情况它们同时被修改
    ]
  },

  // --- 6. 工程规范与安全 (Quality & Compliance) ---
  "engineering_quality": {
    "commit_type_distribution": {           // 提交消息规范统计
      "feat": 450,                          // 新功能
      "fix": 120,                           // 修复
      "docs": 50,                           // 文档
      "refactor": 80,                       // 重构
      "chore": 30,                          // 杂务
      "unknown": 520                        // 不规范提交
    },
    "avg_files_per_commit": 3.2,            // 单次提交平均涉及文件数
    "gpg_signed_percentage": 92.5,          // 经过安全签名的比例
    "potential_secrets_found": 0            // 【新增】泄露风险检测：是否在日志中发现疑似密钥
  }
}