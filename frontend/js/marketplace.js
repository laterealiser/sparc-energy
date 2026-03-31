// ═══════════════════════════════════════════════════════════════
// Sparc Energy — Marketplace JavaScript
// Binance-style trading interface
// ═══════════════════════════════════════════════════════════════

let allCredits = [];
let selectedCredit = null;
let creditChart = null;
let currentView = 'list';
let simulationInterval = null;

const CERT_COLORS = { 'Verra VCS': 'badge-verra', 'Gold Standard': 'badge-gold', 'CDM': 'badge-cdm' };
const TYPE_ICONS = { reforestation:'🌳', solar:'☀️', wind:'💨', blue_carbon:'🌊', methane:'🔥' };

// ── Initial Load ───────────────────────────────────────────────────────────────
async function init() {
  loadMarketStats();
  await loadCredits();
  loadRecentTrades();
  startOrderBookSimulation();
  // Check URL param for pre-selected credit
  const urlId = new URLSearchParams(window.location.search).get('id');
  if (urlId) {
    const credit = allCredits.find(c => c.id === urlId);
    if (credit) selectCredit(credit);
  }
}

// ── Market Stats ───────────────────────────────────────────────────────────────
async function loadMarketStats() {
  const res = await api('/market/stats');
  if (res.success && res.data) {
    const d = res.data;
    const el = (id, val) => { const e = document.getElementById(id); if (e) e.textContent = val; };
    el('tot-credits', fmtNum(d.total_credits_listed));
    el('tot-vol', '$' + fmtNum(d.total_volume_24h));
    el('tot-proj', d.verified_projects);
  }
}

// ── Load Credits ───────────────────────────────────────────────────────────────
async function loadCredits() {
  const res = await api('/credits?limit=50');
  if (res.success && res.data) {
    allCredits = res.data;
    renderCreditList(allCredits);
    if (currentView === 'grid') renderCreditGrid(allCredits);
  }
}

// ── Apply Filters ──────────────────────────────────────────────────────────────
function applyFilters() {
  const cert = document.getElementById('fCert')?.value || '';
  const type = document.getElementById('fType')?.value || '';
  const minP = parseFloat(document.getElementById('fMinPrice')?.value) || 0;
  const maxP = parseFloat(document.getElementById('fMaxPrice')?.value) || 999999;
  const vintage = document.getElementById('fVintage')?.value || '';
  const country = document.getElementById('fCountry')?.value || '';
  const search = document.getElementById('searchInput')?.value.toLowerCase() || '';
  const sortBy = document.getElementById('sortBy')?.value || 'created_at';

  let filtered = allCredits.filter(c => {
    if (cert && c.certification !== cert) return false;
    if (type && c.project_type !== type) return false;
    if (c.price_per_ton < minP || c.price_per_ton > maxP) return false;
    if (vintage && String(c.vintage_year) !== vintage) return false;
    if (country && c.country !== country) return false;
    if (search && !c.project_name.toLowerCase().includes(search) &&
        !c.certification.toLowerCase().includes(search) &&
        !c.country.toLowerCase().includes(search)) return false;
    return true;
  });

  // Sort
  filtered.sort((a, b) => {
    if (sortBy === 'price_asc') return a.price_per_ton - b.price_per_ton;
    if (sortBy === 'price_desc') return b.price_per_ton - a.price_per_ton;
    if (sortBy === 'quantity') return b.quantity_available - a.quantity_available;
    return new Date(b.created_at) - new Date(a.created_at);
  });

  renderCreditList(filtered);
  if (currentView === 'grid') renderCreditGrid(filtered);
}

function resetFilters() {
  ['fCert','fType','fVintage','fCountry'].forEach(id => { const el = document.getElementById(id); if(el) el.value = ''; });
  ['fMinPrice','fMaxPrice','searchInput'].forEach(id => { const el = document.getElementById(id); if(el) el.value = ''; });
  applyFilters();
}

