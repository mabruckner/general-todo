use todo_shared::Task;
use yew::{platform::{pinned::oneshot, spawn_local}, prelude::*};

use crate::{requests::{add_task, get_all_tasks}, util::textinput_callback};

#[derive(Properties, PartialEq)]
pub struct TaskChangeProps {
    pub task_changed: Callback<()>
}

#[component]
pub fn TaskAdd(props: &TaskChangeProps) -> Html {
    let content= use_state(String::default);
    let on_change = textinput_callback(content.clone());
    let on_submit = {
        let content = content.clone();
        let task_changed = props.task_changed.clone();
        move |_| {
            let content = content.clone();
            let task_changed = task_changed.clone();
            spawn_local(async move {
                let res = add_task((*content).clone()).await;
                if let Ok(_task) = res {
                    content.set("".into());
                    task_changed.emit(());
                }
            });
        }
    };
    html! {
        <form class="row mb-3">
            <div class="col-auto">
                <input type="text" class="form-control" placeholder="task content" onchange={on_change} value={ (*content).clone() }/>
            </div>
            <div class="col-auto">
                <button type="button" class="btn btn-primary mb-3" onclick={on_submit}>{"ADD"}</button>
            </div>
        </form>
    }
}

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    pub task_list: UseStateHandle<Vec<Task>>
}

#[component]
pub fn TaskList(props: &TaskListProps) -> Html {
    html! {
        <div>
        for task in &(*props.task_list) {
            <div class="card">
                <div class="card-body">
                    { &task.contents }
                </div>
            </div>
        }
        </div>
    }
}

#[component]
pub fn TaskView() -> Html {
    let task_list: UseStateHandle<Vec<Task>> = use_state(Vec::new);
    let task_changed = {
        let task_list = task_list.clone();
        Callback::from(move |_| {
            let task_list = task_list.clone();
        
            spawn_local(async move {
                let res = get_all_tasks().await;
                if let Ok(tasks) = res {
                    task_list.set(tasks)
                }
            });
        })
    };
    html! {
        <div class="container-md">
            <div class="card">
                <div class="card-body">
                    <TaskAdd task_changed={task_changed}/>
                    <TaskList task_list={task_list}/>
                </div>
            </div>
        </div>
    }

}