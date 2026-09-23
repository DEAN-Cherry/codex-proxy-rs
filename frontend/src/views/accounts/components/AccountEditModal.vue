<script setup lang="ts">
import type { AccountRow } from '../constants'
import type { ApiKeyAccountForm } from '../utils/upstreamApiKey'
import type { AccountGroup, AccountModelAccess, SessionStateLength } from '@/api'
import { shallowRef, watch } from 'vue'

import BaseButton from '@/components/base/BaseButton.vue'
import BaseFormItem from '@/components/base/BaseForm/FormItem.vue'
import BaseInput from '@/components/base/BaseInput.vue'
import BaseModal from '@/components/base/BaseModal/index.vue'
import BaseSwitch from '@/components/base/BaseSwitch.vue'
import BaseTag from '@/components/base/BaseTag.vue'
import BaseTextarea from '@/components/base/BaseTextarea.vue'
import ProviderIconGroup from '@/components/ProviderIconGroup.vue'
import { formatStateLength, normalizeStateLengths, parseStateLengths } from '../utils/sessionStateLengths'
import AccountApiKeyFields from './AccountApiKeyFields.vue'
import AccountIdentityCell from './AccountIdentityCell.vue'
import AccountPlanBadge from './AccountPlanBadge.vue'
import AccountSessionModelsField from './AccountSessionModelsField.vue'
import AccountSessionStateSummary from './AccountSessionStateSummary.vue'
import AccountSettingsFields from './AccountSettingsFields.vue'

defineProps<{
  account: AccountRow | null
  groups: AccountGroup[]
  groupsLoading: boolean
  saving: boolean
  configurationLoading: boolean
  configurationReady: boolean
}>()

const emit = defineEmits<{
  save: []
}>()

const open = defineModel<boolean>({ required: true })
const apiKey = defineModel<ApiKeyAccountForm>('apiKey', { required: true })
const notes = defineModel<string>('notes', { required: true })
const enabled = defineModel<boolean>('enabled', { required: true })
const sessionKeepaliveModels = defineModel<string[]>('sessionKeepaliveModels', { required: true })
const sessionKeepaliveExpectedLengths = defineModel<SessionStateLength[]>('sessionKeepaliveExpectedLengths', { required: true })
const enableSessionKeepalive = defineModel<boolean>('enableSessionKeepalive', { required: true })
const concurrencyLimit = defineModel<string>('concurrencyLimit', { required: true })
const modelAccess = defineModel<AccountModelAccess | undefined>('modelAccess', { required: true })
const weight = defineModel<string>('weight', { required: true })
const proxyMode = defineModel<string>('proxyMode', { required: true })
const proxyId = defineModel<string>('proxyId', { required: true })
const selectedGroupIds = defineModel<string[]>('selectedGroupIds', { required: true })
const newExpectedLength = shallowRef('')
const lengthError = shallowRef('')

watch(open, () => {
  newExpectedLength.value = ''
  lengthError.value = ''
})

function addExpectedLength() {
  const input = newExpectedLength.value.trim()
  if (!input)
    return true
  try {
    sessionKeepaliveExpectedLengths.value = normalizeStateLengths([
      ...sessionKeepaliveExpectedLengths.value,
      ...parseStateLengths(input),
    ])
  }
  catch (error) {
    lengthError.value = error instanceof Error ? error.message : 'State 长度格式无效'
    return false
  }
  newExpectedLength.value = ''
  lengthError.value = ''
  return true
}

function save() {
  if (enableSessionKeepalive.value && !addExpectedLength())
    return
  emit('save')
}

function removeExpectedLength(value: SessionStateLength) {
  sessionKeepaliveExpectedLengths.value = sessionKeepaliveExpectedLengths.value.filter(length => formatStateLength(length) !== formatStateLength(value))
}
</script>

