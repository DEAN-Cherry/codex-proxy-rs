import type { SessionStateLength } from '@/api'

function bounds(rule: SessionStateLength): [number, number] {
  return typeof rule === 'number' ? [rule, rule] : [rule.min, rule.max]
}

export function formatStateLength(rule: SessionStateLength): string {
  const [min, max] = bounds(rule)
  return min === max ? String(min) : `${min}-${max}`
}

export function normalizeStateLengths(rules: SessionStateLength[]): SessionStateLength[] {
  const unique = new Map<string, SessionStateLength>()
  for (const rule of rules) {
    const [min, max] = bounds(rule)
    if (!Number.isInteger(min) || !Number.isInteger(max) || min < 1 || max > 4294967295 || min > max)
      throw new Error('请输入 1～4294967295 的整数或从小到大的区间')
    unique.set(`${min}:${max}`, min === max ? min : { min, max })
  }
  if (unique.size > 32)
    throw new Error('最多配置 32 个长度或区间')
  return [...unique.values()].sort((a, b) => bounds(a)[0] - bounds(b)[0] || bounds(a)[1] - bounds(b)[1])
}

export function parseStateLengths(input: string): SessionStateLength[] {
  if (!input.trim())
    return []
  const rules = input.trim().replace(/\s*-\s*/g, '-').split(/[,，\s]+/).map((token) => {
    const match = /^(\d+)(?:-(\d+))?$/.exec(token)
    if (!match)
      throw new Error('格式示例：200,300,400-500')
    const min = Number(match[1])
    return match[2] === undefined ? min : { min, max: Number(match[2]) }
  })
  return normalizeStateLengths(rules)
}
