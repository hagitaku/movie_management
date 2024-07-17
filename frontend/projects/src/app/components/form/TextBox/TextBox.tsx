import { UseFormReturn } from "react-hook-form";
import type { FieldValues, Path } from "react-hook-form";
import style from "./TextBox.module.css";

type TextBoxProps<T extends FieldValues> = {
    title: string;
    name: Path<T>;
    form: UseFormReturn<T>;
};
const TextBox = <T extends FieldValues>(props : TextBoxProps<T>) =>{
    const{
        title,
        name,
        form,
    } = props;
    const { register } = form;
    return(
        <div className={style.textBoxField}> 
            <label htmlFor={name}>{ title }</label>
            <div className={style.textBoxInput}>
                <input id={name} {...register(name)} type="text"></input>
            </div>
        </div>
    )
}
export default TextBox;
