// ═══════════════════════════════════════════════════════════════
// Sparc Energy — Projects Page JavaScript
// ═══════════════════════════════════════════════════════════════

let allProjects = [];
let currentTypeFilter = '';

const TYPE_ICONS = { reforestation:'🌳', solar:'☀️', wind:'💨', blue_carbon:'🌊', methane:'🔥' };
const TYPE_BG = {
  reforestation:'rgba(34,197,94,0.08)',
  solar:'rgba(245,158,11,0.08)',
  wind:'rgba(96,165,250,0.08)',
  blue_carbon:'rgba(6,182,212,0.08)',
  methane:'rgba(167,139,250,0.08)'
};

async function init() {
  const res = await api('/projects');
  if (res.success && res.data) {
    allProjects = res.data;
    updateProjectStats(allProjects);
    renderProjects(allProjects);
  }
}

function updateProjectStats(projects) {
  const totalEl = document.getElementById('ps-total');
  const verEl = document.getElementById('ps-verified');
  const issuedEl = document.getElementById('ps-issued');
  if (totalEl) totalEl.textContent = projects.length;
  if (verEl) verEl.textContent = projects.filter(p => p.verified).length + ' Verified';
  if (issuedEl) issuedEl.textContent = fmtNum(projects.reduce((s,p) => s + (p.credits_issued||0), 0));
}

function renderProjects(projects) {
  const grid = document.getElementById('projectsGrid');
  if (!grid) return;
  if (!projects.length) {
    grid.innerHTML = '<div style="grid-column:1/-1;text-align:center;color:var(--text-muted);padding:40px;">No projects found.</div>';
    return;
  }
  grid.innerHTML = projects.map(p => buildProjectCard(p)).join('');
}

function buildProjectCard(p) {
  const sdgs = (p.sdg_goals||'').split(',').filter(Boolean);
  return `
  <div class="project-card card-hover" onclick="showProjectDetail('${p.id}')">
    <div class="project-image" style="background:${TYPE_BG[p.project_type]||'var(--bg-secondary)'};font-size:64px;">
      ${TYPE_ICONS[p.project_type]||'🌿'}
      ${p.verified ? '<span class="verified-stamp badge badge-verra" style="position:absolute;top:12px;right:12px;">✅ Verified</span>' : '<span style="position:absolute;top:12px;right:12px;" class="badge badge-pending">⏳ Pending</span>'}
    </div>
    <div class="project-body">
      <div style="display:flex;justify-content:space-between;align-items:flex-start;margin-bottom:8px;">
        <div class="project-name">${p.name}</div>
        <span class="badge ${p.certification==='Gold Standard'?'badge-gold':'badge-verra'}">${p.certification||'—'}</span>
      </div>
      <div class="project-location">📍 ${p.location}, ${p.country}</div>
      <p style="font-size:12px;color:var(--text-secondary);line-height:1.5;margin-bottom:12px;display:-webkit-box;-webkit-line-clamp:2;-webkit-box-orient:vertical;overflow:hidden;">${p.description||''}</p>
      <div class="project-stats">
        <div class="project-stat">
          <div class="project-stat-label">CO₂ / Year</div>
          <div class="project-stat-value text-green">${Number(p.co2_reduction_per_year||0).toLocaleString()} t</div>
        </div>
        <div class="project-stat">
          <div class="project-stat-label">Credits Issued</div>
          <div class="project-stat-value">${fmtNum(p.credits_issued||0)}</div>
        </div>
      </div>
      <div style="display:flex;justify-content:space-between;align-items:center;padding-top:12px;border-top:1px solid var(--border);">
        <div class="sdg-chips">${sdgs.slice(0,4).map(g=>`<div class="sdg-chip" title="SDG ${g.trim()}">SDG${g.trim()}</div>`).join('')}</div>
        <button class="btn btn-primary btn-sm" onclick="event.stopPropagation();location.href='marketplace.html?project=${p.id}'">Buy Credits →</button>
      </div>
    </div>
  </div>`;
}

