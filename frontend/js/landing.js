/* ═══════════════════════════════════════════════════════════════
   SPARC ENERGY — Landing Page Interaction Logic
═══════════════════════════════════════════════════════════════ */

'use strict';

// Global scroll variables
let scrollY = window.scrollY;

function initNav() {
  const nav = document.getElementById('mainNav');
  const fab = document.getElementById('fab');
  const burger = document.getElementById('navBurger');
  const menu = document.getElementById('navMenu');

  if (!nav) return;

  // Scroll behavior
  window.addEventListener('scroll', () => {
    scrollY = window.scrollY;

    // Navbar background transition
    if (scrollY > 100) {
      nav.classList.add('scrolled');
    } else {
      nav.classList.remove('scrolled');
    }

    // FAB visibility
    if (fab) {
      if (scrollY > window.innerHeight * 0.8) {
        fab.classList.add('visible');
      } else {
        fab.classList.remove('visible');
      }
    }
  }, { passive: true });

  // Mobile menu toggle
  if (burger && menu) {
    burger.addEventListener('click', () => {
      const isExpanded = burger.getAttribute('aria-expanded') === 'true';
      burger.setAttribute('aria-expanded', !isExpanded);
      menu.classList.toggle('active');
    });
  }

  // Dropdown toggle for mobile
  const dropdownTriggers = document.querySelectorAll('.nav-trigger');
  dropdownTriggers.forEach(trigger => {
    trigger.addEventListener('click', (e) => {
      if (window.innerWidth <= 1024) {
        e.preventDefault();
        const dropdown = trigger.closest('.nav-dropdown');
        dropdown.classList.toggle('active');
      }
    });
  });

  // Close menu when clicking outside
  document.addEventListener('click', (e) => {
    if (menu && burger && !nav.contains(e.target)) {
      burger.setAttribute('aria-expanded', 'false');
      menu.classList.remove('active');
    }
  });
}

// ── Intersection Observer for Reveals ──
function initReveals() {
  const reveals = document.querySelectorAll('.reveal, .reveal-l, .reveal-r');
  const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        entry.target.classList.add('revealed');
        observer.unobserve(entry.target);
      }
    });
  }, { threshold: 0.15, rootMargin: "0px 0px -50px 0px" });

  reveals.forEach(el => observer.observe(el));
}

// ── Scroll Story Logic (Apple-style scroll pinning) ──
function initScrollStory() {
  const outer = document.querySelector('.story-outer');
  const panels = document.querySelectorAll('.story-panel');
  const vidBgs = document.querySelectorAll('.story-vid-bg');
  const progressFill = document.getElementById('storyFill');

  if (!outer || panels.length === 0 || window.innerWidth <= 1024) {
    if (panels.length > 0) {
      panels.forEach(p => p.classList.add('active'));
    }
    return;
  }

  window.addEventListener('scroll', () => {
    const rect = outer.getBoundingClientRect();
    const outerTop = rect.top;
    const outerHeight = rect.height;

    if (outerTop > window.innerHeight || outerTop < -outerHeight) return;

    const scrollDistance = outerHeight - window.innerHeight;
    let progress = -outerTop / scrollDistance;
    progress = Math.max(0, Math.min(1, progress));

    if (progressFill) progressFill.style.width = `${progress * 100}%`;

    let activeIdx = Math.floor(progress * panels.length);
    if (activeIdx >= panels.length) activeIdx = panels.length - 1;

    panels.forEach((p, idx) => {
      if (idx === activeIdx) {
        p.classList.add('active');
        const panelProgress = (progress * panels.length) % 1;
        p.style.transform = `translateY(${(1 - panelProgress) * 20}px)`;
      } else {
        p.classList.remove('active');
        p.style.transform = `translateY(40px)`;
      }
    });

    // Ensure video backgrounds sync with panels
    vidBgs.forEach((v, idx) => {
      if (idx === activeIdx) {
        v.classList.add('active');
      } else {
        v.classList.remove('active');
      }
    });
  }, { passive: true });

  window.dispatchEvent(new Event('scroll'));
}

// ── Number Counters ──
document.addEventListener('DOMContentLoaded', function() {
  function runCounter(el) {
    const target = parseFloat(el.dataset.count);
    const suffix = el.dataset.suffix || '';
    const duration = 2000;
    const startTime = performance.now();
    function frame(now) {
      const elapsed = now - startTime;
      const progress = Math.min(elapsed / duration, 1);
      const eased = 1 - Math.pow(1 - progress, 3);
      const current = Math.floor(eased * target);
      el.textContent = current >= 1000
        ? current.toLocaleString('en-IN') + suffix
        : current + suffix;
      if (progress < 1) requestAnimationFrame(frame);
      else el.textContent = target >= 1000
        ? target.toLocaleString('en-IN') + suffix
        : target + suffix;
    }
    requestAnimationFrame(frame);
  }
  const seen = new Set();
  const observer = new IntersectionObserver(entries => {
    entries.forEach(entry => {
      if (entry.isIntersecting && !seen.has(entry.target)) {
        seen.add(entry.target);
        runCounter(entry.target);
      }
    });
  }, { threshold: 0.1 });
  document.querySelectorAll('[data-count]').forEach(el => {
    el.textContent = '0';
    observer.observe(el);
  });
});

// ── Contact Form ──
function initForm() {
  const form = document.getElementById('contactForm');
  const btn = document.getElementById('formBtn');
  const txt = document.getElementById('formBtnTxt');
  const ok = document.getElementById('formOk');

  if (!form) return;

  form.addEventListener('submit', (e) => {
    e.preventDefault();

    // basic validation
    const inputs = form.querySelectorAll('input[required], textarea[required]');
    let valid = true;
    inputs.forEach(i => {
      if (!i.value.trim()) {
        valid = false;
        i.style.borderColor = 'red';
        setTimeout(() => i.style.borderColor = '', 2000);
      }
    });

    if (!valid) return;

    btn.disabled = true;
    txt.innerText = 'Sending...';

    // Simulate async send
    setTimeout(() => {
      form.reset();
      btn.disabled = false;
      txt.innerText = 'Send Message';
      ok.classList.remove('hidden');
      setTimeout(() => ok.classList.add('hidden'), 5000);
    }, 1200);
  });
}

// ── Initialization ──
document.addEventListener('DOMContentLoaded', () => {
  initNav();
  initReveals();
  initScrollStory();
  initForm();
});
