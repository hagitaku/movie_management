import style from "./TextBoxArea.module.css";

type TextBoxAreaProps = {
    textBoxAreaName: string;
    textBoxAreaLabel: string;
    textBoxAreaRows: number;
    textBoxAreaCols: number;
}
const TextBoxArea = (props : TextBoxAreaProps) =>{
    return(
        <div className={style.form_group}>
            <label htmlFor="name">{ props.textBoxAreaName }</label>
            <div className={style.form_input}>
                <textarea name={props.textBoxAreaLabel} rows={props.textBoxAreaRows} cols={props.textBoxAreaCols}></textarea>
            </div>
        </div>
    )
}
export default TextBoxArea;