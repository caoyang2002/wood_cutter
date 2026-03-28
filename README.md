# Wood Cutter

> 2D Nesting Optimization Tool

A Rust-based 2D nesting optimization application that provides multiple algorithms for efficiently arranging shapes on a board to minimize material waste.

## Overview

This tool solves the 2D rectangular nesting problem (also known as the 2D bin packing problem), where regular shapes need to be arranged on a rectangular board to maximize material utilization.

## Features

- 🎯 **Multiple Optimization Algorithms**:
  - Genetic Algorithm
  - Simulated Annealing
  - Guillotine Cutting
  - MaxRects Algorithm
  - Bottom-Left Fill Algorithm
  - SVG Nest (No-Fit Polygon)
  - NFP Greedy Algorithm

- 🖥️ **Interactive GUI** built with egui
- 📐 **SVG Support** for shape import
- ⚡ **Real-time visualization** of nesting results
- 📊 **Step-by-step configuration** with adjustable parameters

## Algorithm Descriptions

### Bottom-Left
A classic heuristic algorithm that places shapes in the lowest, leftmost available position. Fast but not necessarily optimal.

### Genetic Algorithm (GA)
An evolutionary computation method that evolves optimal solutions across generations through selection, crossover, and mutation operations.

### Guillotine
A recursive cutting algorithm that divides the board into smaller rectangular regions after each placement.

### MaxRects
An advanced rectangle packing algorithm that maintains a list of available free rectangles to achieve optimal placement.

### NFP Greedy
Utilizes the No-Fit Polygon concept to detect valid placement positions, combined with a greedy strategy for selection.

### Simulated Annealing (SA)
A probabilistic algorithm based on the Metropolis criterion that occasionally accepts suboptimal solutions to escape local optima.

### SVG Nest
An algorithm specifically optimized for SVG vector shapes, supporting complex geometric figures.

## Quick Start

### Installation Steps

1. Clone the repository:
```bash
git clone https://github.com/caoyang2002/wood_cutter.git
cd wood_cutter
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```
```

This translation:
- Maintains the original structure and formatting
- Preserves all technical terminology consistently
- Keeps the code blocks and command examples intact
- Accurately translates algorithm descriptions and feature lists
- Retains the emojis and visual formatting elements
