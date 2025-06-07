

测验目标：
熟练掌握 Anchor 框架中 PDA (Program Derived Address) 的使用、账户约束、指令处理函数的编写以及错误处理。

测验任务：

1.  实现 `remove_task` 功能:
    - **描述**: 用户应该能够从其待办事项列表中移除一个指定的任务。
    - **要求**:
        - 在 `solana_todo_list` 模块中实现 `pub fn remove_task(ctx: Context<RemoveTask>, task_index: u8) -> Result<()>` 函数。
        - 定义 `RemoveTask<'info>` 账户上下文结构体，并为其 `todo_list` 账户设置正确的约束（包括 `mut`, `seeds`, `bump`, 以及权限校验 `has_one = owner`）。
        - 在函数逻辑中，处理 `task_index` 无效（例如超出当前任务列表范围）的情况，返回自定义错误 `InvalidTaskIndex`。
        - 从 `todo_list.tasks` 向量中移除指定索引的任务。
        - 发出一条消息，指明哪个任务被移除了以及剩余任务数量。

2.  实现 `update_task_description` 功能:
    - **描述**: 用户应该能够更新列表中某个已存在任务的描述。
    - **要求**:
        - 在 `solana_todo_list` 模块中实现 `pub fn update_task_description(ctx: Context<UpdateTaskDescription>, task_index: u8, new_description: String) -> Result<()>` 函数。
        - 定义 `UpdateTaskDescription<'info>` 账户上下文结构体，并为其 `todo_list` 账户设置正确的约束。
        - 在函数逻辑中，处理 `task_index` 无效的情况，返回 `InvalidTaskIndex`。
        - 校验 `new_description` 的长度不能超过 `MAX_DESCRIPTION_LENGTH`，如果超过则返回 `DescriptionTooLong`。
        - 更新 `todo_list.tasks[task_index]` 的 `description` 字段为 `new_description`。
        - 发出一条消息，指明哪个任务的描述被更新了。

3.  (可选挑战) 实现 `close_list` 功能:
    - **描述**: 用户应该能够关闭（删除）其整个 `TodoList` PDA，并将账户的租金返还给所有者。
    - **要求**:
        - 在 `solana_todo_list` 模块中实现 `pub fn close_list(ctx: Context<CloseList>) -> Result<()>` 函数。
        - 定义 `CloseList<'info>` 账户上下文结构体。
        - 在其 `todo_list` 账户上使用 `#[account(mut, close = owner, seeds = [...], bump = ...)]` 约束。确保只有列表所有者可以关闭列表。
        - 函数体可以非常简单，因为 `close` 约束会处理大部分工作。
        - 发出一条消息，指明列表已被关闭。

--- 框架代码开始 ---