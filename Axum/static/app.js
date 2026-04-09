const BASE = '';
// This must match the AUTH_TOKEN in your main.rs exactly
const AUTH_TOKEN = 'Bearer secret-poc-token'; 

let raceInterval = null;
let raceStartLocalMs = null; 
let raceRunning = false;

// ── Formatting helpers ──

function fmtMs(ms) {
    if (ms == null || ms === 0) return '—';
    if (ms < 1000) return ms + ' ms';
    return (ms / 1000).toFixed(2) + ' s';
}

function fmtBig(n) {
    if (n == null || n === 0) return '—';
    if (n >= 1_000_000) return (n / 1_000_000).toFixed(2) + 'M';
    if (n >= 1_000)     return (n / 1_000).toFixed(1) + 'k';
    return String(n);
}

function fmtThroughput(swaps, elapsed_ms) {
    if (!swaps || !elapsed_ms || elapsed_ms === 0) return '—';
    const perSec = (swaps / elapsed_ms) * 1000;
    return fmtBig(Math.round(perSec)) + '/s';
}

// ── UI update ──

function setLog(msg, active = false) {
    document.getElementById('race-log-text').textContent = msg;
    document.getElementById('status-dot').className = 'dot' + (active ? ' active' : '');
}

function resetUI() {
    ['rust', 'django'].forEach(e => {
        document.getElementById(`${e}-fill`).style.width = '0%';
        document.getElementById(`${e}-fill`).classList.remove('active');
        document.getElementById(`${e}-pct`).textContent = '0%';
        document.getElementById(`${e}-elapsed`).textContent = '—';
        document.getElementById(`${e}-throughput`).textContent = '—';
        document.getElementById(`${e}-swaps`).textContent = '—';
        document.getElementById(`${e}-comparisons`).textContent = '—';
        ['elapsed','throughput','swaps','comparisons'].forEach(s => {
            document.getElementById(`${e}-${s}`).className = 'stat-value';
        });
    });

    const banner = document.getElementById('winner-banner');
    banner.className = '';
    banner.style.display = 'none';
    document.getElementById('winner-text').textContent = '—';
    // If you kept the winner-detail span in your HTML:
    const detailEl = document.getElementById('winner-detail');
    if (detailEl) detailEl.textContent = '';
}

function updateEngine(engine, data) {
    const pct = data.progress || 0;
    const fill = document.getElementById(`${engine}-fill`);
    fill.style.width = pct + '%';
    document.getElementById(`${engine}-pct`).textContent = pct + '%';

    if (pct > 0 && pct < 100) {
        fill.classList.add('active');
    } else {
        fill.classList.remove('active');
    }

    const elapsed = data.elapsed_ms || 0;
    const swaps = data.swaps || 0;
    const comparisons = data.comparisons || 0;

    document.getElementById(`${engine}-elapsed`).textContent = fmtMs(elapsed);
    document.getElementById(`${engine}-throughput`).textContent = fmtThroughput(swaps, elapsed);
    document.getElementById(`${engine}-swaps`).textContent = fmtBig(swaps);
    document.getElementById(`${engine}-comparisons`).textContent = fmtBig(comparisons);
}

function declareWinner(winner, rustData, djangoData) {
    clearInterval(raceInterval);
    raceInterval = null;
    raceRunning = false;

    document.getElementById('race-btn').disabled = false;
    document.getElementById('rust-fill').classList.remove('active');
    document.getElementById('django-fill').classList.remove('active');

    const winEl = winner === 'rust' ? 'rust' : 'django';
    ['elapsed','throughput','swaps','comparisons'].forEach(s => {
        const el = document.getElementById(`${winEl}-${s}`);
        el.classList.add(`highlight-${winEl}`);
    });

    const rustMs = rustData.elapsed_ms || 0;
    const djangoMs = djangoData.elapsed_ms || 0;
    let detail = '';
    if (rustMs && djangoMs) {
        const faster = Math.max(rustMs, djangoMs);
        const slower = Math.min(rustMs, djangoMs);
        const times = (faster / slower).toFixed(1);
        detail = `${times}× faster · rust ${fmtMs(rustMs)} · django ${fmtMs(djangoMs)}`;
    }

    const banner = document.getElementById('winner-banner');
    const cls = winner === 'rust' ? 'rust-wins' : 'django-wins';
    banner.className = `show ${cls}`;
    document.getElementById('winner-text').textContent =
        winner === 'rust' ? 'RUST WINS' : 'DJANGO WINS';
    
    const detailEl = document.getElementById('winner-detail');
    if (detailEl) detailEl.textContent = detail;

    setLog(`Race complete. Winner: ${winner.toUpperCase()}`, false);
}

