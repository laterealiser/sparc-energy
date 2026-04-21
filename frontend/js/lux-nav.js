/* ═══════════════════════════════════════════════════════════════
   SPARC ENERGY — Luxury Nav + Footer + Reveal Logic
   Include on every page that uses luxury.css
═══════════════════════════════════════════════════════════════ */
'use strict';

/* ── Inject Nav HTML ── */
function renderNav(activePage) {
  const links = [
    { href: 'index.html',        label: 'Home'     },
    { href: 'projects.html',     label: 'Projects'  },
    { href: 'experts.html',      label: 'Experts'   },
    { href: 'about.html',        label: 'About'     },
  ];

  const navLinks = links.map(l => `
    <a href="${l.href}" class="lux-nav-link${activePage === l.label ? ' active' : ''}">${l.label}</a>
  `).join('');

  const dropdown = `
    <div class="lux-dropdown">
      <button class="lux-nav-link lux-dropdown-trigger${activePage === 'Resources' ? ' active' : ''}" aria-expanded="false">
        Resources
        <svg width="12" height="12" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M6 9l6 6 6-6"/></svg>
      </button>
      <div class="lux-dropdown-panel">
        <div class="lux-dd-section">
          <div class="lux-dd-label">Tools</div>
          <a href="tools/estimator.html" class="lux-dd-item">
            <div class="lux-dd-icon">📊</div>
            <div><div class="lux-dd-title">Carbon Credit Estimator</div><div class="lux-dd-desc">Calculate your offset needs</div></div>
          </a>
          <a href="tools/scope-calculator.html" class="lux-dd-item">
            <div class="lux-dd-icon">🔍</div>
            <div><div class="lux-dd-title">Scope 1/2/3 Calculator</div><div class="lux-dd-desc">Measure your emissions</div></div>
          </a>
          <a href="tools/brsr-generator.html" class="lux-dd-item">
            <div class="lux-dd-icon">📄</div>
            <div><div class="lux-dd-title">BRSR Report Generator</div><div class="lux-dd-desc">Generate compliance reports</div></div>
          </a>
        </div>
        <div class="lux-dd-section">
          <div class="lux-dd-label">Learn</div>
          <a href="intelligence.html" class="lux-dd-item">
            <div class="lux-dd-icon">🤖</div>
            <div><div class="lux-dd-title">Carbon Intelligence AI</div><div class="lux-dd-desc">Ask our AI assistant</div></div>
          </a>
          <a href="carbon-101.html" class="lux-dd-item">
            <div class="lux-dd-icon">📚</div>
            <div><div class="lux-dd-title">Carbon 101</div><div class="lux-dd-desc">Learn the basics</div></div>
          </a>
        </div>
      </div>
    </div>`;

  const html = `
    <header class="lux-nav" id="luxNav">
      <div class="lux-nav-inner">
        <a href="index.html" class="lux-brand">
          <div class="lux-brand-icon">
            <svg width="20" height="20" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" viewBox="0 0 24 24">
              <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
            </svg>
          </div>
          <div class="lux-brand-text">
            <span class="lux-brand-name">Sparc Energy</span>
            <span class="lux-brand-sub">Carbon Intelligence</span>
          </div>
        </a>
        <nav class="lux-nav-links" id="luxNavLinks" role="navigation">
          ${navLinks}
          ${dropdown}
        </nav>
        <div class="lux-nav-actions">
          <a href="auth.html" class="lux-btn-ghost">Sign In</a>
          <a href="auth.html?mode=register" class="lux-btn-primary">Sign Up</a>
          <button class="lux-burger" id="luxBurger" aria-label="Menu" aria-expanded="false">
            <span></span><span></span><span></span>
          </button>
        </div>
      </div>
    </header>`;

  document.body.insertAdjacentHTML('afterbegin', html);
}

