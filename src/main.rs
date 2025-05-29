// src/main.js
import { invoke } from '@tauri-apps/api/tauri';

class DiscordBotController {
constructor() {
this.init();
}

async init() {
await this.updateStatus();
setInterval(() => this.updateStatus(), 5000); // Update alle 5 Sekunden
}

async updateStatus() {
try {
const status = await invoke('get_bot_status');
this.displayStatus(status);
} catch (error) {
console.error('Failed to get bot status:', error);
this.displayError('Bot nicht erreichbar');
}
}

async sendMessage(channelId, message) {
try {
const result = await invoke('send_message', {
channelId: channelId,
message: message
});

if (result.success) {
this.showSuccess(`Nachricht gesendet! ID: ${result.message_id}`);
} else {
this.showError(`Fehler: ${result.error}`);
}
} catch (error) {
console.error('Failed to send message:', error);
this.showError('Nachricht konnte nicht gesendet werden');
}
}

async loadGuilds() {
try {
const guilds = await invoke('get_guilds');
this.displayGuilds(guilds);
return guilds;
} catch (error) {
console.error('Failed to load guilds:', error);
this.showError('Server konnten nicht geladen werden');
return [];
}
}

async setBotPresence(status = 'online', activityName = null) {
try {
const result = await invoke('set_bot_presence', {
status: status,
activityName: activityName
});
this.showSuccess('Bot Status aktualisiert');
} catch (error) {
console.error('Failed to set presence:', error);
this.showError('Status konnte nicht aktualisiert werden');
}
}

displayStatus(status) {
const statusElement = document.getElementById('bot-status');
if (statusElement) {
statusElement.innerHTML = `
<div class="status-card ${status.bot_ready ? 'online' : 'offline'}">
<h3>Bot Status</h3>
<p>Status: ${status.status}</p>
<p>Bereit: ${status.bot_ready ? 'Ja' : 'Nein'}</p>
<p>Server: ${status.guilds}</p>
<p>Benutzer: ${status.users}</p>
</div>
`;
}
}

displayGuilds(guilds) {
const guildsElement = document.getElementById('guilds-list');
if (guildsElement) {
guildsElement.innerHTML = guilds.map(guild => `
<div class="guild-card">
<h4>${guild.name}</h4>
<p>Mitglieder: ${guild.member_count}</p>
<div class="channels">
<h5>Kanäle:</h5>
${guild.channels.map(channel => `
<button class="channel-btn" onclick="selectChannel('${channel.id}', '${channel.name}')">
#${channel.name}
</button>
`).join('')}
</div>
</div>
`).join('');
}
}

showSuccess(message) {
this.showNotification(message, 'success');
}

showError(message) {
this.showNotification(message, 'error');
}

showNotification(message, type) {
const notification = document.createElement('div');
notification.className = `notification ${type}`;
notification.textContent = message;
document.body.appendChild(notification);

setTimeout(() => {
notification.remove();
}, 3000);
}

displayError(message) {
const statusElement = document.getElementById('bot-status');
if (statusElement) {
statusElement.innerHTML = `
<div class="status-card offline">
<h3>Bot Status</h3>
<p class="error">${message}</p>
</div>
`;
}
}
}

// Globale Funktionen für die UI
let selectedChannelId = null;
let botController = null;

function selectChannel(channelId, channelName) {
selectedChannelId = channelId;
document.getElementById('selected-channel').textContent = `#${channelName}`;
document.getElementById('send-section').style.display = 'block';
}

async function sendMessageToChannel() {
if (!selectedChannelId) {
alert('Bitte wähle zuerst einen Kanal aus');
return;
}

const messageInput = document.getElementById('message-input');
const message = messageInput.value.trim();

if (!message) {
alert('Bitte gib eine Nachricht ein');
return;
}

await botController.sendMessage(selectedChannelId, message);
messageInput.value = '';
}

async function updateBotStatus() {
const status = document.getElementById('status-select').value;
const activity = document.getElementById('activity-input').value.trim();

await botController.setBotPresence(status, activity || null);
}

// App initialisieren
document.addEventListener('DOMContentLoaded', () => {
botController = new DiscordBotController();

// Event Listeners
document.getElementById('refresh-guilds').addEventListener('click', () => {
botController.loadGuilds();
});

document.getElementById('send-message-btn').addEventListener('click', sendMessageToChannel);
document.getElementById('update-status-btn').addEventListener('click', updateBotStatus);

// Initiale Daten laden
botController.loadGuilds();
});