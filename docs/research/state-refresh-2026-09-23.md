# State 刷新有效性与替代改进调查

调查日期：2026-09-23。问题中的「stage」按当前 Fork 的 State 刷新/重写理解。
本记录区分官方客户端合同、社区观测与本地实现，不代表对 OpenAI 服务端策略的确认。

## 结论

1. 没有找到 OpenAI 宣布全面废弃 `x-codex-turn-state` 的依据。当天官方源码仍使用它维持同一逻辑 turn 内的路由连续性。
2. 旧的「刷指定长度 State 防止模型降档」有较强失效信号：9 月 21 日最初实现作者称「已经封掉 292」，用户报告刷新成功后仍返回 Luna；9 月 22 日另有同流 State 重放无可测效果的实验报告；9 月 23 日还有 780 长度报告。没有找到可确认在当前服务端稳定有效的新版恢复方案。
3. 该测试不等同于完整复现本 Fork 的独立代理采集、指定长度筛选与跨 turn 注入，不能据此证明所有 State 重写方法已经全面失效。它足以反驳「长度命中或成功刷新就保证目标模型」的假设。
4. 最新直接相关替代方向是本项目上游 #239 的「响应模型差异观测 → 调度下线 → 探测确认恢复」，已有 Fork 实现但未获上游合并和完整运行验证。官方已落地的改进则集中在 turn 生命周期、账号隔离与续接/压缩兼容。

## 证据等级

- **官方源码/已合并 PR**：能够说明客户端协议合同与实现，不能证明未公开的服务端路由或推理权重。
- **社区一手报告**：能说明报告者观察到的场景。未取得、复核原始证据包时，数字与实验结论应归因于报告者。
- **二手文章**：仅作查找线索，不能作为协议或有效性依据。

## 1. 官方仍使用 State，但作用域是单个 turn

核对 OpenAI Codex 当天 `main` 的固定提交 `4981037e99a322ee9cf29bc8730cb64d263fa00b`：

