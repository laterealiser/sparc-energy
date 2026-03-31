// ═══════════════════════════════════════════════════════════════
// Sparc Energy — API Helper
// ═══════════════════════════════════════════════════════════════

// IMPORTANT: Change this URL after deploying your backend to Shuttle.rs
// During local development: http://localhost:8080
// After deployment: https://your-app-name.shuttleapp.rs
const API_BASE = 'http://localhost:8080/api';

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
    // Return mock data for demo when backend is offline
    return getMockData(endpoint);
  }
}

// ── Mock Data (for when backend is offline) ──────────────────────────────────
function getMockData(endpoint) {
  if (endpoint.includes('/credits') && !endpoint.includes('/history') && !endpoint.includes('/buy')) {
    return {
      success: true,
      data: MOCK_CREDITS
    };
  }
  if (endpoint.includes('/projects')) {
    return { success: true, data: MOCK_PROJECTS };
  }
  if (endpoint.includes('/market/stats')) {
    return { success: true, data: MOCK_STATS };
  }
  if (endpoint.includes('/market/trades')) {
    return { success: true, data: MOCK_TRADES };
  }
  if (endpoint.includes('/dashboard')) {
    return { success: true, data: MOCK_DASHBOARD };
  }
  return { success: false, error: 'Cannot connect to Sparc Energy server. Make sure the backend is running.' };
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

// ── Mock Data ──────────────────────────────────────────────────────────────────
const MOCK_CREDITS = [
  { id:'c1', project_id:'p1', project_name:'Amazon Reforestation Initiative', project_type:'reforestation', country:'Brazil', seller_id:'s1', seller_name:'Amazon Reforestation Ltd', price_per_ton:18.50, quantity_tons:50000, quantity_available:50000, status:'active', vintage_year:2023, certification:'Verra VCS', serial_number:'VCS-BRA-2023-001', methodology:'VM0007', created_at: new Date().toISOString() },
  { id:'c2', project_id:'p2', project_name:'Rajasthan Solar Farm', project_type:'solar', country:'India', seller_id:'s2', seller_name:'Renewable India Power', price_per_ton:22.75, quantity_tons:30000, quantity_available:30000, status:'active', vintage_year:2024, certification:'Gold Standard', serial_number:'GS-IND-2024-001', methodology:'AMS-I.D', created_at: new Date().toISOString() },
  { id:'c3', project_id:'p3', project_name:'North Sea Wind Offshore', project_type:'wind', country:'Norway', seller_id:'s3', seller_name:'Nordic Clean Energy', price_per_ton:31.20, quantity_tons:25000, quantity_available:25000, status:'active', vintage_year:2024, certification:'Gold Standard', serial_number:'GS-NOR-2024-001', methodology:'AMS-I.D', created_at: new Date().toISOString() },
  { id:'c4', project_id:'p4', project_name:'Gujarat Mangrove Conservation', project_type:'blue_carbon', country:'India', seller_id:'s2', seller_name:'Renewable India Power', price_per_ton:14.80, quantity_tons:20000, quantity_available:20000, status:'active', vintage_year:2023, certification:'Verra VCS', serial_number:'VCS-IND-2023-002', methodology:'VM0033', created_at: new Date().toISOString() },
  { id:'c5', project_id:'p1', project_name:'Amazon Reforestation Initiative', project_type:'reforestation', country:'Brazil', seller_id:'s1', seller_name:'Amazon Reforestation Ltd', price_per_ton:16.40, quantity_tons:40000, quantity_available:40000, status:'active', vintage_year:2022, certification:'Verra VCS', serial_number:'VCS-BRA-2022-001', methodology:'VM0007', created_at: new Date().toISOString() },
];

const MOCK_PROJECTS = [
  { id:'p1', name:'Amazon Reforestation Initiative', description:'Large-scale reforestation in the Brazilian Amazon protecting 50,000 hectares.', project_type:'reforestation', location:'Amazon Basin, Pará State', country:'Brazil', owner_id:'s1', total_credits:500000, credits_issued:350000, credits_retired:0, verified:1, certification:'Verra VCS', sdg_goals:'13,15,17', co2_reduction_per_year:85000, project_start_date:'2020-01-01', created_at:new Date().toISOString(), updated_at:new Date().toISOString() },
  { id:'p2', name:'Rajasthan Solar Farm', description:'800 MW utility-scale solar plant in Rajasthan providing clean electricity to 600,000 homes.', project_type:'solar', location:'Jodhpur, Rajasthan', country:'India', owner_id:'s2', total_credits:200000, credits_issued:180000, credits_retired:0, verified:1, certification:'Gold Standard', sdg_goals:'7,9,13', co2_reduction_per_year:42000, project_start_date:'2021-06-01', created_at:new Date().toISOString(), updated_at:new Date().toISOString() },
  { id:'p3', name:'North Sea Wind Offshore', description:'Offshore wind farm in the North Sea generating 1.2 GW of clean energy for Northern Europe.', project_type:'wind', location:'North Sea, 80km offshore', country:'Norway', owner_id:'s3', total_credits:300000, credits_issued:280000, credits_retired:0, verified:1, certification:'Gold Standard', sdg_goals:'7,8,13', co2_reduction_per_year:65000, project_start_date:'2019-03-01', created_at:new Date().toISOString(), updated_at:new Date().toISOString() },
  { id:'p4', name:'Gujarat Mangrove Conservation', description:'Protection and restoration of 12,000 hectares of mangrove ecosystems along Gujarat coast.', project_type:'blue_carbon', location:'Gulf of Khambhat, Gujarat', country:'India', owner_id:'s2', total_credits:150000, credits_issued:90000, credits_retired:0, verified:1, certification:'Verra VCS', sdg_goals:'14,15,13', co2_reduction_per_year:28000, project_start_date:'2022-01-01', created_at:new Date().toISOString(), updated_at:new Date().toISOString() },
];

const MOCK_STATS = {
  total_credits_listed: 165000,
  total_volume_24h: 0,
  total_transactions: 0,
  avg_price: 20.81,
  highest_price: 31.20,
  lowest_price: 14.80,
  total_co2_offset: 0,
  total_projects: 4,
  verified_projects: 4,
};

const MOCK_TRADES = [];

const MOCK_DASHBOARD = {
  user: { id:'demo', name:'Demo Investor', email:'demo@sparcenergy.com', role:'buyer', balance:50000, total_credits_owned:0, kyc_verified:1, created_at:new Date().toISOString() },
  portfolio: [],
  transactions: [],
  summary: { total_invested:0, total_current_value:0, total_pnl:0, total_pnl_pct:0, total_credits:0, total_retired:0, co2_offset_tons:0 }
};
