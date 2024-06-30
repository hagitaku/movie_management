import style from "./TextBoxArea.module.css";
import { UseFormRegisterReturn} from "react-hook-form";

type TextBoxAreaProps = {
    title: string;
    name: string;
    rows?: number;
    cols?: number;
    register: UseFormRegisterReturn;
}
const TextBoxArea = (props : TextBoxAreaProps) =>{
    const {
        title,
        name,
        rows,
        cols,
        register
    } = props;
    return(
        <div className={style.form_group}>
            <label htmlFor={name}>{ title }</label>
            <div className={style.form_input}>
                <textarea {...register} rows={rows || 6} cols={cols || 40}></textarea>
            </div>
        </div>
    )
}
export default TextBoxArea;