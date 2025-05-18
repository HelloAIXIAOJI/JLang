<?php
// 获取URL参数
$by = isset($_GET['by']) ? $_GET['by'] : '';
$id = isset($_GET['id']) ? $_GET['id'] : '';

// 文件路径安全处理（允许目录遍历但防止恶意路径）
// 移除反斜杠，但保留正斜杠以允许遍历
$by = str_replace(['..\\', '\\'], ['', '/'], $by);
$id = str_replace(['..\\', '\\'], ['', '/'], $id);

// 确保没有访问到上级目录
if (strpos($by, '../') !== false || strpos($id, '../') !== false) {
    die("非法访问路径");
}

// 文件路径
$filePath = './' . ($by ? $by . '/' : '') . $id . '.md';
$filePathZh = './' . ($by ? $by . '/' : '') . $id . '.zh.md';

// 确定加载哪个文件（优先加载中文版本）
$contentFilePath = file_exists($filePathZh) ? $filePathZh : $filePath;

// 读取文件内容
$content = '';
$title = 'JiLang Wiki';
$errorMessage = '';

if ($id) {
    if (file_exists($contentFilePath)) {
        $content = file_get_contents($contentFilePath);
        
        // 从内容中提取标题（第一个#标题）
        if (preg_match('/^# (.+)$/m', $content, $matches)) {
            $title = 'JiLang Wiki - ' . $matches[1];
        }
    } else {
        $errorMessage = "没有找到文档: " . htmlspecialchars($by ? "$by/" : "") . htmlspecialchars($id);
    }
} else {
    $errorMessage = "请指定要查看的文档ID";
}

// 获取目录结构
function getDirStructure($dir) {
    $result = [];
    if (is_dir($dir)) {
        $files = scandir($dir);
        
        // 首先检查是否存在name.txt文件来获取目录标题
        $dirTitle = null;
        if (file_exists("$dir/name.txt")) {
            $dirTitle = trim(file_get_contents("$dir/name.txt"));
        }
        
        foreach ($files as $file) {
            if ($file !== '.' && $file !== '..' && $file !== 'name.txt') {
                if (is_dir("$dir/$file")) {
                    $subdirResult = getDirStructure("$dir/$file");
                    // 只有当子目录有内容时才添加
                    if (!empty($subdirResult)) {
                        $result[$file] = $subdirResult;
                    }
                } else if (preg_match('/\.(md|zh\.md)$/', $file)) {
                    // 提取文件名（不带扩展名）
                    $filename = preg_replace('/\.(zh\.)?md$/', '', $file);
                    // 检查是否已经添加了这个文件（避免.md和.zh.md重复）
                    if (!in_array($filename, $result)) {
                        $result[] = $filename;
                    }
                }
            }
        }
    }
    
    // 如果目录有标题并且有内容，添加标题信息
    if ($dirTitle && !empty($result)) {
        $result['__dir_title'] = $dirTitle;
    }
    
    return $result;
}

$wikiStructure = getDirStructure('.');

// 获取文档的第一个标题
function getDocumentTitle($by, $id) {
    $filePath = './' . ($by ? $by . '/' : '') . $id . '.md';
    $filePathZh = './' . ($by ? $by . '/' : '') . $id . '.zh.md';
    
    // 优先使用中文版本
    $path = file_exists($filePathZh) ? $filePathZh : $filePath;
    
    if (file_exists($path)) {
        $content = file_get_contents($path);
        // 匹配第一个#标题
        if (preg_match('/^# (.+)$/m', $content, $matches)) {
            return $matches[1];  // 返回标题文本（已移除#）
        }
    }
    
    // 如果没有找到标题，返回文件名作为默认值
    return $id;
}

