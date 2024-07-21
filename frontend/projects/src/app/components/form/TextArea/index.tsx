import style from "./style.module.css";
import { UseFormReturn } from "react-hook-form";
import { FieldValues, Path } from "react-hook-form";

type TextAreaProps<T extends FieldValues> = {
    title: string;
    name: Path<T>;
    rows?: number;
    cols?: number;
    form: UseFormReturn<T>;
}
const TextArea = <T extends FieldValues>(props : TextAreaProps<T>) =>{
    const {
        title,
        name,
        rows,
        cols,
        form,
    } = props;
    const { register } = form;
    return(
        <div className={style["text-area-field"]}>
            <label htmlFor={name}>{ title }</label>
            <div className={style["text-area-input"]}>
                <textarea {...register(name)} rows={rows || 6} cols={cols || 40}></textarea>
            </div>
        </div>
    )
}
export default TextArea;
