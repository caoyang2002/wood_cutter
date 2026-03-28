# Wood Cutter

> 2D 排料优化工具

一个基于 Rust 的 2D 排料优化应用程序，提供多种算法，用于在板材上高效排列形状，最大限度地减少材料浪费。

## 概述

本工具解决 2D 矩形排料问题（也称为二维装箱问题），需要将规则形状排列在矩形板材上，最大化材料利用率。

## 功能特性

- 🎯 **多种优化算法**：
  - 遗传算法
  - 模拟退火算法
  - 吉约坦切割算法
  - MaxRects 算法
  - 左下角填充算法
  - SVG Nest（无拟合多边形）
  - NFP 贪心算法

- 🖥️ **交互式图形界面**，基于 egui 构建
- 📐 **支持 SVG 格式**形状导入
- ⚡ **实时可视化**排料结果
- 📊 **分步生成配置**，支持参数调节

## 算法说明

### 左下角算法（Bottom-Left）
经典的启发式算法，将形状放置在最低、最左的可放置位置。速度快但未必最优。

### 遗传算法（Genetic Algorithm, GA）
进化计算方法，通过选择、交叉和变异操作在代际间进化出最优解。

### 吉约坦切割（Guillotine）
递归切割算法，每次放置后将板材分割为更小的矩形区域。

### MaxRects
高级矩形装箱算法，维护可用空闲矩形列表，实现最优放置。

### 无拟合多边形贪心（NFP Greedy）
利用无拟合多边形概念检测有效放置位置，结合贪心策略进行选择。

### 模拟退火（Simulated Annealing, SA）
基于 Metropolis 准则的概率算法，通过偶尔接受较差解来跳出局部最优。

### SVG Nest
专门针对 SVG 矢量形状优化的算法，支持复杂几何图形。

## 快速开始

### 安装步骤

1. 克隆仓库：
```bash
git clone https://github.com/yourusername/2d-nesting-optimizer.git
cd 2d-nesting-optimizer
```

2. 构建项目：
```bash
cargo build --release
```

3. 运行应用：
```bash
cargo run --release
```
