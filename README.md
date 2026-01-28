# GitLog Analysis Tool

A powerful Rust-based Git repository analysis tool that generates interactive visual reports with Swiss Brutalism design aesthetics.

一个基于Rust的Git仓库分析工具，生成具有瑞士粗野主义设计美学的交互式可视化报告。

## 📋 Table of Contents / 目录
- [Overview / 概述](#overview--概述)
- [Features / 特性](#features--特性)
- [Installation / 安装](#installation--安装)
- [Usage / 使用](#usage--使用)
- [Output Examples / 输出示例](#output-examples--输出示例)
- [Project Structure / 项目结构](#project-structure--项目结构)
- [Technical Details / 技术细节](#technical-details--技术细节)
- [Development / 开发](#development--开发)
- [Contributing / 贡献指南](#contributing--贡献指南)
- [Screenshots / 截图](#screenshots--截图)
## Overview / 概述
**GitLog** is a command-line tool that analyzes Git repository history and generates comprehensive visual reports. It provides insights into development patterns, contributor activity, code quality, and project evolution through an interactive HTML interface with modern design.
**GitLog** 是一个命令行工具，用于分析Git仓库历史并生成全面的可视化报告。它通过具有现代设计的交互式HTML界面，提供开发模式、贡献者活动、代码质量和项目演进的深入洞察。
## Features / 特性
### 🔍 Analysis Capabilities / 分析能力
- **Commit Analysis**: Detailed examination of each commit including author info, changes, and statistics
  **提交分析**：详细检查每个提交，包括作者信息、更改和统计
- **Developer Insights**: Contributor leaderboard with impact scores, activity patterns, and roles
  **开发者洞察**：包含影响力分数、活动模式和角色的贡献者排行榜
- **Temporal Trends**: Daily, hourly, and weekly commit distribution analysis
  **时间趋势**：每日、每小时和每周提交分布分析
- **File System Analysis**: Hotspot detection, risk assessment, and file coupling analysis
  **文件系统分析**：热点检测、风险评估和文件耦合分析
- **Engineering Quality**: Commit type distribution, GPG signing rates, and quality metrics
  **工程质量**：提交类型分布、GPG签名率和质量指标
### 📊 Reporting Features / 报告特性
- **Interactive HTML Report**: Swiss Brutalism design with smooth vertical scrolling
  **交互式HTML报告**：瑞士粗野主义设计，支持平滑垂直滚动
- **Data Visualization**: Built-in ECharts for hourly pulse charts and commit heatmaps
  **数据可视化**：内置ECharts，用于小时脉冲图和提交热力图
- **Responsive Design**: Mobile-friendly interface with modern typography
  **响应式设计**：支持移动设备，采用现代排版
- **Multi-language Support**: Chinese localization with proper date formatting
  **多语言支持**：中文本地化，支持正确的日期格式

### ⚡ Performance / 性能
- **Compile-time Resource Embedding**: HTML template embedded at compile time
  **编译时资源嵌入**：HTML模板在编译时嵌入
- **Efficient Git Operations**: Uses libgit2 for fast repository traversal
  **高效的Git操作**：使用libgit2进行快速仓库遍历
- **Parallel Processing**: Optimized data aggregation and analysis
  **并行处理**：优化的数据聚合和分析

## Installation / 安装

### Prerequisites / 先决条件
- **Rust** (version 1.70+): Install from [rustup.rs](https://rustup.rs/)
  **Rust** (版本1.70+): 从 [rustup.rs](https://rustup.rs/) 安装
- **Git**: Git command-line tools
  **Git**: Git命令行工具
- **Cargo**: Rust package manager (included with Rust)
  **Cargo**: Rust包管理器（随Rust一起安装）

### Building from Source / 从源码构建
```bash
# Clone the repository / 克隆仓库
git clone <repository-url>
cd gitLog

# Build the project / 构建项目
cargo build --release

# The executable will be at / 可执行文件位置:
# target/release/gitLog
```
### Installation Methods / 安装方法
```bash
# Method 1: Install globally via Cargo / 方法1: 通过Cargo全局安装
cargo install --path .

# Method 2: Run directly / 方法2: 直接运行
cargo run --release

# Method 3: Use pre-built binary / 方法3: 使用预构建二进制文件
./target/release/gitLog
```
## Usage / 使用

### Quick Start / 快速开始

If you just want to try the tool quickly / 如果你想快速试用工具:

```bash
# Clone and run / 克隆并运行
git clone <repository-url>
cd gitLog
cargo run --release
```
The tool will analyze the current repository and open the report in your browser automatically.
工具将分析当前仓库并自动在浏览器中打开报告。

### Basic Usage / 基本使用

After installation, navigate to any Git repository and run:
安装后，导航到任何Git仓库并运行：

**Note / 注意**: If you installed globally via `cargo install`, use `gitLog`. Otherwise use `cargo run --release` or `./target/release/gitLog`.
**注意**: 如果通过`cargo install`全局安装，请使用`gitLog`命令。否则使用`cargo run --release`或`./target/release/gitLog`。

```bash
# Run in current directory (analyzes current Git repo) / 在当前目录运行（分析当前Git仓库）
gitLog

# Or specify a path / 或指定路径
gitLog /path/to/repository
```
### Command-line Options / 命令行选项

Currently the tool accepts no arguments and analyzes the current directory's Git repository. Future versions may include:


```bash
# Planned options / 计划中的选项
gitLog --output-dir ./reports    # Custom output directory / 自定义输出目录
gitLog --since 2024-01-01        # Analyze commits since date / 分析指定日期后的提交
gitLog --max-commits 1000        # Limit number of commits / 限制提交数量
```
### Output Files / 输出文件
After execution, the following files are generated in the `output/` directory:

- **`report.html`**: Interactive HTML report (opens automatically in browser)
  **`report.html`**: 交互式HTML报告（自动在浏览器中打开）
- **`processed_summary.json`**: Aggregated analysis data in JSON format
  **`processed_summary.json`**: JSON格式的聚合分析数据
- **`raw_commits.json`**: Raw commit data with detailed change information
  **`raw_commits.json`**: 包含详细更改信息的原始提交数据

### Report Navigation / 报告导航

The interactive report features 7 slides that can be navigated using:

1. **Mouse Wheel**: Scroll vertically between slides
   **鼠标滚轮**: 在幻灯片之间垂直滚动
2. **Touch Swipe**: Swipe up/down on touch devices
   **触摸滑动**: 在触摸设备上上下滑动
3. **Keyboard**: Arrow keys (planned for future release)
   **键盘**: 方向键（计划在将来版本中添加）

## Output Examples / 输出示例

### Sample Report Structure / 报告结构示例

The generated report includes the following sections:

#### 1. Project Scale / 工程规模
- Total commits and unique developers
  总提交数和独立开发者数量
- Bus factor calculation (number of core maintainers)
  巴士因子计算（核心维护者数量）
- Project timeline and timezone information
  项目时间线和时区信息

#### 2. Logical Workload / 逻辑负载
- Total lines inserted and deleted
  总新增行数和删除行数
- Net lines of code and refactoring ratio
  净代码行数和重构比率
- Code retention rate metrics
  代码留存率指标

#### 3. Core Contributors / 核心作者
- Developer leaderboard with impact scores
  带影响力分数的开发者排行榜
- Activity patterns and preferred working hours
  活动模式和偏好工作时间
- Role classification (Maintainer/Contributor)
  角色分类（维护者/贡献者）

#### 4. Development Pulse / 研发节律
- 24-hour commit distribution analysis
  24小时提交分布分析
- Weekly activity patterns
  每周活动模式
- GPG signing compliance rates
  GPG签名合规率

#### 5. Contribution Footprint / 贡献足迹
- Daily commit frequency heatmap
  每日提交频率热力图
- Project evolution timeline
  项目演进时间线
- Development intensity visualization
  开发强度可视化

#### 6. Hotspot Observation / 震中观测
- Most frequently changed files
  最频繁更改的文件
- Risk level assessment (High/Medium/Low)
  风险评估（高/中/低）
- Unique authors per file
  每个文件的独立作者数

#### 7. Summary / 总结
- Complete project lifecycle statistics
  完整的项目生命周期统计
- Report generation timestamp
  报告生成时间戳
- Project metadata summary
  项目元数据摘要

## Project Structure / 项目结构

```
gitLog/
├── src/
│   └── main.rs              # Main application logic
│                            # 主要应用程序逻辑
├── assets/
│   └── template.html        # HTML report template (embedded at compile time)
│                            # HTML报告模板（编译时嵌入）
├── output/                  # Generated reports (not tracked in git)
│   ├── report.html          # Interactive HTML report
│   ├── processed_summary.json  # Aggregated analysis data
│   └── raw_commits.json     # Raw commit data
├── Cargo.toml               # Rust dependencies and configuration
├── Cargo.lock               # Dependency lock file
├── .gitignore               # Git ignore rules
└── README.md                # This file
```

### Key Components / 关键组件

#### `src/main.rs`
The main application containing:
主要应用程序包含：
- **Data Structures**: Serializable structs for commit analysis
  **数据结构**: 用于提交分析的可序列化结构体
- **Analysis Functions**: Core logic for processing Git history
  **分析函数**: 处理Git历史的核心逻辑
- **Report Generation**: HTML report creation with embedded template
  **报告生成**: 使用嵌入模板创建HTML报告
- **Browser Integration**: Automatic report opening in default browser
  **浏览器集成**: 在默认浏览器中自动打开报告

#### `assets/template.html`
Modern HTML template with:
现代HTML模板包含：
- **Swiss Brutalism Design**: Minimalist typography and layout
  **瑞士粗野主义设计**: 极简主义排版和布局
- **Interactive Components**: Swiper.js for slide navigation
  **交互式组件**: Swiper.js用于幻灯片导航
- **Data Visualization**: ECharts integration for charts
  **数据可视化**: ECharts集成用于图表
- **Responsive Design**: Mobile-friendly CSS with Tailwind
  **响应式设计**: 使用Tailwind的移动友好CSS

## Technical Details / 技术细节

### Dependencies / 依赖项

```toml
[dependencies]
chrono = { version = "0.4.42", features = ["serde"] }  # Date/time handling / 日期时间处理
git2 = "0.20.3"                                        # Git operations / Git操作
serde = { version = "1.0.228", features = ["derive"] } # Serialization / 序列化
serde_json = "1.0.148"                                 # JSON handling / JSON处理
```

### Key Libraries / 关键库

1. **libgit2**: Native Git implementation for fast repository access
   **libgit2**: 用于快速仓库访问的本地Git实现
2. **Chrono**: Comprehensive date and time handling with timezone support
   **Chrono**: 全面的日期时间处理，支持时区
3. **Serde**: Efficient serialization/deserialization framework
   **Serde**: 高效的序列化/反序列化框架
4. **ECharts & Swiper.js**: Frontend libraries embedded in HTML template
   **ECharts & Swiper.js**: 嵌入在HTML模板中的前端库

### Performance Considerations / 性能考虑

- **Memory Efficiency**: Streaming Git history processing
  **内存效率**: 流式Git历史处理
- **Compile-time Optimization**: Template embedding reduces runtime I/O
  **编译时优化**: 模板嵌入减少运行时I/O
- **Parallel Analysis**: Future optimization potential for large repositories
  **并行分析**: 大型仓库的未来优化潜力

## Development / 开发

### Building and Testing / 构建和测试

```bash
# Development build / 开发构建
cargo build

# Run with debug output / 使用调试输出运行
cargo run

# Release build (optimized) / 发布构建（优化）
cargo build --release

# Run tests / 运行测试
cargo test
```

### Code Organization / 代码组织

The code follows a modular structure:

1. **Data Models** (lines 15-124): Serializable structs for all analysis data
   **数据模型** (15-124行): 所有分析数据的可序列化结构体
2. **Main Logic** (lines 126-163): Application entry point and flow control
   **主要逻辑** (126-163行): 应用程序入口点和流程控制
3. **Analysis Functions** (lines 165-273): Core analysis and aggregation
   **分析函数** (165-273行): 核心分析和聚合
4. **HTML Generation** (lines 275-281): Report creation and browser integration
   **HTML生成** (275-281行): 报告创建和浏览器集成
5. **Commit Analysis** (lines 283-329): Individual commit processing
   **提交分析** (283-329行): 单个提交处理
6. **Utilities** (lines 331-340): Helper functions (browser opening)
   **工具函数** (331-340行): 辅助函数（浏览器打开）

### Extending the Tool / 扩展工具

To add new analysis features:

1. **Add Data Structures**: Define new structs in the appropriate section
   **添加数据结构**: 在适当的部分定义新的结构体
2. **Implement Analysis**: Add functions to calculate new metrics
   **实现分析**: 添加计算新指标的函数
3. **Update Template**: Modify `template.html` to display new data
   **更新模板**: 修改`template.html`以显示新数据
4. **Add Visualization**: Include ECharts configuration for new charts
   **添加可视化**: 为新图表包含ECharts配置

## Contributing / 贡献指南

### How to Contribute / 如何贡献

1. **Fork the Repository** / **Fork仓库**
2. **Create a Feature Branch** / **创建功能分支**
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Commit Your Changes** / **提交更改**
   ```bash
   git commit -m 'Add some amazing feature'
   ```
4. **Push to the Branch** / **推送到分支**
   ```bash
   git push origin feature/amazing-feature
   ```
5. **Open a Pull Request** / **打开Pull Request**

### Development Guidelines / 开发指南

- **Code Style**: Follow Rust formatting standards (`cargo fmt`)
  **代码风格**: 遵循Rust格式化标准 (`cargo fmt`)
- **Documentation**: Add comments for complex logic
  **文档**: 为复杂逻辑添加注释
- **Testing**: Include tests for new functionality
  **测试**: 为新功能包含测试
- **Performance**: Consider memory and runtime efficiency
  **性能**: 考虑内存和运行时效率

### Feature Requests / 功能请求

Potential enhancements for future versions:

1. **Command-line Arguments**: Custom output paths, date ranges, filters
   **命令行参数**: 自定义输出路径、日期范围、过滤器
2. **Export Formats**: PDF, Markdown, CSV exports
   **导出格式**: PDF、Markdown、CSV导出
3. **Advanced Analytics**: Code complexity, dependency analysis
   **高级分析**: 代码复杂度、依赖分析
4. **CI/CD Integration**: GitHub Actions, GitLab CI templates
   **CI/CD集成**: GitHub Actions、GitLab CI模板
5. **Plugin System**: Custom analysis modules
   **插件系统**: 自定义分析模块

## Screenshots / 截图

**Answer to your question / 回答您的问题**: Yes, screenshots can be displayed on GitHub by placing them in a folder within your repository and referencing them using relative paths in Markdown. Below are detailed instructions.

**是的，截图可以通过放在仓库的文件夹中并在Markdown中使用相对路径引用来在GitHub上显示。以下是详细说明。**

To add screenshots to your README:

1. **Take Screenshots**: Capture images of the generated report
   **截图**: 截取生成报告的图片
2. **Save in Repository**: Create a `screenshots/` directory and add images
   **保存在仓库中**: 创建`screenshots/`目录并添加图片
3. **Reference in README**: Use Markdown syntax to display images
   **在README中引用**: 使用Markdown语法显示图片

Example Markdown for screenshots:

```markdown
### Report Screenshots / 报告截图

![Project Scale Slide](screenshots/project-scale.png)
*Slide 1: Project overview and metrics / 幻灯片1: 项目概览和指标*

![Developer Insights](screenshots/developer-insights.png)
*Slide 3: Contributor leaderboard / 幻灯片3: 贡献者排行榜*
```

### Recommended Screenshots / 推荐截图

1. **Full Report Overview**: Complete 7-slide presentation
   **完整报告概览**: 完整的7张幻灯片演示
2. **Interactive Charts**: Hourly pulse and daily heatmap visualizations
   **交互式图表**: 小时脉冲图和每日热力图可视化
3. **Mobile View**: Responsive design on different screen sizes
   **移动视图**: 不同屏幕尺寸上的响应式设计
4. **Data Details**: JSON output examples
   **数据详情**: JSON输出示例

---

## License / 许可证

This project is currently unlicensed. Please contact the maintainers for licensing information.

本项目目前没有许可证。请联系维护者获取许可信息。

## Acknowledgments / 致谢

- **libgit2** developers for the excellent Git library
  **libgit2** 开发者提供了优秀的Git库
- **Rust Community** for the amazing ecosystem
  **Rust社区** 提供了惊人的生态系统
- **Design Inspiration**: Swiss Brutalism and minimalist aesthetics
  **设计灵感**: 瑞士粗野主义和极简主义美学

---


*Report generated with GitLog - Understanding code evolution through data visualization*
*使用GitLog生成的报告 - 通过数据可视化理解代码演进*
