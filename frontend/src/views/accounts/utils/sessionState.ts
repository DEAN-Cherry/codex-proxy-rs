import type { SessionStateObservation } from '@/api'

const validationLabels: Record<SessionStateObservation['validation'], string> = {
  accepted: '通过校验',
  invalid_length: '长度不符',
  invalid_format: '格式不符',
  missing: '未返回 State',
  upstream_error: '上游响应异常',
}

export function stateValidationLabel(observation: SessionStateObservation) {
  const label = validationLabels[observation.validation] ?? '未知校验结果'
  return observation.httpStatus === 200 ? label : `${label} · HTTP ${observation.httpStatus}`
}
