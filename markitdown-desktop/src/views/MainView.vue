<script setup lang="ts">
import TopBar from '@/components/layout/TopBar.vue';
import Sidebar from '@/components/layout/Sidebar.vue';
import Workspace from '@/components/layout/Workspace.vue';
</script>

<template>
  <div class="flex h-screen flex-col bg-white text-zinc-900">
    <TopBar title="MarkItDown Desktop" />
    <div class="flex flex-1 overflow-hidden">
      <Sidebar />
      <Workspace />
    </div>
  </div>
</template>
