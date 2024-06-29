import style from "./TextBox.module.css";

type TextBoxProps = {
    textBoxName: string;
    textBoxLabel: string;
    textBoxType: string;
}
const TextBox = (props : TextBoxProps) =>{
    return(
        <div className={style.form_group}>
            <label htmlFor="name">{ props.textBoxName }</label>
            <div className={style.form_input}>
                <input name={props.textBoxLabel} type={props.textBoxType}></input>
            </div>
        </div>
    )
}
export default TextBox;