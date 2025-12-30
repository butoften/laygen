#!/bin/bash

# Laygen 快速测试脚本
# 此脚本演示如何使用 laygen 工具

set -e

echo "================================"
echo "Laygen 快速测试脚本"
echo "================================"
echo ""

# 创建测试项目目录
TEST_DIR="/tmp/laygen-demo"
echo "1. 创建测试目录: $TEST_DIR"
rm -rf "$TEST_DIR"
mkdir -p "$TEST_DIR"

echo "2. 创建基本目录结构"
mkdir -p "$TEST_DIR/controller/admin"
mkdir -p "$TEST_DIR/controller/api"
mkdir -p "$TEST_DIR/service/admin"
mkdir -p "$TEST_DIR/service/api"

echo ""
echo "目录结构已创建："
tree "$TEST_DIR" 2>/dev/null || find "$TEST_DIR" -type d

echo ""
echo "================================"
echo "现在你可以运行以下命令测试 laygen："
echo "================================"
echo ""
echo "# 使用中文界面"
echo "laygen $TEST_DIR -l zh"
echo ""
echo "# 使用英文界面"
echo "laygen $TEST_DIR -l en"
echo ""
echo "# 自定义目录名称"
echo "laygen $TEST_DIR --controller-dir controllers --service-dir services"
echo ""
echo "================================"
echo "测试建议："
echo "================================"
echo "1. 尝试在不同的子目录中创建文件"
echo "2. 测试文件名和方法名的输入"
echo "3. 测试 service 文件名后缀选项"
echo "4. 检查生成的 mod.rs 文件"
echo ""
echo "测试目录: $TEST_DIR"
echo ""
