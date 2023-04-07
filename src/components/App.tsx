import React, { useRef } from "react";

import { useTodo } from "../hooks/useTodo";
import { Todo } from "../types/Todo";
import { TodoAdd } from "./TodoAdd";
import { TodoList } from "./TodoList";
import { TodoTitle } from "./TodoTitle";

function App() {
  // カスタムフックから必要な変数を取得
  const { todoList, toggleTodoListItemStatus, addTodoListItem, deleteTodoListItem } = useTodo();

  const inputEl = useRef<HTMLTextAreaElement>(null);

  const handleAddTodoListItem = () => {
    if (inputEl.current?.value === "") {
      return;
    }
    addTodoListItem(inputEl.current!.value);
    inputEl.current!.value = "";
  };

  // 未完了リスト
  const incompletedList = todoList.filter((todo: Todo) => !todo.done);
  // 完了リスト
  const completedList = todoList.filter((todo: Todo) => todo.done);

  return (
    <>
      <TodoTitle title="TODO進捗管理" as="h1" />
      <TodoAdd buttonText="+ TODOを追加" inputEl={inputEl} handleAddTodoListItem={handleAddTodoListItem} />
      <TodoList todoList={incompletedList} toggleTodoListItemStatus={toggleTodoListItemStatus} deleteTodoListItem={deleteTodoListItem} title="未完了TODOリスト" as="h2" />
      <TodoList todoList={completedList} toggleTodoListItemStatus={toggleTodoListItemStatus} deleteTodoListItem={deleteTodoListItem} title="完了TODOリスト" as="h2" />
    </>
  );
}

export default App;
