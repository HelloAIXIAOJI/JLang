<?php
// 简单重定向到wiki.php
header('Location: wiki.php' . (empty($_SERVER['QUERY_STRING']) ? '' : '?' . $_SERVER['QUERY_STRING']));
exit;
?> 