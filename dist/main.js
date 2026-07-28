const { invoke } = window.__TAURI__.core;

document.getElementById('greetBtn').addEventListener('click', async () => {
  const name = document.getElementById('nameInput').value.trim();
  if (!name) return;

  try {
    const greeting = await invoke('greet', { name });
    document.getElementById('greetingOutput').textContent = greeting;
  } catch (err) {
    console.error(err);
    document.getElementById('greetingOutput').textContent = 'Error occurred';
  }
});

document.getElementById('nameInput').addEventListener('keypress', (e) => {
  if (e.key === 'Enter') {
    document.getElementById('greetBtn').click();
  }
});
