<script setup lang="ts">
import type { Account } from '@/api'
import { computed } from 'vue'
import { formatDateTime, formatTime } from '@/utils/date'
import { stateValidationLabel } from '../utils/sessionState'

const props = defineProps<{
  account: Pick<Account, 'enableSessionKeepalive' | 'sessionKeepaliveModels' | 'sessionKeepaliveStateLengths' | 'sessionKeepaliveObservations'>
}>()

const models = computed(() => [...new Set([
  ...(props.account.enableSessionKeepalive ? props.account.sessionKeepaliveModels : []),
  ...Object.keys(props.account.sessionKeepaliveStateLengths),
  ...Object.keys(props.account.sessionKeepaliveObservations ?? {}),
])].map(model => ({
  model,
  observation: props.account.sessionKeepaliveObservations?.[model],
  cachedLength: props.account.sessionKeepaliveStateLengths[model],
})))
</script>

<template>
  <div v-if="models.length" class="grid min-w-0 gap-2 text-cp-xs">
    <div v-for="item in models" :key="item.model" class="grid min-w-0 gap-0.5">
      <span class="truncate" :title="item.model">
        <span class="font-mono">{{ item.model }}</span>
        <template v-if="item.observation?.stateLength != null"> · {{ item.observation.stateLength }} 字节</template>
      </span>
      <template v-if="item.observation">
        <span :class="['accepted', 'observed'].includes(item.observation.validation) ? 'text-cp-text-secondary' : 'text-cp-warning-text'">
          {{ stateValidationLabel(item.observation) }}
          <time class="text-cp-text-tertiary" :datetime="item.observation.observedAt" :title="formatDateTime(item.observation.observedAt)"> · {{ formatTime(item.observation.observedAt) }}</time>
        </span>
      </template>
      <span v-else class="text-cp-text-quaternary">暂无 State 观测</span>
      <span v-if="item.cachedLength != null" class="text-cp-text-secondary">可用缓存 {{ item.cachedLength }} 字节</span>
    </div>
  </div>
  <span v-else class="text-cp-text-quaternary">暂无 State 观测</span>
</template>
