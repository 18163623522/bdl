<script setup lang="ts">
import { computed, ref } from 'vue';
import type { DownloadWorkflow, WorkflowAssetOutput } from '../../api/dto';
import UiCheckbox from '../../ui/Checkbox.vue';
import UiSelect from '../../ui/Select.vue';
import UiInlineNotice from '../../ui/InlineNotice.vue';
import UiTabs from '../../ui/Tabs.vue';
import { workflowError, workflowOutputs } from '../../utils/downloadWorkflow';
import { isAndroidPlatform } from '../../utils/platform';

const flow = defineModel<DownloadWorkflow>({ required: true });
const { disabled = false } = defineProps<{ disabled?: boolean }>();
const android = isAndroidPlatform();
const output = computed(() => workflowOutputs(flow.value));
const tab = ref('content');
const tabs = [{ label: '内容', value: 'content' }, { label: '转换', value: 'conversion' }, { label: '封装', value: 'packaging' }];
const error = computed(() => workflowError(flow.value));
const merge = computed({ get: () => flow.value.merge_media && flow.value.video.enabled, set: (value: boolean) => {
  flow.value.merge_media = value;
  if (!value) {
    for (const rule of [flow.value.video, flow.value.audio]) if (rule.enabled) rule.save = true;
    for (const rule of [flow.value.cover, flow.value.subtitles]) { if (rule.embed && !rule.save) rule.save = true; rule.embed = false; }
  }
} });
const mediaEnabled = (kind: 'video' | 'audio') => computed({ get: () => flow.value[kind].enabled, set: (value: boolean) => {
  flow.value[kind].enabled = value;
  if (!flow.value.video.enabled) merge.value = false;
  if (value && !flow.value.merge_media) flow.value[kind].save = true;
} });
const video = mediaEnabled('video');
const audio = mediaEnabled('audio');
const format = computed({ get: () => flow.value.container, set: (value: string) => {
  flow.value.container = value === 'mkv' ? 'mkv' : 'mp4';
  if (value !== 'mkv') for (const rule of [flow.value.cover, flow.value.subtitles]) { if (rule.embed && !rule.save) rule.save = true; rule.embed = false; }
} });
const assetEnabled = (key: 'cover' | 'subtitles' | 'danmaku') => computed({ get: () => flow.value[key].enabled, set: (value: boolean) => { flow.value[key].enabled = value; if (!value) flow.value[key].embed = false; } });
const cover = assetEnabled('cover');
const subtitles = assetEnabled('subtitles');
const danmaku = assetEnabled('danmaku');
const embed = (key: 'cover' | 'subtitles') => computed({ get: () => flow.value[key].embed, set: (value: boolean) => {
  flow.value[key].embed = value;
  if (value) { flow.value.merge_media = true; flow.value.container = 'mkv'; if (key === 'subtitles' && flow.value.subtitles.format === 'original') flow.value.subtitles.format = 'srt'; }
} });
const embedCover = embed('cover');
const embedSubtitles = embed('subtitles');
const videoSave = computed({ get: () => flow.value.video.save || flow.value.video.retain_original, set: (value: boolean) => { flow.value.video.save = value; flow.value.video.retain_original = false; } });
const subtitleFormat = computed({ get: () => flow.value.subtitles.format, set: (value: WorkflowAssetOutput['format']) => { flow.value.subtitles.format = value; if (value === 'original') flow.value.subtitles.embed = false; } });
</script>

