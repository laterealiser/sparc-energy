// ═══════════════════════════════════════════════════════════════
// Sparc Energy — Dashboard JavaScript  
// Groww / Angel One style portfolio tracker
// ═══════════════════════════════════════════════════════════════

let dashData = null;
let portfolioChart = null;
let allocationChart = null;

// ── Init ───────────────────────────────────────────────────────────────────────
async function init() {
  const user = getUser();

  // Show login wall if not logged in
  if (!user) {
    document.getElementById('dashMain').style.display = 'none';
    document.getElementById('loginWall').style.display = 'flex';
    return;
  }

  await loadDashboard();
}

async function loadDashboard() {
  const res = await api('/dashboard');
  if (!res.success) {
    showToast('error', 'Failed to load dashboard data');
    return;
  }

  dashData = res.data;
  renderUserInfo(dashData.user || getUser());
  renderSummary(dashData.summary);
  renderPortfolio(dashData.portfolio);
  renderTransactions(dashData.transactions);
  buildPortfolioChart(dashData.portfolio);
  buildAllocationChart(dashData.portfolio);
  buildRetireDropdown(dashData.portfolio);

  // Phase 4: Operational Hubs Init
  const user = getUser();
  if (user?.role === 'admin') {
    const adminNav = document.getElementById('nav-admin');
    if (adminNav) adminNav.style.display = 'flex';
    loadAdminHub();
  } else if (user?.role === 'pdd_writer' || user?.role === 'auditor') {
    const workNav = document.getElementById('nav-workroom');
    if (workNav) workNav.style.display = 'flex';
    loadWorkroom();
  }
}

// ── User Info ──────────────────────────────────────────────────────────────────
function renderUserInfo(user) {
  if (!user) return;
  const nameEl = document.getElementById('dash-user-name');
  const roleEl = document.getElementById('dash-user-role');
  const balEl = document.getElementById('dash-balance');
  if (nameEl) nameEl.textContent = user.name;
  if (roleEl) roleEl.textContent = user.role.toUpperCase();
  if (balEl) balEl.textContent = fmtCurrency(user.balance || 0);

  // Store refreshed user
  localStorage.setItem('sparc_user', JSON.stringify(user));
  updateNavAuth();
}

// ── Summary Cards ──────────────────────────────────────────────────────────────
function renderSummary(s) {
  if (!s) return;
  const set = (id, val) => { const el = document.getElementById(id); if (el) el.textContent = val; };
  set('d-invested', fmtCurrency(s.total_invested));
  set('d-value', fmtCurrency(s.total_current_value));
  set('d-credits', fmt(s.total_credits) + ' t');
  set('d-retired', fmt(s.total_retired || 0) + ' t');

  const pnlBadge = document.getElementById('d-pnl-badge');
  if (pnlBadge) {
    const isUp = s.total_pnl >= 0;
    pnlBadge.className = `stat-change ${isUp ? 'up' : 'down'}`;
    pnlBadge.textContent = `${isUp ? '+' : ''}${fmtCurrency(s.total_pnl)} (${fmt(s.total_pnl_pct)}%)`;
  }
}

