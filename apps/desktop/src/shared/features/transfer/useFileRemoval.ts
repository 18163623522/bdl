import { ref, watch } from 'vue';
import type { RemovalPreview } from '../../api/dto';

export function useFileRemoval(
  kind: 'tasks' | 'temp',
  previewFiles: (ids: string[]) => Promise<RemovalPreview>,
  execute: (token: string) => Promise<unknown>,
) {
  const open = ref(false);
  const busy = ref(false);
  const executing = ref(false);
  const preview = ref<RemovalPreview | null>(null);
  const error = ref('');
  const canConfirm = ref(false);
  let ids: string[] = [];
  let generation = 0;
  watch(open, (value) => { if (!value) { generation++; if (!executing.value) busy.value = false; } });
  const refresh = async () => {
    const current = ++generation;
    busy.value = true;
    error.value = '';
    canConfirm.value = false;
    try {
      const result = await previewFiles(ids);
      if (current === generation && open.value) {
        preview.value = result;
        canConfirm.value = true;
      }
    } catch (failure) {
      if (current === generation) error.value = failure instanceof Error ? failure.message : String(failure);
    } finally {
      if (current === generation) busy.value = false;
    }
  };
  const show = (taskIds: string[] = []) => {
    if (busy.value || (kind === 'tasks' && taskIds.length === 0)) return;
    ids = [...new Set(taskIds)];
    preview.value = null;
    open.value = true;
    void refresh();
  };
  const confirm = async () => {
    if (!preview.value || !canConfirm.value || busy.value) return;
    busy.value = true;
    canConfirm.value = false;
    executing.value = true;
    error.value = '';
    try {
      await execute(preview.value.token);
      open.value = false;
    } catch (failure) {
      error.value = failure instanceof Error ? failure.message : String(failure);
    } finally { busy.value = false; executing.value = false; }
  };
  return { kind, open, busy, executing, preview, error, canConfirm, show, refresh, confirm };
}
export type FileRemovalController = ReturnType<typeof useFileRemoval>;
