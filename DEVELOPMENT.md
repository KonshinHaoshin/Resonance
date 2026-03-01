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
- [ ] 添加日志系统 (tracing + log)
- [ ] 音频引擎基础架构
- [ ] 项目结构规划

**待办:**
- [ ] 添加核心 Rust crates (audio, midi, format)
- [ ] 搭建 React 组件结构
- [ ] 配置 CI/CD

### Phase 2: USTX 格式解析
- [ ] 设计数据结构
- [ ] 实现 serde 序列化/反序列化
- [ ] 单元测试

### Phase 3: MIDI 导入/导出
- [ ] 集成 midly crate
- [ ] MIDI → USTX 转换
- [ ] USTX → MIDI 导出

### Phase 4: 钢琴卷帘 UI
- [ ] Canvas/WebGL 渲染
- [ ] 音符编辑交互
- [ ] 缩放/滚动

### Phase 5: 播放控制 & 渲染
- [ ] 音频播放 (rodio/cpal)
- [ ] 实时预览
- [ ] 渲染管线

### Phase 6: Resampler 接口
- [ ] Resampler trait 定义
- [ ] 内置 WORLDLINE
- [ ] 外部 resampler 支持

### Phase 7: Phonemizer 插件
- [ ] 插件系统架构
- [ ] 内置 phonemizers
- [ ] 第三方扩展支持

---

## 更新日志

### 2026-03-01
- 初始化项目，Tauri + React + TypeScript 骨架搭建
- 配置 tauri.conf.json (窗口大小、devtools)
- 推送到 GitHub
