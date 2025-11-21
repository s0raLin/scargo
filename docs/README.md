# Sinter 项目介绍网站

这是一个为 Sinter 项目创建的现代化介绍网站，展示了项目的功能特性，特别是新添加的 SBT 互操作功能。

## 文件结构

```
docs/
├── index.html      # 主页面
├── styles.css      # 样式文件
├── script.js       # JavaScript 交互
└── README.md       # 说明文档
```

## 功能特性

- **现代化设计**: 使用现代 CSS 特性和响应式布局
- **交互体验**: 平滑滚动、悬停效果、代码复制功能
- **SBT 集成展示**: 专门的区域展示 SBT 互操作功能
- **移动端适配**: 完全响应式的移动端体验
- **性能优化**: 轻量级设计，无需外部依赖

## 本地预览

### 方法 1: 使用 Python 简单服务器

```bash
cd docs
python -m http.server 8000
```

然后在浏览器中访问 `http://localhost:8000`

### 方法 2: 使用 Node.js

```bash
cd docs
npx serve .
```

### 方法 3: 直接在浏览器中打开

直接在浏览器中打开 `index.html` 文件即可预览。

## 部署

### GitHub Pages

1. 将 `docs/` 目录推送到 GitHub 仓库
2. 在仓库设置中启用 GitHub Pages
3. 选择 `main` 分支的 `docs/` 目录作为源
4. 或者选择 `main` 分支作为源（自动检测 docs 目录）

### 其他平台

可以将文件上传到任何支持静态网站的平台，如：
- Netlify
- Vercel
- Firebase Hosting
- AWS S3 + CloudFront

## 自定义

### 修改内容

- 编辑 `index.html` 修改页面内容
- 修改 `styles.css` 调整样式
- 更新 `script.js` 改变交互行为

### 添加新功能

- 在 HTML 中添加新的区域
- 为新元素添加相应的 CSS 样式
- 在 JavaScript 中添加交互逻辑

## 技术栈

- **HTML5**: 语义化标记
- **CSS3**: 现代样式特性（Flexbox, Grid, 渐变等）
- **Vanilla JavaScript**: 原生 JavaScript，无框架依赖
- **Font Awesome**: 图标库
- **Google Fonts**: Inter 字体

## 浏览器支持

- Chrome 80+
- Firefox 75+
- Safari 13+
- Edge 80+

## 许可证

与主项目保持一致，使用 MIT 许可证。