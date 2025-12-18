#!/bin/bash
# 修仙编程游戏启动脚本

cd "$(dirname "$0")"

# 检查是否编译过
if [ ! -f "target/release/xiuxian_game" ]; then
    echo "第一次运行，正在编译游戏..."
    cargo build --release
fi

# 运行游戏
./target/release/xiuxian_game