// ── Race logic ──

async function startRace() {
    if (raceRunning) return;
    raceRunning = true;

    document.getElementById('race-btn').disabled = true;
    resetUI();
    setLog('Starting race…', true);
    raceStartLocalMs = Date.now();

    const fetchOptions = { headers: { 'Authorization': AUTH_TOKEN } };
    fetch(`${BASE}/rust/sort`, fetchOptions).catch(() => {});
    fetch(`${BASE}/django/sort`, fetchOptions).catch(() => {});

    await new Promise(r => setTimeout(r, 200));
    setLog('Race running — polling telemetry…', true);

    raceInterval = setInterval(async () => {
        try {
            const res = await fetch(`${BASE}/race-status`, {
                headers: { 'Authorization': AUTH_TOKEN }
            });
            const d = await res.json();

            updateEngine('rust', d.rust);
            updateEngine('django', d.django);

            if (d.winner !== 'none') {
                updateEngine('rust', d.rust);
                updateEngine('django', d.django);
                declareWinner(d.winner, d.rust, d.django);
            }
        } catch (e) {
            setLog('Polling… (Redis ping)', true);
        }
    }, 400);
}

// ── User CRUD ──

function setRes(id, text, ok) {
    const el = document.getElementById(id);
    el.textContent = text;
    el.className = 'response ' + (ok ? 'ok' : 'err');
}

async function createUser() {
    const name  = document.getElementById('create-name').value.trim();
    const email = document.getElementById('create-email').value.trim();
    if (!name || !email) return setRes('create-res', 'name and email required', false);
    try {
        const r = await fetch(`${BASE}/users`, {
            method: 'POST',
            headers: { 
                'Content-Type': 'application/json',
                'Authorization': AUTH_TOKEN 
            },
            body: JSON.stringify({ name, email })
        });
        setRes('create-res', `${r.status} ${r.statusText}`, r.ok);
    } catch (e) { setRes('create-res', e.message, false); }
}

async function getUser() {
    const id = document.getElementById('get-id').value.trim();
    if (!id) return setRes('get-res', 'id required', false);
    try {
        const r = await fetch(`${BASE}/users/${id}`, {
            headers: { 'Authorization': AUTH_TOKEN }
        });
        const text = await r.text();
        let display = text;
        try { display = JSON.stringify(JSON.parse(text), null, 2); } catch {}
        setRes('get-res', `${r.status} ${r.statusText}\n${display}`, r.ok);
    } catch (e) { setRes('get-res', e.message, false); }
}

async function updateEmail() {
    const id    = document.getElementById('update-id').value.trim();
    const email = document.getElementById('update-email').value.trim();
    if (!id || !email) return setRes('update-res', 'id and email required', false);
    try {
        const r = await fetch(`${BASE}/users/${id}`, {
            method: 'PUT',
            headers: { 
                'Content-Type': 'application/json',
                'Authorization': AUTH_TOKEN 
            },
            body: JSON.stringify({ email })
        });
        setRes('update-res', `${r.status} ${r.statusText}`, r.ok);
    } catch (e) { setRes('update-res', e.message, false); }
}

async function deleteUser() {
    const id = document.getElementById('delete-id').value.trim();
    if (!id) return setRes('delete-res', 'id required', false);
    try {
        const r = await fetch(`${BASE}/users/${id}`, { 
            method: 'DELETE',
            headers: { 'Authorization': AUTH_TOKEN }
        });
        setRes('delete-res', `${r.status} ${r.statusText}`, r.ok);
    } catch (e) { setRes('delete-res', e.message, false); }
}

// ── CSP-COMPLIANT EVENT LISTENERS ──
document.addEventListener('DOMContentLoaded', () => {
    // Race button
    document.getElementById('race-btn')?.addEventListener('click', startRace);

    // CRUD buttons
    document.getElementById('btn-create')?.addEventListener('click', createUser);
    document.getElementById('btn-get')?.addEventListener('click', getUser);
    document.getElementById('btn-update')?.addEventListener('click', updateEmail);
    document.getElementById('btn-delete')?.addEventListener('click', deleteUser);

    console.log("System: Event listeners attached. CSP compliant.");
});