// 构建目录导航
function buildNavigation($structure, $currentPath = '') {
    $html = '<ul>';
    
    // 提取并移除目录标题（如果存在）
    $dirTitle = null;
    if (isset($structure['__dir_title'])) {
        $dirTitle = $structure['__dir_title'];
        unset($structure['__dir_title']);
    }
    
    foreach ($structure as $key => $value) {
        if (is_array($value)) {
            // 这是一个子目录
            // 检查是否有目录标题
            $folderTitle = isset($value['__dir_title']) ? $value['__dir_title'] : $key;
            
            $html .= '<li class="nav-folder"><span class="nav-folder-name">' . htmlspecialchars($folderTitle) . '</span>';
            $html .= buildNavigation($value, $currentPath . ($currentPath ? '/' : '') . $key);
            $html .= '</li>';
        } else {
            // 这是一个文件，获取文档的第一个标题
            $title = getDocumentTitle($currentPath, $value);
            $html .= '<li class="nav-file"><a href="?by=' . urlencode($currentPath) . '&id=' . urlencode($value) . '">' . htmlspecialchars($title) . '</a></li>';
        }
    }
    $html .= '</ul>';
    return $html;
}

// 设置页面标题
$pageTitle = $title;

// 设置额外的样式
$extraStyles = '
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/github-markdown-css@5.1.0/github-markdown.min.css">
<style>
    /* 导航菜单样式 */
    .nav-list ul {
        list-style: none;
        margin-left: 15px;
    }

    .nav-list > ul {
        margin-left: 0;
    }

    .nav-folder {
        margin-bottom: 10px;
    }

    .nav-folder-name {
        display: block;
        font-weight: 600;
        color: var(--text-dark);
        margin-bottom: 5px;
        cursor: pointer;
    }

    .nav-file {
        margin-bottom: 5px;
        padding-left: 15px;
    }

    .nav-file a {
        color: var(--text-light);
        text-decoration: none;
        font-size: 14px;
        display: block;
        padding: 3px 0;
        transition: color 0.2s;
    }

    .nav-file a:hover {
        color: var(--primary);
    }

    .error-message {
        padding: 20px;
        background-color: #feeeee;
        border-left: 4px solid #f56565;
        margin-bottom: 20px;
        border-radius: 4px;
    }
</style>
';

// 设置额外的脚本
$extraScripts = '<script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>';

// 设置底部脚本
$footerScripts = '
<script>
    // 目录折叠
    document.querySelectorAll(".nav-folder-name").forEach(folderName => {
        folderName.addEventListener("click", function() {
            const folder = this.parentElement;
            const subList = folder.querySelector("ul");
            if (subList) {
                subList.style.display = subList.style.display === "none" ? "block" : "none";
            }
        });
    });

    // Markdown渲染
    ' . (!$errorMessage && $content ? '
        const markdownContent = ' . json_encode($content) . ';
        document.getElementById("markdown-content").innerHTML = marked.parse(markdownContent);
        
        // 处理文档中的链接
        document.querySelectorAll("#markdown-content a").forEach(link => {
            const href = link.getAttribute("href");
            // 如果是相对链接，处理为wiki内部链接
            if (href && !href.startsWith("http") && !href.startsWith("#")) {
                // 去掉.md扩展名
                const cleanHref = href.replace(/\.md$/, "");
                // 检查是否包含路径分隔符
                if (cleanHref.includes("/")) {
                    const parts = cleanHref.split("/");
                    const id = parts.pop();
                    const by = parts.join("/");
                    link.href = `?by=${encodeURIComponent(by)}&id=${encodeURIComponent(id)}`;
                } else {
                    link.href = `?id=${encodeURIComponent(cleanHref)}`;
                }
            }
        });
    ' : '') . '
</script>
';

// 包含头部文件（相对路径）
require_once __DIR__ . '/../includes/header.php';
?>

    <!-- Wiki主体 -->
    <div class="wiki-container">
        <!-- 侧边栏 -->
        <div class="wiki-sidebar">
            <div class="sidebar-heading">文档导航</div>
            <div class="nav-list">
                <?php echo buildNavigation($wikiStructure); ?>
            </div>
        </div>

        <!-- 内容区 -->
        <div class="wiki-content">
            <div class="wiki-content-inner">
                <?php if ($errorMessage): ?>
                    <div class="error-message"><?php echo htmlspecialchars($errorMessage); ?></div>
                <?php else: ?>
                    <div id="markdown-content" class="markdown-body"></div>
                <?php endif; ?>
            </div>
        </div>
    </div>

<?php
// 包含底部文件（相对路径）
require_once __DIR__ . '/../includes/footer.php';
?> 