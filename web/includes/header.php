<?php
// 包含初始化文件
require_once __DIR__ . '/init.php';

// 获取页面标题，如果没有设置则使用默认标题
if (!isset($pageTitle)) {
    $pageTitle = $siteName . ' - ' . $siteDescription;
}
?>
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title><?php echo htmlspecialchars($pageTitle); ?></title>
    <style>
        /* 基础样式 */
        :root {
            --primary: #f59e0b;
            --primary-dark: #d97706;
            --primary-light: #fcd34d;
            --primary-bg: #fef9c3;
            --text-dark: #78350f;
            --text-light: #92400e;
            --white: #ffffff;
            --border-color: #fde68a;
            --sidebar-width: 280px;
        }
        
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            line-height: 1.6;
            color: var(--text-dark);
            background: linear-gradient(135deg, var(--primary-bg) 0%, var(--white) 50%, var(--primary-bg) 100%);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }
        
        .container {
            width: 100%;
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 20px;
        }
        
        /* 标题和文本样式 */
        h1, h2, h3, h4 {
            color: var(--text-dark);
            margin-bottom: 1rem;
        }
        
        .gradient-text {
            background: linear-gradient(90deg, var(--primary) 0%, var(--primary-dark) 100%);
            -webkit-background-clip: text;
            background-clip: text;
            color: transparent;
        }
        
        /* 头部样式 */
        header {
            position: sticky;
            top: 0;
            z-index: 50;
            background-color: rgba(255, 255, 255, 0.8);
            backdrop-filter: blur(10px);
            -webkit-backdrop-filter: blur(10px);
            border-bottom: 1px solid var(--border-color);
        }
        
        .header-content {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 15px 0;
        }
        
        .logo {
            display: flex;
            align-items: center;
            gap: 10px;
        }
        
        .logo-icon {
            width: 40px;
            height: 40px;
            background-color: var(--primary-bg);
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            color: var(--primary);
            font-weight: bold;
            font-size: 20px;
        }
        
        .logo-text {
            font-size: 24px;
            font-weight: bold;
        }
        
        /* 导航 */
        nav {
            display: flex;
            gap: 30px;
        }
        
        nav a {
            color: var(--text-dark);
            text-decoration: none;
            font-weight: 500;
            transition: color 0.3s;
        }
        
        nav a:hover {
            color: var(--primary);
        }
        
        .mobile-menu-btn {
            display: none;
            background: none;
            border: none;
            cursor: pointer;
        }

        /* 按钮样式 */
        .btn-primary {
            background-color: var(--primary);
            color: white;
            border: none;
            padding: 12px 25px;
            border-radius: 9999px;
            font-weight: 600;
            cursor: pointer;
            transition: background-color 0.3s, transform 0.2s;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        }
        
        .btn-primary:hover {
            background-color: var(--primary-dark);
            transform: translateY(-2px);
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        }
        
        .btn-secondary {
            background-color: transparent;
            color: var(--primary-dark);
            border: 2px solid var(--primary-light);
            padding: 10px 25px;
            border-radius: 9999px;
            font-weight: 600;
            cursor: pointer;
            transition: background-color 0.3s, transform 0.2s;
        }
        
        .btn-secondary:hover {
            background-color: rgba(253, 230, 138, 0.2);
            transform: translateY(-2px);
        }

        /* Wiki特定样式 */
        .wiki-container {
            display: flex;
            flex: 1;
            margin-top: 20px;
        }

        .wiki-sidebar {
            width: var(--sidebar-width);
            flex-shrink: 0;
            background-color: rgba(255, 255, 255, 0.7);
            border-right: 1px solid var(--border-color);
            padding: 20px;
            position: sticky;
            top: 80px;
            height: calc(100vh - 80px);
            overflow-y: auto;
        }

        .sidebar-heading {
            font-size: 18px;
            font-weight: bold;
            margin-bottom: 15px;
            color: var(--primary-dark);
            padding-bottom: 10px;
            border-bottom: 1px solid var(--border-color);
        }

        .wiki-content {
            flex: 1;
            padding: 20px 40px;
            max-width: calc(100% - var(--sidebar-width));
        }

        .wiki-content-inner {
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
            padding: 30px;
            margin-bottom: 30px;
        }

        /* Markdown内容样式 */
        .markdown-body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            color: var(--text-dark);
        }

        .markdown-body h1 {
            padding-bottom: 10px;
            border-bottom: 1px solid var(--border-color);
            color: var(--primary-dark);
        }

        .markdown-body code {
            background-color: #f8fafc;
            border-radius: 3px;
            padding: 2px 5px;
        }

        .markdown-body pre {
            background-color: #f8fafc;
            border-radius: 6px;
            padding: 15px;
            border-left: 4px solid var(--primary);
            margin: 15px 0;
            overflow-x: auto;
        }

        .markdown-body a {
            color: var(--primary-dark);
            text-decoration: none;
        }

        .markdown-body a:hover {
            text-decoration: underline;
        }
        
        /* 响应式设计 */
        @media (max-width: 768px) {
            .header-content {
                padding: 15px;
            }
            
            nav {
                display: none;
            }
            
            .mobile-menu-btn {
                display: block;
                width: 40px;
                height: 40px;
                background-color: var(--primary-bg);
                border-radius: 50%;
                display: flex;
                align-items: center;
                justify-content: center;
                color: var(--primary);
            }
            
            .hero h1 {
                font-size: 36px;
            }
            
            .hero p {
                font-size: 18px;
            }
            
            .features-grid {
                grid-template-columns: 1fr;
            }

            .doc-cards {
                grid-template-columns: 1fr;
            }
            
            .social-links {
                flex-wrap: wrap;
                justify-content: center;
            }
            
            .wiki-container {
                flex-direction: column;
            }
            
            .wiki-sidebar {
                width: 100%;
                position: static;
                height: auto;
                margin-bottom: 20px;
            }
            
            .wiki-content {
                max-width: 100%;
                padding: 20px;
            }
        }
    </style>
    <?php if (isset($extraStyles)) echo $extraStyles; ?>
    <?php if (isset($extraScripts)) echo $extraScripts; ?>
</head>
<body>
    <!-- 头部 -->
    <header>
        <div class="container">
            <div class="header-content">
                <div class="logo">
                    <a href="/" style="text-decoration: none; display: flex; align-items: center; gap: 10px;">
                        <div class="logo-icon">J</div>
                        <span class="logo-text gradient-text"><?php echo htmlspecialchars($siteName); ?></span>
                    </a>
                </div>
                <nav>
                    <a href="/">首页</a>
                    <?php foreach ($navItems as $item): ?>
                    <a href="<?php echo getNavUrl($item['href']); ?>"><?php echo htmlspecialchars($item['text']); ?></a>
                    <?php endforeach; ?>
                </nav>
                <button class="mobile-menu-btn">≡</button>
            </div>
        </div>
    </header>
</body>
</html> 