// ── Portfolio Table ────────────────────────────────────────────────────────────
function renderPortfolio(portfolio) {
  const tbody = document.getElementById('portfolioTableBody');
  if (!tbody) return;

  if (!portfolio || !portfolio.length) {
    tbody.innerHTML = `<tr><td colspan="9" style="text-align:center;padding:40px;color:var(--text-muted);">
      <div style="font-size:32px;margin-bottom:10px;">🌿</div>
      No credits in portfolio yet.
      <br><a href="marketplace.html" style="color:var(--sparc-green);font-weight:600;">Buy your first carbon credits →</a>
    </td></tr>`;
    return;
  }

  tbody.innerHTML = portfolio.map(p => {
    const pnl = p.pnl || 0;
    const pnlPct = p.total_invested > 0 ? (pnl / p.total_invested * 100) : 0;
    const certClass = p.certification === 'Gold Standard' ? 'badge-gold' : 'badge-verra';
    return `<tr class="portfolio-row">
      <td>
        <div style="font-size:13px;font-weight:600;">${p.project_name}</div>
        <div style="font-size:11px;color:var(--text-muted);">${p.project_type} · ${p.country}</div>
      </td>
      <td><span class="badge ${certClass}">${p.certification}</span></td>
      <td style="font-weight:600;">${Number(p.quantity_tons).toFixed(2)}</td>
      <td class="t-muted">${fmtCurrency(p.average_buy_price)}</td>
      <td style="font-weight:600;">${fmtCurrency(p.current_price)}</td>
      <td class="${pnl >= 0 ? 'pnl-positive' : 'pnl-negative'}" style="font-weight:600;">
        ${pnl >= 0 ? '+' : ''}${fmtCurrency(pnl)}
      </td>
      <td class="${pnlPct >= 0 ? 'pnl-positive' : 'pnl-negative'}">
        ${pnlPct >= 0 ? '+' : ''}${fmt(pnlPct)}%
      </td>
      <td class="t-muted">${Number(p.retired_tons).toFixed(2)}</td>
      <td>
        <button class="btn btn-secondary btn-sm retire-btn" onclick="openRetireFromPortfolio('${p.credit_id}','${p.project_name}',${p.quantity_tons})">
          🏅 Retire
        </button>
      </td>
    </tr>`;
  }).join('');
}

// ── Transactions ───────────────────────────────────────────────────────────────
function renderTransactions(transactions) {
  const tbody = document.getElementById('txTableBody');
  if (!tbody) return;

  if (!transactions || !transactions.length) {
    tbody.innerHTML = `<tr><td colspan="8" style="text-align:center;padding:32px;color:var(--text-muted);">No transactions yet.</td></tr>`;
    return;
  }

  const user = getUser();
  tbody.innerHTML = transactions.map(t => {
    const isBuy = t.buyer_id === user?.id;
    const typeLabel = t.retired ? '🏅 Retired' : (isBuy ? '🛒 Buy' : '📦 Sale');
    const typeColor = t.retired ? 'var(--sparc-gold)' : (isBuy ? 'var(--sparc-green)' : 'var(--sparc-red)');
    return `<tr>
      <td class="t-muted">${new Date(t.created_at).toLocaleDateString('en-IN', {day:'numeric',month:'short',year:'numeric'})}</td>
      <td><span style="color:${typeColor};font-weight:600;">${typeLabel}</span></td>
      <td>
        <div style="font-size:13px;">${t.project_name}</div>
        <div style="font-size:11px;color:var(--text-muted);">${t.project_type}</div>
      </td>
      <td style="font-weight:600;">${Number(t.quantity_tons).toFixed(2)}</td>
      <td>${fmtCurrency(t.price_per_ton)}</td>
      <td style="font-weight:600;">${fmtCurrency(t.total_price)}</td>
      <td><span class="badge ${t.certification==='Gold Standard'?'badge-gold':'badge-verra'}">${t.certification}</span></td>
      <td><span class="badge badge-verified" style="font-size:10px;">${t.status}</span></td>
    </tr>`;
  }).join('');
}

