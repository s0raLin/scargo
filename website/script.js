// 导航栏滚动效果
window.addEventListener('scroll', function() {
    const navbar = document.querySelector('.navbar');
    if (window.scrollY > 50) {
        navbar.style.background = 'rgba(255, 255, 255, 0.98)';
        navbar.style.boxShadow = '0 2px 10px rgba(0, 0, 0, 0.1)';
    } else {
        navbar.style.background = 'rgba(255, 255, 255, 0.95)';
        navbar.style.boxShadow = 'none';
    }
});

// 平滑滚动到锚点
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
        e.preventDefault();
        const target = document.querySelector(this.getAttribute('href'));
        if (target) {
            const offsetTop = target.offsetTop - 70; // 减去导航栏高度
            window.scrollTo({
                top: offsetTop,
                behavior: 'smooth'
            });
        }
    });
});

// 功能卡片悬停效果增强
document.querySelectorAll('.feature-card').forEach(card => {
    card.addEventListener('mouseenter', function() {
        this.style.transform = 'translateY(-8px) scale(1.02)';
    });

    card.addEventListener('mouseleave', function() {
        this.style.transform = 'translateY(0) scale(1)';
    });
});

// 代码块复制功能
document.querySelectorAll('pre code').forEach(codeBlock => {
    const copyButton = document.createElement('button');
    copyButton.innerHTML = '<i class="fas fa-copy"></i>';
    copyButton.className = 'copy-button';
    copyButton.title = '复制代码';

    copyButton.addEventListener('click', function() {
        const code = codeBlock.textContent;
        navigator.clipboard.writeText(code).then(() => {
            const originalIcon = this.innerHTML;
            this.innerHTML = '<i class="fas fa-check"></i>';
            this.style.color = '#10b981';

            setTimeout(() => {
                this.innerHTML = originalIcon;
                this.style.color = '';
            }, 2000);
        });
    });

    const pre = codeBlock.parentElement;
    pre.style.position = 'relative';
    pre.appendChild(copyButton);
});

// 添加复制按钮样式
const style = document.createElement('style');
style.textContent = `
    .copy-button {
        position: absolute;
        top: 10px;
        right: 10px;
        background: rgba(0, 0, 0, 0.7);
        color: white;
        border: none;
        border-radius: 4px;
        padding: 5px 8px;
        cursor: pointer;
        font-size: 12px;
        opacity: 0;
        transition: all 0.3s ease;
    }

    pre:hover .copy-button {
        opacity: 1;
    }

    .copy-button:hover {
        background: rgba(0, 0, 0, 0.9);
    }
`;
document.head.appendChild(style);

// 页面加载动画
document.addEventListener('DOMContentLoaded', function() {
    // 为功能卡片添加延迟动画
    document.querySelectorAll('.feature-card').forEach((card, index) => {
        card.style.opacity = '0';
        card.style.transform = 'translateY(20px)';

        setTimeout(() => {
            card.style.transition = 'all 0.6s ease';
            card.style.opacity = '1';
            card.style.transform = 'translateY(0)';
        }, index * 100);
    });

    // 为使用步骤添加动画
    document.querySelectorAll('.usage-step').forEach((step, index) => {
        step.style.opacity = '0';
        step.style.transform = 'translateY(30px)';

        setTimeout(() => {
            step.style.transition = 'all 0.6s ease';
            step.style.opacity = '1';
            step.style.transform = 'translateY(0)';
        }, index * 150);
    });
});

// 响应式导航菜单（移动端）
function createMobileMenu() {
    const nav = document.querySelector('.navbar');
    const navLinks = document.querySelector('.nav-links');

    if (window.innerWidth <= 768) {
        // 创建汉堡菜单按钮
        const menuButton = document.createElement('button');
        menuButton.innerHTML = '<i class="fas fa-bars"></i>';
        menuButton.className = 'mobile-menu-btn';
        menuButton.style.cssText = `
            background: none;
            border: none;
            color: var(--text-secondary);
            font-size: 1.2rem;
            cursor: pointer;
            padding: 0.5rem;
        `;

        menuButton.addEventListener('click', function() {
            navLinks.style.display = navLinks.style.display === 'flex' ? 'none' : 'flex';
            navLinks.style.flexDirection = 'column';
            navLinks.style.position = 'absolute';
            navLinks.style.top = '70px';
            navLinks.style.left = '0';
            navLinks.style.right = '0';
            navLinks.style.background = 'white';
            navLinks.style.padding = '1rem';
            navLinks.style.boxShadow = '0 4px 6px rgba(0, 0, 0, 0.1)';
        });

        nav.querySelector('.nav-container').appendChild(menuButton);
    }
}

// 初始化移动菜单
createMobileMenu();

// 窗口大小改变时重新检查
window.addEventListener('resize', function() {
    const mobileBtn = document.querySelector('.mobile-menu-btn');
    if (window.innerWidth > 768 && mobileBtn) {
        mobileBtn.remove();
        document.querySelector('.nav-links').style.display = '';
    } else if (window.innerWidth <= 768 && !mobileBtn) {
        createMobileMenu();
    }
});