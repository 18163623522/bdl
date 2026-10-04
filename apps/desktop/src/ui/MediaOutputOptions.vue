<script setup lang="ts">
import { computed } from 'vue';
import UiCheckbox from './Checkbox.vue';
import UiSelect from './Select.vue';

const format = defineModel<'mp4' | 'mkv'>('format', { required: true });
const cover = defineModel<boolean>('embedCover', { required: true });
const subtitles = defineModel<boolean>('embedSubtitles', { required: true });
const raw = defineModel<boolean>('retainRawStreams', { required: true });
const { disabled = false, embeddingSupported = true } = defineProps<{
  disabled?: boolean;
  embeddingSupported?: boolean;
}>();
const formats = [{ label: 'MP4', value: 'mp4' }, { label: 'MKV', value: 'mkv' }];
const outputFormat = computed({
  get: () => format.value,
  set: (value: string) => {
    if (value !== 'mkv') {
      cover.value = false;
      subtitles.value = false;
    }
    format.value = value === 'mkv' ? 'mkv' : 'mp4';
  },
});
const embedCover = computed({
  get: () => cover.value,
  set: (value: boolean) => {
    if (value) format.value = 'mkv';
    cover.value = value;
  },
});
const embedSubtitles = computed({
  get: () => subtitles.value,
  set: (value: boolean) => {
    if (value) format.value = 'mkv';
    subtitles.value = value;
  },
});
</script>

<template>
  <div class="media-output-options">
    <UiSelect
      v-model="outputFormat"
      label="封装格式"
      :options="formats"
      :disabled="disabled"
      :helper="embeddingSupported ? '勾选嵌入时自动使用 MKV' : undefined"
    />
    <div class="media-embedding-options">
      <UiCheckbox v-model="embedCover" label="嵌入封面" :disabled="disabled || !embeddingSupported" />
      <UiCheckbox v-model="embedSubtitles" label="嵌入字幕（SRT）" :disabled="disabled || !embeddingSupported" />
    </div>
    <p v-if="!embeddingSupported" class="media-output-note">Android 暂不支持嵌入，可在下方保存封面和字幕。</p>
    <UiCheckbox v-model="raw" label="保留原始视频/音频轨道" :disabled="disabled" />
  </div>
</template>

<style scoped>
.media-output-options {
  min-width: 0;
  display: grid;
  gap: var(--space-12);
}

.media-embedding-options {
  min-width: 0;
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-8) var(--space-lg);
}

.media-output-note {
  margin: 0;
  color: var(--color-muted);
  font-size: var(--font-12);
  line-height: 1.5;
}
</style>
