<script setup lang="ts">
import { Splitpanes, Pane } from 'splitpanes';
import 'splitpanes/dist/splitpanes.css';
import SourcePanel from './SourcePanel.vue';
import MarkdownRenderer from './MarkdownRenderer.vue';
</script>

<template>
  <Splitpanes class="flex-1 default-theme">
    <Pane min-size="20" size="30" class="bg-white">
      <SourcePanel />
    </Pane>
    <Pane min-size="30" size="70" class="bg-white">
      <MarkdownRenderer />
    </Pane>
  </Splitpanes>
</template>

<style>
.splitpanes__splitter {
  background-color: #e4e4e7 !important;
  width: 2px !important;
  position: relative;
  transition: background-color 0.2s ease;
}
.splitpanes__splitter:hover {
  background-color: #a1a1aa !important;
}
.splitpanes__splitter::before,
.splitpanes__splitter::after {
  display: none !important;
}
</style>
