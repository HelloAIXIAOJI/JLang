    <!-- 底部 -->
    <footer>
        <div class="container">
            <div class="footer-content">
                <div class="footer-logo">
                    <div class="logo-icon">J</div>
                    <span class="logo-text gradient-text"><?php echo htmlspecialchars($siteName); ?></span>
                </div>
                <div class="social-links">
                    <a href="#" class="social-link">🐦</a>
                    <a href="#" class="social-link">💬</a>
                    <a href="#" class="social-link">👥</a>
                    <a href="#" class="social-link">📚</a>
                </div>
                <div class="footer-divider"></div>
                <div class="copyright">
                    © <?php echo date('Y'); ?> <?php echo htmlspecialchars($siteName); ?>项目，保留所有权利。
                </div>
            </div>
        </div>
    </footer>

    <script>
        // 移动菜单切换
        document.querySelector('.mobile-menu-btn').addEventListener('click', function() {
            const nav = document.querySelector('nav');
            nav.style.display = nav.style.display === 'flex' ? 'none' : 'flex';
        });

        // 平滑滚动
        document.querySelectorAll('a[href^="#"]').forEach(anchor => {
            anchor.addEventListener('click', function (e) {
                e.preventDefault();
                document.querySelector(this.getAttribute('href')).scrollIntoView({
                    behavior: 'smooth'
                });
            });
        });
    </script>
    <?php if (isset($footerScripts)) echo $footerScripts; ?>
</body>
</html> 