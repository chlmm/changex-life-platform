<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface PluginMetadata {
  name: string;
  version: string;
  description: string;
  author: string;
  permissions: string[];
  entry_point: string;
}

interface PluginResult {
  success: boolean;
  output: string;
  error: string | null;
}

const plugins = ref<string[]>([]);
const selectedPlugin = ref<PluginMetadata | null>(null);
const executionResult = ref<PluginResult | null>(null);
const pluginsDir = ref("");
const isLoading = ref(false);

// Load plugins on mount
onMounted(async () => {
  await loadPlugins();
});

async function loadPlugins() {
  try {
    const dir = await invoke<string>("get_plugins_dir");
    pluginsDir.value = dir;
    plugins.value = await invoke<string[]>("list_plugins");
  } catch (error) {
    console.error("Failed to load plugins:", error);
  }
}

async function selectPlugin(name: string) {
  try {
    selectedPlugin.value = await invoke<PluginMetadata>("get_plugin_metadata", { pluginName: name });
    executionResult.value = null;
  } catch (error) {
    console.error("Failed to load plugin metadata:", error);
  }
}

async function executePlugin() {
  if (!selectedPlugin.value) return;

  isLoading.value = true;
  executionResult.value = null;

  try {
    executionResult.value = await invoke<PluginResult>("execute_plugin", {
      pluginName: selectedPlugin.value.name
    });
  } catch (error) {
    console.error("Failed to execute plugin:", error);
    executionResult.value = {
      success: false,
      output: "",
      error: String(error)
    };
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <main class="container">
    <h1>Life Platform - Plugin Manager</h1>

    <div class="content">
      <div class="sidebar">
        <h2>Installed Plugins</h2>
        <div v-if="plugins.length === 0" class="empty-state">
          <p>No plugins installed</p>
          <p class="hint">Place plugins in: {{ pluginsDir }}</p>
        </div>
        <div v-else class="plugin-list">
          <div
            v-for="plugin in plugins"
            :key="plugin"
            class="plugin-item"
            :class="{ active: selectedPlugin?.name === plugin }"
            @click="selectPlugin(plugin)"
          >
            {{ plugin }}
          </div>
        </div>
      </div>

      <div class="main-panel">
        <div v-if="selectedPlugin" class="plugin-details">
          <h2>{{ selectedPlugin.name }}</h2>
          <p class="version">Version: {{ selectedPlugin.version }}</p>
          <p class="author">Author: {{ selectedPlugin.author }}</p>
          <p class="description">{{ selectedPlugin.description }}</p>

          <div class="permissions">
            <h3>Permissions</h3>
            <div class="permission-tags">
              <span v-for="perm in selectedPlugin.permissions" :key="perm" class="tag">
                {{ perm }}
              </span>
            </div>
          </div>

          <button
            class="execute-btn"
            :disabled="isLoading"
            @click="executePlugin"
          >
            {{ isLoading ? 'Executing...' : 'Execute Plugin' }}
          </button>

          <div v-if="executionResult" class="result">
            <h3>Result</h3>
            <div :class="['result-content', executionResult.success ? 'success' : 'error']">
              <p v-if="executionResult.output"><strong>Output:</strong> {{ executionResult.output }}</p>
              <p v-if="executionResult.error"><strong>Error:</strong> {{ executionResult.error }}</p>
            </div>
          </div>
        </div>
        <div v-else class="empty-state">
          <p>Select a plugin to view details</p>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.container {
  min-height: 100vh;
  padding: 2rem;
  box-sizing: border-box;
}

h1 {
  text-align: center;
  color: #0f0f0f;
  margin-bottom: 1rem;
}

.content {
  display: flex;
  gap: 2rem;
  margin-top: 2rem;
}

.sidebar {
  width: 250px;
  background: #ffffff;
  border-radius: 12px;
  padding: 1.5rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.sidebar h2 {
  margin: 0 0 1rem 0;
  font-size: 1.25rem;
  color: #0f0f0f;
}

.plugin-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.plugin-item {
  padding: 0.75rem 1rem;
  background: #f6f6f6;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.plugin-item:hover {
  background: #e8e8e8;
}

.plugin-item.active {
  background: #396cd8;
  color: white;
}

.main-panel {
  flex: 1;
  background: #ffffff;
  border-radius: 12px;
  padding: 2rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.plugin-details h2 {
  margin: 0 0 0.5rem 0;
  font-size: 1.75rem;
  color: #0f0f0f;
}

.version, .author {
  margin: 0.5rem 0;
  color: #666;
}

.description {
  margin: 1rem 0;
  line-height: 1.6;
  color: #333;
}

.permissions {
  margin: 1.5rem 0;
}

.permissions h3 {
  margin: 0 0 0.75rem 0;
  font-size: 1.1rem;
  color: #0f0f0f;
}

.permission-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.tag {
  padding: 0.4rem 0.8rem;
  background: #f0f0f0;
  border-radius: 4px;
  font-size: 0.875rem;
  color: #555;
}

.execute-btn {
  margin-top: 1rem;
  padding: 0.75rem 2rem;
  background: #396cd8;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.execute-btn:hover:not(:disabled) {
  background: #2a5bb5;
}

.execute-btn:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.result {
  margin-top: 2rem;
  padding: 1.5rem;
  border-radius: 8px;
  background: #f9f9f9;
}

.result h3 {
  margin: 0 0 1rem 0;
  font-size: 1.1rem;
  color: #0f0f0f;
}

.result-content {
  padding: 1rem;
  border-radius: 4px;
}

.result-content.success {
  background: #d4edda;
  border: 1px solid #c3e6cb;
}

.result-content.error {
  background: #f8d7da;
  border: 1px solid #f5c6cb;
}

.result-content p {
  margin: 0.5rem 0;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #999;
}

.empty-state p {
  margin: 0.5rem 0;
}

.hint {
  font-size: 0.875rem;
  color: #bbb;
}
</style>
