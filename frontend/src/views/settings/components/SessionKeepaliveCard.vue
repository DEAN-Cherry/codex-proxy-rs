<script setup lang="ts">
import { shallowRef } from 'vue'
import BaseCard from '@/components/base/BaseCard.vue'
import BaseConfirmModal from '@/components/base/BaseConfirmModal.vue'
import BaseFormItem from '@/components/base/BaseForm/FormItem.vue'
import BaseForm from '@/components/base/BaseForm/index.vue'
import BaseInput from '@/components/base/BaseInput.vue'
import BaseSwitch from '@/components/base/BaseSwitch.vue'

const props = defineProps<{ disabled: boolean }>()
const enabled = defineModel<boolean>({ required: true })
const concurrency = defineModel<string>('concurrency', { required: true })
const retryIntervalSeconds = defineModel<string>('retryIntervalSeconds', { required: true })
const confirming = shallowRef(false)
function toggle() {
  if (props.disabled)
    return
  if (!enabled.value)
    confirming.value = true
  else enabled.value = false
}
function confirm() {
  enabled.value = true
  confirming.value = false
}
</script>

<template>
  <BaseCard title="State 重写">
    <div class="grid gap-3">
      <BaseSwitch :model-value="enabled" label="开启 State 重写" show-label :disabled="disabled" @click.capture.prevent="toggle" />
      <p class="m-0 text-cp-sm text-cp-text-secondary">
        仅对已开启的账号和所选模型发送 hi 获取 State
        请先在 <RouterLink to="/proxies" class="text-cp-primary-text">
          代理管理
        </RouterLink> 保存一个动态代理并测试通过
      </p>
      <BaseForm class="sm:grid-cols-2">
        <BaseFormItem label="探测并发数" description="前三轮每模型 1 个探针，第 4 轮起使用此值">
          <BaseInput v-model="concurrency" aria-label="State 重写探测并发数" type="number" min="1" max="10" step="1" :disabled="disabled" />
        </BaseFormItem>
        <BaseFormItem label="重试间隔（秒）" description="前三轮等待 6 秒，第 4 轮起使用此值，限流时可延长">
          <BaseInput v-model="retryIntervalSeconds" aria-label="State 重写重试间隔（秒）" type="number" min="1" max="300" step="1" :disabled="disabled" />
        </BaseFormItem>
      </BaseForm>
      <p class="m-0 text-cp-sm text-cp-text-secondary">
        后台在 State 剩余不足 10 分钟时刷新，连续失败 100 轮后暂停，可手动强制刷新恢复
        缺少有效 State 时暂停对应账号／模型的业务请求
      </p>
      <p class="m-0 text-cp-sm text-cp-warning-text">
        探测会消耗模型额度，可能触发上游限流或账号异常，保存设置后生效
      </p>
    </div>
  </BaseCard>
  <BaseConfirmModal v-model="confirming" title="确认开启 State 重写" confirm-text="我已了解风险，开启" @confirm="confirm">
    探测会消耗模型额度，可能触发上游限流或账号异常，确认开启？
  </BaseConfirmModal>
</template>
