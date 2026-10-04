<script setup lang="ts">
import { computed } from 'vue';
import MediaPreferenceEditor from './MediaPreferenceEditor.vue';
import UiInlineNotice from '../../ui/InlineNotice.vue';
import UiSelect from '../../ui/Select.vue';
import MediaOutputOptions from '../../ui/MediaOutputOptions.vue';
import { audioQualityOptions, codecOptions, missingQualityOptions, videoQualityOptions } from './settingsCatalog';
import type { SettingsForm } from './useSettingsForm';
import { isAndroidPlatform } from '../../utils/platform';

const { form } = defineProps<{ form: SettingsForm }>();
const { settings, settingsVideoQuality, settingsAudioQuality, settingsCodec, settingsMissingQualityPolicy, settingsOutputFormat, settingsEmbedCover, settingsEmbedSubtitles, settingsRetainRawStreams, settingsEmbeddingFormatError } = form;
const androidPlatform = isAndroidPlatform();
const preferences = computed({
  get: () => form.settings.draft.media_preferences,
  set: (value) => form.settings.setMediaPreferences(value),
});
const mediaMode = computed({ get: () => settings.draft.media_mode, set: (value) => { settings.draft.media_mode = value; } });
const audioFormat = computed({ get: () => settings.draft.audio_output_format, set: (value) => { settings.draft.audio_output_format = value; } });
const subtitleFormat = computed({ get: () => settings.draft.subtitle_format, set: (value) => { settings.draft.subtitle_format = value; } });
const danmakuFormat = computed({ get: () => settings.draft.danmaku_format, set: (value) => { settings.draft.danmaku_format = value; } });
const assets = computed({ get: () => ({ cover: form.settingsArchiveCover.value, subtitles: form.settingsArchiveSubtitles.value, danmaku: form.settingsArchiveDanmaku.value, nfo: form.settingsArchiveNfo.value }), set: (value) => { settings.draft.archive_mode = 'custom'; settings.draft.archive_assets = { ...value }; } });
</script>

<template>
  <section class="settings-block">
      <MediaOutputOptions
        v-model:mode="mediaMode"
        v-model:audio-format="audioFormat"
        v-model:subtitle-format="subtitleFormat"
        v-model:danmaku-format="danmakuFormat"
        v-model:assets="assets"
        v-model:format="settingsOutputFormat"
        v-model:embed-cover="settingsEmbedCover"
        v-model:embed-subtitles="settingsEmbedSubtitles"
        v-model:retain-raw-streams="settingsRetainRawStreams"
        :embedding-supported="!androidPlatform"
        :disabled="settings.loading || settings.saving"
      >
        <template #video>
          <UiSelect v-model="settingsVideoQuality" label="视频清晰度" :options="videoQualityOptions" :disabled="preferences.video.length > 0" />
          <UiSelect v-model="settingsCodec" label="视频编码偏好" :options="codecOptions" :disabled="preferences.video.length > 0" />
        </template>
        <template #audio><UiSelect v-model="settingsAudioQuality" label="音频质量" :options="audioQualityOptions" :disabled="preferences.audio.length > 0" /></template>
      </MediaOutputOptions>
      <UiSelect v-model="settingsMissingQualityPolicy" label="指定质量不可用时" :options="missingQualityOptions" />
    <UiInlineNotice v-if="settingsEmbeddingFormatError" tone="danger">
      {{ settingsEmbeddingFormatError }}
    </UiInlineNotice>
    <fieldset class="settings-group">
      <legend>媒体优先顺序</legend>
      <MediaPreferenceEditor v-model="preferences" />
    </fieldset>
  </section>
</template>
