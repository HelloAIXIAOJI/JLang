<?php
// 网站基本信息
$siteName = "JiLang";
$siteDescription = "基于JSON的编程语言";

// 导航菜单项
$navItems = [
    ["href" => "#features", "text" => "特性"],
    ["href" => "#examples", "text" => "示例"],
    ["href" => "#installation", "text" => "安装"],
    ["href" => "#documentation", "text" => "文档"],
    ["href" => "#community", "text" => "社区"],
    ["href" => "/maker/index.php", "text" => "贡献者"]
];

// 获取当前页面路径
$currentPage = basename($_SERVER['PHP_SELF']);

// 根据页面设置正确的导航链接
function getNavUrl($href) {
    global $currentPage;
    
    // 如果当前页不是index.php且链接是锚点，则添加前缀
    if ($currentPage != 'index.php' && substr($href, 0, 1) === '#') {
        return '/index.php' . $href;
    }
    
    return $href;
}
?> 