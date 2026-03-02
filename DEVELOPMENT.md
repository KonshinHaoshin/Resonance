# Resonance 开发日志

## 项目概述

**Resonance** - 用 Rust + React 重写 OpenUtau 的开源歌声合成平台

### 技术栈
- Frontend: React 18 + TypeScript + Vite + Tailwind CSS
- Backend: Rust + Tauri 2.0

### GitHub
https://github.com/Akanclaw/Resonance

---

## 开发阶段

### Phase 1: 项目骨架 & 基础设施
- [x] 初始化 Tauri + React 项目
- [x] 配置 Tailwind CSS
- [x] 添加日志系统 (tracing)
- [x] 音频引擎基础架构
- [x] 项目结构规划

**待办:**
- [ ] 添加核心 Rust crates (audio, midi, format)
- [ ] 搭建 React 组件结构
- [ ] 配置 CI/CD

### Phase 2: USTX 格式解析
- [x] 设计数据结构
- [x] 实现 serde 序列化/反序列化
- [x] 单元测试

### Phase 3: MIDI 导入/导出
- [x] 集成 midly crate
- [x] MIDI → USTX 转换
- [x] USTX → MIDI 导出

### Phase 4: 钢琴卷帘 UI
- [x] Canvas/WebGL 渲染
- [x] 音符编辑交互（拖拽、调整大小、双击添加）
- [x] 缩放/滚动
- [x] Timeline 时间轴

### Phase 5: 播放控制 & 渲染
- [x] 音频引擎重构
- [x] 项目渲染到音频缓冲区
- [x] 获取音频样本命令
- [x] 前端 Web Audio API 集成
- [x] TransportBar 播放控制
- [x] AudioPlayer 组件

### Phase 6: Resampler 接口
- [x] Resampler trait 定义
- [x] 内置 Sine/Triangle/Sawtooth resampler
- [x] WORLDLINE placeholder
- [x] 外部 resampler 支持

### Phase 7: Phonemizer 插件
- [x] 插件系统架构
- [x] 内置 JapanesePhonemizer (带假名词典)
- [x] 内置 EnglishArpabetPhonemizer (带英语词典)
- [x] PhonemizerManager 管理器

### Phase 8: CI/CD & 发布
- [x] GitHub Actions CI 配置
- [x] GitHub Actions Release 配置

---

## 更新日志

### 2026-03-02
- ✅ Rust 编译通过
- ✅ 添加日志系统 (tracing)
- ✅ 添加 rodio 和 midly 依赖
- ✅ 安装 ALSA 开发库
- ✅ 修复 Tauri 命令参数
- ✅ 修复 TypeScript 前端构建
- ✅ 前端编译成功
- ✅ 添加 AudioPlayer 组件 (Web Audio API)
- ✅ 添加 TransportBar 播放控制
- ✅ 配置 GitHub Actions CI/CD

### 2026-03-01
- ✅ 初始化项目，Tauri + React + TypeScript 骨架搭建
- ✅ 配置 tauri.conf.json (窗口大小、devtools)
- ✅ 推送到 GitHub