// ── Render Credit List (Binance-style table) ───────────────────────────────────
function renderCreditList(credits) {
  const container = document.getElementById('creditsList');
  if (!container) return;

  // Simulate 24h changes
  credits.forEach(c => {
    if (!c._change) c._change = parseFloat(((Math.random() - 0.4) * 6).toFixed(2));
  });

  container.innerHTML = credits.map(c => {
    const isSelected = selectedCredit?.id === c.id;
    const change = c._change || 0;
    return `
    <div class="credit-list-item ${isSelected?'selected':''}" onclick="selectCredit(${JSON.stringify(c).replace(/"/g,'&quot;')})">
      <div>
        <div style="font-size:13px;font-weight:600;color:var(--text-primary);display:flex;align-items:center;gap:6px;">
          ${TYPE_ICONS[c.project_type]||'🌿'} ${c.project_name}
        </div>
        <div style="font-size:11px;color:var(--text-muted);">${c.country} · #${c.serial_number||c.id.slice(0,8)}</div>
      </div>
      <div>
        <div style="font-size:14px;font-weight:700;color:var(--text-primary);">$${fmt(c.price_per_ton)}</div>
        <div style="font-size:11px;color:var(--text-muted);">/tCO₂e</div>
      </div>
      <div class="${change>=0?'t-green':'t-red'}" style="font-size:13px;font-weight:600;">
        ${change>=0?'+':''}${change}%
      </div>
      <div style="font-size:13px;color:var(--text-secondary);">${fmtNum(c.quantity_available)} t</div>
      <div><span class="badge ${CERT_COLORS[c.certification]||'badge-type'}" style="font-size:10px;">${c.certification.split(' ')[0]}</span></div>
      <div style="text-align:right;">
        <button class="btn btn-primary btn-sm" onclick="event.stopPropagation();selectCredit(${JSON.stringify(c).replace(/"/g,'&quot;')})">
          Buy →
        </button>
      </div>
    </div>`;
  }).join('') || '<div style="padding:32px;text-align:center;color:var(--text-muted);">No credits match your filters</div>';
}

// ── Render Credit Grid ─────────────────────────────────────────────────────────
function renderCreditGrid(credits) {
  const container = document.getElementById('creditsGrid');
  if (!container) return;
  container.innerHTML = credits.map(c => `
    <div class="credit-card card-hover" onclick="selectCredit(${JSON.stringify(c).replace(/"/g,'&quot;')})">
      <div class="credit-card-header">
        <div>
          <div class="credit-project">${TYPE_ICONS[c.project_type]||'🌿'} ${c.project_name}</div>
          <div class="credit-type">${c.country} · Vintage ${c.vintage_year}</div>
        </div>
        <span class="badge ${CERT_COLORS[c.certification]||'badge-type'}">${c.certification}</span>
      </div>
      <div class="credit-price">$${fmt(c.price_per_ton)} <span>/ tCO₂e</span></div>
      <div class="credit-meta">
        <span class="badge badge-type">${c.project_type.replace('_',' ')}</span>
        <span class="badge ${(c._change||0)>=0?'badge-verra':'badge-cdm'} ">${(c._change||0)>=0?'+':''}${c._change||0}%</span>
      </div>
      <div class="progress-bar"><div class="progress-fill" style="width:${Math.min(95,30+Math.random()*60)}%"></div></div>
      <div class="credit-footer">
        <div class="credit-available">Available: <strong>${fmtNum(c.quantity_available)} t</strong></div>
        <button class="btn btn-primary btn-sm">Buy</button>
      </div>
    </div>`).join('');
}

