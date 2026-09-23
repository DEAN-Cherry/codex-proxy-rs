<script setup lang="ts">
import type { AccountRow } from '../constants'
import type { SessionStateRefresh } from '@/api'
import { computed } from 'vue'
import BaseButton from '@/components/base/BaseButton.vue'
import BaseModal from '@/components/base/BaseModal/index.vue'
import { formatDateTime } from '@/utils/date'
import { stateValidationLabel } from '../utils/sessionState'

const props = defineProps<{
  account: AccountRow | null
  result: SessionStateRefresh | null
  loading: boolean
  error: string
}>()
const emit = defineEmits<{ retry: [account: AccountRow], cancel: [] }>()
const open = defineModel<boolean>({ required: true })
const models = computed(() => (props.account?.sessionKeepaliveModels ?? []).map(model => props.result?.models.find(item => item.model === model)
  ?? { model, error: null, expireAt: null, refreshedAt: null, observation: props.account?.sessionKeepaliveObservations?.[model] ?? null }))
const successCount = computed(() => models.value.filter(model => model.expireAt && !model.error).length)
</script>

<template>
  <BaseModal v-model="open" title="刷新 State" size="md">
    <div class="grid gap-4">
      <p class="m-0 text-cp text-cp-text-secondary">
        {{ account?.name }} · 各模型独立刷新，成功后缓存 60 分钟
      </p>
      <p v-if="loading" role="status" class="m-0 text-cp text-cp-text-secondary">
        已成功 {{ successCount }} / {{ models.length }} 个模型，其余模型刷新中，最多重试 100 轮
      </p>
      <p v-if="error" role="alert" class="m-0 text-cp text-cp-error">
        {{ error }}
      </p>
      <div v-for="model in models" :key="model.model" class="grid gap-2 rounded-cp bg-cp-fill-quaternary p-4">
        <div class="flex items-center justify-between gap-3">
          <span class="font-mono text-cp font-medium text-cp-text">{{ model.model }}</span>
          <span role="status" class="text-cp-sm" :class="model.error ? 'text-cp-error' : model.expireAt ? 'text-cp-success' : 'text-cp-text-tertiary'">
            {{ model.error ? '刷新失败' : model.expireAt ? '刷新成功' : loading ? '刷新中' : '未完成' }}
          </span>
        </div>
        <div v-if="model.observation" class="grid gap-1 text-cp-sm">
          <span class="text-cp-text-secondary">
            最近响应：{{ model.observation.stateLength == null ? '未返回 State' : `${model.observation.stateLength} 字节` }}
          </span>
          <span :class="model.observation.validation === 'accepted' ? 'text-cp-text-secondary' : 'text-cp-warning-text'">
            {{ stateValidationLabel(model.observation) }} · {{ formatDateTime(model.observation.observedAt) }}
          </span>
        </div>
        <p v-if="model.error" class="m-0 text-cp-sm text-cp-error">
          {{ model.error }}，仍有效的旧 State 保留至原到期时间
        </p>
        <dl v-if="model.expireAt" class="m-0 grid gap-1 text-cp-sm text-cp-text-secondary">
          <div class="flex justify-between gap-2">
            <dt>刷新时间</dt><dd class="m-0">
              {{ formatDateTime(model.refreshedAt) }}
            </dd>
          </div>
          <div class="flex justify-between gap-2">
            <dt>到期时间</dt><dd class="m-0">
              {{ formatDateTime(model.expireAt * 1000) }}
            </dd>
          </div>
        </dl>
      </div>
      <p class="m-0 text-cp-sm text-cp-text-tertiary">
        最近响应仅记录长度与校验结果，重启后重新观测，校验通过不代表缓存写入成功
      </p>
      <p class="m-0 text-cp-sm text-cp-text-tertiary">
        强制刷新会重置重试计数和已有探测冷却，重新向上游获取 State
      </p>
    </div>
    <template #footer>
      <BaseButton v-if="loading" variant="secondary" @click="emit('cancel')">
        停止刷新
      </BaseButton>
      <BaseButton variant="secondary" @click="open = false">
        关闭
      </BaseButton>
      <BaseButton variant="primary" :loading="loading" :disabled="!account || !account.enabled || !account.enableSessionKeepalive" @click="account && emit('retry', account)">
        {{ loading ? '刷新中' : '强制刷新' }}
      </BaseButton>
    </template>
  </BaseModal>
</template>
