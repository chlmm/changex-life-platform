#!/bin/bash

# Life Platform - noVNC 启动脚本
# 用于在云服务器上一键启动 VNC + noVNC + Tauri 应用

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 配置
XVFB_DISPLAY=":99"
XVFB_RESOLUTION="1920x1080x24"
VNC_PORT=5900
NOVNC_PORT=6080
TAURI_DIR="/workspace/life-platform"

# 获取服务器 IP
SERVER_IP=$(hostname -I | awk '{print $1}')

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Life Platform - noVNC 启动脚本${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# 函数：停止现有服务
cleanup() {
    echo -e "${YELLOW}[1/5] 清理现有进程...${NC}"
    # 使用 killall 按进程名清理（更安全，不会匹配脚本路径）
    killall life-platform 2>/dev/null || true
    killall vite 2>/dev/null || true
    killall websockify 2>/dev/null || true
    killall x11vnc 2>/dev/null || true
    killall Xvfb 2>/dev/null || true
    sleep 2
    echo -e "${GREEN}✓ 清理完成${NC}"
}

# 函数：检查依赖
check_deps() {
    echo -e "${YELLOW}[2/5] 检查依赖...${NC}"
    
    for cmd in Xvfb x11vnc websockify; do
        if ! command -v $cmd &> /dev/null; then
            echo -e "${RED}✗ 错误: $cmd 未安装${NC}"
            echo "请安装: apt-get install xvfb x11vnc websockify"
            exit 1
        fi
    done
    
    # 检查 noVNC 文件
    if [ ! -d "/usr/share/novnc" ]; then
        echo -e "${RED}✗ 错误: noVNC 未安装${NC}"
        echo "请安装: apt-get install novnc"
        exit 1
    fi
    
    echo -e "${GREEN}✓ 所有依赖已就绪${NC}"
}

# 函数：启动 Xvfb
start_xvfb() {
    echo -e "${YELLOW}[3/5] 启动 Xvfb 虚拟显示...${NC}"
    Xvfb $XVFB_DISPLAY -screen 0 $XVFB_RESOLUTION -ac &
    export DISPLAY=$XVFB_DISPLAY
    sleep 2
    
    if pgrep -x "Xvfb" > /dev/null; then
        echo -e "${GREEN}✓ Xvfb 启动成功 (显示: $XVFB_DISPLAY)${NC}"
    else
        echo -e "${RED}✗ Xvfb 启动失败${NC}"
        exit 1
    fi
}

# 函数：启动 x11vnc
start_x11vnc() {
    echo -e "${YELLOW}[4/5] 启动 x11vnc VNC 服务...${NC}"
    x11vnc -display $XVFB_DISPLAY -forever -shared -rfbport $VNC_PORT -nopw -bg -o /tmp/x11vnc.log 2>/dev/null
    sleep 1
    
    if ss -tlnp | grep -q ":$VNC_PORT"; then
        echo -e "${GREEN}✓ x11vnc 启动成功 (端口: $VNC_PORT)${NC}"
    else
        echo -e "${RED}✗ x11vnc 启动失败${NC}"
        exit 1
    fi
}

# 函数：启动 noVNC (websockify)
start_novnc() {
    echo -e "${YELLOW}[5/5] 启动 noVNC Web 服务...${NC}"
    websockify --web=/usr/share/novnc $NOVNC_PORT localhost:$VNC_PORT &
    sleep 2
    
    if ss -tlnp | grep -q ":$NOVNC_PORT"; then
        echo -e "${GREEN}✓ noVNC 启动成功 (端口: $NOVNC_PORT)${NC}"
    else
        echo -e "${RED}✗ noVNC 启动失败${NC}"
        exit 1
    fi
}

# 函数：显示访问信息
show_info() {
    echo ""
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}  所有服务已启动！${NC}"
    echo -e "${GREEN}========================================${NC}"
    echo ""
    echo -e "📱 ${YELLOW}浏览器访问 noVNC:${NC}"
    echo "   http://$SERVER_IP:$NOVNC_PORT/vnc.html"
    echo "   http://localhost:$NOVNC_PORT/vnc.html"
    echo ""
    echo -e "🔌 ${YELLOW}VNC 客户端连接:${NC}"
    echo "   $SERVER_IP:$VNC_PORT"
    echo ""
    echo -e "⚠️  ${YELLOW}注意:${NC}"
    echo "   - 无需密码，直接连接"
    echo "   - 在 noVNC 页面点击 'Connect' 按钮"
    echo ""
    echo -e "📁 ${YELLOW}插件目录:${NC}"
    echo "   /root/.life-platform/plugins"
    echo ""
    echo -e "🛑 ${YELLOW}停止服务:${NC}"
    echo "   ./stop-novnc.sh"
    echo ""
}

# 主流程
main() {
    # 检查参数
    AUTO_TAURI=false
    if [[ "$1" == "--tauri" ]]; then
        AUTO_TAURI=true
    fi
    
    cleanup
    check_deps
    start_xvfb
    start_x11vnc
    start_novnc
    show_info
    
    if [[ "$AUTO_TAURI" == true ]]; then
        echo -e "${YELLOW}自动启动 Tauri 开发模式...${NC}"
        cd $TAURI_DIR
        DISPLAY=$XVFB_DISPLAY npm run tauri dev
    else
        # 可选：启动 Tauri 开发模式
        read -p "是否启动 Tauri 开发模式? (y/n): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo -e "${YELLOW}启动 Tauri 开发模式...${NC}"
            cd $TAURI_DIR
            DISPLAY=$XVFB_DISPLAY npm run tauri dev
        else
            echo -e "${GREEN}服务已就绪，请在浏览器中访问 noVNC 并手动启动应用${NC}"
            # 保持脚本运行
            wait
        fi
    fi
}

# 执行
main