<template>
  <BaseModal
    v-model="open"
    title="编辑账号"
    size="md-wide"
    :dismissible="!saving"
  >
    <div v-if="account" class="grid gap-5">
      <div
        class="flex flex-wrap items-center justify-between gap-4 rounded-cp bg-cp-fill-quaternary px-4 py-3.5"
      >
        <AccountIdentityCell
          class="min-w-0 flex-1"
          :account="account"
          size="lg"
        />
        <div class="flex shrink-0 items-center gap-3">
          <AccountPlanBadge :authentication-kind="account.authenticationKind" :plan-type="account.planType" :plan-type-display="account.planTypeDisplay" size="sm" />
          <ProviderIconGroup
            :provider="account.provider"
            :authentication-kind="account.authenticationKind"
          />
        </div>
      </div>

      <section v-if="account.authenticationKind === 'api_key'" class="grid gap-4">
        <h3 class="m-0 text-cp font-heavy text-cp-text">
          上游连接
        </h3>
        <p v-if="configurationLoading" role="status" class="m-0 text-cp-sm text-cp-text-secondary">
          正在读取上游设置…
        </p>
        <p v-else-if="!configurationReady" role="alert" class="m-0 text-cp-sm text-cp-error">
          上游设置读取失败，请关闭后重试
        </p>
        <AccountApiKeyFields v-else v-model="apiKey" editing :disabled="saving" />
      </section>

      <AccountSettingsFields
        v-model:enabled="enabled"
        v-model:concurrency-limit="concurrencyLimit"
        v-model:weight="weight"
        v-model:model-access="modelAccess"
        v-model:selected-group-ids="selectedGroupIds"
        v-model:proxy-mode="proxyMode"
        v-model:proxy-id="proxyId"
        :groups="groups"
        :groups-loading="groupsLoading"
        :disabled="saving"
        :endpoint="account.outboundProxyEndpoint"
        :account-id="account.id"
      />

      <div v-if="account.provider === 'openai' && account.authenticationKind === 'oauth'" class="flex items-center justify-between gap-3">
        <div class="grid gap-1">
          <span class="text-cp font-medium text-cp-text-secondary">State 重写</span>
          <span class="text-cp-sm text-cp-text-tertiary">使用全局动态代理获取各模型的 State，须先在设置页开启</span>
        </div>
        <BaseSwitch v-model="enableSessionKeepalive" label="切换账号 State 重写" :disabled="saving" />
      </div>

      <AccountSessionModelsField v-if="enableSessionKeepalive && account.provider === 'openai' && account.authenticationKind === 'oauth'" v-model="sessionKeepaliveModels" :account-id="account.id" :disabled="saving" />

      <BaseFormItem
        v-if="enableSessionKeepalive && account.provider === 'openai' && account.authenticationKind === 'oauth'"
        label="期望 State 长度（字节）"
        description="留空不限长度，支持单值和闭区间，多个规则用逗号分隔"
        :error="lengthError"
      >
        <div class="grid gap-2">
          <div v-if="sessionKeepaliveExpectedLengths.length" class="flex flex-wrap gap-2">
            <BaseTag v-for="length in sessionKeepaliveExpectedLengths" :key="formatStateLength(length)" type="info" round>
              {{ formatStateLength(length) }}
              <button type="button" class="ml-1 border-0 bg-transparent p-0 text-current" :disabled="saving" :aria-label="`删除允许长度 ${formatStateLength(length)}`" @click="removeExpectedLength(length)">
                ×
              </button>
            </BaseTag>
          </div>
          <div class="flex gap-2">
            <BaseInput v-model="newExpectedLength" class="min-w-0 flex-1" placeholder="例如 200,300,400-500" :disabled="saving" @keydown.enter.prevent="addExpectedLength" />
            <BaseButton variant="secondary" :disabled="saving || !newExpectedLength.trim()" @click="addExpectedLength">
              添加
            </BaseButton>
          </div>
        </div>
      </BaseFormItem>

      <BaseFormItem v-if="account.provider === 'openai'" label="State 观测">
        <AccountSessionStateSummary :account="account" />
      </BaseFormItem>

      <BaseFormItem label="备注">
        <BaseTextarea
          v-model="notes"
          :rows="3"
          :maxlength="500"
          placeholder="最多 500 字，留空可清除备注"
          :disabled="saving"
        />
      </BaseFormItem>
    </div>

    <template #footer>
      <BaseButton variant="secondary" :disabled="saving" @click="open = false">
        取消
      </BaseButton>
      <BaseButton
        variant="primary"
        :loading="saving"
        :disabled="!account || groupsLoading || (account.authenticationKind === 'api_key' && !configurationReady)"
        @click="save"
      >
        保存更改
      </BaseButton>
    </template>
  </BaseModal>
</template>
