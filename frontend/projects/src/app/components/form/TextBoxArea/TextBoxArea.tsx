import style from "./TextBoxArea.module.css";
import { UseFormReturn } from "react-hook-form";
import { FieldValues, Path } from "react-hook-form";

type TextBoxAreaProps<T extends FieldValues> = {
    title: string;
    name: Path<T>;
    rows?: number;
    cols?: number;
    form: UseFormReturn<T>;
}
const TextBoxArea = <T extends FieldValues>(props : TextBoxAreaProps<T>) =>{
    const {
        title,
        name,
        rows,
        cols,
        form,
    } = props;
    const { register } = form;
    return(
        <div className={style.textBoxAreaField}>
            <label htmlFor={name}>{ title }</label>
            <div className={style.textBoxAreaInput}>
                <textarea {...register(name)} rows={rows || 6} cols={cols || 40}></textarea>
            </div>
        </div>
    )
}
export default TextBoxArea;
