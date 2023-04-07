import React, { useState, useEffect } from "react";
import { ulid } from "ulid";

import * as todoData from "../apis/todos";
import { Todo } from "../types/Todo";

export const useTodo = () => {
  const [todoList, setTodoList] = useState<Todo[]>([]);

  useEffect(() => {
    todoData.getAllTodosData().then((todo) => {
      console.log(...todo);
      setTodoList([...todo].reverse());
    });
  }, []);

  // todoのdoneを反転させる
  const toggleTodoListItemStatus = (id: string, done: boolean) => {
    // todoListから、idが一致する1件を取り出す
    const todoItem = todoList.find((item: Todo) => item.id === id);
    // doneを反転させて、新たなitemを作成
    const newTodoItem: Todo = { ...todoItem!, done: !done };
    // サーバに更新API呼ぶ
    todoData.updateTodoData(id, newTodoItem).then((updatedTodo) => {
      // 成功したら、todoListを更新。idが一致しているものを、サーバーから返ってきたupdatedTodoで更新する
      const newTodoList = todoList.map((item) => (item.id !== updatedTodo.id ? item : updatedTodo));
      // 新しいtodoListをstateにセットする
      setTodoList(newTodoList);
    });
  };

  const addTodoListItem = (todoContent: string) => {
    // あたらしいitemを作成する
    const newTodoItem = { id: ulid(), content: todoContent, done: false };

    // サーバーの追加APIを呼ぶ
    todoData.addTodoData(newTodoItem).then((addTodo) => {
      // addTodoをtodoListに追加してstateにセットする
      setTodoList([addTodo, ...todoList]);
    });
  };

  const deleteTodoListItem = (id: string) => {
    // サーバーの削除APIを呼ぶ
    todoData.deleteTodoData(id).then((deletedid) => {
      const newTodoList = todoList.filter((item) => item.id !== deletedid);
      // 1件削除された新しいtodoListに追加してstateにセットする
      setTodoList(newTodoList);
    });
  };

  // 作成した関数を返す
  return { todoList, toggleTodoListItemStatus, addTodoListItem, deleteTodoListItem };
};
