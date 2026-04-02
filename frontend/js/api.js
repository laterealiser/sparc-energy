// ═══════════════════════════════════════════════════════════════
// Sparc Energy — Professional Marketplace API & BaaS Helper
// ═══════════════════════════════════════════════════════════════

// 1. Initialize Supabase Client (BaaS)
const SUPABASE_URL = 'https://loldpnnmjqttgvsxcgnr.supabase.co'; 
const SUPABASE_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImxvbGRwbm5tanF0dGd2c3hjZ25yIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NzUxMzEzODYsImV4cCI6MjA5MDcwNzM4Nn0.Jy12HHwMsWgrFA-TKdJ8WcWOMZYB97G9-SSJGdvwT3w'; 
const supabase = supabase.createClient(SUPABASE_URL, SUPABASE_KEY);

// 2. Custom Rust API Base (Supabase-ready Matching Engine)
const API_BASE = 'https://sparc-energy.onrender.com/api'; // Render Deployment URL

/**
 * Main API fetch wrapper for Custom Rust endpoints
 */
async function api(endpoint, options = {}) {
  const { data: { session } } = await supabase.auth.getSession();
  const token = session?.access_token;
  
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
    return await response.json();
  } catch (error) {
    console.error(`API Error [${endpoint}]:`, error);
    throw error;
  }
}

// ── Auth Logic (via Supabase) ────────────────────────────────────────────────

async function login(email, password) {
  const { data, error } = await supabase.auth.signInWithPassword({ email, password });
  if (error) throw error;
  return data;
}

async function register(email, password, fullName) {
  const { data, error } = await supabase.auth.signUp({
    email,
    password,
    options: { data: { full_name: fullName } }
  });
  if (error) throw error;
  return data;
}

async function logout() {
  await supabase.auth.signOut();
  location.href = 'index.html';
}

// ── KYC & Documents (via Supabase Storage) ────────────────────────────────────

async function uploadKYC(file, userId) {
  const { data, error } = await supabase.storage
    .from('kyc-documents')
    .upload(`${userId}/id_proof_${Date.now()}`, file);
  
  if (error) throw error;
  return data.path; // Return the path to store in Oracle DB
}

// ── Real-time Updates (via Supabase Realtime) ───────────────────────────────

function subscribeToMarket() {
  supabase
    .channel('market-updates')
    .on('postgres_changes', { event: 'INSERT', schema: 'public', table: 'trades' }, payload => {
      console.log('💎 New Trade Matched!', payload);
      // Update UI in real-time
    })
    .subscribe();
}

// ── Payments Integration ─────────────────────────────────────────────────────

function initRazorpay(amount, orderId) {
  const options = {
    key: "rzp_test_...", // From Supabase config / .env
    amount: amount * 100, // In paise
    currency: "INR",
    name: "Sparc Energy",
    description: "Credit Purchase Settlement",
    order_id: orderId,
    handler: async function (res) {
      await api('/payments/razorpay', {
        method: 'POST',
        body: JSON.stringify({
          order_id: orderId,
          payment_id: res.razorpay_payment_id,
          signature: res.razorpay_signature
        })
      });
      showToast('success', '💰 Payment Successful!', 'Credits updating...');
    }
  };
  const rzp = new Razorpay(options);
  rzp.open();
}
