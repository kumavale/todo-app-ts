import { Todo } from "../types/Todo";

// 1つのTodo、内容と移動・削除ボタン
export const TodoItem = ({ todo, toggleTodoListItemStatus, deleteTodoListItem }: { todo: Todo; toggleTodoListItemStatus: any; deleteTodoListItem: any }) => {
  // onClickイベントが発生したら、useTodoフックを呼び出す
  const handleToggleTodoListItemStatus = () => toggleTodoListItemStatus(todo.id, todo.done);
  const handleDeleteTodoListItem = () => deleteTodoListItem(todo.id);

  return (
    <>
      {todo.content}
      <button onClick={handleToggleTodoListItemStatus}>{todo.done ? "未完了リストへ" : "完了リストへ"}</button>
      <button onClick={handleDeleteTodoListItem}>削除</button>
    </>
  );
};
