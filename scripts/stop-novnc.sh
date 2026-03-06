#!/bin/bash

# Life Platform - 停止 noVNC 服务脚本

echo "正在停止所有服务..."

# 停止 Tauri/应用
pkill -f "life-platform" 2>/dev/null && echo "✓ 已停止 Tauri 应用"
pkill -f "tauri" 2>/dev/null && echo "✓ 已停止 tauri CLI"
pkill -f "vite" 2>/dev/null && echo "✓ 已停止 Vite 开发服务器"

# 停止 noVNC
pkill -f "websockify" 2>/dev/null && echo "✓ 已停止 noVNC (websockify)"

# 停止 x11vnc
pkill -f "x11vnc" 2>/dev/null && echo "✓ 已停止 x11vnc"

# 停止 Xvfb
pkill -f "Xvfb" 2>/dev/null && echo "✓ 已停止 Xvfb"

echo ""
echo "所有服务已停止"
echo ""
echo "如需重新启动，请运行: ./start-novnc.sh"
