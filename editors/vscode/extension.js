// The X_eTaL debugger in VS Code: the adapter is `xetal dap`, the
// binary named by the setting xetal.path (docs/debugging.md).
const vscode = require("vscode");

function activate(context) {
  const factory = {
    createDebugAdapterDescriptor() {
      const xetal = vscode.workspace.getConfiguration("xetal").get("path") || "xetal";
      return new vscode.DebugAdapterExecutable(xetal, ["dap"]);
    },
  };
  context.subscriptions.push(
    vscode.debug.registerDebugAdapterDescriptorFactory("xetal", factory)
  );
}

function deactivate() {}

module.exports = { activate, deactivate };
