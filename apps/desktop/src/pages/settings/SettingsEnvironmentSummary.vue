<script setup lang="ts">
import type { EnvironmentHealthSnapshot } from '../../api/dto';
import UiButton from '../../ui/Button.vue';
import UiStatusBadge from '../../ui/StatusBadge.vue';

const { health, checking = false } = defineProps<{
  health: EnvironmentHealthSnapshot | null;
  checking?: boolean;
}>();

const emit = defineEmits<{
  check: [];
  chooseFfmpeg: [];
  useSystemFfmpeg: [];
}>();
</script>

<template>
  <section
    class="mt-1 grid gap-3 rounded-lg border border-(--color-border) bg-(--color-panel) p-3"
    aria-label="运行环境"
  >
    <header class="flex min-w-0 flex-wrap items-center justify-between gap-3">
      <div class="flex min-w-0 items-center gap-2">
        <UiStatusBadge :status="health?.ffmpeg.status === 'ready' ? 'done' : 'warning'">
          {{ checking ? '检查中' : !health ? '未检查' : health.ffmpeg.status === 'ready' ? '就绪' : '需要处理' }}
        </UiStatusBadge>
        <span class="truncate text-xs text-(--color-muted)" :title="health?.ffmpeg.path ?? undefined">{{ health?.ffmpeg.version ?? health?.ffmpeg.message ?? '自动检测系统 FFmpeg' }}</span>
      </div>
      <UiButton size="compact" variant="ghost" :disabled="checking" @click="emit('check')">重新检查</UiButton>
    </header>

    <div v-if="health && health.ffmpeg.status !== 'ready'" class="flex flex-wrap items-center gap-2 border-t border-(--color-border) pt-2">
      <UiButton
        size="compact"
        variant="secondary"
        @click="emit('chooseFfmpeg')"
      >
        选择 FFmpeg
      </UiButton>
      <UiButton
        v-if="health.ffmpeg.source === 'configured'"
        size="compact"
        variant="ghost"
        @click="emit('useSystemFfmpeg')"
      >
        使用系统版本
      </UiButton>
    </div>
  </section>
</template>
