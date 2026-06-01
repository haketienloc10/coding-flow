import './style.css';

// --- State ---
const STORAGE_KEY = 'focus_garden_sessions';
let garden = JSON.parse(localStorage.getItem(STORAGE_KEY)) || [];
let currentSession = null;
let timerInterval = null;
let timeRemaining = 0; // in seconds

let selectedDuration = 25; // default 25 minutes

// --- DOM Elements ---
const idleState = document.getElementById('idle-state');
const runningState = document.getElementById('running-state');
const goalInput = document.getElementById('goal-input');
const durationBtns = document.querySelectorAll('.duration-btn');
const startBtn = document.getElementById('start-btn');

const currentGoalEl = document.getElementById('current-goal');
const timerDisplay = document.getElementById('timer-display');
const cancelBtn = document.getElementById('cancel-btn');

const gardenGrid = document.getElementById('garden-grid');
const emptyState = document.getElementById('empty-state');
const clearBtn = document.getElementById('clear-btn');

// --- Initialization ---
function init() {
  renderGarden();
  setupEventListeners();
}

function setupEventListeners() {
  // Goal Input Validation
  goalInput.addEventListener('input', (e) => {
    startBtn.disabled = e.target.value.trim() === '';
  });

  // Duration Selection
  durationBtns.forEach(btn => {
    btn.addEventListener('click', (e) => {
      durationBtns.forEach(b => b.classList.remove('active'));
      e.target.classList.add('active');
      selectedDuration = parseInt(e.target.dataset.duration, 10);
    });
  });

  // Start Session
  startBtn.addEventListener('click', startSession);

  // Cancel Session
  cancelBtn.addEventListener('click', cancelSession);

  // Clear Garden
  clearBtn.addEventListener('click', clearGarden);
}

// --- Logic ---
function startSession() {
  const goal = goalInput.value.trim();
  if (!goal) return;

  currentSession = {
    id: Date.now().toString(),
    goal,
    durationMinutes: selectedDuration,
    status: 'running',
    startedAt: new Date().toISOString(),
  };

  timeRemaining = selectedDuration * 60;
  
  // UI Update
  currentGoalEl.textContent = goal;
  updateTimerDisplay();
  
  idleState.classList.remove('active');
  runningState.classList.add('active');

  // Start Timer
  timerInterval = setInterval(() => {
    timeRemaining--;
    updateTimerDisplay();

    if (timeRemaining <= 0) {
      completeSession();
    }
  }, 1000);
}

function updateTimerDisplay() {
  const m = Math.floor(timeRemaining / 60).toString().padStart(2, '0');
  const s = (timeRemaining % 60).toString().padStart(2, '0');
  timerDisplay.textContent = `${m}:${s}`;
  document.title = `${m}:${s} - Focus Garden`;
}

function completeSession() {
  clearInterval(timerInterval);
  document.title = 'Focus Garden';
  
  if (currentSession) {
    currentSession.status = 'grown';
    currentSession.endedAt = new Date().toISOString();
    garden.unshift(currentSession); // add to top
    saveGarden();
  }

  resetToIdle();
  renderGarden();
}

function cancelSession() {
  clearInterval(timerInterval);
  document.title = 'Focus Garden';

  if (currentSession) {
    currentSession.status = 'withered';
    currentSession.endedAt = new Date().toISOString();
    garden.unshift(currentSession);
    saveGarden();
  }

  resetToIdle();
  renderGarden();
}

function resetToIdle() {
  currentSession = null;
  timeRemaining = 0;
  goalInput.value = '';
  startBtn.disabled = true;
  
  runningState.classList.remove('active');
  idleState.classList.add('active');
}

// --- Garden Data & UI ---
function saveGarden() {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(garden));
}

function clearGarden() {
  // We should only clear today's garden to be precise, or all.
  // The requirement says "Clear today’s garden", so let's just clear today's items.
  const today = new Date().toDateString();
  garden = garden.filter(item => new Date(item.startedAt).toDateString() !== today);
  saveGarden();
  renderGarden();
}

function renderGarden() {
  gardenGrid.innerHTML = '';
  
  if (garden.length === 0) {
    emptyState.style.display = 'block';
    gardenGrid.style.display = 'none';
    return;
  }

  const today = new Date().toDateString();
  const todaysGarden = garden.filter(item => new Date(item.startedAt).toDateString() === today);

  if (todaysGarden.length === 0) {
    emptyState.style.display = 'block';
    gardenGrid.style.display = 'none';
    return;
  }

  emptyState.style.display = 'none';
  gardenGrid.style.display = 'grid';

  todaysGarden.forEach(item => {
    const el = document.createElement('div');
    el.className = `garden-item ${item.status}`;
    
    const emoji = item.status === 'grown' ? '🌳' : '🥀';
    const time = new Date(item.endedAt).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

    el.innerHTML = `
      <div class="item-emoji">${emoji}</div>
      <div class="item-goal" title="${item.goal}">${item.goal}</div>
      <div class="item-meta">${item.durationMinutes}m • ${time}</div>
    `;
    gardenGrid.appendChild(el);
  });
}

// Boot
init();