<template>
  <div class="flex min-h-0 flex-1 flex-col gap-3">
    <UiTabs v-model="tab" :tabs="tabs" />
    <div class="grid min-h-0 content-start gap-4 overflow-y-auto pr-1">
      <template v-if="tab === 'content'">
        <p class="settings-note">选择需要下载的内容，转换和封装方式在其他分页设置。</p>
        <div class="grid grid-cols-2 gap-3">
          <UiCheckbox v-model="video" label="下载视频" :disabled="disabled" />
          <UiCheckbox v-model="audio" label="下载音频" :disabled="disabled" />
          <UiCheckbox v-model="subtitles" label="下载字幕" :disabled="disabled" />
          <UiCheckbox v-model="cover" label="下载封面" :disabled="disabled" />
          <UiCheckbox v-model="danmaku" label="下载弹幕" :disabled="disabled" />
          <UiCheckbox v-model="flow.nfo" label="生成 NFO" :disabled="disabled" />
        </div>
      </template>
      <template v-else-if="tab === 'conversion'">
        <fieldset v-if="video" class="settings-group">
          <legend>视频</legend>
          <UiCheckbox v-model="videoSave" label="保存独立原始视频（m4s）" :disabled="disabled" />
        </fieldset>
        <fieldset v-if="audio" class="settings-group">
          <legend>音频</legend>
          <UiCheckbox v-model="flow.audio.save" label="保存独立音频" :disabled="disabled" />
          <UiSelect v-if="flow.audio.save" v-model="flow.audio.format" label="独立音频格式" :options="[{ label: '不转换（m4s）', value: 'm4s' }, { label: '转为 MP3', value: 'mp3' }]" :disabled="disabled" />
          <UiCheckbox v-if="!flow.audio.save || flow.audio.format === 'mp3' || flow.audio.retain_original" v-model="flow.audio.retain_original" label="额外保留原始音频（m4s）" :disabled="disabled" />
        </fieldset>
        <fieldset v-if="subtitles" class="settings-group">
          <legend>字幕</legend>
          <UiSelect v-model="subtitleFormat" label="字幕格式" :options="[{ label: '不转换（原始 JSON）', value: 'original' }, { label: 'SRT', value: 'srt' }, { label: 'ASS', value: 'ass' }]" :disabled="disabled" />
          <UiCheckbox v-model="flow.subtitles.save" label="保存独立字幕" :disabled="disabled" />
          <UiCheckbox v-if="flow.subtitles.format !== 'original'" v-model="flow.subtitles.retain_original" label="额外保留原始字幕（JSON）" :disabled="disabled" />
        </fieldset>
        <fieldset v-if="cover" class="settings-group"><legend>封面</legend><UiCheckbox v-model="flow.cover.save" label="保存独立封面" :disabled="disabled" /></fieldset>
        <fieldset v-if="danmaku" class="settings-group">
          <legend>弹幕</legend>
          <UiSelect v-model="flow.danmaku.format" label="弹幕格式" :options="[{ label: '不转换（XML）', value: 'xml' }, { label: 'HTML（离线播放）', value: 'html' }]" :disabled="disabled" />
          <UiCheckbox v-model="flow.danmaku.save" label="保存独立弹幕" :disabled="disabled" />
          <UiCheckbox v-if="flow.danmaku.format === 'html'" v-model="flow.danmaku.retain_original" label="额外保留原始弹幕（XML）" :disabled="disabled" />
        </fieldset>
        <p v-if="!video && !audio && !subtitles && !cover && !danmaku" class="settings-note">当前没有需要转换的内容。</p>
      </template>
      <template v-else>
        <template v-if="video">
          <UiCheckbox v-model="merge" label="生成媒体成品" :disabled="disabled" />
          <UiSelect v-if="merge" v-model="format" label="封装格式" :options="[{ label: 'MP4', value: 'mp4' }, { label: 'MKV', value: 'mkv' }]" :disabled="disabled" />
          <UiCheckbox v-if="subtitles" v-model="embedSubtitles" label="嵌入字幕" :disabled="disabled || (android && !flow.subtitles.embed)" />
          <UiCheckbox v-if="cover" v-model="embedCover" label="嵌入封面" :disabled="disabled || (android && !flow.cover.embed)" />
          <p class="settings-note">嵌入字幕或封面时自动使用 MKV。</p>
          <UiInlineNotice v-if="android && (cover || subtitles)" tone="warning">Android 暂不支持嵌入，请保存独立文件。</UiInlineNotice>
        </template>
        <p v-else class="settings-note">没有视频时，按转换设置保存独立文件。</p>
        <div class="rounded-lg border border-(--color-border) p-3" aria-label="预设输出预览"><strong class="text-sm">最终输出</strong><p class="m-0 mt-2 text-sm text-(--color-muted)">{{ output.join('、') || '尚未选择输出内容' }}</p></div>
      </template>
      <UiInlineNotice v-if="error" tone="danger">{{ error }}</UiInlineNotice>
    </div>
  </div>
</template>
