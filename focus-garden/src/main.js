import './style.css';

const STORAGE_KEY = 'time_capsule_notes';
const REFRESH_INTERVAL_MS = 1000;

let capsules = loadCapsules();

const capsuleForm = document.getElementById('capsule-form');
const titleInput = document.getElementById('capsule-title');
const contentInput = document.getElementById('capsule-content');
const unlockInput = document.getElementById('capsule-unlock');
const errorEl = document.getElementById('capsule-error');
const sealButton = document.getElementById('seal-btn');
const capsuleList = document.getElementById('capsule-list');
const emptyState = document.getElementById('capsule-empty-state');

function init() {
  capsuleForm.addEventListener('submit', handleCreateCapsule);
  [titleInput, contentInput, unlockInput].forEach((input) => {
    input.addEventListener('input', validateForm);
    input.addEventListener('change', validateForm);
  });

  renderCapsules();
  validateForm();
  setInterval(() => {
    renderCapsules();
    validateForm();
  }, REFRESH_INTERVAL_MS);
}

function loadCapsules() {
  try {
    const parsed = JSON.parse(localStorage.getItem(STORAGE_KEY));

    if (!Array.isArray(parsed)) {
      return [];
    }

    return parsed
      .filter(isValidCapsule)
      .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
  } catch {
    return [];
  }
}

function isValidCapsule(capsule) {
  return Boolean(
    capsule &&
      typeof capsule.id === 'string' &&
      typeof capsule.title === 'string' &&
      typeof capsule.content === 'string' &&
      typeof capsule.unlockAt === 'string' &&
      typeof capsule.createdAt === 'string' &&
      !Number.isNaN(new Date(capsule.unlockAt).getTime()) &&
      !Number.isNaN(new Date(capsule.createdAt).getTime())
  );
}

function saveCapsules() {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(capsules));
}

function validateForm() {
  const validation = getValidation();
  sealButton.disabled = !validation.valid;
  errorEl.textContent = validation.reason === 'past-unlock'
    ? 'Unlock time must be in the future.'
    : '';

  return validation;
}

function getValidation() {
  const title = titleInput.value.trim();
  const content = contentInput.value.trim();
  const unlockAt = unlockInput.value;
  const unlockDate = new Date(unlockAt);

  if (!title || !content || !unlockAt || Number.isNaN(unlockDate.getTime())) {
    return { valid: false, reason: 'missing-fields' };
  }

  if (unlockDate.getTime() <= Date.now()) {
    return { valid: false, reason: 'past-unlock' };
  }

  return { valid: true, title, content, unlockDate };
}

function handleCreateCapsule(event) {
  event.preventDefault();

  const validation = getValidation();

  if (!validation.valid) {
    validateForm();
    return;
  }

  const now = new Date();
  const capsule = {
    id: createCapsuleId(),
    title: validation.title,
    content: validation.content,
    unlockAt: validation.unlockDate.toISOString(),
    createdAt: now.toISOString(),
  };

  capsules = [capsule, ...capsules];
  saveCapsules();
  capsuleForm.reset();
  errorEl.textContent = '';
  validateForm();
  renderCapsules();
}

function createCapsuleId() {
  if (globalThis.crypto?.randomUUID) {
    return globalThis.crypto.randomUUID();
  }

  return `${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

function deleteCapsule(id) {
  capsules = capsules.filter((capsule) => capsule.id !== id);
  saveCapsules();
  renderCapsules();
}

function isUnlocked(capsule) {
  return new Date(capsule.unlockAt).getTime() <= Date.now();
}

function renderCapsules() {
  capsuleList.textContent = '';

  if (capsules.length === 0) {
    emptyState.hidden = false;
    capsuleList.hidden = true;
    return;
  }

  emptyState.hidden = true;
  capsuleList.hidden = false;

  capsules.forEach((capsule) => {
    capsuleList.appendChild(createCapsuleElement(capsule));
  });
}

function createCapsuleElement(capsule) {
  const unlocked = isUnlocked(capsule);
  const unlockTime = formatUnlockTime(capsule.unlockAt);
  const item = document.createElement('article');
  item.className = `capsule-item ${unlocked ? 'is-unlocked' : 'is-locked'}`;

  const header = document.createElement('div');
  header.className = 'capsule-header';

  const title = document.createElement('h3');
  title.textContent = capsule.title;

  const badge = document.createElement('span');
  badge.className = `status-badge ${unlocked ? 'unlocked' : 'locked'}`;
  badge.textContent = unlocked ? 'Unlocked' : 'Locked';

  const meta = document.createElement('p');
  meta.className = 'capsule-meta';
  meta.textContent = `Unlock at ${unlockTime}`;

  const body = document.createElement('p');
  body.className = 'capsule-body';
  body.textContent = unlocked
    ? capsule.content
    : `This capsule is locked until ${unlockTime}.`;

  const deleteButton = document.createElement('button');
  deleteButton.type = 'button';
  deleteButton.className = 'delete-btn';
  deleteButton.textContent = 'Delete';
  deleteButton.addEventListener('click', () => deleteCapsule(capsule.id));

  header.append(title, badge);
  item.append(header, meta, body, deleteButton);

  return item;
}

function formatUnlockTime(value) {
  return new Date(value).toLocaleString([], {
    dateStyle: 'medium',
    timeStyle: 'short',
  });
}

init();