// ── Portfolio Value Chart ──────────────────────────────────────────────────────
function buildPortfolioChart(portfolio) {
  const ctx = document.getElementById('portfolioChart');
  if (!ctx) return;
  if (portfolioChart) { portfolioChart.destroy(); portfolioChart = null; }

  const labels = Array.from({length:30}, (_,i) => {
    const d = new Date(); d.setDate(d.getDate()-29+i);
    return d.toLocaleDateString('en',{month:'short',day:'numeric'});
  });

  const total_invested = (dashData?.summary?.total_invested || 0);
  let v = total_invested * 0.85 || 1000;
  const data = labels.map(() => {
    v += (Math.random() - 0.38) * (v * 0.015);
    return parseFloat(v.toFixed(2));
  });

  const isUp = data[data.length-1] >= data[0];
  const color = isUp ? '#00d4a1' : '#f03e3e';

  portfolioChart = new Chart(ctx, {
    type: 'line',
    data: {
      labels,
      datasets: [{
        label: 'Portfolio Value',
        data,
        borderColor: color,
        backgroundColor: `${color}12`,
        fill: true,
        tension: 0.4,
        pointRadius: 0,
        borderWidth: 2.5,
      }]
    },
    options: {
      responsive: true, maintainAspectRatio: false,
      interaction: { mode: 'index', intersect: false },
      plugins: {
        legend: { display: false },
        tooltip: {
          backgroundColor: 'rgba(10,18,32,0.95)',
          borderColor: 'rgba(0,212,161,0.3)', borderWidth: 1,
          callbacks: { label: c => fmtCurrency(c.raw) }
        }
      },
      scales: {
        x: { display: false },
        y: {
          grid: { color: 'rgba(255,255,255,0.04)' },
          ticks: { color: '#7a8fa6', font: { size: 11 }, callback: v => '₹'+fmtNum(v) }
        }
      }
    }
  });
}

// ── Allocation Donut Chart ─────────────────────────────────────────────────────
function buildAllocationChart(portfolio) {
  const ctx = document.getElementById('allocationChart');
  const legendEl = document.getElementById('allocationLegend');
  if (!ctx) return;
  if (allocationChart) { allocationChart.destroy(); allocationChart = null; }

  const COLORS = ['#00d4a1','#f5c518','#4f8ef7','#f03e3e','#a78bfa'];
  let labels, data, colors;

  if (portfolio && portfolio.length) {
    labels = portfolio.map(p => p.project_name.split(' ').slice(0,2).join(' '));
    data = portfolio.map(p => p.current_value || p.total_invested);
    colors = portfolio.map((_, i) => COLORS[i % COLORS.length]);
  } else {
    labels = ['No Assets Owned'];
    data = [100];
    colors = ['rgba(255,255,255,0.05)'];
  }

  allocationChart = new Chart(ctx, {
    type: 'doughnut',
    data: { labels, datasets: [{ data, backgroundColor: colors, borderWidth: 2, borderColor: 'var(--bg-secondary)' }] },
    options: {
      responsive: true, maintainAspectRatio: true,
      cutout: '65%',
      plugins: {
        legend: { display: false },
        tooltip: { callbacks: { label: c => `${c.label}: ${fmtCurrency(c.raw)}` } }
      }
    }
  });

  if (legendEl) {
    legendEl.innerHTML = labels.map((l, i) => `
      <div style="display:flex;align-items:center;justify-content:space-between;">
        <div style="display:flex;align-items:center;gap:6px;">
          <div style="width:10px;height:10px;border-radius:2px;background:${colors[i]};flex-shrink:0;"></div>
          <span>${l}</span>
        </div>
        <span style="font-weight:600;">${fmtCurrency(data[i])}</span>
      </div>`).join('');
  }
}

// ── Section Navigation (Sidebar) ──────────────────────────────────────────────
function setDashSection(el, sectionId) {
  ['section-overview','section-portfolio','section-history', 'section-retire', 'section-admin', 'section-workroom'].forEach(id => {
    const s = document.getElementById(id);
    if (s) s.style.display = id === sectionId ? 'block' : 'none';
  });
  document.querySelectorAll('.dash-nav-link').forEach(l => l.classList.remove('active'));
  if (el) el.classList.add('active');
}

// ── Retire Credits ─────────────────────────────────────────────────────────────
function buildRetireDropdown(portfolio) {
  const select = document.getElementById('retireCredit');
  if (!select) return;
  if (!portfolio || !portfolio.length) {
    select.innerHTML = '<option value="">No credits in portfolio</option>';
    return;
  }
  select.innerHTML = '<option value="">Choose credit...</option>' + portfolio
    .filter(p => p.quantity_tons > 0)
    .map(p => `<option value="${p.credit_id}" data-qty="${p.quantity_tons}" data-name="${p.project_name}">
      ${p.project_name} (${Number(p.quantity_tons).toFixed(2)}t available)
    </option>`).join('');
  select.addEventListener('change', updateRetireInfo);
  document.getElementById('retireQty')?.addEventListener('input', updateRetireInfo);
}

