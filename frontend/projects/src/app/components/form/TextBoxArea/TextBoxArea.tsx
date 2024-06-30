import style from "./TextBoxArea.module.css";
import { UseFormRegister } from "react-hook-form";

type TextBoxAreaProps = {
    title: string;
    name: string;
    rows?: number;
    cols?: number;
}
const TextBoxArea = (props : TextBoxAreaProps) =>{
    const {
        title,
        name,
        rows,
        cols
    } = props;
    return(
        <div className={style.form_group}>
            <label htmlFor={name}>{ title }</label>
            <div className={style.form_input}>
                <textarea name={name} rows={rows || 6} cols={cols || 40}></textarea>
            </div>
        </div>
    )
}
export default TextBoxArea;