/* ── Inject Footer HTML ── */
function renderFooter() {
  const html = `
    <footer class="lux-footer">
      <div class="lux-footer-grid">
        <div>
          <a href="index.html" style="display:inline-flex;align-items:center;gap:10px;margin-bottom:4px;">
            <div style="width:32px;height:32px;background:rgba(82,183,136,0.15);border-radius:8px;display:flex;align-items:center;justify-content:center;color:#52b788;">
              <svg width="16" height="16" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" viewBox="0 0 24 24"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>
            </div>
            <div>
              <span class="lux-footer-brand-name">Sparc Energy</span>
              <span class="lux-footer-brand-sub">Carbon Intelligence</span>
            </div>
          </a>
          <p class="lux-footer-desc">Boutique climate intelligence for organisations serious about net zero. Science-based, AI-augmented, personally delivered.</p>
          <div class="lux-footer-socials">
            <a href="https://linkedin.com/in/sparcenergy" class="lux-footer-soc" aria-label="LinkedIn" target="_blank" rel="noopener">
              <svg width="15" height="15" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-4 0v7h-4v-7a6 6 0 0 1 6-6z"/><rect x="2" y="9" width="4" height="12"/><circle cx="4" cy="4" r="2"/></svg>
            </a>
            <a href="mailto:info@sparcenergy.in" class="lux-footer-soc" aria-label="Email">
              <svg width="15" height="15" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/><polyline points="22,6 12,13 2,6"/></svg>
            </a>
          </div>
        </div>
        <div>
          <div class="lux-footer-col-title">Services</div>
          <a href="index.html#services" class="lux-footer-link">Carbon Measurement</a>
          <a href="index.html#services" class="lux-footer-link">Net Zero Strategy</a>
          <a href="index.html#services" class="lux-footer-link">Credit Procurement</a>
          <a href="index.html#services" class="lux-footer-link">ESG Reporting</a>
        </div>
        <div>
          <div class="lux-footer-col-title">Tools</div>
          <a href="tools/estimator.html" class="lux-footer-link">Carbon Estimator</a>
          <a href="tools/scope-calculator.html" class="lux-footer-link">Scope Calculator</a>
          <a href="tools/brsr-generator.html" class="lux-footer-link">BRSR Generator</a>
          <a href="intelligence.html" class="lux-footer-link">Carbon AI</a>
        </div>
        <div>
          <div class="lux-footer-col-title">Company</div>
          <a href="about.html" class="lux-footer-link">About</a>
          <a href="experts.html" class="lux-footer-link">Experts</a>
          <a href="carbon-101.html" class="lux-footer-link">Carbon 101</a>
          <a href="index.html#contact" class="lux-footer-link">Contact</a>
          <a href="privacy.html" class="lux-footer-link">Privacy</a>
          <a href="terms.html" class="lux-footer-link">Terms</a>
        </div>
      </div>
      <div class="lux-footer-bottom">
        <span>© 2025 Sparc Energy. All rights reserved.</span>
        <div class="lux-footer-certs">
          <span>🌿 Verra VCS</span>
          <span>🥇 Gold Standard</span>
          <span>📊 SBTi Aligned</span>
        </div>
      </div>
    </footer>`;

  document.body.insertAdjacentHTML('beforeend', html);
}

/* ── Nav Behaviour ── */
function initLuxNav() {
  const nav    = document.getElementById('luxNav');
  const burger = document.getElementById('luxBurger');
  const links  = document.getElementById('luxNavLinks');

  if (!nav) return;

  // Scroll
  window.addEventListener('scroll', () => {
    nav.classList.toggle('scrolled', window.scrollY > 40);
  }, { passive: true });

  // Burger
  if (burger && links) {
    burger.addEventListener('click', () => {
      const open = burger.getAttribute('aria-expanded') === 'true';
      burger.setAttribute('aria-expanded', String(!open));
      links.classList.toggle('open', !open);
    });
  }

  // Mobile dropdown
  document.querySelectorAll('.lux-dropdown-trigger').forEach(btn => {
    btn.addEventListener('click', e => {
      if (window.innerWidth <= 1024) {
        e.preventDefault();
        btn.closest('.lux-dropdown').classList.toggle('open');
      }
    });
  });

  // Close on outside click
  document.addEventListener('click', e => {
    if (links && burger && !nav.contains(e.target)) {
      burger.setAttribute('aria-expanded', 'false');
      links.classList.remove('open');
    }
  });
}

/* ── Reveal on Scroll ── */
function initReveals() {
  const els = document.querySelectorAll('.lux-reveal, .lux-reveal-l, .lux-reveal-r');
  const io = new IntersectionObserver(entries => {
    entries.forEach(e => {
      if (e.isIntersecting) {
        e.target.classList.add('visible');
        io.unobserve(e.target);
      }
    });
  }, { threshold: 0.12, rootMargin: '0px 0px -40px 0px' });
  els.forEach(el => io.observe(el));
}

/* ── Auto-init ── */
document.addEventListener('DOMContentLoaded', () => {
  initLuxNav();
  initReveals();
});
