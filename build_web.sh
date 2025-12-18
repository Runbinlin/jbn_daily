#!/bin/bash
# 构建 WebAssembly 版本的修仙游戏

set -e

echo "🎮 开始构建修仙编程游戏 Web 版..."

# 检查 wasm32 target
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "📦 安装 wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# 检查 wasm-bindgen-cli
if ! command -v wasm-bindgen &> /dev/null; then
    echo "📦 安装 wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

# 编译 WASM
echo "🔨 编译 WebAssembly..."
cargo build --release --target wasm32-unknown-unknown

# 生成 JS 绑定
echo "🔗 生成 JavaScript 绑定..."
wasm-bindgen target/wasm32-unknown-unknown/release/xiuxian_game.wasm \
    --out-dir web \
    --target web \
    --no-typescript

# 优化 WASM 大小（可选，需要安装 wasm-opt）
if command -v wasm-opt &> /dev/null; then
    echo "⚡ 优化 WASM 体积..."
    wasm-opt -Oz web/xiuxian_game_bg.wasm -o web/xiuxian_game_bg.wasm
fi

echo ""
echo "✅ 构建完成！"
echo ""
echo "📁 输出文件在 web/ 目录:"
ls -lh web/
echo ""
echo "🚀 本地测试方法:"
echo "   cd web && python3 -m http.server 8080"
echo "   然后打开浏览器访问 http://localhost:8080"
echo ""
echo "📤 部署方法:"
echo "   将 web/ 目录上传到任意静态托管服务:"
echo "   - GitHub Pages"
echo "   - Vercel"
echo "   - Netlify"
echo "   - Cloudflare Pages"
