import { UseFormReturn } from "react-hook-form";
import type { FieldValues, Path } from "react-hook-form";
import style from "./style.module.css";

type TextBoxProps<T extends FieldValues> = {
    title: string;
    name: Path<T>;
    type?: "text" | "password";
    form: UseFormReturn<T>;
};
const TextBox = <T extends FieldValues>(props : TextBoxProps<T>) =>{
    const{
        title,
        name,
        type,
        form,
    } = props;
    const { register } = form;
    
    return(
        <div className={style["text-box-field"]}> 
            <label htmlFor={name}>{ title }</label>
            <div className={style["text-box-input"]}>
                <input id={name} {...register(name)} type={type || "text"}></input>
            </div>
        </div>
    )
}
export default TextBox;