function updateRetireInfo() {
  const select = document.getElementById('retireCredit');
  const qtyInput = document.getElementById('retireQty');
  const infoEl = document.getElementById('retireInfo');
  if (!select || !qtyInput || !infoEl) return;

  const qty = parseFloat(qtyInput.value) || 0;
  if (!select.value || !qty) { infoEl.style.display = 'none'; return; }

  infoEl.style.display = 'block';
  document.getElementById('ri-co2').textContent = qty.toFixed(2) + ' tons CO₂e';
  const treesEquiv = Math.round(qty * 45);
  const kmEquiv = Math.round(qty * 4000);
  document.getElementById('ri-equiv').textContent = `≈ ${treesEquiv.toLocaleString()} trees planted or ${kmEquiv.toLocaleString()} km not driven`;
}

function openRetireFromPortfolio(creditId, projectName, available) {
  setDashSection(null, 'section-retire');
  document.querySelectorAll('.dash-nav-link').forEach(l => l.classList.remove('active'));
  const retireLink = document.querySelector('.dash-nav-link:nth-child(4)');
  if (retireLink) retireLink.classList.add('active');

  const select = document.getElementById('retireCredit');
  if (select) select.value = creditId;
  const qtyInput = document.getElementById('retireQty');
  if (qtyInput) qtyInput.focus();
}

async function executeRetire() {
  const creditId = document.getElementById('retireCredit')?.value;
  const qty = parseFloat(document.getElementById('retireQty')?.value || 0);
  const reason = document.getElementById('retireReason')?.value;
  const msgEl = document.getElementById('retireMsg');

  if (!creditId) { msgEl.textContent = '⚠️ Please select a credit'; msgEl.style.color = 'var(--sparc-red)'; return; }
  if (!qty || qty <= 0) { msgEl.textContent = '⚠️ Enter a valid quantity'; msgEl.style.color = 'var(--sparc-red)'; return; }
  if (!isLoggedIn()) { location.href = 'auth.html'; return; }

  msgEl.textContent = 'Processing retirement...';
  msgEl.style.color = 'var(--text-secondary)';

  const res = await api('/dashboard/retire', {
    method: 'POST',
    body: JSON.stringify({ credit_id: creditId, quantity_tons: qty, retirement_reason: reason })
  });

  if (res.success) {
    const certId = res.data?.certificate || `SPARC-RET-${Math.random().toString(36).slice(2,10).toUpperCase()}`;
    document.getElementById('certId').textContent = certId;
    document.getElementById('certQty').textContent = `${qty} tons CO₂e permanently retired`;
    document.getElementById('retireModalDesc').textContent = `${qty} tons of CO₂e have been retired and permanently removed from market.`;
    openModal('retireModal');
    msgEl.textContent = '';
    showToast('success', '🏅 Retirement certificate issued!', `${qty} tons CO₂e offset`);
    await loadDashboard();
  } else {
    msgEl.textContent = res.error || 'Retirement failed. Backend may be offline.';
    msgEl.style.color = 'var(--sparc-red)';
  }
}

function updateNavAuth() {
  const user = getUser();
  const guestEl = document.getElementById('nav-auth-guest');
  const userEl = document.getElementById('nav-auth-user');
  const balEl = document.getElementById('nav-balance');
  if (!guestEl || !userEl) return;
  if (user) {
    guestEl.style.display = 'none';
    userEl.style.display = 'flex';
    if (balEl) balEl.textContent = fmtCurrency(user.balance || 0);
  }
}

// ── Admin Functions ────────────────────────────────────────────────────────────

