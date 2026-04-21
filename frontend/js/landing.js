/* ═══════════════════════════════════════════════════════════════
   SPARC ENERGY — Landing Page Interaction Logic
═══════════════════════════════════════════════════════════════ */

'use strict';

// Global scroll variables
let scrollY = window.scrollY;

function initNav() {
  const nav = document.getElementById('mainNav');
  const fab = document.getElementById('fab');

  if (!nav) return;

  window.addEventListener('scroll', () => {
    scrollY = window.scrollY;

    // Navbar background transition
    if (scrollY > 50) {
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

    if (vidBgs.length === panels.length) {
      vidBgs.forEach((v, idx) => {
        if (idx === activeIdx) v.classList.add('active');
        else v.classList.remove('active');
      });
    }
  }, { passive: true });

  window.dispatchEvent(new Event('scroll'));
}

// ── Number Counters ──
function initCounters() {
  const counters = document.querySelectorAll('[data-count]');
  const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        const el = entry.target;
        const target = parseInt(el.getAttribute('data-count'), 10);
        const suffix = el.getAttribute('data-suffix') || '';

        let startTime = null;
        const duration = 2000; // ms

        const easeOutExpo = (t) => t === 1 ? 1 : 1 - Math.pow(2, -10 * t);

        const step = (now) => {
          if (!startTime) startTime = now;
          const progress = Math.min((now - startTime) / duration, 1);
          const current = Math.floor(easeOutExpo(progress) * target);
          el.innerText = current + suffix;
          if (progress < 1) {
            requestAnimationFrame(step);
          } else {
            el.innerText = target + suffix;
          }
        };

        requestAnimationFrame(step);
        observer.unobserve(el);
      }
    });
  }, { threshold: 0.5 });

  counters.forEach(c => observer.observe(c));
}

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
  initCounters();
  initForm();
});
