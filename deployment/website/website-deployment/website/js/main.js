/**
 * ËTRID Website - Main JavaScript
 * Handles interactions, animations, and dynamic content
 */

// ==================== //
// Mobile Menu Toggle   //
// ==================== //

document.addEventListener('DOMContentLoaded', function() {
    const mobileMenuBtn = document.getElementById('mobile-menu-btn');
    const mobileMenu = document.getElementById('mobile-menu');

    if (mobileMenuBtn && mobileMenu) {
        mobileMenuBtn.addEventListener('click', function() {
            mobileMenu.classList.toggle('hidden');
        });

        // Close mobile menu when clicking on a link
        const mobileLinks = mobileMenu.querySelectorAll('a');
        mobileLinks.forEach(link => {
            link.addEventListener('click', () => {
                mobileMenu.classList.add('hidden');
            });
        });
    }
});

// ==================== //
// Smooth Scrolling     //
// ==================== //

document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function(e) {
        const href = this.getAttribute('href');

        // Skip if href is just "#"
        if (href === '#') {
            e.preventDefault();
            window.scrollTo({ top: 0, behavior: 'smooth' });
            return;
        }

        const target = document.querySelector(href);
        if (target) {
            e.preventDefault();
            const navHeight = 64; // Height of fixed nav
            const targetPosition = target.offsetTop - navHeight;

            window.scrollTo({
                top: targetPosition,
                behavior: 'smooth'
            });
        }
    });
});

// ==================== //
// Scroll Reveal        //
// ==================== //

function revealOnScroll() {
    const reveals = document.querySelectorAll('.scroll-reveal');

    reveals.forEach(element => {
        const elementTop = element.getBoundingClientRect().top;
        const elementVisible = 150;

        if (elementTop < window.innerHeight - elementVisible) {
            element.classList.add('active');
        }
    });
}

window.addEventListener('scroll', revealOnScroll);
revealOnScroll(); // Initial check

// ==================== //
// Navigation Scroll    //
// ==================== //

let lastScroll = 0;
const nav = document.querySelector('nav');

window.addEventListener('scroll', () => {
    const currentScroll = window.pageYOffset;

    // Add/remove shadow based on scroll position
    if (currentScroll > 10) {
        nav.style.boxShadow = '0 4px 6px -1px rgba(0, 0, 0, 0.1)';
    } else {
        nav.style.boxShadow = 'none';
    }

    lastScroll = currentScroll;
});

// ==================== //
// Stats Counter        //
// ==================== //

function animateCounter(element, target, duration = 2000) {
    const start = 0;
    const increment = target / (duration / 16);
    let current = start;

    const timer = setInterval(() => {
        current += increment;
        if (current >= target) {
            element.textContent = formatNumber(target);
            clearInterval(timer);
        } else {
            element.textContent = formatNumber(Math.floor(current));
        }
    }, 16);
}

function formatNumber(num) {
    if (num >= 1000) {
        return (num / 1000).toFixed(0) + 'k+';
    }
    return num.toString();
}

// Trigger counter animation when stats section is visible
const statsObserver = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            const statCards = entry.target.querySelectorAll('.stat-card');
            statCards.forEach((card, index) => {
                const value = card.querySelector('div:first-child');
                const text = value.textContent;

                // Parse the numeric value
                let targetValue = 0;
                if (text.includes('142k+')) {
                    targetValue = 142;
                } else if (text.includes('<1s')) {
                    // Skip animation for time-based stats
                    return;
                } else if (text.includes('15+')) {
                    targetValue = 15;
                } else if (text.includes('100%')) {
                    targetValue = 100;
                }

                // Delay each counter slightly
                setTimeout(() => {
                    if (targetValue > 0) {
                        value.textContent = '0';
                        animateCounter(value, targetValue);

                        // Add the suffix back
                        setTimeout(() => {
                            if (text.includes('k+')) {
                                value.textContent = value.textContent + 'k+';
                            } else if (text.includes('+')) {
                                value.textContent = value.textContent + '+';
                            } else if (text.includes('%')) {
                                value.textContent = value.textContent + '%';
                            }
                        }, 2100);
                    }
                }, index * 100);
            });

            statsObserver.unobserve(entry.target);
        }
    });
}, { threshold: 0.5 });

const statsSection = document.querySelector('.stat-card')?.parentElement?.parentElement;
if (statsSection) {
    statsObserver.observe(statsSection);
}

// ==================== //
// Feature Cards Stagger //
// ==================== //

const featureObserver = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            const cards = entry.target.querySelectorAll('.feature-card');
            cards.forEach((card, index) => {
                setTimeout(() => {
                    card.style.opacity = '0';
                    card.style.transform = 'translateY(20px)';
                    card.style.transition = 'all 0.5s ease-out';

                    setTimeout(() => {
                        card.style.opacity = '1';
                        card.style.transform = 'translateY(0)';
                    }, 50);
                }, index * 100);
            });

            featureObserver.unobserve(entry.target);
        }
    });
}, { threshold: 0.2 });

const featuresGrid = document.querySelector('#features .grid');
if (featuresGrid) {
    featureObserver.observe(featuresGrid);
}

// ==================== //
// App Cards Animation  //
// ==================== //

