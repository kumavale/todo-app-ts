import { RefObject } from "react";

export const TodoAdd = ({ buttonText, inputEl, handleAddTodoListItem }: { buttonText: string; inputEl: RefObject<HTMLTextAreaElement>; handleAddTodoListItem: () => void }) => {
    return (
        <>
        <div id="add-todo">
            <textarea ref={inputEl} rows={4} placeholder={"次に何をするべき？"} />
            <br />
            <button onClick={handleAddTodoListItem}>{buttonText}</button>
        </div>
        </>
    );
};
