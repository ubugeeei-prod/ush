const vscode = require('vscode');
const { LanguageClient } = require('vscode-languageclient/node');
let client;
async function activate(context) {
  const command = vscode.workspace.getConfiguration('ush').get('server.path', 'ush_lsp');
  client = new LanguageClient('ush', 'ush', { command, args: [] }, {
    documentSelector: [{ scheme: 'file', language: 'ush' }, { scheme: 'untitled', language: 'ush' }],
    synchronize: { configurationSection: 'ush' },
  });
  context.subscriptions.push(client);
  try { await client.start(); }
  catch (error) { vscode.window.showErrorMessage(`Could not start ush_lsp (${command}). Run ush editor install again or set ush.server.path. ${error.message}`); }
}
async function deactivate() { if (client) await client.stop(); }
module.exports = { activate, deactivate };
