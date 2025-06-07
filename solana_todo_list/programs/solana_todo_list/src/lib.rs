use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// 定义常量
const MAX_DESCRIPTION_LENGTH: usize = 100;

#[program]
pub mod solana_todo_list {
    use super::*;

    // 创建待办事项列表
    pub fn create_list(ctx: Context<CreateList>) -> Result<()> {
        let todo_list = &mut ctx.accounts.todo_list;
        todo_list.owner = ctx.accounts.owner.key();
        todo_list.tasks = Vec::new();
        todo_list.bump = *ctx.bumps.get("todo_list").unwrap();

        msg!("Todo list created for {}", todo_list.owner);
        Ok(())
    }

    // 添加任务到列表
    pub fn add_task(ctx: Context<AddTask>, description: String) -> Result<()> {
        // 验证描述长度
        if description.len() > MAX_DESCRIPTION_LENGTH {
            return err!(ErrorCode::DescriptionTooLong);
        }

        let todo_list = &mut ctx.accounts.todo_list;
        
        // 创建新任务
        let task = Task {
            description,
            completed: false,
        };

        // 添加到列表
        todo_list.tasks.push(task);

        msg!("Task added to the list. Total tasks: {}", todo_list.tasks.len());
        Ok(())
    }

    // 移除任务
    pub fn remove_task(ctx: Context<RemoveTask>, task_index: u8) -> Result<()> {
        let todo_list = &mut ctx.accounts.todo_list;
        
        // 检查任务索引是否有效
        if task_index as usize >= todo_list.tasks.len() {
            return err!(ErrorCode::InvalidTaskIndex);
        }
        
        // 获取要移除的任务描述（用于消息）
        let task_description = todo_list.tasks[task_index as usize].description.clone();
        
        // 移除任务
        todo_list.tasks.remove(task_index as usize);
        
        msg!("Task '{}' removed. Remaining tasks: {}", task_description, todo_list.tasks.len());
        Ok(())
    }

    // 更新任务描述
    pub fn update_task_description(ctx: Context<UpdateTaskDescription>, task_index: u8, new_description: String) -> Result<()> {
        // 验证描述长度
        if new_description.len() > MAX_DESCRIPTION_LENGTH {
            return err!(ErrorCode::DescriptionTooLong);
        }
        
        let todo_list = &mut ctx.accounts.todo_list;
        
        // 检查任务索引是否有效
        if task_index as usize >= todo_list.tasks.len() {
            return err!(ErrorCode::InvalidTaskIndex);
        }
        
        // 获取旧描述（用于消息）
        let old_description = todo_list.tasks[task_index as usize].description.clone();
        
        // 更新描述
        todo_list.tasks[task_index as usize].description = new_description.clone();
        
        msg!("Task description updated from '{}' to '{}'", old_description, new_description);
        Ok(())
    }

    // 关闭待办事项列表
    pub fn close_list(ctx: Context<CloseList>) -> Result<()> {
        msg!("Todo list for {} has been closed", ctx.accounts.owner.key());
        Ok(())
    }
}

// 任务结构
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

// 待办事项列表账户
#[account]
pub struct TodoList {
    pub owner: Pubkey,
    pub tasks: Vec<Task>,
    pub bump: u8,
}

// 创建列表的上下文
#[derive(Accounts)]
pub struct CreateList<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 4 + (MAX_DESCRIPTION_LENGTH + 1) * 10 + 1, // 预留10个任务的空间
        seeds = [b"todo-list", owner.key().as_ref()],
        bump
    )]
    pub todo_list: Account<'info, TodoList>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// 添加任务的上下文
#[derive(Accounts)]
pub struct AddTask<'info> {
    #[account(
        mut,
        seeds = [b"todo-list", owner.key().as_ref()],
        bump = todo_list.bump,
        has_one = owner
    )]
    pub todo_list: Account<'info, TodoList>,
    
    pub owner: Signer<'info>,
}

// 移除任务的上下文
#[derive(Accounts)]
pub struct RemoveTask<'info> {
    #[account(
        mut,
        seeds = [b"todo-list", owner.key().as_ref()],
        bump = todo_list.bump,
        has_one = owner
    )]
    pub todo_list: Account<'info, TodoList>,
    
    pub owner: Signer<'info>,
}

// 更新任务描述的上下文
#[derive(Accounts)]
pub struct UpdateTaskDescription<'info> {
    #[account(
        mut,
        seeds = [b"todo-list", owner.key().as_ref()],
        bump = todo_list.bump,
        has_one = owner
    )]
    pub todo_list: Account<'info, TodoList>,
    
    pub owner: Signer<'info>,
}

// 关闭列表的上下文
#[derive(Accounts)]
pub struct CloseList<'info> {
    #[account(
        mut,
        close = owner,
        seeds = [b"todo-list", owner.key().as_ref()],
        bump = todo_list.bump,
        has_one = owner
    )]
    pub todo_list: Account<'info, TodoList>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
}

// 错误代码
#[error_code]
pub enum ErrorCode {
    #[msg("描述长度超过最大限制")]
    DescriptionTooLong,
    
    #[msg("无效的任务索引")]
    InvalidTaskIndex,
}