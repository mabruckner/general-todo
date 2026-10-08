use todo_shared::Task;
use yew::{platform::{pinned::oneshot, spawn_local}, prelude::*};

use crate::{requests::{add_task, get_all_tasks, remove_task, update_task}, util::{textinput_callback, textinput_value_from_event}};

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
        Callback::from(move |evt: SubmitEvent| {
            evt.prevent_default();
            let content = content.clone();
            let task_changed = task_changed.clone();
            spawn_local(async move {
                let res = add_task((*content).clone()).await;
                if let Ok(_task) = res {
                    content.set("".into());
                    task_changed.emit(());
                }
            });
        })
    };
    html! {
        <form class="row mb-3" onsubmit={on_submit.clone()}>
            <div class="col-auto">
                <input type="text" class="form-control" placeholder="task content" onchange={on_change} value={ (*content).clone() }/>
            </div>
            <div class="col-auto">
                <button type="submit" class="btn btn-primary mb-3" >{"ADD"}</button>
            </div>
        </form>
    }
}

#[derive(Properties, PartialEq)]
pub struct TaskEntryProps {
    pub id: i32,
    pub contents: String,
    pub completed: bool,
    pub task_changed: Callback<()>,
}

#[component]
pub fn TaskEntry(props: &TaskEntryProps) -> Html {
    let id = props.id;
    let on_delete = {
        let task_changed = props.task_changed.clone();
        Callback::from(move |_| {
            let task_changed = task_changed.clone();
            spawn_local(async move {
                let res = remove_task(id).await;
                if let Ok(task) = res {
                    task_changed.emit(());
                }
            });
        })
    };
    let update = {
        let task_changed = props.task_changed.clone();
        Callback::from(move |task: Task|{
            let task_changed = task_changed.clone();
            spawn_local(async move {
                let res = update_task(task).await;
                if let Ok(task) = res {
                    task_changed.emit(());
                }
            });
        })
    };
    let on_toggle = {
        let toggled_task = Task {
            id: props.id,
            contents: props.contents.clone(),
            completed: !props.completed
        };
        let update = update.clone();
        Callback::from(move|_| {
            update.emit(toggled_task.clone());
        })
    };
    let on_text_change = {
        let update = update.clone();
        let base_task = Task {
            id: props.id,
            contents: props.contents.clone(),
            completed: props.completed
        };
        Callback::from(move|e:Event| {
            let value = textinput_value_from_event(e);
            let task = Task {
                contents: value,
                .. base_task.clone()
            };
            update.emit(task);
        })
    };
    html! {
        <div class="card">
            <div class="card-body">
                <div class="row mb-3">
                    <div class="col-auto">
                        <input type="checkbox" class="form-check-input" onchange={on_toggle} checked={props.completed}/>
                    </div>
                    <div class="col-auto">
                        <input type="text" class="form-control" onchange={ on_text_change } value={ props.contents.clone() }/>
                    </div>
                    
                    <div class="col-auto">
                        <button type="button" class="btn-close" onclick={on_delete}></button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    pub task_list: UseStateHandle<Vec<Task>>,
    pub task_changed: Callback<()>
}

#[component]
pub fn TaskList(props: &TaskListProps) -> Html {
    html! {
        <>
        for task in &(*props.task_list) {
            <TaskEntry id={task.id} contents={task.contents.clone()} completed={task.completed} task_changed={props.task_changed.clone()}/>
        }
        </>
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
    {
        let task_changed = task_changed.clone();
        use_state(|| {
            task_changed.emit(());
        });
    };
    html! {
        <div class="container-md">
            <div class="card">
                <div class="card-body">
                    <TaskAdd task_changed={task_changed.clone()}/>
                    <TaskList task_list={task_list} task_changed={task_changed.clone()}/>
                </div>
            </div>
        </div>
    }

}