// ── Project Detail Modal ───────────────────────────────────────────────────────
async function showProjectDetail(id) {
  openModal('projectModal');
  document.getElementById('projectModalContent').innerHTML = '<div style="text-align:center;padding:40px;"><div class="loading-spinner" style="margin:0 auto;width:32px;height:32px;"></div></div>';

  const res = await api(`/projects/${id}`);
  let p, credits;
  if (res.success && res.data) {
    p = res.data.project;
    credits = res.data.credits || [];
  } else {
    p = allProjects.find(proj => proj.id === id);
    credits = [];
  }

  if (!p) {
    document.getElementById('projectModalContent').innerHTML = '<div style="color:var(--sparc-red);">Project not found</div>';
    return;
  }

  const sdgs = (p.sdg_goals||'').split(',').filter(Boolean);
  document.getElementById('projectModalContent').innerHTML = `
    <div style="font-size:56px;text-align:center;margin-bottom:16px;">${TYPE_ICONS[p.project_type]||'🌿'}</div>
    <div style="display:flex;justify-content:space-between;align-items:flex-start;margin-bottom:8px;">
      <h2 style="font-size:20px;font-weight:700;">${p.name}</h2>
      ${p.verified ? '<span class="badge badge-verra">✅ Verified</span>' : '<span class="badge badge-pending">Pending</span>'}
    </div>
    <div style="display:flex;gap:8px;margin-bottom:16px;">
      <span class="badge badge-type">📍 ${p.location}</span>
      <span class="badge badge-type">🌍 ${p.country}</span>
      <span class="badge ${p.certification==='Gold Standard'?'badge-gold':'badge-verra'}">${p.certification||'—'}</span>
    </div>
    <p style="font-size:13px;color:var(--text-secondary);line-height:1.7;margin-bottom:20px;">${p.description||'No description available.'}</p>
    <div class="grid-2" style="gap:12px;margin-bottom:20px;">
      <div class="stat-card">
        <div class="stat-label">CO₂ Reduction/yr</div>
        <div class="stat-value text-green">${Number(p.co2_reduction_per_year||0).toLocaleString()} t</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">Total Credits</div>
        <div class="stat-value">${fmtNum(p.total_credits||0)}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">Credits Issued</div>
        <div class="stat-value">${fmtNum(p.credits_issued||0)}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">Project Started</div>
        <div class="stat-value" style="font-size:16px;">${p.project_start_date||'—'}</div>
      </div>
    </div>
    ${sdgs.length ? `
    <div style="margin-bottom:16px;">
      <div style="font-size:12px;font-weight:600;text-transform:uppercase;color:var(--text-muted);margin-bottom:8px;">SDG Goals</div>
      <div class="sdg-list">${sdgs.map(g => `<div class="sdg-item">SDG<br>${g.trim()}</div>`).join('')}</div>
    </div>` : ''}
    ${credits.length ? `
    <div style="margin-bottom:16px;">
      <div style="font-size:12px;font-weight:600;text-transform:uppercase;color:var(--text-muted);margin-bottom:10px;">Available Credit Listings</div>
      ${credits.map(c => `
        <div style="display:flex;justify-content:space-between;align-items:center;padding:10px;background:var(--bg-card);border-radius:8px;margin-bottom:6px;font-size:12px;">
          <div>
            <div style="font-weight:600;">Vintage ${c.vintage_year} · ${c.serial_number}</div>
            <div style="color:var(--text-muted);">${fmtNum(c.quantity_available)} tons available</div>
          </div>
          <div style="text-align:right;">
            <div style="font-weight:700;color:var(--sparc-green);">$${fmt(c.price_per_ton)}/t</div>
            <button class="btn btn-primary btn-sm" style="margin-top:4px;" onclick="location.href='marketplace.html?id=${c.id}'">Buy</button>
          </div>
        </div>`).join('')}
    </div>` : ''}
    <a href="marketplace.html" class="btn btn-primary w-full">🛒 Browse & Buy These Credits</a>`;
}

// ── Filters ────────────────────────────────────────────────────────────────────
function filterByType(el, type) {
  currentTypeFilter = type;
  document.querySelectorAll('.type-filter-bar .filter-chip').forEach(c => c.classList.remove('active'));
  el.classList.add('active');
  applyProjectFilter();
}

function applyProjectFilter() {
  const search = document.getElementById('projSearch')?.value.toLowerCase() || '';
  const filtered = allProjects.filter(p => {
    if (currentTypeFilter && p.project_type !== currentTypeFilter) return false;
    if (search && !p.name.toLowerCase().includes(search) && !p.country.toLowerCase().includes(search)) return false;
    return true;
  });
  renderProjects(filtered);
}

// ── Submit Project ─────────────────────────────────────────────────────────────
function openSubmitModal() {
  if (!isLoggedIn()) {
    showToast('info', 'Please login to submit a project');
    setTimeout(() => location.href = 'auth.html', 1500);
    return;
  }
  openModal('submitModal');
}

async function submitProject(e) {
  e.preventDefault();
  const msgEl = document.getElementById('submitProjectMsg');
  msgEl.textContent = 'Submitting...';
  msgEl.style.color = 'var(--text-secondary)';

  const res = await api('/projects', {
    method: 'POST',
    body: JSON.stringify({
      name: document.getElementById('pName').value,
      project_type: document.getElementById('pType').value,
      certification: document.getElementById('pCert').value,
      location: document.getElementById('pLocation').value,
      country: document.getElementById('pCountry').value,
      total_credits: parseFloat(document.getElementById('pCredits').value) || 0,
      description: document.getElementById('pDesc')?.value,
      co2_reduction_per_year: parseFloat(document.getElementById('pCo2')?.value) || null,
    })
  });

  if (res.success) {
    msgEl.textContent = '✅ Project submitted for review!';
    msgEl.style.color = 'var(--sparc-green)';
    showToast('success', 'Project submitted!', 'Our team will verify within 5 business days');
    setTimeout(() => closeModal('submitModal'), 2000);
    const newRes = await api('/projects');
    if (newRes.success) { allProjects = newRes.data; renderProjects(allProjects); }
  } else {
    msgEl.textContent = res.error || 'Submission failed. Please login first.';
    msgEl.style.color = 'var(--sparc-red)';
  }
}

init();