const appObserver = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            const cards = entry.target.querySelectorAll('.app-card');
            cards.forEach((card, index) => {
                setTimeout(() => {
                    card.style.opacity = '0';
                    card.style.transform = 'translateY(20px)';
                    card.style.transition = 'all 0.5s ease-out';

                    setTimeout(() => {
                        card.style.opacity = '1';
                        card.style.transform = 'translateY(0)';
                    }, 50);
                }, index * 75);
            });

            appObserver.unobserve(entry.target);
        }
    });
}, { threshold: 0.2 });

const appsGrid = document.querySelector('#apps .grid');
if (appsGrid) {
    appObserver.observe(appsGrid);
}

// ==================== //
// Floating Orbs        //
// ==================== //

function animateOrbs() {
    const orbs = document.querySelectorAll('.floating-orb');

    orbs.forEach((orb, index) => {
        // Random movement
        const randomX = Math.random() * 100 - 50;
        const randomY = Math.random() * 100 - 50;

        setInterval(() => {
            orb.style.transition = 'transform 10s ease-in-out';
            orb.style.transform = `translate(${randomX}px, ${randomY}px)`;
        }, 10000 + index * 1000);
    });
}

animateOrbs();

// ==================== //
// Technology Layer Animation //
// ==================== //

const techObserver = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            const layers = entry.target.querySelectorAll('.tech-layer');
            layers.forEach((layer, index) => {
                setTimeout(() => {
                    layer.style.opacity = '1';
                }, index * 150);
            });

            techObserver.unobserve(entry.target);
        }
    });
}, { threshold: 0.5 });

const techViz = document.querySelector('.tech-visualization');
if (techViz) {
    techObserver.observe(techViz);
}

// ==================== //
// Mouse Parallax       //
// ==================== //

let mouseX = 0;
let mouseY = 0;

document.addEventListener('mousemove', (e) => {
    mouseX = e.clientX / window.innerWidth - 0.5;
    mouseY = e.clientY / window.innerHeight - 0.5;

    // Apply parallax to floating orbs
    const orbs = document.querySelectorAll('.floating-orb');
    orbs.forEach((orb, index) => {
        const speed = (index + 1) * 20;
        const x = mouseX * speed;
        const y = mouseY * speed;

        orb.style.transform = `translate(${x}px, ${y}px)`;
    });
});

// ==================== //
// Performance Monitoring //
// ==================== //

// Log performance metrics (can be removed in production)
if (window.performance && window.performance.timing) {
    window.addEventListener('load', () => {
        setTimeout(() => {
            const perfData = window.performance.timing;
            const pageLoadTime = perfData.loadEventEnd - perfData.navigationStart;
            console.log(`Page load time: ${pageLoadTime}ms`);
        }, 0);
    });
}

// ==================== //
// External Link Handling //
// ==================== //

document.querySelectorAll('a[target="_blank"]').forEach(link => {
    link.setAttribute('rel', 'noopener noreferrer');
});

// ==================== //
// Console Easter Egg   //
// ==================== //

console.log('%c ËTRID Protocol ', 'background: linear-gradient(135deg, #3B82F6, #8B5CF6); color: white; font-size: 20px; font-weight: bold; padding: 10px;');
console.log('%c The Future of Multichain Infrastructure ', 'color: #3B82F6; font-size: 14px;');
console.log('%c GitHub: https://github.com/EojEdred/Etrid ', 'color: #666; font-size: 12px;');
console.log('%c Interested in contributing? Check out our docs! ', 'color: #8B5CF6; font-size: 12px;');

// ==================== //
// Accessibility        //
// ==================== //

// Add keyboard navigation support
document.querySelectorAll('.feature-card, .app-card, .social-card').forEach(element => {
    element.setAttribute('tabindex', '0');

    element.addEventListener('keypress', (e) => {
        if (e.key === 'Enter' || e.key === ' ') {
            element.click();
        }
    });
});

// Focus visible polyfill
document.addEventListener('keydown', () => {
    document.body.classList.add('keyboard-navigation');
});

document.addEventListener('mousedown', () => {
    document.body.classList.remove('keyboard-navigation');
});

// ==================== //
// Lazy Loading Images  //
// ==================== //

if ('loading' in HTMLImageElement.prototype) {
    const images = document.querySelectorAll('img[loading="lazy"]');
    images.forEach(img => {
        img.src = img.dataset.src;
    });
} else {
    // Fallback for browsers that don't support lazy loading
    const script = document.createElement('script');
    script.src = 'https://cdnjs.cloudflare.com/ajax/libs/lazysizes/5.3.2/lazysizes.min.js';
    document.body.appendChild(script);
}

// ==================== //
// Error Handling       //
// ==================== //

window.addEventListener('error', (e) => {
    console.error('Global error:', e.error);
    // You could send this to an error tracking service in production
});

// ==================== //
// Service Worker (optional) //
// ==================== //

// Uncomment to enable offline support
/*
if ('serviceWorker' in navigator) {
    window.addEventListener('load', () => {
        navigator.serviceWorker.register('/sw.js')
            .then(reg => console.log('Service Worker registered'))
            .catch(err => console.log('Service Worker registration failed'));
    });
}
*/