- [`core/src/client.rs:275–298`](https://github.com/openai/codex/blob/4981037e99a322ee9cf29bc8730cb64d263fa00b/codex-rs/core/src/client.rs#L275-L298) 明确要求每轮创建新 `ModelClientSession`；同一轮内回传第一次收到的 State，不得跨轮回传。注释明确指出跨轮复用会违反客户端/服务端合同并可能造成路由问题。
- [`client.rs:595–611`](https://github.com/openai/codex/blob/4981037e99a322ee9cf29bc8730cb64d263fa00b/codex-rs/core/src/client.rs#L595-L611) 每轮建立新的 `OnceLock`。
- [`responses_websocket.rs:529–535`](https://github.com/openai/codex/blob/4981037e99a322ee9cf29bc8730cb64d263fa00b/codex-rs/codex-api/src/endpoint/responses_websocket.rs#L529-L535) 从 WS 握手捕获 State；[同文件:760–763](https://github.com/openai/codex/blob/4981037e99a322ee9cf29bc8730cb64d263fa00b/codex-rs/codex-api/src/endpoint/responses_websocket.rs#L760-L763) 也可从流事件捕获。首次写入后不覆盖。
- [`turn_state.rs`](https://github.com/openai/codex/blob/4981037e99a322ee9cf29bc8730cb64d263fa00b/codex-rs/core/tests/suite/turn_state.rs#L207-L258) 测试了服务端先后给出不同值时，同一轮后续请求继续发送首值。

因此，State 不是源码中定义的「账号一小时能力通行证」，也没有在这些合同中按 292/312 长度承诺模型质量。

## 2. 近期注入效果的反向证据

### 本项目上游的直接反馈

| 时间（UTC） | 来源 | 观察与边界 |
| --- | --- | --- |
| 2026-09-21 01:56 | [#144 用户 mik-myp](https://github.com/zyycn/codex-proxy-rs/issues/144#issuecomment-5754415538) | Plus 与 Pro 刷票成功后仍返回 Luna，旧实验版及 #195 作者版本均有该现象；属于一手使用反馈 |
| 2026-09-21 06:18 | [#144 原实现作者 chenyanshan](https://github.com/zyycn/codex-proxy-rs/issues/144#issuecomment-5756208327) | 明确说「OpenAI 已经封掉 292 了」；该条没有附服务端变更或对照数据，不视作 OpenAI 官方声明 |
| 2026-09-23 02:53 | [#227 用户 raynax](https://github.com/zyycn/codex-proxy-rs/pull/227#issuecomment-5788159165) | 报告「turn state length 已经变成了 780，292 大法失效了」；没有采样规模，不能认定所有账号/模型都统一变成 780 |

如果 780 长度在当前部署出现，默认 200～600 或指定 `[292,312]` 的本地准入条件会拒绝它。这可以解释一类刷新失败，但将 780 加入允许列表，只改变本地接受条件，不能证明注入重新有效。

### 独立的官方仓库用户报告

2026-09-22，OpenAI Codex [#46632 的跟进评论](https://github.com/openai/codex/issues/46632#issuecomment-5774312472) 报告：

- 同一账号约 14 小时记录 926 个 `response.created` 事件，请求主要为 `gpt-6-astra`，响应模型字段多数为 Luna/Sol。
- 292/312 字符 State 与响应模型字段有相关性，但报告中存在 292 而该时段没有 Astra 的例外。
- 约 70 个 flow 的注入/对照实验没有观察到 State 重放恢复 Astra 的效果。
- 作者将结论限定为 **same-flow replay**，并说明原始脱敏证据包可通过反馈渠道提供。

**边界：**这是普通用户的一手报告，不是 OpenAI 官方结论。正文虽称 randomized A/B，也描述了交替分配；本次未取得原始实验分配和数据包独立复核，不能当成严格随机对照试验。实验也不是本 Fork 全部采集策略的等价测试。

同问题 [9 月 20 日评论](https://github.com/openai/codex/issues/46632#issuecomment-5750813814)、[9 月 23 日评论](https://github.com/openai/codex/issues/46632#issuecomment-5787835762) 有其他用户独立报告请求 Astra、响应模型字段为 Luna。
另一个 [#47015](https://github.com/openai/codex/issues/47015) 提供经过适配器的请求/响应模型不一致记录，并明确承认不能排除适配器或标签问题。

本次只将这些记录解释为「响应元数据不一致」，不把 `response.model` 当作对真实推理权重的独立证明。

## 3. 已落地、值得参考的改进

| 来源 | 已确认改动 | 对本项目的意义 |
| --- | --- | --- |
| [OpenAI #44489](https://github.com/openai/codex/pull/44489)，2026-09-10 合并 | 账号归属变化时重建 WS，清除旧增量响应状态和 turn-state，发送完整输入；覆盖轮内与跨轮换号测试 | 核对账号/凭据切换边界，避免用另一个账号的连接、续接句柄或 State |
| [OpenAI #28002](https://github.com/openai/codex/pull/28002)，2026-06-13 合并 | 同一逻辑 turn 的压缩与采样共享 State；首次值生效 | 续接、压缩和传输切换要保持同一 turn 的合同，而不是另建后台票据覆盖 |
| [oh-my-pi #9279](https://github.com/can1357/oh-my-pi/pull/9279)，2026-08-22 合并 | 修复 State 被后续响应覆盖、重试/完整上下文回放时丢失、压缩阶段错用；按凭据、后端、模型与 Lite 模式隔离 | 有实现和回归测试可参考，但不构成恢复模型能力的证据 |

这些时间早于本次提问，不能包装为 9 月 23 日出现的新采集办法。

另有 [OpenAI #30536](https://github.com/openai/codex/issues/30536) 自 6 月起报告 WS 已成功升级但缺少 State header/metadata；截至检查仍 open，评论中没有维护者确认废弃协议。该问题说明「连通成功」「收到 State」「推理成功」需要分别观察。

### 最新候选：#239 模型差异驱动的调度与恢复

[zyycn/codex-proxy-rs #239](https://github.com/zyycn/codex-proxy-rs/issues/239) 创建于 **2026-09-23 03:04 UTC**，仍 open：

- 作者在 10 个 Pro 的账号池观察到请求 Astra、HTTP 200、返回模型字段为 Luna；并记录实际命中账号，检查模型映射、额度、套餐和凭据新鲜度。
- 提案按管理员配置的模型档位表比较实际发往上游的模型和返回模型，窗口内达到阈值则下线账号。
- 冷却到期只允许探测；完整请求成功且返回模型不再降档，才重新上线。不是按时间无条件恢复。
- 有 [Fork 实现提交 `4bb4fe99`](https://github.com/jiahao6635/codex-proxy-rs/commit/4bb4fe9905d63c0ec0b23492a985c08738a1e6be)，但尚不是已经合入的上游修复。

**需要调整/验证：**作者当前实现整账号冻结，且 PostgreSQL/Redis 集成未跑。维护者此前在 [#167](https://github.com/zyycn/codex-proxy-rs/issues/167#issuecomment-5741586909) 建议使用智能调度的「账号＋模型」评分。同一账号可能仅 Astra 异常而 Sol 正常，按账号＋模型隔离更贴合问题，也能避免整账号冻结过度减少容量。小账号池的行为、未知模型/缺失返回模型、模型映射和恢复标准需先评估，不能直接当作已验证方案移植。

作者在 #239 中以同 `/24` 网段的代理作为排除出口 IP 的依据，这不是严格同一出口对照，故本记录不采用其「已排除 IP 因素」的强结论。

### 其他候选

- [#227](https://github.com/zyycn/codex-proxy-rs/pull/227)：open、未合并，新增恰好 312 字节时阻断交付和长度展示。PR 自身承认长度/质量是经验关系而非官方合同。780 报告就出现在该 PR 下，因此不建议把 312 硬编码为降档事实。
- [#208](https://github.com/zyycn/codex-proxy-rs/pull/208)：open、未合并，为连接测试展示上游返回模型，并增加可选 ModelTrace 相似度探测。前者有直接诊断价值；后者作者明确标注是封闭候选集合中的相似度归因，不是模型身份认证，也未提供最终版本的线上识别准确率验证，不应独立驱动账号冻结。

## 4. 本 Fork 的具体差异

以本地提交 `ce3da970` 为基线：

- `backend/crates/providers/openai/src/session_manager/manager.rs:541–569`：探测要求 HTTP 200、响应头中存在满足长度/前缀条件的 State，随后释放响应，不消费 SSE 业务终态。
- 同文件 `:635–640`：以采集时间加固定 3600 秒作为本地过期时刻，按账号与模型存入票据。
- 同文件 `:701–738`：受管理模型缺票时不可用，有票时覆盖请求头/正文 metadata 中的 State。
- `backend/crates/providers/openai/src/credential/selector.rs:406–422`：开启全局和账号 State 开关后，选定模型缺票会在选号阶段被排除。
- `backend/crates/providers/openai/src/provider/mod.rs:560`：发送前的重写也受全局开关控制。

**推断：**成功刷新只证明拿到了符合本地条件的响应头，不能证明后续完整推理成功、上游接受跨轮复用或目标模型能力恢复。持续拿不到符合条件的票据还会通过 fail-closed 降低账号池可用性。这是源码层面的影响分析，尚未在用户线上实例复现。

先前的新模型 State 测试使用模拟上游，只验证模型配置、票据隔离和重写链路；不应把它解释为当前线上采集策略有效。

## 5. 建议的下一步

1. **先通过现有全局开关停用实验性 State 重写进行对照。** 不删除账号原有配置；比较正常转发与当前策略的完整请求成功率和响应元数据。该操作尚未执行。
2. **优先恢复正常 turn-state 传递合同。** 客户端已有正确 State 时按账号/turn 边界透传；核查重试、换号、WS 重连和压缩路径。网关不能凭账号和模型猜测逻辑 turn，也不宜与客户端维护两套互相覆盖的生命周期。
3. **利用现有模型观测。** 使用统计详情已有「上游返回模型」（`upstreamResponseModel`），列表辅助信息也展示返回模型。先核对真正发往上游的模型与返回字段，再考虑增加差异筛选/汇总，避免把配置映射误算成上游差异。
   优先评估 #239 的思路，并结合 #167 改为「账号＋模型」粒度；先完成观测与告警，再确定自动降权或冻结的策略。
4. **把可用性拆成可验证事实。** 区分是否收到 State、是否匹配本地长度规则、完整推理是否完成、响应模型是否匹配、是否出现明确容量/额度错误；不以长度作健康或能力结论。
5. **保留可控的正常重试与限流退避。** 本轮已同步的快照冲突、安装 ID 和并发修复有独立价值，但没有证据表明它们能改变服务端模型分配。

不建议仅增加允许长度、扩大采集并发或缩短刷新间隔，就宣称修复了当前问题。

## 调查限制

- 未读取真实凭据、请求转储或生产数据库，未对用户账号发起上游实验。
- 未取得 OpenAI 服务端实现，也未独立复核社区报告的原始证据包。
- 未修改产品代码或运行配置，本文件是研究记录。