async function loadAdminHub() {
  const [usersRes, statsRes] = await Promise.all([
    api('/admin/users'),
    api('/admin/stats')
  ]);

  if (usersRes.success) renderAdminKYCs(usersRes.data);
  // Projects list usually comes from project registry with ?verified=0
  const projectsRes = await api('/projects'); 
  if (projectsRes.success) renderAdminProjects(projectsRes.data.filter(p => !p.verified));
}

function renderAdminKYCs(users) {
  const listEl = document.getElementById('admin-kyc-list');
  if (!listEl) return;
  const pending = users.filter(u => u.kyc_status === 'pending');
  if (!pending.length) { listEl.innerHTML = '<div class="stat-label">No pending KYCs</div>'; return; }

  listEl.innerHTML = pending.map(u => `
    <div class="stat-item" style="display:flex;justify-content:space-between;align-items:center;padding:12px;background:var(--bg-secondary);border-radius:12px;">
      <div>
        <div style="font-weight:700;">${u.name}</div>
        <div style="font-size:11px;color:var(--text-muted);">${u.email}</div>
      </div>
      <button class="btn btn-primary btn-sm" onclick="verifyKYC('${u.id}')">Approve</button>
    </div>
  `).join('');
}

async function verifyKYC(id) {
  const res = await api(`/admin/kyc/${id}/verify`, { method: 'POST' });
  if (res.success) { showToast('success', 'User Verified'); loadAdminHub(); }
}

function renderAdminProjects(projects) {
  const listEl = document.getElementById('admin-project-list');
  if (!listEl) return;
  if (!projects.length) { listEl.innerHTML = '<div class="stat-label">No pending projects</div>'; return; }

  listEl.innerHTML = projects.map(p => `
    <div class="stat-item" style="display:flex;justify-content:space-between;align-items:center;padding:12px;background:var(--bg-secondary);border-radius:12px;">
      <div>
        <div style="font-weight:700;">${p.name}</div>
        <div style="font-size:11px;color:var(--text-muted);">${p.project_type} · ${p.country}</div>
      </div>
      <button class="btn btn-primary btn-sm" onclick="approveProject('${p.id}')">Approve</button>
    </div>
  `).join('');
}

async function approveProject(id) {
  const res = await api(`/admin/projects/${id}/approve`, { method: 'POST' });
  if (res.success) { showToast('success', 'Project Registered'); loadAdminHub(); }
}

// ── Professional Workroom ──────────────────────────────────────────────────────

async function loadWorkroom() {
  const res = await api('/services/contracts');
  if (!res.success) return;
  renderWorkroom(res.data);
  const badge = document.getElementById('workroom-badge');
  if (badge) badge.textContent = `⚒️ ${res.data.length} Active Contracts`;
}

function renderWorkroom(contracts) {
  const listEl = document.getElementById('workroom-list');
  if (!listEl) return;
  if (!contracts.length) return;

  listEl.innerHTML = contracts.map(c => `
    <div class="card card-glow" style="border-color:var(--sparc-blue-border); background:rgba(79, 142, 247, 0.02);">
      <div style="display:flex; justify-content:space-between; align-items:center;">
        <div>
          <div style="font-weight:800; font-size:16px;">Contract #${c.id.slice(0,8)}</div>
          <p style="font-size:12px; color:var(--text-muted);">Client ID: ${c.client_id}</p>
        </div>
        <div style="text-align:right;">
          <div class="stat-label">Escrowed</div>
          <div style="font-weight:800; color:var(--sparc-blue);">${fmtCurrency(c.total_amount)}</div>
        </div>
      </div>
      <div style="margin-top:16px; padding-top:16px; border-top:1px solid var(--glass-border); display:flex; gap:12px;">
        <button class="btn btn-secondary btn-sm" onclick="showToast('info','Proof Submission','Upload deliverable proof to release milestone.')">Submit Milestone</button>
        <button class="btn btn-primary btn-sm">Message Client</button>
      </div>
    </div>
  `).join('');
}

init();
