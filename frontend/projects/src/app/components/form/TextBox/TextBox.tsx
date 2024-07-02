import { UseFormReturn} from "react-hook-form";
import style from "./TextBox.module.css";

type TextBoxProps={
    title: string;
    name: string;
    form: UseFormReturn;
};
const TextBox = (props : TextBoxProps) =>{
    const{
        title,
        name,
        form,
    } = props;
    const { register } = form;
    return(
        <div className={style.form_group}> 
            <label htmlFor={name}>{ title }</label>
            <div className={style.form_input}>
                <input id={name} {...register(name)} type="text"></input>
            </div>
        </div>
    )
}
export default TextBox;
