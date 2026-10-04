<script setup lang="ts">
import { computed } from 'vue';
import MediaPreferenceEditor from './MediaPreferenceEditor.vue';
import SettingsArchiveSection from './SettingsArchiveSection.vue';
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
</script>

<template>
  <section class="settings-block">
    <div class="settings-inline-grid">
      <UiSelect v-model="settingsVideoQuality" label="视频清晰度" :options="videoQualityOptions" :disabled="preferences.video.length > 0" />
      <UiSelect v-model="settingsAudioQuality" label="音频质量" :options="audioQualityOptions" :disabled="preferences.audio.length > 0" />
      <UiSelect v-model="settingsCodec" label="视频编码偏好" :options="codecOptions" :disabled="preferences.video.length > 0" />
      <UiSelect v-model="settingsMissingQualityPolicy" label="指定质量不可用时" :options="missingQualityOptions" />
    </div>
    <fieldset class="settings-group">
      <legend>成品文件</legend>
      <MediaOutputOptions
        v-model:format="settingsOutputFormat"
        v-model:embed-cover="settingsEmbedCover"
        v-model:embed-subtitles="settingsEmbedSubtitles"
        v-model:retain-raw-streams="settingsRetainRawStreams"
        :embedding-supported="!androidPlatform"
        :disabled="settings.loading || settings.saving"
      />
    </fieldset>
    <UiInlineNotice v-if="settingsEmbeddingFormatError" tone="danger">
      {{ settingsEmbeddingFormatError }}
    </UiInlineNotice>
    <fieldset class="settings-group">
      <legend>附加文件</legend>
      <SettingsArchiveSection :form="form" />
    </fieldset>
    <fieldset class="settings-group">
      <legend>媒体优先顺序</legend>
      <MediaPreferenceEditor v-model="preferences" />
    </fieldset>
  </section>
</template>