// ── Select Credit (Opens Buy Panel + Chart) ────────────────────────────────────
function selectCredit(credit) {
  selectedCredit = credit;

  // Update list selection highlight
  renderCreditList(allCredits.filter(c => {
    const cert = document.getElementById('fCert')?.value || '';
    const type = document.getElementById('fType')?.value || '';
    if (cert && c.certification !== cert) return false;
    if (type && c.project_type !== type) return false;
    return true;
  }));

  // Show credit info panel
  const panel = document.getElementById('selectedCreditPanel');
  if (panel) {
    panel.style.display = 'block';
    document.getElementById('sc-name').textContent = `${TYPE_ICONS[credit.project_type]||'🌿'} ${credit.project_name}`;
    document.getElementById('sc-meta').textContent = `${credit.country} · ${credit.certification} · Vintage ${credit.vintage_year} · ${credit.serial_number}`;
    document.getElementById('sc-price').textContent = `$${fmt(credit.price_per_ton)}`;
    const change = credit._change || 0;
    const chEl = document.getElementById('sc-change');
    chEl.textContent = `${change >= 0 ? '+' : ''}${change}% (24h)`;
    chEl.style.background = change >= 0 ? 'rgba(0,212,161,0.12)' : 'rgba(240,62,62,0.12)';
    chEl.style.color = change >= 0 ? 'var(--sparc-green)' : 'var(--sparc-red)';
    loadCreditChart(credit.id, credit.price_per_ton);
  }

  // Show buy panel
  const buyPanel = document.getElementById('buyPanel');
  const buyEmpty = document.getElementById('buyPanelEmpty');
  if (buyPanel) {
    buyPanel.style.display = 'block';
    if (buyEmpty) buyEmpty.style.display = 'none';
    document.getElementById('bp-name').textContent = credit.project_name;
    document.getElementById('bp-serial').textContent = credit.serial_number || credit.id;
    document.getElementById('bp-price').textContent = '$' + fmt(credit.price_per_ton);
    document.getElementById('bp-cert').textContent = credit.certification;
    document.getElementById('bp-vintage').textContent = credit.vintage_year;
    document.getElementById('bp-avail').textContent = fmtNum(credit.quantity_available) + ' tons';
    document.getElementById('bp-seller').textContent = credit.seller_name;
    document.getElementById('bp-method').textContent = credit.methodology || 'Standard';
    document.getElementById('calc-price').textContent = fmtCurrency(credit.price_per_ton);
    document.getElementById('buyQty').value = '';
    document.getElementById('calc-fee').textContent = '$0.00';
    document.getElementById('calc-total').textContent = '$0.00';

    const user = getUser();
    const balEl = document.getElementById('bp-balance');
    if (balEl) balEl.textContent = user ? fmtCurrency(user.balance) : 'Login to buy';
  }

  // Simulate order book
  buildOrderBook(credit.price_per_ton);
}

