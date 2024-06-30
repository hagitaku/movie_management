import { UseFormRegisterReturn} from "react-hook-form";
import style from "./TextBox.module.css";

type TextBoxProps={
    title: string;
    name: string;
    register: UseFormRegisterReturn;
};
const TextBox = (props : TextBoxProps) =>{
    const{
        title,
        name,
        register
    } = props;
    return(
        <div className={style.form_group}> 
            <label htmlFor={name}>{ title }</label>
            <div className={style.form_input}>
                <input id={name} {...register} type="text"></input>
            </div>
        </div>
    )
}
export default TextBox;