import { RefObject } from "react";

export const TodoAdd = ({ buttonText, inputEl, handleAddTodoListItem }: { buttonText: string; inputEl: RefObject<HTMLTextAreaElement>; handleAddTodoListItem: () => void }) => {
    return (
        <>
        <textarea ref={inputEl} cols={32} rows={4} />
        <button onClick={handleAddTodoListItem}>{buttonText}</button>
        </>
    );
};