// ── Credit Price Chart ─────────────────────────────────────────────────────────
async function loadCreditChart(creditId, basePrice) {
  const ctx = document.getElementById('creditChart');
  if (!ctx) return;

  if (creditChart) { creditChart.destroy(); creditChart = null; }

  const res = await api(`/credits/${creditId}/history`);
  let prices = [];
  let labels = [];

  if (res.success && res.data?.length > 0) {
    prices = res.data.map(h => h.price);
    labels = res.data.map(h => new Date(h.recorded_at).toLocaleDateString('en',{month:'short',day:'numeric'}));
  } else {
    // Generate synthetic chart data
    let p = basePrice * 0.8;
    for (let i = 29; i >= 0; i--) {
      p += (Math.random() - 0.44) * 0.8;
      p = Math.max(5, p);
      prices.push(parseFloat(p.toFixed(2)));
      const d = new Date(); d.setDate(d.getDate() - i);
      labels.push(d.toLocaleDateString('en',{month:'short',day:'numeric'}));
    }
  }

  const isUp = prices[prices.length-1] >= prices[0];
  const color = isUp ? '#00d4a1' : '#f03e3e';

  creditChart = new Chart(ctx, {
    type: 'line',
    data: {
      labels,
      datasets: [{
        data: prices,
        borderColor: color,
        backgroundColor: `${color}15`,
        fill: true,
        tension: 0.4,
        pointRadius: 0,
        borderWidth: 2,
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      interaction: { mode: 'index', intersect: false },
      plugins: {
        legend: { display: false },
        tooltip: {
          backgroundColor: 'rgba(10,18,32,0.95)',
          borderColor: 'rgba(0,212,161,0.3)',
          borderWidth: 1,
          callbacks: { label: c => `$${c.raw.toFixed(2)}/tCO₂e` }
        }
      },
      scales: {
        x: { display: false },
        y: {
          display: true,
          grid: { color: 'rgba(255,255,255,0.04)' },
          ticks: { color: '#7a8fa6', font: { size: 11 }, callback: v => '$' + v }
        }
      }
    }
  });
}

// ── Order Book Simulation ──────────────────────────────────────────────────────
function buildOrderBook(midPrice) {
  const asksContainer = document.getElementById('orderBookAsks');
  const bidsContainer = document.getElementById('orderBookBids');
  const spreadEl = document.getElementById('spreadRow');
  if (!asksContainer || !bidsContainer) return;

  const asks = [];
  const bids = [];
  let askP = midPrice + 0.05;
  let bidP = midPrice - 0.07;

  for (let i = 0; i < 8; i++) {
    askP += Math.random() * 0.15;
    const askSize = parseFloat((Math.random() * 2000 + 100).toFixed(0));
    asks.push({ price: askP, size: askSize, total: askP * askSize });

    bidP -= Math.random() * 0.12;
    const bidSize = parseFloat((Math.random() * 2000 + 100).toFixed(0));
    bids.push({ price: bidP, size: bidSize, total: bidP * bidSize });
  }

  const maxAsk = Math.max(...asks.map(a => a.size));
  const maxBid = Math.max(...bids.map(b => b.size));

  asksContainer.innerHTML = [...asks].reverse().map(a => `
    <div class="order-row ask" style="--depth:${(a.size/maxAsk*100).toFixed(0)}%">
      <span class="order-price ask">$${a.price.toFixed(2)}</span>
      <span class="text-right" style="color:var(--text-secondary);font-size:11px;">${fmtNum(a.size)}</span>
      <span class="text-right" style="color:var(--text-muted);font-size:11px;">${fmtNum(a.total)}</span>
      <style>.order-row.ask::before{width:${(a.size/maxAsk*100).toFixed(0)}%;}</style>
    </div>`).join('');

  bidsContainer.innerHTML = bids.map(b => `
    <div class="order-row bid">
      <span class="order-price bid">$${b.price.toFixed(2)}</span>
      <span class="text-right" style="color:var(--text-secondary);font-size:11px;">${fmtNum(b.size)}</span>
      <span class="text-right" style="color:var(--text-muted);font-size:11px;">${fmtNum(b.total)}</span>
    </div>`).join('');

  const spread = (asks[0].price - bids[0].price).toFixed(2);
  const spreadPct = ((asks[0].price - bids[0].price) / bids[0].price * 100).toFixed(2);
  if (spreadEl) spreadEl.textContent = `Spread: $${spread} (${spreadPct}%)`;
}

function startOrderBookSimulation() {
  setInterval(() => {
    if (selectedCredit) buildOrderBook(selectedCredit.price_per_ton);
    else buildOrderBook(18.50);
  }, 2000);
}

// ── Trade History ──────────────────────────────────────────────────────────────
async function loadRecentTrades() {
  const container = document.getElementById('tradeHistory');
  if (!container) return;

  const res = await api('/market/trades');
  const trades = (res.success && res.data?.length) ? res.data.slice(0, 20) : generateFakeTrades();

  container.innerHTML = trades.map(t => {
    const isBuy = t.buyer_id !== 's1';
    return `<div class="feed-item ${isBuy?'feed-buy':'feed-sell'}">
      <span class="${isBuy?'t-green':'t-red'}">${isBuy?'BUY':'SELL'}</span>
      <span style="color:var(--text-primary)">$${fmt(t.price_per_ton||t.price)}</span>
      <span style="color:var(--text-muted)">${fmtNum(t.quantity_tons||t.qty)}</span>
    </div>`;
  }).join('');

  // Simulate live trades
  setInterval(() => {
    const prices = allCredits.map(c => c.price_per_ton);
    const p = prices[Math.floor(Math.random()*prices.length)] || 18.5;
    const size = Math.floor(Math.random()*500+50);
    const isBuy = Math.random() > 0.45;
    const row = document.createElement('div');
    row.className = `feed-item new ${isBuy?'feed-buy':'feed-sell'}`;
    row.innerHTML = `
      <span class="${isBuy?'t-green':'t-red'}">${isBuy?'BUY':'SELL'}</span>
      <span style="color:var(--text-primary)">$${fmt(p + (Math.random()-0.5)*0.5)}</span>
      <span style="color:var(--text-muted)">${size}</span>`;
    if (container.firstChild) container.insertBefore(row, container.firstChild);
    if (container.children.length > 25) container.lastChild.remove();
  }, 1500);
}

function generateFakeTrades() {
  return Array.from({length:15}, (_, i) => ({
    buyer_id: i%3===0?'s1':'buyer',
    price_per_ton: 15 + Math.random()*20,
    quantity_tons: Math.floor(Math.random()*800+50),
    price: 15 + Math.random()*20,
    qty: Math.floor(Math.random()*800+50),
  }));
}

// ── Buy Widget ─────────────────────────────────────────────────────────────────
function calcTotal() {
  if (!selectedCredit) return;
  const qty = parseFloat(document.getElementById('buyQty')?.value || 0);
  const price = selectedCredit.price_per_ton;
  const total = qty * price;
  const fee = total * 0.025;
  document.getElementById('calc-price').textContent = fmtCurrency(price);
  document.getElementById('calc-fee').textContent = fmtCurrency(fee);
  document.getElementById('calc-total').textContent = fmtCurrency(total + fee);

  const user = getUser();
  const warnEl = document.getElementById('buyBalWarning');
  if (user && warnEl) {
    warnEl.style.display = (user.balance < total + fee) ? 'block' : 'none';
  }
}

function switchTrade(mode) {
  document.getElementById('buyTab').classList.toggle('active', mode === 'buy');
  document.getElementById('sellTab').classList.toggle('active', mode !== 'buy');
  document.getElementById('buyBtn').textContent = mode === 'buy' ? '🛒 Buy Carbon Credits' : '🏅 Retire Credits';
}

async function executeBuy() {
  if (!selectedCredit) return showToast('error', 'Please select a credit to buy');
  if (!isLoggedIn()) {
    showToast('info', 'Please login to buy', 'Redirecting to login...');
    setTimeout(() => location.href = 'auth.html', 1500);
    return;
  }

  const qty = parseFloat(document.getElementById('buyQty')?.value || 0);
  if (!qty || qty <= 0) return showToast('error', 'Enter a valid quantity');

  const btn = document.getElementById('buyBtn');
  btn.disabled = true;
  btn.textContent = 'Processing...';

  try {
    const res = await api('/credits/buy', {
      method: 'POST',
      body: JSON.stringify({ credit_id: selectedCredit.id, quantity_tons: qty })
    });

    if (res.success) {
      showToast('success', `✅ Purchased ${qty} tons!`, `TX: ${res.data?.tx_hash?.slice(0,16)}...`);
      // Update stored user balance
      const user = getUser();
      if (user) {
        user.balance -= qty * selectedCredit.price_per_ton * 1.025;
        user.total_credits_owned += qty;
        localStorage.setItem('sparc_user', JSON.stringify(user));
        updateNavAuth();
        document.getElementById('bp-balance').textContent = fmtCurrency(user.balance);
      }
      document.getElementById('buyQty').value = '';
      calcTotal();
      await loadCredits();
    } else {
      showToast('error', res.error || 'Purchase failed');
    }
  } catch (e) {
    showToast('error', 'Failed to connect to server');
  }

  btn.disabled = false;
  btn.textContent = '🛒 Buy Carbon Credits';
}

// ── View Toggle ────────────────────────────────────────────────────────────────
function setView(btn, view) {
  currentView = view;
  document.querySelectorAll('.tabs .tab-btn').forEach(b => b.classList.remove('active'));
  btn.classList.add('active');
  document.getElementById('creditsList').style.display = view === 'list' ? 'block' : 'none';
  document.getElementById('creditListHeader').style.display = view === 'list' ? 'grid' : 'none';
  document.getElementById('creditsGrid').style.display = view === 'grid' ? 'grid' : 'none';
  if (view === 'grid') renderCreditGrid(allCredits);
}

function switchPeriod(btn) {
  document.querySelectorAll('.chart-period').forEach(b => b.classList.remove('active'));
  btn.classList.add('active');
  if (selectedCredit) loadCreditChart(selectedCredit.id, selectedCredit.price_per_ton);
}

function toggleSDG(el) { el.classList.toggle('active'); }

// Start the app
init();
