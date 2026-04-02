// ═══════════════════════════════════════════════════════════════
// Sparc Energy — API Helper
// ═══════════════════════════════════════════════════════════════

// Connect exactly to your newly-deployed Render Cloud DB Service!
const API_BASE = 'https://sparc-energy.onrender.com/api';


/**
 * Main API fetch wrapper
 * Automatically attaches JWT token and handles JSON parsing
 */
async function api(endpoint, options = {}) {
  const token = localStorage.getItem('sparc_token');
  const headers = {
    'Content-Type': 'application/json',
    ...(token ? { 'Authorization': `Bearer ${token}` } : {}),
    ...(options.headers || {}),
  };

  try {
    const response = await fetch(`${API_BASE}${endpoint}`, {
      ...options,
      headers,
    });

    const data = await response.json();

    // Handle token expiry
    if (response.status === 401) {
      localStorage.removeItem('sparc_token');
      localStorage.removeItem('sparc_user');
    }

    return data;
  } catch (error) {
    console.error(`API Error [${endpoint}]:`, error);
    throw error;
  }
}

// ── Toast Notifications ───────────────────────────────────────────────────────
function showToast(type, message, sub = '') {
  const container = document.getElementById('toastContainer');
  if (!container) return;
  const icons = { success: '✅', error: '❌', info: 'ℹ️', warning: '⚠️' };
  const el = document.createElement('div');
  el.className = `toast toast-${type}`;
  el.innerHTML = `
    <div class="toast-icon">${icons[type]||'ℹ️'}</div>
    <div>
      <div class="toast-msg">${message}</div>
      ${sub ? `<div class="toast-sub">${sub}</div>` : ''}
    </div>`;
  container.appendChild(el);
  setTimeout(() => {
    el.style.opacity = '0';
    el.style.transform = 'translateX(20px)';
    el.style.transition = 'all 0.3s ease';
    setTimeout(() => el.remove(), 300);
  }, 4000);
}

// ── Modal Helpers ─────────────────────────────────────────────────────────────
function openModal(id) { document.getElementById(id)?.classList.add('open'); }
function closeModal(id) { document.getElementById(id)?.classList.remove('open'); }
document.addEventListener('click', e => {
  if (e.target.classList.contains('modal-overlay')) {
    e.target.classList.remove('open');
  }
});

// ── Number Formatting ─────────────────────────────────────────────────────────
function fmt(n, decimals = 2) { return Number(n).toFixed(decimals); }
function fmtCurrency(n) { return '$' + Number(n).toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 }); }
function fmtNum(n) {
  n = Number(n);
  if (n >= 1000000) return (n/1000000).toFixed(1) + 'M';
  if (n >= 1000) return (n/1000).toFixed(1) + 'K';
  return n.toLocaleString();
}
function fmtDate(str) {
  return new Date(str).toLocaleDateString('en-IN', { year:'numeric', month:'short', day:'numeric', hour:'2-digit', minute:'2-digit' });
}

// ── Ticker Builder ────────────────────────────────────────────────────────────
const TICKER_DATA = [
  { name:'AMZN-REFOR-VCS', price:18.50, change:3.2 },
  { name:'RAJ-SOLAR-GS', price:22.75, change:1.8 },
  { name:'NORDIC-WIND-GS', price:31.20, change:-0.5 },
  { name:'GUJ-BLUE-VCS', price:14.80, change:2.1 },
  { name:'AMZN-VCS-22', price:16.40, change:-1.2 },
  { name:'KA-SOLAR-BEE', price:19.60, change:4.1 },
  { name:'BRA-REDD-CDM', price:12.30, change:0.8 },
  { name:'GH-COOK-GS', price:8.90, change:1.5 },
  { name:'MH-WIND-VCS', price:21.10, change:0.9 },
  { name:'SEA-MANGROVE-GS', price:17.30, change:2.6 },
];

function buildTickerTrack() {
  const el = document.getElementById('tickerTrack');
  if (!el) return;
  const doubled = [...TICKER_DATA, ...TICKER_DATA];
  el.innerHTML = doubled.map(t => `
    <div class="ticker-item">
      <div class="t-dot"></div>
      <span class="t-name">${t.name}</span>
      <span class="t-price">$${t.price.toFixed(2)}</span>
      <span class="t-change ${t.change >= 0 ? 'up' : 'down'}">${t.change >= 0 ? '+' : ''}${t.change}%</span>
    </div>`).join('');
  // Live price simulation
  setInterval(() => {
    TICKER_DATA.forEach(t => {
      t.price += (Math.random() - 0.48) * 0.1;
      t.price = Math.max(5, t.price);
      t.change = parseFloat(((Math.random() - 0.4) * 5).toFixed(2));
    });
    el.innerHTML = [...TICKER_DATA, ...TICKER_DATA].map(t => `
      <div class="ticker-item">
        <div class="t-dot"></div>
        <span class="t-name">${t.name}</span>
        <span class="t-price">$${t.price.toFixed(2)}</span>
        <span class="t-change ${t.change >= 0 ? 'up' : 'down'}">${t.change >= 0 ? '+' : ''}${t.change}%</span>
      </div>`).join('');
  }, 3000);
}
buildTickerTrack();
