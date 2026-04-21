<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { marked } from 'marked';
import { createHighlighter, type Highlighter } from 'shiki';
import { convertFileSrc } from '@tauri-apps/api/core';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Skeleton } from '@/components/ui/skeleton';
import { useAppStore } from '@/lib/composables/useAppStore';

function getDir(filepath: string): string {
  const sep = filepath.includes('\\') ? '\\' : '/';
  const idx = filepath.lastIndexOf(sep);
  return idx >= 0 ? filepath.substring(0, idx) : '';
}

function joinPath(dir: string, relative: string): string {
  const sep = dir.includes('\\') ? '\\' : '/';
  const normalized = relative.replace(/[/\\]/g, sep);
  return dir + sep + normalized;
}

const { state } = useAppStore();

const renderedHtml = ref<string>('');
const highlighter = ref<Highlighter | null>(null);
const loading = ref(true);

onMounted(async () => {
  try {
    highlighter.value = await createHighlighter({
      themes: ['github-light'],
      langs: ['javascript', 'typescript', 'python', 'rust', 'json', 'html', 'css', 'bash', 'markdown'],
    });
  } catch (e) {
    console.error('Failed to init shiki:', e);
  } finally {
    loading.value = false;
    renderMarkdown();
  }
});

watch(
  () => state.currentConversion?.markdown_content,
  () => {
    renderMarkdown();
  },
);

function renderMarkdown() {
  if (!state.currentConversion?.markdown_content) {
    renderedHtml.value = '';
    return;
  }

  marked.setOptions({
    gfm: true,
    breaks: true,
  });

  // Custom renderer for code blocks with shiki
  const renderer = new marked.Renderer();
  const hl = highlighter.value;

  renderer.code = function ({ text, lang }: { text: string; lang?: string }) {
    if (hl && lang && hl.getLoadedLanguages().includes(lang)) {
      try {
        return hl.codeToHtml(text, {
          lang,
          theme: 'github-light',
        });
      } catch {
        // fallback to plain text
      }
    }
    const escaped = text
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
    return `<pre class="shiki"><code>${escaped}</code></pre>`;
  };

  // Image renderer: convert local paths to Tauri asset URLs
  renderer.image = function ({ href, title, text }: { href: string; title?: string; text?: string }) {
    if (href && !href.startsWith('http') && !href.startsWith('data:')) {
      const sourcePath = state.currentConversion?.source_path;
      if (sourcePath) {
        const sourceDir = getDir(sourcePath);
        const absolutePath = joinPath(sourceDir, href);
        href = convertFileSrc(absolutePath);
      }
    }
    const escapedHref = (href || '').replace(/"/g, '&quot;');
    const escapedAlt = (text || '').replace(/"/g, '&quot;');
    const titleAttr = title ? ` title="${title.replace(/"/g, '&quot;')}"` : '';
    return `<img src="${escapedHref}" alt="${escapedAlt}"${titleAttr} />`;
  };

  marked.use({ renderer });
  renderedHtml.value = marked(state.currentConversion.markdown_content) as string;
}
</script>

<template>
  <ScrollArea class="h-full">
    <div class="p-6">
      <div v-if="loading" class="space-y-3">
        <Skeleton class="h-6 w-3/4" />
        <Skeleton class="h-4 w-full" />
        <Skeleton class="h-4 w-5/6" />
        <Skeleton class="h-4 w-4/5" />
      </div>
      <div
        v-else-if="renderedHtml"
        class="prose prose-zinc max-w-none prose-headings:font-semibold prose-headings:text-zinc-900 prose-p:text-zinc-700 prose-code:text-zinc-800 prose-pre:bg-zinc-50 prose-pre:border prose-pre:border-zinc-200"
        v-html="renderedHtml"
      />
      <div v-else class="py-12 text-center text-xs text-zinc-400">
        暂无 Markdown 内容
      </div>
    </div>
  </ScrollArea>
</template>
