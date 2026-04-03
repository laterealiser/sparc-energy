// ═══════════════════════════════════════════════════════════════
// Sparc Energy — Auth Utilities (shared across all pages)
// ═══════════════════════════════════════════════════════════════

function getToken() { return localStorage.getItem('sparc_token'); }
function getUser() {
  try { return JSON.parse(localStorage.getItem('sparc_user')); } catch { return null; }
}
function isLoggedIn() { return !!getToken(); }

function logout() {
  localStorage.removeItem('sparc_token');
  localStorage.removeItem('sparc_user');
  location.href = 'index.html';
}

// Update navbar auth state — called on every page
(function updateNavAuth() {
  const user = getUser();
  const guestEl = document.getElementById('nav-auth-guest');
  const userEl = document.getElementById('nav-auth-user');
  const balEl = document.getElementById('nav-balance');
  if (!guestEl || !userEl) return;
  if (user) {
    guestEl.classList.add('hidden');
    guestEl.style.display = 'none';
    userEl.classList.remove('hidden');
    userEl.style.display = 'flex';
    if (balEl) balEl.textContent = fmtCurrency(user.balance || 0);
  } else {
    guestEl.style.display = 'flex';
    guestEl.classList.remove('hidden');
    userEl.style.display = 'none';
    userEl.classList.add('hidden');
  }
})();
